use anyhow::Result;
use axum::{extract::Form, http::StatusCode, response::Redirect};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

#[derive(Deserialize, Debug)]
pub struct SetCurrencyInput {
    pub currency_code: String,
}

pub async fn post_set_currency(
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
