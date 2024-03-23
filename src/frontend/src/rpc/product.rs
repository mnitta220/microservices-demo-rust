use super::{
    currency, CurrencyConversionRequest, Empty, GetProductRequest, Money, Product,
    ProductCatalogServiceClient,
};
use crate::PageProps;
use std::env;

pub async fn get_product_catalog_service_client(
) -> Result<ProductCatalogServiceClient<tonic::transport::Channel>, &'static str> {
    let product_catalog_service_addr = match env::var("PRODUCT_CATALOG_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get PRODUCT_CATALOG_SERVICE_ADDR");
        }
    };

    let product_catalog_service_client = match ProductCatalogServiceClient::connect(format!(
        "http://{}",
        product_catalog_service_addr
    ))
    .await
    {
        Ok(client) => client,
        Err(_) => {
            return Err("get_product_catalog_service_client failed");
        }
    };

    Ok(product_catalog_service_client)
}

pub async fn get_product_list(user_currency: &String) -> Result<Vec<Product>, &'static str> {
    let mut product_catalog_service_client = match get_product_catalog_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let mut currency_service_client = match currency::get_currency_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let products = match product_catalog_service_client.list_products(Empty {}).await {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return Err("get_products: list_products failed");
        }
    };

    let mut list: Vec<Product> = Vec::new();
    for product in products.products.iter() {
        let mut p = product.clone();
        if product.price_usd.as_ref().unwrap().currency_code != *user_currency {
            let request = CurrencyConversionRequest {
                from: product.price_usd.clone(),
                to_code: user_currency.clone(),
            };
            let changed = match currency_service_client.convert(request).await {
                Ok(changed) => changed.into_inner(),
                Err(_) => {
                    return Err("currency convert failed");
                }
            };
            p.price_usd = Some(changed);
        }
        list.push(p);
    }

    Ok(list)
}

pub async fn get_product(page_props: &PageProps) -> Result<Product, &'static str> {
    let product_id = match page_props.product_id.clone() {
        Some(id) => id,
        None => return Err("product id not specified"),
    };
    if product_id.len() == 0 {
        return Err("product id not specified");
    }

    let mut product_catalog_service_client = match get_product_catalog_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let mut currency_service_client = match currency::get_currency_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let request = GetProductRequest { id: product_id };

    let mut product = match product_catalog_service_client.get_product(request).await {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return Err("get_product failed");
        }
    };
    let price: Money;
    if product.price_usd.as_ref().unwrap().currency_code != page_props.user_currency {
        let request = CurrencyConversionRequest {
            from: product.price_usd.clone(),
            to_code: page_props.user_currency.clone(),
        };
        price = match currency_service_client.convert(request).await {
            Ok(changed) => changed.into_inner(),
            Err(_) => {
                return Err("get_currencies: get_supported_currencies failed");
            }
        };
    } else {
        price = product.price_usd.unwrap();
    }

    product.price_usd = Some(price);

    Ok(product)
}
