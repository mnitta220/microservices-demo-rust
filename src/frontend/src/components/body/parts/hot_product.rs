use crate::{components::Component, model, rpc, Props};

impl Component for model::hot_product::HotProducts {
    fn write(&self, props: &Props, buf: &mut String) {
        for product in &self.products {
            product.write(props, buf);
        }
    }
}

impl Component for rpc::hipstershop::Product {
    fn write(&self, _props: &Props, buf: &mut String) {
        let money = self.price_usd.as_ref().unwrap();

        *buf += r#"<div class="col-md-4 hot-product-card">"#;
        {
            *buf += r#"<a href="/product/"#;
            *buf += &self.id;
            *buf += r#"">"#;
            {
                *buf += r#"<img alt="" src=""#;
                *buf += &self.picture;
                *buf += r#"">"#;
                *buf += r#"<div class="hot-product-card-img-overlay"></div>"#;
            }
            *buf += r#"</a>"#;

            *buf += r#"<div>"#;
            {
                *buf += r#"<div class="hot-product-card-name">"#;
                *buf += &self.name;
                *buf += r#"</div>"#;

                *buf += r#"<div class="hot-product-card-price">"#;
                *buf += &money.money_for_display();
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</div>"#;
    }
}
