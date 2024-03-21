use crate::{
    components,
    rpc::{currency, product, recommendation, shipping},
    Page, PageProps,
};
use anyhow::Result;

pub async fn build_home_page(props: &PageProps) -> Result<Page> {
    let currencies = match currency::get_supported_currencies().await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let product_list = match product::get_product_list(&props.user_currency).await {
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

pub async fn build_product_page(props: &PageProps) -> Result<Page> {
    let currencies = match currency::get_supported_currencies().await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let product = match product::get_product(&props).await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let recommendation_list = match recommendation::get_recommendations(&props).await {
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
    let recommendations = components::recommendations::Recommendations {
        recommendation_list,
    };
    let footer = components::body_footer::Footer {};
    let body = components::product_body::ProductBody {
        body_header: Box::new(body_header),
        footer: Box::new(footer),
        product,
        recommendations: Box::new(recommendations),
    };
    let page = crate::Page {
        lang: Some("en".to_string()),
        head: Box::new(head),
        body: Box::new(body),
    };

    Ok(page)
}

pub async fn build_cart_page(props: &PageProps) -> Result<Page> {
    let currencies = match currency::get_supported_currencies().await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let recommendation_list = match recommendation::get_recommendations(&props).await {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let quote = match shipping::get_quote(&props.cart_items, &props.user_currency).await {
        Ok(res) => res,
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
    let recommendations = components::recommendations::Recommendations {
        recommendation_list,
    };
    let footer = components::body_footer::Footer {};
    let body = components::cart_body::CartBody {
        body_header: Box::new(body_header),
        footer: Box::new(footer),
        recommendations: Box::new(recommendations),
        shipping_cost: quote.money_for_display(),
        total_cost: "100yen".to_string(),
    };
    let page = crate::Page {
        lang: Some("en".to_string()),
        head: Box::new(head),
        body: Box::new(body),
    };

    Ok(page)
}
