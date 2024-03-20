pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use super::currency;
use crate::PageProps;
use hipstershop::currency_service_client::CurrencyServiceClient;
use hipstershop::product_catalog_service_client::ProductCatalogServiceClient;
use hipstershop::{CurrencyConversionRequest, Empty, GetProductRequest, Money, Product};
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
                buf.push_str(currency::currency_logo(money.currency_code.as_str()));
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

async fn get_product_catalog_service_client(
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
            return Err("get_products: connect failed");
        }
    };

    Ok(product_catalog_service_client)
}

async fn get_currency_service_client(
) -> Result<CurrencyServiceClient<tonic::transport::Channel>, &'static str> {
    let currency_service_addr = match env::var("CURRENCY_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get CURRENCY_SERVICE_ADDR");
        }
    };

    let currency_service_client =
        match CurrencyServiceClient::connect(format!("http://{}", currency_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err("get_currencies: connect failed");
            }
        };

    Ok(currency_service_client)
}

pub async fn get_products(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
    let mut product_catalog_service_client = match get_product_catalog_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let mut currency_service_client = match get_currency_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
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
            if product.price_usd.as_ref().unwrap().currency_code != page_props.user_currency {
                let request = CurrencyConversionRequest {
                    from: product.price_usd.clone(),
                    to_code: page_props.user_currency.clone(),
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

pub async fn get_product(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
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

    let mut currency_service_client = match get_currency_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let request = GetProductRequest { id: product_id };

    let product = match product_catalog_service_client.get_product(request).await {
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

    buf.push_str(r#"<div class="h-product container">"#);
    {
        buf.push_str(r#"<div class="row">"#);
        {
            buf.push_str(r#"<div class="col-md-6">"#);
            {
                buf.push_str(r#"<img class="product-image" alt src=""#);
                buf.push_str(&product.picture);
                buf.push_str(r#"" />"#);
            }
            buf.push_str(r#"</div>"#);
            buf.push_str(r#"<div class="product-info col-md-5">"#);
            {
                buf.push_str(r#"<div class="product-wrapper">"#);
                {
                    buf.push_str(r#"<h2>"#);
                    buf.push_str(&product.name);
                    buf.push_str(r#"</h2>"#);
                    buf.push_str(r#"<p class="product-price">"#);
                    buf.push_str(currency::currency_logo(price.currency_code.as_str()));
                    buf.push_str(price.units.to_string().as_str());
                    buf.push_str(".");
                    buf.push_str(format!("{:.2}", price.nanos / 10000000).as_str());
                    buf.push_str(r#"</p>"#);
                    buf.push_str(r#"<p>"#);
                    buf.push_str(&product.description);
                    buf.push_str(r#"</p>"#);
                    buf.push_str(r#"<form method="POST" action="/cart">"#);
                    {
                        buf.push_str(r#"<input type="hidden" name="product_id" value=""#);
                        buf.push_str(&product.id);
                        buf.push_str(r#"" />"#);
                        buf.push_str(r#"<div class="product-quantity-dropdown">"#);
                        {
                            buf.push_str(r#"<select name="quantity" id="quantity">"#);
                            {
                                buf.push_str(r#"<option>1</option>"#);
                                buf.push_str(r#"<option>2</option>"#);
                                buf.push_str(r#"<option>3</option>"#);
                                buf.push_str(r#"<option>4</option>"#);
                                buf.push_str(r#"<option>5</option>"#);
                                buf.push_str(r#"<option>10</option>"#);
                            }
                            buf.push_str(r#"</select>"#);
                            buf.push_str(r#"<img src="/static/icons/Hipster_DownArrow.svg" alt>"#);
                        }
                        buf.push_str(r#"</div>"#);
                        buf.push_str(r#"<button type="submit" class="cymbal-button-primary">Add To Cart</button>"#);
                    }
                    buf.push_str(r#"</form>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);
    }
    buf.push_str(r#"</div>"#);

    Ok(())
}
