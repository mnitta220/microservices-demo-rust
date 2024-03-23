use super::{
    currency, CartItem, CurrencyConversionRequest, GetQuoteRequest, Money, ShippingServiceClient,
};
use crate::CartItemView;
use std::env;

async fn get_shipping_service_client(
) -> Result<ShippingServiceClient<tonic::transport::Channel>, &'static str> {
    let shipping_service_addr = match env::var("SHIPPING_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get SHIPPING_SERVICE_ADDR");
        }
    };

    let shipping_service_client =
        match ShippingServiceClient::connect(format!("http://{}", shipping_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err("get_shipping_service_client failed");
            }
        };

    Ok(shipping_service_client)
}

pub async fn get_quote(
    items: &Vec<CartItemView>,
    user_currency: &String,
) -> Result<Money, &'static str> {
    let mut shipping_service_client = match get_shipping_service_client().await {
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

    let mut cart_items: Vec<CartItem> = Vec::new();
    for item in items.iter() {
        let i = CartItem {
            product_id: item.product.id.clone(),
            quantity: item.quantity,
        };
        cart_items.push(i);
    }

    let request = GetQuoteRequest {
        address: None,
        items: cart_items,
    };

    let mut quote = match shipping_service_client.get_quote(request).await {
        Ok(response) => match response.into_inner().cost_usd {
            Some(m) => m,
            None => {
                return Err("get_quote failed");
            }
        },
        Err(_) => {
            return Err("get_quote failed");
        }
    };

    if quote.currency_code != *user_currency {
        let request = CurrencyConversionRequest {
            from: Some(quote),
            to_code: user_currency.clone(),
        };
        let changed = match currency_service_client.convert(request).await {
            Ok(changed) => changed.into_inner(),
            Err(_) => {
                return Err("currency convert failed");
            }
        };
        quote = changed;
    }

    Ok(quote)
}
