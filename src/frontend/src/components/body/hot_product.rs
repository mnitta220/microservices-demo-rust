use crate::rpc::hipstershop::Product;

pub struct HotProduct {}

impl HotProduct {
    pub fn write(product: &Product, buf: &mut String) {
        let money = product.price_usd.as_ref().unwrap();
        buf.push_str(r#"<div class="col-md-4 hot-product-card">"#);
        {
            buf.push_str(r#"<a href="/product/"#);
            buf.push_str(&product.id);
            buf.push_str(r#"">"#);
            {
                buf.push_str(r#"<img alt="" src=""#);
                buf.push_str(&product.picture);
                buf.push_str(r#"">"#);
                buf.push_str(r#"<div class="hot-product-card-img-overlay"></div>"#);
            }
            buf.push_str(r#"</a>"#);

            buf.push_str(r#"<div>"#);
            {
                buf.push_str(r#"<div class="hot-product-card-name">"#);
                buf.push_str(&product.name);
                buf.push_str(r#"</div>"#);

                buf.push_str(r#"<div class="hot-product-card-price">"#);
                buf.push_str(&money.money_for_display());
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);
    }
}
