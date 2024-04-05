use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use hostname;
use once_cell::sync::OnceCell;
use pages::page::Props;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_cookies::CookieManagerLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod components;
mod handlers;
mod model;
mod pages;
mod rpc;

static AD_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static CART_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static CHECKOUT_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static CURRENCY_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static PRODUCT_CATALOG_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static RECOMMENDATION_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static SHIPPING_SERVICE_ADDR: OnceCell<String> = OnceCell::new();
static PLATFORM_NAME: OnceCell<String> = OnceCell::new();
static PLATFORM_CSS: OnceCell<String> = OnceCell::new();
static HOST_NAME: OnceCell<String> = OnceCell::new();

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "frontend=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Err(e) = get_environment_values() {
        tracing::error!("failed to get environment values: {:?}", e);
        std::process::exit(0x0100);
    }

    let app = Router::new()
        .route("/", get(handlers::get_home))
        .route("/product/:id", get(handlers::get_product))
        .route("/setCurrency", post(handlers::post_set_currency))
        .route("/cart", get(handlers::get_cart).post(handlers::post_cart))
        .route("/cart/empty", post(handlers::post_cart_empty))
        .route("/cart/checkout", post(handlers::post_cart_checkout))
        .route("/_healthz", get(handlers::health))
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

fn get_environment_values() -> Result<()> {
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

    // get ENV_PLATFORM env
    let env_platform = match std::env::var("ENV_PLATFORM") {
        Ok(e) => e,
        Err(_) => "local".to_string(),
    };
    let (provider, css) = match env_platform.as_str() {
        "aws" => ("AWS".to_string(), "aws-platform".to_string()),
        "onprem" => ("On-Premises".to_string(), "onprem-platform".to_string()),
        "azure" => ("Azure".to_string(), "azure-platform".to_string()),
        "gcp" => ("Google Cloud".to_string(), "gcp-platform".to_string()),
        "alibaba" => ("Alibaba Cloud".to_string(), "alibaba-platform".to_string()),
        _ => ("local".to_string(), "local".to_string()),
    };
    // set PLATFORM_NAME static
    if let Err(_) = PLATFORM_NAME.set(provider) {
        return Err(anyhow::anyhow!("Failed to set PLATFORM_NAME"));
    }
    // set PLATFORM_CSS static
    if let Err(_) = PLATFORM_CSS.set(css) {
        return Err(anyhow::anyhow!("Failed to set PLATFORM_CSS"));
    }

    match hostname::get() {
        Ok(os_name) => match os_name.into_string() {
            Ok(host_name) => {
                // set HOST_NAME static
                if let Err(_) = HOST_NAME.set(host_name) {}
            }
            Err(_) => {}
        },
        Err(_) => {}
    }

    Ok(())
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
