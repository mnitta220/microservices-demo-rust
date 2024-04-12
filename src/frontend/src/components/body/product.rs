use super::super::Component;
use super::parts::{footer::Footer, header::BodyHeader};
use crate::Props;

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
    fn write(&self, props: &Props, buf: &mut String) {
        if let Some(product) = &props.product {
            let money = product.product.price_usd.as_ref().unwrap();

            *buf += r#"<body>"#;
            {
                self.body_header.write(props, buf);

                *buf += r#"<main role="main">"#;
                {
                    *buf += r#"<div class="h-product container">"#;
                    {
                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<div class="col-md-6">"#;
                            {
                                *buf += r#"<img class="product-image" alt src=""#;
                                *buf += &product.product.picture;
                                *buf += r#"" />"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="product-info col-md-5">"#;
                            {
                                *buf += r#"<div class="product-wrapper">"#;
                                {
                                    *buf += r#"<h2>"#;
                                    *buf += &product.product.name;
                                    *buf += r#"</h2>"#;

                                    *buf += r#"<p class="product-price">"#;
                                    *buf += &money.money_for_display();
                                    *buf += r#"</p>"#;

                                    *buf += r#"<p>"#;
                                    *buf += &product.product.description;
                                    *buf += r#"</p>"#;

                                    *buf += r#"<form method="POST" action="/cart">"#;
                                    {
                                        *buf += r#"<input type="hidden" name="product_id" value=""#;
                                        *buf += &product.product.id;
                                        *buf += r#"" />"#;

                                        *buf += r#"<div class="product-quantity-dropdown">"#;
                                        {
                                            *buf += r#"<select name="quantity" id="quantity">"#;
                                            {
                                                *buf += r#"<option>1</option>"#;
                                                *buf += r#"<option>2</option>"#;
                                                *buf += r#"<option>3</option>"#;
                                                *buf += r#"<option>4</option>"#;
                                                *buf += r#"<option>5</option>"#;
                                                *buf += r#"<option>10</option>"#;
                                            }
                                            *buf += r#"</select>"#;
                                            *buf += r#"<img src="/static/icons/Hipster_DownArrow.svg" alt>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<button type="submit" class="cymbal-button-primary">Add To Cart</button>"#;
                                    }
                                    *buf += r#"</form>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;

                    if let Some(recommendations) = &props.recommendations {
                        *buf += r#"<div>"#;
                        {
                            recommendations.write(props, buf)
                        }
                        *buf += r#"</div>"#;
                    }

                    if let Some(ad) = &props.ad {
                        ad.write(props, buf);
                    }
                }
                *buf += r#"</main>"#;

                self.footer.write(props, buf);
            }
            *buf += r#"</body>"#;
        }
    }
}
