use crate::{
    pages::{cart_page::CartPage, home_page::HomePage, product_page::ProductPage},
    rpc, AppError,
};
use axum::{
    extract::{Form, Path},
    http::StatusCode,
    response::{Html, Redirect},
};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

const COOKIE_SESSION_ID: &str = "shop_session-id";
const COOKIE_CURRENCY: &str = "shop_currency";

fn get_set_session(cookies: Cookies) -> (String, String) {
    let session_id = match cookies.get(COOKIE_SESSION_ID) {
        Some(s) => s.value().to_string(),
        None => {
            let id = Uuid::new_v4().to_string();
            cookies.add(Cookie::new(COOKIE_SESSION_ID, id.clone()));
            id
        }
    };

    let currency = match cookies.get(COOKIE_CURRENCY) {
        Some(s) => s.value().to_string(),
        None => {
            cookies.add(Cookie::new(COOKIE_CURRENCY, "USD".to_string()));
            "USD".to_string()
        }
    };

    (session_id, currency)
}

pub async fn home_handler(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /");

    let (session_id, currency) = get_set_session(cookies);

    match HomePage::generate(&session_id, &currency).await {
        Ok(r) => Ok(Html(r)),
        Err(e) => Err(AppError(anyhow::anyhow!(e.to_string()))),
    }
}

pub async fn product_handler(
    cookies: Cookies,
    Path(id): Path<String>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /product {}", id);

    let (session_id, currency) = get_set_session(cookies);

    match ProductPage::generate(&session_id, &currency, id).await {
        Ok(r) => Ok(Html(r)),
        Err(e) => Err(AppError(anyhow::anyhow!(e.to_string()))),
    }
}

pub async fn view_cart_handler(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /cart");

    let (session_id, currency) = get_set_session(cookies);

    let ret = match CartPage::generate(&session_id, &currency).await {
        Ok(r) => r,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    Ok(Html(ret))
}

#[derive(Deserialize, Debug)]
pub struct SetCurrencyInput {
    pub currency_code: String,
}

pub async fn set_currency_handler(
    cookies: Cookies,
    Form(input): Form<SetCurrencyInput>,
) -> Result<Redirect, StatusCode> {
    tracing::debug!("POST /setCurrency {}", input.currency_code);

    let cookie = Cookie::parse(format!(
        "shop_currency={}; Max-Age={}",
        input.currency_code,
        60 * 60 * 48
    ));
    if let Ok(c) = cookie {
        cookies.add(c);
    }

    Ok(Redirect::to("/"))
}

#[derive(Deserialize, Debug)]
pub struct AddToCartInput {
    pub product_id: String,
    pub quantity: i32,
}

pub async fn add_to_cart_handler(
    cookies: Cookies,
    Form(input): Form<AddToCartInput>,
) -> Result<Redirect, AppError> {
    tracing::debug!("POST /cart {}, {}", input.product_id, input.quantity);

    let session_id = match cookies.get(COOKIE_SESSION_ID) {
        Some(s) => s.value().to_string(),
        None => {
            let id = Uuid::new_v4().to_string();
            cookies.add(Cookie::new(COOKIE_SESSION_ID, id.clone()));
            id
        }
    };

    if let Err(e) = rpc::cart::add_to_cart(session_id, input.product_id, input.quantity).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    Ok(Redirect::to("/cart"))
}

pub async fn empty_cart_handler(cookies: Cookies) -> Result<Redirect, AppError> {
    tracing::debug!("POST /cart/empty");

    let session_id = match cookies.get(COOKIE_SESSION_ID) {
        Some(s) => s.value().to_string(),
        None => {
            let id = Uuid::new_v4().to_string();
            cookies.add(Cookie::new(COOKIE_SESSION_ID, id.clone()));
            id
        }
    };

    if let Err(e) = rpc::cart::empty_cart(session_id).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    Ok(Redirect::to("/"))
}

pub async fn place_order_handler(
    cookies: Cookies,
    Form(input): Form<AddToCartInput>,
) -> Result<Redirect, AppError> {
    tracing::debug!("POST /cart {}, {}", input.product_id, input.quantity);

    let session_id = match cookies.get(COOKIE_SESSION_ID) {
        Some(s) => s.value().to_string(),
        None => {
            let id = Uuid::new_v4().to_string();
            cookies.add(Cookie::new(COOKIE_SESSION_ID, id.clone()));
            id
        }
    };

    if let Err(e) = rpc::cart::add_to_cart(session_id, input.product_id, input.quantity).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    Ok(Redirect::to("/cart"))
}

pub async fn health_handler() -> &'static str {
    "OK"
}
