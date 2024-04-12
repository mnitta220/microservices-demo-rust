use crate::{components::Component, model, Props};

impl Component for model::cart::CartItem {
    fn write(&self, _props: &Props, buf: &mut String) {
        if let Some(m) = &self.product.price_usd {
            *buf += r#"<div class="row cart-summary-item-row">"#;
            {
                *buf += r#"<div class="col-md-4 pl-md-0">"#;
                {
                    *buf += r#"<a href="/product/"#;
                    *buf += &self.product.id;
                    *buf += r#"">"#;
                    {
                        *buf += r#"<img class="img-fluid" alt="" src=""#;
                        *buf += &self.product.picture;
                        *buf += r#"" />"#;
                    }
                    *buf += r#"</a>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div class="col-md-8 pr-md-0">"#;
                {
                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col">"#;
                        {
                            *buf += r#"<h4>"#;
                            *buf += &self.product.name;
                            *buf += r#"</h4>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="row cart-summary-item-row-item-id-row">"#;
                    {
                        *buf += r#"<div class="col">"#;
                        {
                            *buf += r#"SKU #"#;
                            *buf += &self.product.id;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;

                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col">"#;
                        {
                            *buf += r#"Quantity: "#;
                            *buf += &self.quantity.to_string();
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="col pr-md-0 text-right">"#;
                        {
                            *buf += r#"<strong>"#;
                            *buf += &m.money_for_display();
                            *buf += r#"</strong>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
    }
}
