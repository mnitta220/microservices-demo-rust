use super::{currency, hipstershop::Money, product, shipping};
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

pub async fn get_cart_info(
    user_id: String,
    currency_code: String,
) -> Result<crate::CartInfo, &'static str> {
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

    let mut total_price = Money {
        currency_code: currency_code.clone(),
        units: 0,
        nanos: 0,
    };
    let mut total_quantity = 0;
    let mut list: Vec<crate::CartItemView> = Vec::new();
    for item in cart.items.iter() {
        let request = super::GetProductRequest {
            id: item.product_id.clone(),
        };

        if let Ok(response) = product_catalog_service_client.get_product(request).await {
            let mut product = response.into_inner();
            if let Some(ref price) = product.price_usd {
                let mult_price: Money;
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

                    mult_price = multiply_slow(&changed, item.quantity)?;
                    product.price_usd = Some(changed);
                } else {
                    mult_price = multiply_slow(&price, item.quantity)?;
                }

                total_price = sum(&total_price, &mult_price)?;
                total_quantity += item.quantity;

                let ci = crate::CartItemView {
                    product: product,
                    quantity: item.quantity,
                    price: mult_price,
                };

                list.push(ci);
            }
        }
    }

    let quote = shipping::get_quote(&list, &currency_code).await?;
    total_price = sum(&total_price, &quote)?;

    let cart_info = crate::CartInfo {
        cart_items: list,
        shipping_cost: quote,
        total_price: total_price,
        total_quantity,
    };

    Ok(cart_info)
}

pub async fn empty_cart(user_id: String) -> Result<(), &'static str> {
    let mut cart_service_client = match get_cart_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let request = super::EmptyCartRequest { user_id };

    if let Err(_e) = cart_service_client.empty_cart(request).await {
        return Err("empty cart failed");
    }

    Ok(())
}

fn is_valid(m: &Money) -> bool {
    sign_matches(m) && valid_nanos(m.nanos)
}

fn sign_matches(m: &Money) -> bool {
    m.nanos == 0 || m.units == 0 || (m.nanos < 0) == (m.units < 0)
}

const NANOS_MIN: i32 = -999999999;
const NANOS_MAX: i32 = 999999999;
const NANOS_MOD: i32 = 1000000000;

fn valid_nanos(nanos: i32) -> bool {
    NANOS_MIN < nanos && nanos <= NANOS_MAX
}

fn sum(l: &Money, r: &Money) -> Result<Money, &'static str> {
    if !is_valid(l) || !is_valid(r) {
        return Err("one of the specified money values is invalid");
    }
    if l.currency_code != r.currency_code {
        return Err("mismatching currency codes");
    }
    let mut units = l.units + r.units;
    let mut nanos = l.nanos + r.nanos;

    if (units == 0 && nanos == 0) || (units > 0 && nanos >= 0) || (units < 0 && nanos <= 0) {
        // same sign <units, nanos>
        units += (nanos / NANOS_MOD) as i64;
        nanos = nanos % NANOS_MOD;
    } else {
        // different sign. nanos guaranteed to not to go over the limit
        if units > 0 {
            units -= 1;
            nanos += NANOS_MOD;
        } else {
            units += 1;
            nanos -= NANOS_MOD;
        }
    }

    Ok(Money {
        currency_code: l.currency_code.clone(),
        units: units,
        nanos: nanos,
    })
}

// multiply_slow is a slow multiplication operation done through adding the value
// to itself n-1 times.
fn multiply_slow(m: &Money, n: i32) -> Result<Money, &'static str> {
    let mut out = m.clone();
    let mut n = n;
    while n > 1 {
        out = sum(&out, m)?;
        n -= 1;
    }
    Ok(out)
}
