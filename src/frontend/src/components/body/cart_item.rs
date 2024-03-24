use crate::CartItemView;

pub struct CartItem {}

impl CartItem {
    pub fn write(item: &CartItemView, buf: &mut String) {
        if let Some(m) = &item.product.price_usd {
            buf.push_str(r#"<div class="row cart-summary-item-row">"#);
            {
                buf.push_str(r#"<div class="col-md-4 pl-md-0">"#);
                {
                    buf.push_str(r#"<a href="/product/"#);
                    buf.push_str(&item.product.id);
                    buf.push_str(r#"">"#);
                    {
                        buf.push_str(r#"<img class="img-fluid" alt="" src=""#);
                        buf.push_str(&item.product.picture);
                        buf.push_str(r#"" />"#);
                    }
                    buf.push_str(r#"</a>"#);
                }
                buf.push_str(r#"</div>"#);

                buf.push_str(r#"<div class="col-md-8 pr-md-0">"#);
                {
                    buf.push_str(r#"<div class="row">"#);
                    {
                        buf.push_str(r#"<div class="col">"#);
                        {
                            buf.push_str(r#"<h4>"#);
                            buf.push_str(&item.product.name);
                            buf.push_str(r#"</h4>"#);
                        }
                        buf.push_str(r#"</div>"#);
                    }
                    buf.push_str(r#"</div>"#);

                    buf.push_str(r#"<div class="row cart-summary-item-row-item-id-row">"#);
                    {
                        buf.push_str(r#"<div class="col">"#);
                        {
                            buf.push_str(r#"SKU #"#);
                            buf.push_str(&item.product.id);
                        }
                        buf.push_str(r#"</div>"#);
                    }
                    buf.push_str(r#"</div>"#);

                    buf.push_str(r#"<div class="row">"#);
                    {
                        buf.push_str(r#"<div class="col">"#);
                        {
                            buf.push_str(r#"Quantity: "#);
                            buf.push_str(&item.quantity.to_string());
                        }
                        buf.push_str(r#"</div>"#);

                        buf.push_str(r#"<div class="col pr-md-0 text-right">"#);
                        {
                            buf.push_str(r#"<strong>"#);
                            buf.push_str(&m.money_for_display());
                            buf.push_str(r#"</strong>"#);
                        }
                        buf.push_str(r#"</div>"#);
                    }
                    buf.push_str(r#"</div>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
    }
}
