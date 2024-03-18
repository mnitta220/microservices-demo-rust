use axum::{
    extract::{Form, FromRequest},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod components;
mod rpc;

lazy_static! {
    static ref CURRENCY_LOGO: HashMap<&'static str, &'static str> = {
        let currency_logo = HashMap::from([
            ("USD", "$"),
            ("EUR", "€"),
            ("CAD", "$"),
            ("JPY", "¥"),
            ("GBP", "£"),
            ("TRY", "₺"),
        ]);
        currency_logo
    };
    static ref WHITELISTED_CURRENCIES: HashMap<&'static str, bool> = {
        let whitelisted_currencies = std::collections::HashMap::from([
            ("USD", true),
            ("EUR", true),
            ("CAD", true),
            ("JPY", true),
            ("GBP", true),
            ("TRY", true),
        ]);
        whitelisted_currencies
    };
}

pub struct PageProps {
    pub user_currency: String,
}

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
        .route("/setCurrency", post(set_currency_handler))
        .route("/_healthz", get(health_handler))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn home_handler(jar: CookieJar) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /");

    let cur: Option<String> = jar
        .get("shop_currency")
        .map(|cookie| cookie.value().to_owned());
    let cur = match cur {
        Some(c) => c,
        None => "USD".to_string(),
    };
    let mut page_props = PageProps { user_currency: cur };

    let mut buf = String::with_capacity(100000);
    let home_page = components::home::HomePage {};
    match home_page.write_page(&mut buf, &mut page_props).await {
        Ok(_) => Ok(Html(buf)),
        Err(e) => Err(AppError::ServerError(e)),
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    currency_code: String,
}

async fn set_currency_handler(
    jar: CookieJar,
    Form(input): Form<Input>,
) -> Result<(CookieJar, Redirect), StatusCode> {
    tracing::debug!("POST /setCurrency {}", input.currency_code);

    Ok((
        jar.add(
            Cookie::parse(format!(
                "shop_currency={}; Max-Age={}",
                input.currency_code,
                60 * 60 * 48
            ))
            .unwrap(),
        ),
        Redirect::to("/"),
    ))
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
