use crate::{components::Component, model, PageProps};
use anyhow::Result;

impl Component for model::hot_product::HotProducts {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        for item in &self.items {
            item.write(props, buf)?;
        }
        Ok(())
    }
}

impl Component for model::hot_product::HotProductItem {
    fn write(&self, _props: &PageProps, buf: &mut String) -> Result<()> {
        let money = self.product.price_usd.as_ref().unwrap();
        buf.push_str(r#"<div class="col-md-4 hot-product-card">"#);
        {
            buf.push_str(r#"<a href="/product/"#);
            buf.push_str(&self.product.id);
            buf.push_str(r#"">"#);
            {
                buf.push_str(r#"<img alt="" src=""#);
                buf.push_str(&self.product.picture);
                buf.push_str(r#"">"#);
                buf.push_str(r#"<div class="hot-product-card-img-overlay"></div>"#);
            }
            buf.push_str(r#"</a>"#);

            buf.push_str(r#"<div>"#);
            {
                buf.push_str(r#"<div class="hot-product-card-name">"#);
                buf.push_str(&self.product.name);
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
