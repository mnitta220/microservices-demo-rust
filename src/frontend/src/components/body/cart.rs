use super::super::Component;
use super::parts::{checkout::CheckoutForm, footer::Footer, header::BodyHeader};
use crate::Props;

pub struct CartBody {
    pub header: Box<dyn Component + Send>,
    pub checkout_form: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl CartBody {
    pub fn new() -> Self {
        let body_header = BodyHeader::new();
        let checkout_form = CheckoutForm {};
        let footer = Footer {};

        CartBody {
            header: Box::new(body_header),
            checkout_form: Box::new(checkout_form),
            footer: Box::new(footer),
        }
    }
}

impl Component for CartBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            self.header.write(props, buf);

            *buf += r#"<main role="main" class="cart-sections">"#;
            {
                if let Some(cart) = &props.cart {
                    if cart.items.len() == 0 {
                        *buf += r#"<section class="empty-cart-section">"#;
                        {
                            *buf += r#"<h3>Your shopping cart is empty!</h3>"#;
                            *buf +=
                                r#"<p>Items you add to your shopping cart will appear here.</p>"#;
                            *buf += r#"<a class="cymbal-button-primary" href="/" role="button">Continue Shopping</a>"#;
                        }
                        *buf += r#"</section>"#;
                    } else {
                        *buf += r#"<section class="container">"#;
                        {
                            *buf += r#"<div class="row">"#;
                            {
                                *buf += r#"<div class="col-lg-6 col-xl-5 offset-xl-1 cart-summary-section">"#;
                                {
                                    *buf += r#"<div class="row mb-3 py-2">"#;
                                    {
                                        *buf += r#"<div class="col-4 pl-md-0">"#;
                                        {
                                            *buf += r#"<h3>Cart ("#;
                                            *buf += &cart.cart_size().to_string();
                                            *buf += r#")</h3>"#;
                                        }
                                        *buf += r#"</div>"#;

                                        *buf += r#"<div class="col-8 pr-md-0 text-right">"#;
                                        {
                                            *buf += r#"<form method="POST" action="/cart/empty">"#;
                                            {
                                                *buf += r#"<button class="cymbal-button-secondary cart-summary-empty-cart-button" type="submit">"#;
                                                *buf += r#"Empty Cart"#;
                                                *buf += r#"</button>"#;
                                                *buf += r#"<a class="cymbal-button-primary" href="/" role="button">"#;
                                                *buf += r#"Continue Shopping"#;
                                                *buf += r#"</a>"#;
                                            }
                                            *buf += r#"</form>"#;
                                        }
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    for item in &cart.items {
                                        item.write(props, buf);
                                    }

                                    *buf += r#"<div class="row cart-summary-shipping-row">"#;
                                    {
                                        *buf += r#"<div class="col pl-md-0">Shipping</div>"#;
                                        *buf += r#"<div class="col pr-md-0 text-right">"#;
                                        *buf += &cart.shipping_cost.money_for_display();
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;

                                    *buf += r#"<div class="row cart-summary-total-row">"#;
                                    {
                                        *buf += r#"<div class="col pl-md-0">Total</div>"#;
                                        *buf += r#"<div class="col pr-md-0 text-right">"#;
                                        *buf += &cart.total_price.money_for_display();
                                        *buf += r#"</div>"#;
                                    }
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;

                                *buf += r#"<div class="col-lg-5 offset-lg-1 col-xl-4">"#;
                                {
                                    self.checkout_form.write(props, buf);
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</section>"#;
                    }
                }
            }
            *buf += r#"</main>"#;

            if let Some(recommendations) = &props.recommendations {
                recommendations.write(props, buf)
            }

            self.footer.write(props, buf);
        }
        *buf += r#"</body>"#;
    }
}
