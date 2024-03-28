use crate::{
    pages::{
        cart_page::CartPage, home_page::HomePage, order_page::OrderPage, product_page::ProductPage,
    },
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

#[derive(Deserialize, Debug)]
pub struct PlaceOrderInput {
    pub email: String,
    pub street_address: String,
    pub zip_code: i32,
    pub city: String,
    pub state: String,
    pub country: String,
    pub credit_card_number: String,
    pub credit_card_expiration_month: i32,
    pub credit_card_expiration_year: i32,
    pub credit_card_cvv: i32,
}

pub async fn place_order_handler(
    cookies: Cookies,
    Form(input): Form<PlaceOrderInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!(
        "POST /cart/checkout email={}, street_address={} zip_code={} city={} state={} country={} credit_card_number={} credit_card_expiration_month={} credit_card_expiration_year={} credit_card_cvv={}",
        input.email,
        input.street_address,
        input.zip_code,
        input.city,
        input.state,
        input.country,
        input.credit_card_number,
        input.credit_card_expiration_month,
        input.credit_card_expiration_year,
        input.credit_card_cvv,
    );

    let session_id = match cookies.get(COOKIE_SESSION_ID) {
        Some(s) => s.value().to_string(),
        None => {
            return Err(AppError(anyhow::anyhow!("failed to get session")));
        }
    };

    let currency = match cookies.get(COOKIE_CURRENCY) {
        Some(s) => s.value().to_string(),
        None => {
            return Err(AppError(anyhow::anyhow!("failed to get currency")));
        }
    };

    let request = rpc::hipstershop::PlaceOrderRequest {
        user_id: session_id.clone(),
        user_currency: currency.clone(),
        address: Some(rpc::hipstershop::Address {
            street_address: input.street_address,
            city: input.city,
            state: input.state,
            country: input.country,
            zip_code: input.zip_code,
        }),
        email: input.email,
        credit_card: Some(rpc::hipstershop::CreditCardInfo {
            credit_card_number: input.credit_card_number,
            credit_card_cvv: input.credit_card_cvv,
            credit_card_expiration_year: input.credit_card_expiration_year,
            credit_card_expiration_month: input.credit_card_expiration_month,
        }),
    };

    let (order, total_cost) = match rpc::checkout::place_order(request, currency.clone()).await {
        Ok(ret) => ret,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    match OrderPage::generate(&session_id, &currency, order, total_cost).await {
        Ok(r) => Ok(Html(r)),
        Err(e) => Err(AppError(anyhow::anyhow!(e.to_string()))),
    }
}

pub async fn health_handler() -> &'static str {
    "OK"
}
