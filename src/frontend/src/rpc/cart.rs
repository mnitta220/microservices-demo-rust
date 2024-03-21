use super::{currency, product};
use std::env;

async fn get_cart_service_client(
) -> Result<super::CartServiceClient<tonic::transport::Channel>, &'static str> {
    let cart_service_addr = match env::var("CART_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get CART_SERVICE_ADDR");
        }
    };

    let cart_service_client =
        match super::CartServiceClient::connect(format!("http://{}", cart_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err("get_cart_service_client failed");
            }
        };

    Ok(cart_service_client)
}

pub async fn add_to_cart(
    user_id: String,
    product_id: String,
    quantity: i32,
) -> Result<(), &'static str> {
    let mut cart_service_client = match get_cart_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let cart_item = super::CartItem {
        product_id,
        quantity,
    };

    let request = super::AddItemRequest {
        user_id,
        item: Some(cart_item),
    };

    if let Err(_e) = cart_service_client.add_item(request).await {
        return Err("cart add_item failed");
    }

    Ok(())
}

pub async fn get_cart(
    user_id: String,
    currency_code: String,
) -> Result<Vec<crate::CartItem>, &'static str> {
    let mut cart_service_client = match get_cart_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let mut product_catalog_service_client =
        match product::get_product_catalog_service_client().await {
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

    let cart = match cart_service_client
        .get_cart(super::GetCartRequest { user_id })
        .await
    {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return Err("get_cart failed");
        }
    };

    let mut list: Vec<crate::CartItem> = Vec::new();
    for item in cart.items.iter() {
        let request = super::GetProductRequest {
            id: item.product_id.clone(),
        };

        if let Ok(response) = product_catalog_service_client.get_product(request).await {
            let mut product = response.into_inner();
            if let Some(ref price) = product.price_usd {
                if price.currency_code != currency_code {
                    let request = super::CurrencyConversionRequest {
                        from: Some(price.clone()),
                        to_code: currency_code.clone(),
                    };
                    let changed = match currency_service_client.convert(request).await {
                        Ok(changed) => changed.into_inner(),
                        Err(_) => {
                            return Err("currency convert failed");
                        }
                    };
                    product.price_usd = Some(changed);
                }
            }
            let cp = crate::CartItem {
                product: product,
                quantity: item.quantity,
            };
            list.push(cp);
        }
    }

    Ok(list)
}
