use super::super::Component;
use super::parts::{checkout::CheckoutForm, footer::Footer, header::BodyHeader};
use crate::PageProps;
use anyhow::Result;

pub struct CartBody {
    pub header: Box<dyn Component + Send>,
    pub checkout_form: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl CartBody {
    pub fn load() -> Result<Box<Self>> {
        let body_header = match BodyHeader::load() {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let checkout_form = CheckoutForm {};
        let footer = Footer {};

        let body = CartBody {
            header: Box::new(body_header),
            checkout_form: Box::new(checkout_form),
            footer: Box::new(footer),
        };

        Ok(Box::new(body))
    }
}

impl Component for CartBody {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        buf.push_str(r#"<body>"#);
        {
            self.header.write(props, buf)?;

            buf.push_str(r#"<main role="main" class="cart-sections">"#);
            {
                if let Some(cart) = &props.cart {
                    if cart.items.len() == 0 {
                        buf.push_str(r#"<section class="empty-cart-section">"#);
                        {
                            buf.push_str(r#"<h3>Your shopping cart is empty!</h3>"#);
                            buf.push_str(
                                r#"<p>Items you add to your shopping cart will appear here.</p>"#,
                            );
                            buf.push_str(r#"<a class="cymbal-button-primary" href="/" role="button">Continue Shopping</a>"#);
                        }
                        buf.push_str(r#"</section>"#);
                    } else {
                        buf.push_str(r#"<section class="container">"#);
                        {
                            buf.push_str(r#"<div class="row">"#);
                            {
                                buf.push_str(
                            r#"<div class="col-lg-6 col-xl-5 offset-xl-1 cart-summary-section">"#,
                            );
                                {
                                    buf.push_str(r#"<div class="row mb-3 py-2">"#);
                                    {
                                        buf.push_str(r#"<div class="col-4 pl-md-0">"#);
                                        {
                                            buf.push_str(r#"<h3>Cart ("#);
                                            buf.push_str(&cart.cart_size().to_string());
                                            buf.push_str(r#")</h3>"#);
                                        }
                                        buf.push_str(r#"</div>"#);

                                        buf.push_str(r#"<div class="col-8 pr-md-0 text-right">"#);
                                        {
                                            buf.push_str(
                                                r#"<form method="POST" action="/cart/empty">"#,
                                            );
                                            {
                                                buf.push_str(r#"<button class="cymbal-button-secondary cart-summary-empty-cart-button" type="submit">"#);
                                                buf.push_str(r#"Empty Cart"#);
                                                buf.push_str(r#"</button>"#);
                                                buf.push_str(
                                            r#"<a class="cymbal-button-primary" href="/" role="button">"#,
                                            );
                                                buf.push_str(r#"Continue Shopping"#);
                                                buf.push_str(r#"</a>"#);
                                            }
                                            buf.push_str(r#"</form>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);

                                    for item in &cart.items {
                                        item.write(props, buf)?;
                                    }

                                    buf.push_str(r#"<div class="row cart-summary-shipping-row">"#);
                                    {
                                        buf.push_str(r#"<div class="col pl-md-0">Shipping</div>"#);
                                        buf.push_str(r#"<div class="col pr-md-0 text-right">"#);
                                        buf.push_str(&cart.shipping_cost.money_for_display());
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);

                                    buf.push_str(r#"<div class="row cart-summary-total-row">"#);
                                    {
                                        buf.push_str(r#"<div class="col pl-md-0">Total</div>"#);
                                        buf.push_str(r#"<div class="col pr-md-0 text-right">"#);
                                        buf.push_str(&cart.total_price.money_for_display());
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                }
                                buf.push_str(r#"</div>"#);

                                buf.push_str(r#"<div class="col-lg-5 offset-lg-1 col-xl-4">"#);
                                {
                                    self.checkout_form.write(props, buf)?;
                                }
                                buf.push_str(r#"</div>"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</section>"#);
                    }
                }
            }
            buf.push_str(r#"</main>"#);

            if let Some(recommendations) = &props.recommendations {
                recommendations.write(props, buf)?
            }

            self.footer.write(props, buf)?;
        }
        buf.push_str(r#"</body>"#);

        Ok(())
    }
}
