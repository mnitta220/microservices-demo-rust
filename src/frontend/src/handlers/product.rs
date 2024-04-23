use crate::{
    model,
    pages::{page, product_page::ProductPage},
    AppError,
};
use anyhow::Result;
use axum::{extract::Path, response::Html};
use tower_cookies::Cookies;

pub async fn get_product(
    cookies: Cookies,
    Path(id): Path<String>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /product {}", id);

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

        // fetch product info
        let product = model::product::Product::load(&id, &currency).await?;
        props.product = Some(product);

        // fetch recommendation info
        let recommendations =
            model::recommendation::RecommendationList::load(Some(id), &session_id).await?;
        props.recommendations = Some(recommendations);

        // fetch advertisement info
        let ad = model::ad::AdItem::load().await;
        props.ad = ad;
    }

    let mut page = ProductPage::new(props);
    let buf = page.write();

    Ok(Html(buf))
}
