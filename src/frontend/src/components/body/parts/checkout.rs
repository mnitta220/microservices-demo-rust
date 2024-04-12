use crate::{components::Component, Props};
use chrono::prelude::*;

pub struct CheckoutForm {}

impl Component for CheckoutForm {
    fn write(&self, _props: &Props, buf: &mut String) {
        *buf += r#"<form class="cart-checkout-form" action="/cart/checkout" method="POST">"#;
        {
            *buf += r#"<div class="row">"#;
            {
                *buf += r#"<div class="col">"#;
                {
                    *buf += r#"<h3>Shipping Address</h3>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="form-row">"#;
            {
                *buf += r#"<div class="col cymbal-form-field">"#;
                {
                    *buf += r#"<label for="email">E-mail Address</label>"#;
                    *buf += r#"<input type="email" id="email" name="email" value="someone@example.com" required>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="form-row">"#;
            {
                *buf += r#"<div class="col cymbal-form-field">"#;
                {
                    *buf += r#"<label for="street_address">Street Address</label>"#;
                    *buf += r#"<input type="text" name="street_address" id="street_address" value="1600 Amphitheatre Parkway" required>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="form-row">"#;
            {
                *buf += r#"<div class="col cymbal-form-field">"#;
                {
                    *buf += r#"<label for="zip_code">Zip Code</label>"#;
                    *buf += r#"<input type="text" name="zip_code" id="zip_code" value="94043" required pattern="\d{4,5}">"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="form-row">"#;
            {
                *buf += r#"<div class="col cymbal-form-field">"#;
                {
                    *buf += r#"<label for="city">City</label>"#;
                    *buf += r#"<input type="text" name="city" id="city" value="Mountain View" required>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="form-row">"#;
            {
                *buf += r#"<div class="col-md-5 cymbal-form-field">"#;
                {
                    *buf += r#"<label for="state">State</label>"#;
                    *buf += r#"<input type="text" name="state" id="state" value="CA" required>"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div class="col-md-7 cymbal-form-field">"#;
                {
                    *buf += r#"<label for="country">Country</label>"#;
                    *buf += r#"<input type="text" id="country" placeholder="Country Name" name="country" value="United States" required>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="row">"#;
            {
                *buf += r#"<div class="col">"#;
                {
                    *buf += r#"<h3 class="payment-method-heading">Payment Method</h3>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="form-row">"#;
            {
                *buf += r#"<div class="col cymbal-form-field">"#;
                {
                    *buf += r#"<label for="credit_card_number">Credit Card Number</label>"#;
                    *buf += r#"<input type="text" id="credit_card_number" name="credit_card_number" placeholder="0000-0000-0000-0000""#;
                    *buf += r#" value="4432-8015-6152-0454" required pattern="\d{4}-\d{4}-\d{4}-\d{4}">"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="form-row">"#;
            {
                *buf += r#"<div class="col-md-5 cymbal-form-field">"#;
                {
                    *buf += r#"<label for="credit_card_expiration_month">Month</label>"#;
                    *buf += r#"<select name="credit_card_expiration_month" id="credit_card_expiration_month">"#;
                    {
                        *buf += r#"<option value="1">January</option>"#;
                        *buf += r#"<option value="2">February</option>"#;
                        *buf += r#"<option value="3">March</option>"#;
                        *buf += r#"<option value="4">April</option>"#;
                        *buf += r#"<option value="5">May</option>"#;
                        *buf += r#"<option value="6">June</option>"#;
                        *buf += r#"<option value="7">July</option>"#;
                        *buf += r#"<option value="8">August</option>"#;
                        *buf += r#"<option value="9">September</option>"#;
                        *buf += r#"<option value="10">October</option>"#;
                        *buf += r#"<option value="11">November</option>"#;
                        *buf += r#"<option value="12">January</option>"#;
                    }
                    *buf += r#"</select>"#;
                    *buf += r#"<img src="/static/icons/Hipster_DownArrow.svg" alt="" class="cymbal-dropdown-chevron">"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div class="col-md-4 cymbal-form-field">"#;
                {
                    *buf += r#"<label for="credit_card_expiration_year">Year</label>"#;

                    *buf += r#"<select name="credit_card_expiration_year" id="credit_card_expiration_year">"#;
                    {
                        let now = Local::now();
                        let year = now.year();
                        let mut y = year;
                        loop {
                            if y > year + 4 {
                                break;
                            }
                            *buf += r#"<option value=""#;
                            *buf += &y.to_string();
                            *buf += r#"""#;
                            if y == year + 1 {
                                *buf += r#" selected="selected""#;
                            }
                            *buf += r#">"#;
                            *buf += &y.to_string();
                            *buf += r#"</option>"#;
                            y += 1;
                        }
                    }
                    *buf += r#"</select>"#;

                    *buf += r#"<img src="/static/icons/Hipster_DownArrow.svg" alt="" class="cymbal-dropdown-chevron">"#;
                }
                *buf += r#"</div>"#;

                *buf += r#"<div class="col-md-3 cymbal-form-field">"#;
                {
                    *buf += r#"<label for="credit_card_cvv">CVV</label>"#;
                    *buf += r#"<input type="password" id="credit_card_cvv" name="credit_card_cvv" value="672" required pattern="\d{3}">"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;

            *buf += r#"<div class="form-row justify-content-center">"#;
            {
                *buf += r#"<div class="col text-center">"#;
                {
                    *buf += r#"<button class="cymbal-button-primary" type="submit">"#;
                    *buf += r#"Place Order"#;
                    *buf += r#"</button>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</form>"#;
    }
}
