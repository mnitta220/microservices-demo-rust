use crate::{components, Page, PageProps};
use anyhow::Result;

pub async fn build_home_page(props: &PageProps) -> Result<Page> {
    let head = components::head::Head {};

    let body = match components::home_body::HomeBody::load(props).await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let page = crate::Page {
        lang: Some("en".to_string()),
        head: Box::new(head),
        body: Box::new(body),
    };

    Ok(page)
}

pub async fn build_product_page(props: &PageProps) -> Result<Page> {
    let head = components::head::Head {};

    let body = match components::product_body::ProductBody::load(props).await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let page = crate::Page {
        lang: Some("en".to_string()),
        head: Box::new(head),
        body: Box::new(body),
    };

    Ok(page)
}

pub async fn build_cart_page(props: &PageProps) -> Result<Page> {
    let head = components::head::Head {};

    let body = match components::cart_body::CartBody::load(props).await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let page = crate::Page {
        lang: Some("en".to_string()),
        head: Box::new(head),
        body: Box::new(body),
    };

    Ok(page)
}
