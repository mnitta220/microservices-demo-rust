use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use pages::page::PageProps;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_cookies::CookieManagerLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod components;
mod handlers;
mod pages;
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
        .route("/", get(handlers::home_handler))
        .route("/product/:id", get(handlers::product_handler))
        .route("/setCurrency", post(handlers::set_currency_handler))
        .route(
            "/cart",
            get(handlers::view_cart_handler).post(handlers::add_to_cart_handler),
        )
        .route("/cart/empty", post(handlers::empty_cart_handler))
        .route("/cart/checkout", post(handlers::place_order_handler))
        .route("/_healthz", get(handlers::health_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .layer(CookieManagerLayer::new())
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
