use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod components;
mod handlers;
mod page_builder;
mod rpc;

pub enum PageType {
    Home,
    Product,
    Cart,
}

pub struct PageProps {
    pub session_id: String,
    pub request_id: String,
    pub user_currency: String,
    pub product_id: Option<String>,
    pub cart_items: Vec<CartItem>,
}

pub struct CartItem {
    pub product: rpc::hipstershop::Product,
    pub quantity: i32,
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
        .route("/", get(handlers::home_handler))
        .route("/product/:id", get(handlers::product_handler))
        .route("/setCurrency", post(handlers::set_currency_handler))
        .route(
            "/cart",
            get(handlers::view_cart_handler).post(handlers::add_to_cart_handler),
        )
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
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub trait Component {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()>;
}

pub struct Page {
    pub lang: Option<String>,
    pub head: Box<dyn Component>,
    pub body: Box<dyn Component>,
}

impl Component for Page {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        buf.push_str(r#"<!DOCTYPE html>"#);
        if let Some(lang) = &self.lang {
            buf.push_str(r#"<html lang=""#);
            buf.push_str(lang);
            buf.push_str(r#"">"#);
        } else {
            buf.push_str(r#"<html>"#);
        }

        if let Err(e) = self.head.write(props, buf) {
            return Err(e);
        }

        if let Err(e) = self.body.write(props, buf) {
            return Err(e);
        }

        buf.push_str(r#"</html>"#);

        Ok(())
    }
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
