use axum::{
    error_handling::HandleErrorLayer,
    extract::{Form, Path},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
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

pub enum PageType {
    Home,
    Product,
}

pub struct PageProps {
    pub page_type: PageType,
    pub session_id: String,
    pub request_id: String,
    pub user_currency: String,
    pub product_id: Option<String>,
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
        .route("/product/:id", get(product_handler))
        .route("/setCurrency", post(set_currency_handler))
        .route("/_healthz", get(health_handler))
        .nest_service("/static", ServeDir::new("static"))
        // Add middleware to all routes
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
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn home_handler(jar: CookieJar) -> Result<(CookieJar, Html<String>), AppError> {
    tracing::debug!("GET /");

    handler_sub(jar, PageType::Home, None).await
}

async fn product_handler(
    jar: CookieJar,
    Path(id): Path<String>,
) -> Result<(CookieJar, Html<String>), AppError> {
    tracing::debug!("GET /product {}", id);

    handler_sub(jar, PageType::Product, Some(id)).await
}

async fn handler_sub(
    jar: CookieJar,
    page_type: PageType,
    product_id: Option<String>,
) -> Result<(CookieJar, Html<String>), AppError> {
    let cur: Option<String> = jar
        .get("shop_currency")
        .map(|cookie| cookie.value().to_owned());
    let cur = match cur {
        Some(c) => c,
        None => "USD".to_string(),
    };
    let session_id: Option<String> = jar
        .get("shop_session-id")
        .map(|cookie| cookie.value().to_owned());
    let (session_id, is_new_session) = match session_id {
        Some(s) => (s, false),
        None => {
            let id = Uuid::new_v4().to_string();

            (id, true)
        }
    };

    let mut page_props = PageProps {
        page_type: page_type,
        session_id: session_id,
        request_id: Uuid::new_v4().to_string(),
        user_currency: cur,
        product_id: product_id,
    };

    let mut buf = String::with_capacity(100000);
    let home_page = components::home::HomePage {};

    if let Err(e) = home_page.write_page(&mut buf, &mut page_props).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    if is_new_session {
        Ok((
            jar.add(
                Cookie::parse(format!(
                    "shop_session-id={}; Max-Age={}",
                    page_props.session_id,
                    60 * 60 * 48
                ))
                .unwrap(),
            ),
            Html(buf),
        ))
    } else {
        Ok((jar, Html(buf)))
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

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

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
