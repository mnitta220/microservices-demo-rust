use crate::{page_builder, rpc, AppError, Component, PageProps, PageType};
use anyhow::Result;
use axum::{
    extract::{Form, Path},
    http::StatusCode,
    response::Html,
    response::Redirect,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use uuid::Uuid;

pub async fn home_handler(jar: CookieJar) -> Result<(CookieJar, Html<String>), AppError> {
    tracing::debug!("GET /");

    handler_sub(jar, PageType::Home, None).await
}

pub async fn product_handler(
    jar: CookieJar,
    Path(id): Path<String>,
) -> Result<(CookieJar, Html<String>), AppError> {
    tracing::debug!("GET /product {}", id);

    handler_sub(jar, PageType::Product, Some(id)).await
}

pub async fn view_cart_handler(jar: CookieJar) -> Result<(CookieJar, Html<String>), AppError> {
    tracing::debug!("GET /cart");

    handler_sub(jar, PageType::Cart, None).await
}

async fn handler_sub(
    jar: CookieJar,
    page_type: PageType,
    product_id: Option<String>,
) -> Result<(CookieJar, Html<String>), AppError> {
    tracing::debug!("GET /");

    let currency: Option<String> = jar
        .get("shop_currency")
        .map(|cookie| cookie.value().to_owned());
    let currency = match currency {
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

    let cart_items = match rpc::cart::get_cart(session_id.clone(), currency.clone()).await {
        Ok(r) => r,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let props = PageProps {
        session_id: session_id.clone(),
        request_id: Uuid::new_v4().to_string(),
        user_currency: currency,
        product_id: product_id,
        cart_items,
    };

    let page = match page_type {
        PageType::Home => page_builder::build_home_page(&props).await?,
        PageType::Product => page_builder::build_product_page(&props).await?,
        PageType::Cart => page_builder::build_cart_page(&props).await?,
    };

    let mut buf = String::with_capacity(100000);

    if let Err(e) = page.write(&props, &mut buf) {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    if is_new_session {
        let cookie = match Cookie::parse(format!(
            "shop_session-id={}; Max-Age={}",
            session_id,
            60 * 60 * 48
        )) {
            Ok(c) => c,
            Err(_) => {
                return Err(AppError(anyhow::anyhow!("Cookie parse error")));
            }
        };

        Ok((jar.add(cookie), Html(buf)))
    } else {
        Ok((jar, Html(buf)))
    }
}

#[derive(Deserialize, Debug)]
pub struct SetCurrencyInput {
    pub currency_code: String,
}

pub async fn set_currency_handler(
    jar: CookieJar,
    Form(input): Form<SetCurrencyInput>,
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

#[derive(Deserialize, Debug)]
pub struct AddToCartInput {
    pub product_id: String,
    pub quantity: i32,
}

pub async fn add_to_cart_handler(
    jar: CookieJar,
    Form(input): Form<AddToCartInput>,
) -> Result<Redirect, AppError> {
    tracing::debug!("POST /cart {}, {}", input.product_id, input.quantity);
    let session_id = jar
        .get("shop_session-id")
        .map(|cookie| cookie.value().to_owned());
    let session_id = match session_id {
        Some(s) => s,
        None => {
            return Err(AppError(anyhow::anyhow!("failed to get session_id")));
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
