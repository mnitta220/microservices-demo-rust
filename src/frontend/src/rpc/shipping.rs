use super::{
    currency, CartItem, CurrencyConversionRequest, GetQuoteRequest, Money, ShippingServiceClient,
};
use crate::model;
use anyhow::Result;

async fn get_shipping_service_client() -> Result<ShippingServiceClient<tonic::transport::Channel>> {
    let shipping_service_addr = crate::SHIPPING_SERVICE_ADDR.get().unwrap();

    let shipping_service_client =
        match ShippingServiceClient::connect(format!("http://{}", shipping_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err(anyhow::anyhow!("get_shipping_service_client failed"));
            }
        };

    Ok(shipping_service_client)
}

pub async fn get_quote(
    items: &Vec<model::cart::CartItem>,
    user_currency: &String,
) -> Result<Money> {
    let mut shipping_service_client = get_shipping_service_client().await?;

    let mut currency_service_client = currency::get_currency_service_client().await?;

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
                return Err(anyhow::anyhow!("get_quote failed"));
            }
        },
        Err(_) => {
            return Err(anyhow::anyhow!("get_quote failed"));
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
                return Err(anyhow::anyhow!("currency convert failed"));
            }
        };

        quote = changed;
    }

    Ok(quote)
}
