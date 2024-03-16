use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Serialize;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod components;
mod rpc;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "frontend=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/_healthz", get(health_handler))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn home_handler() -> Result<Html<String>, AppError> {
    tracing::debug!("GET /");
    let mut buf = String::with_capacity(100000);
    let home_page = components::home::HomePage {};
    match home_page.write_page(&mut buf).await {
        Ok(_) => Ok(Html(buf)),
        Err(e) => Err(AppError::ServerError(e)),
    }
    /*
    match components::home::generate_page(&mut buf).await {
        Ok(_) => Ok(Html(buf)),
        Err(e) => Err(AppError::ServerError(e)),
    }
    */
    //components::home::generate_page(&mut buf).await?;
    //Ok(Html(buf))
}

async fn health_handler() -> &'static str {
    "OK"
}

// Create our own JSON extractor by wrapping `axum::Json`. This makes it easy to override the
// rejection and provide our own which formats errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

// The kinds of errors we can hit in our application.
enum AppError {
    // The request body contained invalid JSON
    ServerError(&'static str),
}

// Tell axum how `AppError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::ServerError(rejection) => {
                // This error is caused by bad user input so don't log it
                (StatusCode::INTERNAL_SERVER_ERROR, rejection.to_owned())
            }
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}
