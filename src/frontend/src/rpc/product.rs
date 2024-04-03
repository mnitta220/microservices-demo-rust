use super::{
    currency, CurrencyConversionRequest, Empty, GetProductRequest, Money, Product,
    ProductCatalogServiceClient,
};
use anyhow::Result;

pub async fn get_product_catalog_service_client(
) -> Result<ProductCatalogServiceClient<tonic::transport::Channel>> {
    let product_catalog_service_addr = crate::PRODUCT_CATALOG_SERVICE_ADDR.get().unwrap();

    let product_catalog_service_client = match ProductCatalogServiceClient::connect(format!(
        "http://{}",
        product_catalog_service_addr
    ))
    .await
    {
        Ok(client) => client,
        Err(_) => {
            return Err(anyhow::anyhow!("get_product_catalog_service_client failed"));
        }
    };

    Ok(product_catalog_service_client)
}

pub async fn get_product_list(user_currency: &String) -> Result<Vec<Product>> {
    let mut product_catalog_service_client = get_product_catalog_service_client().await?;

    let mut currency_service_client = currency::get_currency_service_client().await?;

    let products = match product_catalog_service_client.list_products(Empty {}).await {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return Err(anyhow::anyhow!("get_products: list_products failed"));
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
                    return Err(anyhow::anyhow!("currency convert failed"));
                }
            };

            p.price_usd = Some(changed);
        }

        list.push(p);
    }

    Ok(list)
}

pub async fn get_product(product_id: String, user_currency: String) -> Result<Product> {
    let mut product_catalog_service_client = get_product_catalog_service_client().await?;

    let mut currency_service_client = currency::get_currency_service_client().await?;

    let request = GetProductRequest { id: product_id };

    let mut product = match product_catalog_service_client.get_product(request).await {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return Err(anyhow::anyhow!("get_product failed"));
        }
    };

    let price: Money;

    if product.price_usd.as_ref().unwrap().currency_code != user_currency {
        let request = CurrencyConversionRequest {
            from: product.price_usd.clone(),
            to_code: user_currency,
        };

        price = match currency_service_client.convert(request).await {
            Ok(changed) => changed.into_inner(),
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "get_currencies: get_supported_currencies failed"
                ));
            }
        };
    } else {
        price = product.price_usd.unwrap();
    }

    product.price_usd = Some(price);

    Ok(product)
}
