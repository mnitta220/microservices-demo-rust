use crate::{
    model,
    pages::{cart_page::CartPage, order_page::OrderPage, page},
    rpc, AppError,
};
use anyhow::Result;
use axum::{
    extract::Form,
    response::{Html, Redirect},
};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub async fn get_cart(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /cart");

    let (session_id, currency) = super::session_info(cookies, false)?;
    let mut props = page::Props::new(&session_id, &currency);

    // load and setting props
    {
        // fetch currency codes
        let currencies = model::currency::SupportedCurrencies::load().await?;
        props.currency_codes = Some(currencies);

        // fetch cart info
        let cart = model::cart::Cart::load(&session_id, &currency).await?;
        props.cart = Some(cart);

        // fetch recommendation info
        let recommendations =
            model::recommendation::RecommendationList::load(None, &session_id).await?;
        props.recommendations = Some(recommendations);
    }

    let mut page = CartPage::new(props);
    let buf = page.write();

    Ok(Html(buf))
}

#[derive(Deserialize, Debug)]
pub struct AddToCartInput {
    pub product_id: String,
    pub quantity: i32,
}

pub async fn post_cart(
    cookies: Cookies,
    Form(input): Form<AddToCartInput>,
) -> Result<Redirect, AppError> {
    tracing::debug!("POST /cart {}, {}", input.product_id, input.quantity);

    let session_id = match cookies.get(super::COOKIE_SESSION_ID) {
        Some(s) => s.value().to_string(),
        None => {
            let id = Uuid::new_v4().to_string();
            cookies.add(Cookie::new(super::COOKIE_SESSION_ID, id.clone()));
            id
        }
    };

    if let Err(e) = rpc::cart::add_to_cart(session_id, input.product_id, input.quantity).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    Ok(Redirect::to("/cart"))
}

pub async fn post_cart_empty(cookies: Cookies) -> Result<Redirect, AppError> {
    tracing::debug!("POST /cart/empty");

    let session_id = match cookies.get(super::COOKIE_SESSION_ID) {
        Some(s) => s.value().to_string(),
        None => {
            let id = Uuid::new_v4().to_string();
            cookies.add(Cookie::new(super::COOKIE_SESSION_ID, id.clone()));
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

pub async fn post_cart_checkout(
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

    let (session_id, currency) = super::session_info(cookies, true)?;
    let mut props = page::Props::new(&session_id, &currency);

    // load and setting props
    {
        // place order
        let order = model::order::Order::place_order(input, &session_id, &currency).await?;
        props.order = Some(order);

        // fetch currency codes
        let currencies = model::currency::SupportedCurrencies::load().await?;
        props.currency_codes = Some(currencies);

        // fetch cart info
        let cart = model::cart::Cart::load(&session_id, &currency).await?;
        props.cart = Some(cart);

        // fetch recommendation info
        let recommendations =
            model::recommendation::RecommendationList::load(None, &session_id).await?;
        props.recommendations = Some(recommendations);
    }

    let mut page = OrderPage::new(props);
    let buf = page.write();

    Ok(Html(buf))
}
