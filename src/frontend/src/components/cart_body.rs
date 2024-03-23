use crate::{components, rpc::recommendation, Body, Component, PageProps};
use anyhow::Result;
use chrono::prelude::*;

pub struct CartBody {
    pub body_header: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub recommendations: Box<dyn Component + Send>,
}

impl Body for CartBody {
    async fn load(props: &PageProps) -> Result<Box<Self>> {
        let recommendation_list = match recommendation::get_recommendations(&props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let body_header = match components::body_header::BodyHeader::load(props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let recommendations = components::recommendations::Recommendations {
            recommendation_list,
        };

        let footer = components::body_footer::Footer {};

        let body = components::cart_body::CartBody {
            body_header: Box::new(body_header),
            footer: Box::new(footer),
            recommendations: Box::new(recommendations),
        };

        Ok(Box::new(body))
    }
}

impl Component for CartBody {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        buf.push_str(r#"<body>"#);
        {
            self.body_header.write(props, buf)?;

            buf.push_str(r#"<main role="main" class="cart-sections">"#);
            {
                if props.cart_info.cart_items.len() == 0 {
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
                                        buf.push_str(&props.cart_info.cart_size().to_string());
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

                                for item in props.cart_info.cart_items.iter() {
                                    if let Some(m) = &item.product.price_usd {
                                        buf.push_str(r#"<div class="row cart-summary-item-row">"#);
                                        {
                                            buf.push_str(r#"<div class="col-md-4 pl-md-0">"#);
                                            {
                                                buf.push_str(r#"<a href="/product/"#);
                                                buf.push_str(&item.product.id);
                                                buf.push_str(r#"">"#);
                                                {
                                                    buf.push_str(
                                                        r#"<img class="img-fluid" alt="" src=""#,
                                                    );
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
                                                buf.push_str(
                                                r#"<div class="row cart-summary-item-row-item-id-row">"#,
                                                );
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
                                                    buf.push_str(
                                                        r#"<div class="col pr-md-0 text-right">"#,
                                                    );
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
                                buf.push_str(r#"<div class="row cart-summary-shipping-row">"#);
                                {
                                    buf.push_str(r#"<div class="col pl-md-0">Shipping</div>"#);
                                    buf.push_str(r#"<div class="col pr-md-0 text-right">"#);
                                    buf.push_str(
                                        &props.cart_info.shipping_cost.money_for_display(),
                                    );
                                    buf.push_str(r#"</div>"#);
                                }
                                buf.push_str(r#"</div>"#);
                                buf.push_str(r#"<div class="row cart-summary-total-row">"#);
                                {
                                    buf.push_str(r#"<div class="col pl-md-0">Total</div>"#);
                                    buf.push_str(r#"<div class="col pr-md-0 text-right">"#);
                                    buf.push_str(&props.cart_info.total_price.money_for_display());
                                    buf.push_str(r#"</div>"#);
                                }
                                buf.push_str(r#"</div>"#);
                            }
                            buf.push_str(r#"</div>"#);
                            buf.push_str(r#"<div class="col-lg-5 offset-lg-1 col-xl-4">"#);
                            {
                                buf.push_str(r#"<form class="cart-checkout-form" action="/cart/checkout" method="POST">"#);
                                {
                                    buf.push_str(r#"<div class="row">"#);
                                    {
                                        buf.push_str(r#"<div class="col">"#);
                                        {
                                            buf.push_str(r#"<h3>Shipping Address</h3>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(r#"<div class="form-row">"#);
                                    {
                                        buf.push_str(r#"<div class="col cymbal-form-field">"#);
                                        {
                                            buf.push_str(
                                                r#"<label for="email">E-mail Address</label>"#,
                                            );
                                            buf.push_str(r#"<input type="email" id="email" name="email" value="someone@example.com" required>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(r#"<div class="form-row">"#);
                                    {
                                        buf.push_str(r#"<div class="col cymbal-form-field">"#);
                                        {
                                            buf.push_str(
                                            r#"<label for="street_address">Street Address</label>"#,
                                            );
                                            buf.push_str(r#"<input type="text" name="street_address" id="street_address" value="1600 Amphitheatre Parkway" required>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(r#"<div class="form-row">"#);
                                    {
                                        buf.push_str(r#"<div class="col cymbal-form-field">"#);
                                        {
                                            buf.push_str(
                                                r#"<label for="zip_code">Zip Code</label>"#,
                                            );
                                            buf.push_str(r#"<input type="text" name="zip_code" id="zip_code" value="94043" required pattern="\d{4,5}">"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(r#"<div class="form-row">"#);
                                    {
                                        buf.push_str(r#"<div class="col cymbal-form-field">"#);
                                        {
                                            buf.push_str(r#"<label for="city">City</label>"#);
                                            buf.push_str(r#"<input type="text" name="city" id="city" value="Mountain View" required>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(r#"<div class="form-row">"#);
                                    {
                                        buf.push_str(r#"<div class="col-md-5 cymbal-form-field">"#);
                                        {
                                            buf.push_str(r#"<label for="state">State</label>"#);
                                            buf.push_str(r#"<input type="text" name="state" id="state" value="CA" required>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                        buf.push_str(r#"<div class="col-md-7 cymbal-form-field">"#);
                                        {
                                            buf.push_str(r#"<label for="country">Country</label>"#);
                                            buf.push_str(r#"<input type="text" id="country" placeholder="Country Name" name="country" value="United States" required>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(r#"<div class="row">"#);
                                    {
                                        buf.push_str(r#"<div class="col">"#);
                                        {
                                            buf.push_str(
                                            r#"<h3 class="payment-method-heading">Payment Method</h3>"#,
                                            );
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(r#"<div class="form-row">"#);
                                    {
                                        buf.push_str(r#"<div class="col cymbal-form-field">"#);
                                        {
                                            buf.push_str(
                                            r#"<label for="credit_card_number">Credit Card Number</label>"#,
                                            );
                                            buf.push_str(r#"<input type="text" id="credit_card_number" name="credit_card_number" placeholder="0000-0000-0000-0000""#);
                                            buf.push_str(r#" value="4432-8015-6152-0454" required pattern="\d{4}-\d{4}-\d{4}-\d{4}">"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(r#"<div class="form-row">"#);
                                    {
                                        buf.push_str(r#"<div class="col-md-5 cymbal-form-field">"#);
                                        {
                                            buf.push_str(
                                            r#"<label for="credit_card_expiration_month">Month</label>"#,
                                            );
                                            buf.push_str(r#"<select name="credit_card_expiration_month" id="credit_card_expiration_month">"#);
                                            {
                                                buf.push_str(
                                                    r#"<option value="1">January</option>"#,
                                                );
                                                buf.push_str(
                                                    r#"<option value="2">February</option>"#,
                                                );
                                                buf.push_str(r#"<option value="3">March</option>"#);
                                                buf.push_str(r#"<option value="4">April</option>"#);
                                                buf.push_str(r#"<option value="5">May</option>"#);
                                                buf.push_str(r#"<option value="6">June</option>"#);
                                                buf.push_str(r#"<option value="7">July</option>"#);
                                                buf.push_str(
                                                    r#"<option value="8">August</option>"#,
                                                );
                                                buf.push_str(
                                                    r#"<option value="9">September</option>"#,
                                                );
                                                buf.push_str(
                                                    r#"<option value="10">October</option>"#,
                                                );
                                                buf.push_str(
                                                    r#"<option value="11">November</option>"#,
                                                );
                                                buf.push_str(
                                                    r#"<option value="12">January</option>"#,
                                                );
                                            }
                                            buf.push_str(r#"</select>"#);
                                            buf.push_str(r#"<img src="/static/icons/Hipster_DownArrow.svg" alt="" class="cymbal-dropdown-chevron">"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                        buf.push_str(r#"<div class="col-md-4 cymbal-form-field">"#);
                                        {
                                            buf.push_str(
                                            r#"<label for="credit_card_expiration_year">Year</label>"#,
                                            );
                                            buf.push_str(r#"<select name="credit_card_expiration_year" id="credit_card_expiration_year">"#);
                                            {
                                                let now = Local::now();
                                                let year = now.year();
                                                let mut y = year;
                                                loop {
                                                    if y > year + 4 {
                                                        break;
                                                    }
                                                    buf.push_str(r#"<option value=""#);
                                                    buf.push_str(&y.to_string());
                                                    buf.push_str(r#"""#);
                                                    if y == year + 1 {
                                                        buf.push_str(r#" selected="selected""#);
                                                    }
                                                    buf.push_str(r#">"#);
                                                    buf.push_str(&y.to_string());
                                                    buf.push_str(r#"</option>"#);
                                                    y += 1;
                                                }
                                            }
                                            buf.push_str(r#"</select>"#);
                                            buf.push_str(r#"<img src="/static/icons/Hipster_DownArrow.svg" alt="" class="cymbal-dropdown-chevron">"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                        buf.push_str(r#"<div class="col-md-3 cymbal-form-field">"#);
                                        {
                                            buf.push_str(
                                                r#"<label for="credit_card_cvv">CVV</label>"#,
                                            );
                                            buf.push_str(r#"<input type="password" id="credit_card_cvv" name="credit_card_cvv" value="672" required pattern="\d{3}">"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(
                                        r#"<div class="form-row justify-content-center">"#,
                                    );
                                    {
                                        buf.push_str(r#"<div class="col text-center">"#);
                                        {
                                            buf.push_str(
                                            r#"<button class="cymbal-button-primary" type="submit">"#,
                                            );
                                            buf.push_str(r#"Place Order"#);
                                            buf.push_str(r#"</button>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                }
                                buf.push_str(r#"</form>"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);
                    }
                    buf.push_str(r#"</section>"#);
                }
            }
            buf.push_str(r#"</main>"#);

            self.recommendations.write(props, buf)?;
            self.footer.write(props, buf)?;
        }
        buf.push_str(r#"</body>"#);

        Ok(())
    }
}
