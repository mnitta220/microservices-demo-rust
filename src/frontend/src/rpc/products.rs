pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use super::super::CURRENCY_LOGO;
use hipstershop::currency_service_client::CurrencyServiceClient;
use hipstershop::product_catalog_service_client::ProductCatalogServiceClient;
use hipstershop::{CurrencyConversionRequest, Empty, Money, Product};
use std::env;

impl Product {
    /// Write Hot Products HTML
    pub fn write_hot_products_html(&self, buf: &mut String, money: &Money) {
        buf.push_str(r#"<div class="col-md-4 hot-product-card">"#);
        {
            buf.push_str(r#"<a href="/product/"#);
            buf.push_str(&self.id);
            buf.push_str(r#"">"#);
            {
                buf.push_str(r#"<img alt="" src=""#);
                buf.push_str(&self.picture);
                buf.push_str(r#"">"#);
                buf.push_str(r#"<div class="hot-product-card-img-overlay"></div>"#);
            }
            buf.push_str(r#"</a>"#);
            buf.push_str(r#"<div>"#);
            {
                buf.push_str(r#"<div class="hot-product-card-name">"#);
                buf.push_str(&self.name);
                buf.push_str(r#"</div>"#);

                buf.push_str(r#"<div class="hot-product-card-price">"#);
                if let Some(c) = CURRENCY_LOGO.get(money.currency_code.as_str()) {
                    buf.push_str(c);
                } else {
                    buf.push_str("$");
                }
                buf.push_str(money.units.to_string().as_str());
                buf.push_str(".");
                buf.push_str(format!("{:.2}", money.nanos / 10000000).as_str());
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);
    }
}

pub async fn get_products(buf: &mut String, user_currency: String) -> Result<(), &'static str> {
    let product_catalog_service_addr = match env::var("PRODUCT_CATALOG_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get PRODUCT_CATALOG_SERVICE_ADDR");
        }
    };

    let mut product_catalog_service_client = match ProductCatalogServiceClient::connect(format!(
        "http://{}",
        product_catalog_service_addr
    ))
    .await
    {
        Ok(client) => client,
        Err(_) => {
            return Err("get_products: connect failed");
        }
    };

    let currency_service_addr = match env::var("CURRENCY_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get CURRENCY_SERVICE_ADDR");
        }
    };

    let mut currency_service_client =
        match CurrencyServiceClient::connect(format!("http://{}", currency_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err("get_currencies: connect failed");
            }
        };

    let products = match product_catalog_service_client.list_products(Empty {}).await {
        Ok(response) => response,
        Err(_) => {
            return Err("get_products: list_products failed");
        }
    };

    buf.push_str(r#"<div class="row hot-products-row px-xl-6">"#);
    {
        buf.push_str(r#"<div class="col-12">"#);
        {
            buf.push_str(r#"<h3>Hot Products</h3>"#);
        }
        buf.push_str(r#"</div>"#);
        for product in products.get_ref().products.iter() {
            if product.price_usd.as_ref().unwrap().currency_code != user_currency {
                let request = CurrencyConversionRequest {
                    from: product.price_usd.clone(),
                    to_code: user_currency.clone(),
                };
                let changed = match currency_service_client.convert(request).await {
                    Ok(changed) => changed.into_inner(),
                    Err(_) => {
                        return Err("get_currencies: get_supported_currencies failed");
                    }
                };
                product.write_hot_products_html(buf, &changed);
            } else {
                product.write_hot_products_html(buf, &product.price_usd.as_ref().unwrap());
            }
        }
    }
    buf.push_str(r#"</div>"#);

    Ok(())
}
