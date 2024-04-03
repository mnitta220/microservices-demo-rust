use super::super::Component;
use super::parts::{footer::Footer, header::BodyHeader};
use crate::PageProps;

pub struct ProductBody {
    pub body_header: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl ProductBody {
    pub fn new() -> Self {
        let body_header = BodyHeader::new();
        let footer = Footer {};

        ProductBody {
            body_header: Box::new(body_header),
            footer: Box::new(footer),
        }
    }
}

impl Component for ProductBody {
    fn write(&self, props: &PageProps, buf: &mut String) {
        if let Some(product) = &props.product {
            let money = product.product.price_usd.as_ref().unwrap();

            buf.push_str(r#"<body>"#);
            {
                self.body_header.write(props, buf);

                buf.push_str(r#"<main role="main">"#);
                {
                    buf.push_str(r#"<div class="h-product container">"#);
                    {
                        buf.push_str(r#"<div class="row">"#);
                        {
                            buf.push_str(r#"<div class="col-md-6">"#);
                            {
                                buf.push_str(r#"<img class="product-image" alt src=""#);
                                buf.push_str(&product.product.picture);
                                buf.push_str(r#"" />"#);
                            }
                            buf.push_str(r#"</div>"#);

                            buf.push_str(r#"<div class="product-info col-md-5">"#);
                            {
                                buf.push_str(r#"<div class="product-wrapper">"#);
                                {
                                    buf.push_str(r#"<h2>"#);
                                    buf.push_str(&product.product.name);
                                    buf.push_str(r#"</h2>"#);

                                    buf.push_str(r#"<p class="product-price">"#);
                                    buf.push_str(&money.money_for_display());
                                    buf.push_str(r#"</p>"#);

                                    buf.push_str(r#"<p>"#);
                                    buf.push_str(&product.product.description);
                                    buf.push_str(r#"</p>"#);

                                    buf.push_str(r#"<form method="POST" action="/cart">"#);
                                    {
                                        buf.push_str(
                                            r#"<input type="hidden" name="product_id" value=""#,
                                        );
                                        buf.push_str(&product.product.id);
                                        buf.push_str(r#"" />"#);

                                        buf.push_str(r#"<div class="product-quantity-dropdown">"#);
                                        {
                                            buf.push_str(
                                                r#"<select name="quantity" id="quantity">"#,
                                            );
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

                    if let Some(recommendations) = &props.recommendations {
                        buf.push_str(r#"<div>"#);
                        {
                            recommendations.write(props, buf)
                        }
                        buf.push_str(r#"</div>"#);
                    }

                    if let Some(ad) = &props.ad {
                        ad.write(props, buf);
                    }
                }
                buf.push_str(r#"</main>"#);

                self.footer.write(props, buf);
            }
            buf.push_str(r#"</body>"#);
        }
    }
}
