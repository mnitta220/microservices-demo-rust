use super::{
    currency, hipstershop::Money, CheckoutServiceClient, CurrencyConversionRequest, OrderResult,
    PlaceOrderRequest,
};
use anyhow::Result;
use tonic::transport::Channel;

async fn get_checkout_service_client() -> Result<CheckoutServiceClient<Channel>> {
    let checkout_service_addr = crate::CHECKOUT_SERVICE_ADDR.get().unwrap();

    let checkout_service_client =
        match CheckoutServiceClient::connect(format!("http://{}", checkout_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err(anyhow::anyhow!("get_recommendation_service_client failed"));
            }
        };

    Ok(checkout_service_client)
}

pub async fn place_order(
    request: PlaceOrderRequest,
    currency_code: &String,
) -> Result<(OrderResult, Money)> {
    let mut checkout_service_client = get_checkout_service_client().await?;

    let mut currency_service_client = currency::get_currency_service_client().await?;

    let request: tonic::Request<PlaceOrderRequest> = tonic::Request::new(request);

    let order = match checkout_service_client.place_order(request).await {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return Err(anyhow::anyhow!("failed place_order"));
        }
    };

    let order = match order.order {
        Some(o) => o,
        None => {
            return Err(anyhow::anyhow!("failed to get order result"));
        }
    };

    let mut total_cost = Money {
        currency_code: currency_code.clone(),
        units: 0,
        nanos: 0,
    };

    for item in &order.items {
        if let Some(m) = &item.cost {
            let mult_price: Money;

            if m.currency_code != *currency_code {
                let request = CurrencyConversionRequest {
                    from: Some(m.clone()),
                    to_code: currency_code.clone(),
                };

                let changed = match currency_service_client.convert(request).await {
                    Ok(changed) => changed.into_inner(),
                    Err(_) => {
                        return Err(anyhow::anyhow!("currency convert failed"));
                    }
                };

                mult_price =
                    super::cart::multiply_slow(&changed, item.item.as_ref().unwrap().quantity)?;
            } else {
                mult_price = super::cart::multiply_slow(&m, item.item.as_ref().unwrap().quantity)?;
            }

            total_cost = super::cart::sum(&total_cost, &mult_price)?;
        }
    }

    if let Some(m) = &order.shipping_cost {
        if m.currency_code != *currency_code {
            let request = CurrencyConversionRequest {
                from: Some(m.clone()),
                to_code: currency_code.clone(),
            };

            let changed = match currency_service_client.convert(request).await {
                Ok(changed) => changed.into_inner(),
                Err(_) => {
                    return Err(anyhow::anyhow!("currency convert failed"));
                }
            };

            total_cost = super::cart::sum(&total_cost, &changed)?;
        } else {
            total_cost = super::cart::sum(&total_cost, &m)?;
        }
    }

    Ok((order, total_cost))
}
