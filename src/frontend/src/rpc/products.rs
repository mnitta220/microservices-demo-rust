pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use super::super::CURRENCY_LOGO;
use hipstershop::product_catalog_service_client::ProductCatalogServiceClient;
use hipstershop::{Empty, Product};
use std::env;

impl Product {
    /// Write Hot Products HTML
    pub fn write_hot_products_html(&self, buf: &mut String) {
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
                if let Some(m) = &self.price_usd {
                    if let Some(c) = CURRENCY_LOGO.get(m.currency_code.as_str()) {
                        buf.push_str(c);
                    } else {
                        buf.push_str("$");
                    }
                    buf.push_str(m.units.to_string().as_str());
                    buf.push_str(".");
                    buf.push_str(format!("{:.2}", m.nanos / 10000000).as_str());
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);
    }
}

pub async fn get_products(buf: &mut String) -> Result<(), &'static str> {
    let addr = match env::var("PRODUCT_CATALOG_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get PRODUCT_CATALOG_SERVICE_ADDR");
        }
    };

    let mut client = match ProductCatalogServiceClient::connect(format!("http://{}", addr)).await {
        Ok(client) => client,
        Err(_) => {
            return Err("get_products: connect failed");
        }
    };

    let response = match client.list_products(Empty {}).await {
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
        for product in response.get_ref().products.iter() {
            product.write_hot_products_html(buf);
        }
    }
    buf.push_str(r#"</div>"#);

    Ok(())
}
