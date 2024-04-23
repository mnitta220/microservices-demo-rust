use crate::{
    model,
    pages::{home_page::HomePage, page},
    AppError,
};
use anyhow::Result;
use axum::response::Html;
use tower_cookies::Cookies;

pub async fn get_home(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /");

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

        // fetch hot product info
        let hot_products = model::hot_product::HotProducts::load(&currency).await?;
        props.hot_products = Some(hot_products);
    }

    let mut page = HomePage::new(props);
    let buf = page.write();

    Ok(Html(buf))
}
