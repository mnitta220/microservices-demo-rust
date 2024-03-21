use super::super::components;
use super::super::rpc::{currency, products, recommendation};
use crate::Page;
use crate::PageProps;
use anyhow::Result;

pub async fn create_home_page(props: &PageProps) -> Result<Page> {
    let currencies = match currency::get_supported_currencies().await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let product_list = match products::get_product_list(&props.user_currency).await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let head = components::head::Head {};
    let currency_form = components::currency_form::CurrencyForm {
        currency_codes: currencies,
    };
    let body_header = components::body_header::BodyHeader {
        currency_form: Box::new(currency_form),
    };
    let footer = components::body_footer::Footer {};
    let body = components::home_body::HomeBody {
        body_header: Box::new(body_header),
        footer: Box::new(footer),
        product_list,
    };
    let page = crate::Page {
        lang: Some("en".to_string()),
        head: Box::new(head),
        body: Box::new(body),
    };

    Ok(page)
}

pub async fn create_product_page(props: &PageProps) -> Result<Page> {
    let currencies = match currency::get_supported_currencies().await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let product = match products::get_product(&props).await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let recommendations = match recommendation::get_recommendations(&props).await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let head = components::head::Head {};
    let currency_form = components::currency_form::CurrencyForm {
        currency_codes: currencies,
    };
    let body_header = components::body_header::BodyHeader {
        currency_form: Box::new(currency_form),
    };
    let footer = components::body_footer::Footer {};
    let body = components::product_body::ProductBody {
        body_header: Box::new(body_header),
        footer: Box::new(footer),
        product,
        recommendation_list: recommendations,
    };
    let page = crate::Page {
        lang: Some("en".to_string()),
        head: Box::new(head),
        body: Box::new(body),
    };

    Ok(page)
}
