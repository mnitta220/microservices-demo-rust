use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use once_cell::sync::OnceCell;
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

static AD_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static CART_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static CHECKOUT_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static CURRENCY_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static PRODUCT_CATALOG_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static RECOMMENDATION_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static SHIPPING_SERVICE_ADDR: OnceCell<String> = OnceCell::new();

fn get_env() -> Result<()> {
    // get AD_SERVICE_ADDR env
    let addr = match std::env::var("AD_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get AD_SERVICE_ADDR"));
        }
    };
    // set AD_SERVICE_ADDR static
    if let Err(_) = AD_SERVICE_ADDR.set(addr) {
        return Err(anyhow::anyhow!("Failed to set AD_SERVICE_ADDR"));
    }

    // get CART_SERVICE_ADDR env
    let addr = match std::env::var("CART_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get CART_SERVICE_ADDR"));
        }
    };
    // set CART_SERVICE_ADDR static
    if let Err(_) = CART_SERVICE_ADDR.set(addr) {
        return Err(anyhow::anyhow!("Failed to set CART_SERVICE_ADDR"));
    }

    // get CHECKOUT_SERVICE_ADDR env
    let addr = match std::env::var("CHECKOUT_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get CHECKOUT_SERVICE_ADDR"));
        }
    };
    // set CHECKOUT_SERVICE_ADDR static
    if let Err(_) = CHECKOUT_SERVICE_ADDR.set(addr) {
        return Err(anyhow::anyhow!("Failed to set CHECKOUT_SERVICE_ADDR"));
    }

    // get CURRENCY_SERVICE_ADDR env
    let addr = match std::env::var("CURRENCY_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get CURRENCY_SERVICE_ADDR"));
        }
    };
    // set CURRENCY_SERVICE_ADDR static
    if let Err(_) = CURRENCY_SERVICE_ADDR.set(addr) {
        return Err(anyhow::anyhow!("Failed to set CURRENCY_SERVICE_ADDR"));
    }

    // get PRODUCT_CATALOG_SERVICE_ADDR env
    let addr = match std::env::var("PRODUCT_CATALOG_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err(anyhow::anyhow!(
                "Failed to get PRODUCT_CATALOG_SERVICE_ADDR"
            ));
        }
    };
    // set PRODUCT_CATALOG_SERVICE_ADDR static
    if let Err(_) = PRODUCT_CATALOG_SERVICE_ADDR.set(addr) {
        return Err(anyhow::anyhow!(
            "Failed to set PRODUCT_CATALOG_SERVICE_ADDR"
        ));
    }

    // get RECOMMENDATION_SERVICE_ADDR env
    let addr = match std::env::var("RECOMMENDATION_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get RECOMMENDATION_SERVICE_ADDR"));
        }
    };
    // set RECOMMENDATION_SERVICE_ADDR static
    if let Err(_) = RECOMMENDATION_SERVICE_ADDR.set(addr) {
        return Err(anyhow::anyhow!("Failed to set RECOMMENDATION_SERVICE_ADDR"));
    }

    // get SHIPPING_SERVICE_ADDR env
    let addr = match std::env::var("SHIPPING_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get SHIPPING_SERVICE_ADDR"));
        }
    };
    // set SHIPPING_SERVICE_ADDR static
    if let Err(_) = SHIPPING_SERVICE_ADDR.set(addr) {
        return Err(anyhow::anyhow!("Failed to set SHIPPING_SERVICE_ADDR"));
    }

    Ok(())
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

    if let Err(e) = get_env() {
        tracing::error!("failed to get env: {:?}", e);
        std::process::exit(0x0100);
    }

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
