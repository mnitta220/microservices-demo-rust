use crate::{components::Component, model, rpc, PageProps};
use anyhow::Result;

impl Component for model::hot_product::HotProducts {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        for product in &self.products {
            product.write(props, buf)?;
        }

        Ok(())
    }
}

impl Component for rpc::hipstershop::Product {
    fn write(&self, _props: &PageProps, buf: &mut String) -> Result<()> {
        let money = self.price_usd.as_ref().unwrap();

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
                buf.push_str(&money.money_for_display());
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);

        Ok(())
    }
}
