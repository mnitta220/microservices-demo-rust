use crate::{components::Component, PageProps};
use anyhow::Result;
use chrono::prelude::*;

pub struct CheckoutForm {}

impl Component for CheckoutForm {
    fn write(&self, _props: &PageProps, buf: &mut String) -> Result<()> {
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
                    buf.push_str(r#"<label for="email">E-mail Address</label>"#);
                    buf.push_str(r#"<input type="email" id="email" name="email" value="someone@example.com" required>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);

            buf.push_str(r#"<div class="form-row">"#);
            {
                buf.push_str(r#"<div class="col cymbal-form-field">"#);
                {
                    buf.push_str(r#"<label for="street_address">Street Address</label>"#);
                    buf.push_str(r#"<input type="text" name="street_address" id="street_address" value="1600 Amphitheatre Parkway" required>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);

            buf.push_str(r#"<div class="form-row">"#);
            {
                buf.push_str(r#"<div class="col cymbal-form-field">"#);
                {
                    buf.push_str(r#"<label for="zip_code">Zip Code</label>"#);
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
                    buf.push_str(
                        r#"<input type="text" name="state" id="state" value="CA" required>"#,
                    );
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
                    buf.push_str(r#"<h3 class="payment-method-heading">Payment Method</h3>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);

            buf.push_str(r#"<div class="form-row">"#);
            {
                buf.push_str(r#"<div class="col cymbal-form-field">"#);
                {
                    buf.push_str(r#"<label for="credit_card_number">Credit Card Number</label>"#);
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
                    buf.push_str(r#"<label for="credit_card_expiration_month">Month</label>"#);
                    buf.push_str(r#"<select name="credit_card_expiration_month" id="credit_card_expiration_month">"#);
                    {
                        buf.push_str(r#"<option value="1">January</option>"#);
                        buf.push_str(r#"<option value="2">February</option>"#);
                        buf.push_str(r#"<option value="3">March</option>"#);
                        buf.push_str(r#"<option value="4">April</option>"#);
                        buf.push_str(r#"<option value="5">May</option>"#);
                        buf.push_str(r#"<option value="6">June</option>"#);
                        buf.push_str(r#"<option value="7">July</option>"#);
                        buf.push_str(r#"<option value="8">August</option>"#);
                        buf.push_str(r#"<option value="9">September</option>"#);
                        buf.push_str(r#"<option value="10">October</option>"#);
                        buf.push_str(r#"<option value="11">November</option>"#);
                        buf.push_str(r#"<option value="12">January</option>"#);
                    }
                    buf.push_str(r#"</select>"#);
                    buf.push_str(r#"<img src="/static/icons/Hipster_DownArrow.svg" alt="" class="cymbal-dropdown-chevron">"#);
                }
                buf.push_str(r#"</div>"#);

                buf.push_str(r#"<div class="col-md-4 cymbal-form-field">"#);
                {
                    buf.push_str(r#"<label for="credit_card_expiration_year">Year</label>"#);
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
                    buf.push_str(r#"<label for="credit_card_cvv">CVV</label>"#);
                    buf.push_str(r#"<input type="password" id="credit_card_cvv" name="credit_card_cvv" value="672" required pattern="\d{3}">"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);

            buf.push_str(r#"<div class="form-row justify-content-center">"#);
            {
                buf.push_str(r#"<div class="col text-center">"#);
                {
                    buf.push_str(r#"<button class="cymbal-button-primary" type="submit">"#);
                    buf.push_str(r#"Place Order"#);
                    buf.push_str(r#"</button>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</form>"#);

        Ok(())
    }
}
