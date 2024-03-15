pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use hipstershop::product_catalog_service_client::ProductCatalogServiceClient;
use hipstershop::{Empty, GetProductRequest, Product, SearchProductsRequest};
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
                // TODO: change currency
                buf.push_str(r#"<div class="hot-product-card-price">$19.99</div>"#);
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
    //println!("product_catalog_service_addr={}", addr);
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
    //println!("RESPONSE={:?}\n", response1.get_ref());
    let hot_products_head = r#"
              <div class="row hot-products-row px-xl-6">
                <div class="col-12">
                  <h3>Hot Products</h3>
                </div>
                "#;

    buf.push_str(hot_products_head);

    for product in response.get_ref().products.iter() {
        //println!("{}: {}", product.id, product.name);
        product.write_hot_products_html(buf);
    }
    buf.push_str(r#"</div>"#);

    Ok(())
}
