use super::super::rpc::{products, recommendation};
use super::super::PageType;
use super::{body_footer, script};
use crate::PageProps;

pub async fn write(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
    match page_props.page_type {
        PageType::Home => write_main_home(buf, page_props).await,
        PageType::Product => write_main_product(buf, page_props).await,
        PageType::Cart => write_main_cart(buf, page_props).await,
    }
}

async fn write_main_home(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
    buf.push_str(r#"<main role="main" class="home">"#);
    {
        buf.push_str(r#"<div class="home-mobile-hero-banner d-lg-none"></div>"#);
        buf.push_str(r#"<div class="container-fluid">"#);
        {
            buf.push_str(r#"<div class="row">"#);
            {
                buf.push_str(
                    r#"<div class="col-4 d-none d-lg-block home-desktop-left-image"></div>"#,
                );
                buf.push_str(r#"<div class="col-12 col-lg-8">"#);
                {
                    if let Err(e) = products::get_products(buf, page_props).await {
                        return Err(e);
                    }
                    buf.push_str(r#"<div class="row d-none d-lg-block home-desktop-footer-row">"#);
                    {
                        buf.push_str(r#"<div class="col-12 p-0">"#);
                        {
                            if let Err(e) = body_footer::write(buf, page_props) {
                                return Err(e);
                            }
                            if let Err(e) = script::write(buf) {
                                return Err(e);
                            }
                        }
                        buf.push_str(r#"</div>"#);
                    }
                    buf.push_str(r#"</div>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);
    }
    buf.push_str(r#"</main>"#);

    Ok(())
}

async fn write_main_product(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
    buf.push_str(r#"<main role="main">"#);
    {
        if let Err(e) = products::get_product(buf, page_props).await {
            return Err(e);
        }
        buf.push_str(r#"<div>"#);
        if let Err(e) = recommendation::get_recommendations(buf, page_props).await {
            return Err(e);
        }
        buf.push_str(r#"</div>"#);

        buf.push_str(r#"<div class="ad">"#);
        {
            buf.push_str(r#"<div class="container py-3 px-lg-5 py-lg-5">"#);
            {
                buf.push_str(r#"<div role="alert">"#);
                {
                    buf.push_str(r#"<strong>Ad</strong>"#);
                    buf.push_str(
                        r#"<a href="/product/1YMWWN1N4O" rel="nofollow" target="_blank">"#,
                    );
                    buf.push_str(r#"Watch for sale. Buy one, get second kit for free"#);
                    buf.push_str(r#"</a>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);
    }
    buf.push_str(r#"</main>"#);
    if let Err(e) = body_footer::write(buf, page_props) {
        return Err(e);
    }
    if let Err(e) = script::write(buf) {
        return Err(e);
    }

    Ok(())
}

async fn write_main_cart(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
    buf.push_str(r#"<main role="main" class="cart-sections">"#);
    {
        buf.push_str(r#"<section class="container">"#);
        {
            buf.push_str(r#"<div class="row">"#);
            {
                buf.push_str(r#"<div class="col-lg-6 col-xl-5 offset-xl-1 cart-summary-section">"#);
                {
                    buf.push_str(r#"<div class="row mb-3 py-2">"#);
                    {
                        buf.push_str(r#"<div class="col-4 pl-md-0">"#);
                        {
                            buf.push_str(r#"<h3>Cart (5)</h3>"#);
                        }
                        buf.push_str(r#"</div>"#);
                        buf.push_str(r#"<div class="col-8 pr-md-0 text-right">"#);
                        {
                            buf.push_str(r#"<form method="POST" action="/cart/empty">"#);
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
                    buf.push_str(r#"<div class="row cart-summary-item-row">"#);
                    {
                        buf.push_str(r#"<div class="col-md-4 pl-md-0">"#);
                        {
                            buf.push_str(r#"<a href="/product/2ZYFJ3GM2N">"#);
                            {
                                buf.push_str(r#"<img class="img-fluid" alt="" src="/static/img/products/hairdryer.jpg" />"#);
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
                                    buf.push_str(r#"<h4>Hairdryer</h4>"#);
                                }
                                buf.push_str(r#"</div>"#);
                            }
                            buf.push_str(r#"</div>"#);
                            buf.push_str(r#"<div class="row cart-summary-item-row-item-id-row">"#);
                            {
                                buf.push_str(r#"<div class="col">"#);
                                {
                                    buf.push_str(r#"SKU #2ZYFJ3GM2N"#);
                                }
                                buf.push_str(r#"</div>"#);
                            }
                            buf.push_str(r#"</div>"#);
                            buf.push_str(r#"<div class="row">"#);
                            {
                                buf.push_str(r#"<div class="col">"#);
                                {
                                    buf.push_str(r#"Quantity: 3"#);
                                }
                                buf.push_str(r#"</div>"#);
                                buf.push_str(r#"<div class="col pr-md-0 text-right">"#);
                                {
                                    buf.push_str(r#"<strong>"#);
                                    buf.push_str(r#"₺406.16"#);
                                    buf.push_str(r#"</strong>"#);
                                }
                                buf.push_str(r#"</div>"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);
                    }
                    buf.push_str(r#"</div>"#);
                    buf.push_str(r#"<div class="row cart-summary-item-row">"#);
                    {
                        buf.push_str(r#"<div class="col-md-4 pl-md-0">"#);
                        {
                            buf.push_str(r#"<a href="/product/OLJCESPC7Z">"#);
                            {
                                buf.push_str(r#"<img class="img-fluid" alt="" src="/static/img/products/sunglasses.jpg" />"#);
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
                                    buf.push_str(r#"<h4>Sunglasses1</h4>"#);
                                }
                                buf.push_str(r#"</div>"#);
                            }
                            buf.push_str(r#"</div>"#);
                            buf.push_str(r#"<div class="row cart-summary-item-row-item-id-row">"#);
                            {
                                buf.push_str(r#"<div class="col">"#);
                                {
                                    buf.push_str(r#"SKU #OLJCESPC7Z"#);
                                }
                                buf.push_str(r#"</div>"#);
                            }
                            buf.push_str(r#"</div>"#);
                            buf.push_str(r#"<div class="row">"#);
                            {
                                buf.push_str(r#"<div class="col">"#);
                                {
                                    buf.push_str(r#"Quantity: 2"#);
                                }
                                buf.push_str(r#"</div>"#);
                                buf.push_str(r#"<div class="col pr-md-0 text-right">"#);
                                {
                                    buf.push_str(r#"<strong>"#);
                                    buf.push_str(r#"₺216.59"#);
                                    buf.push_str(r#"</strong>"#);
                                }
                                buf.push_str(r#"</div>"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);
                    }
                    buf.push_str(r#"</div>"#);
                    buf.push_str(r#"<div class="row cart-summary-shipping-row">"#);
                    {
                        buf.push_str(r#"<div class="col pl-md-0">Shipping</div>"#);
                        buf.push_str(r#"<div class="col pr-md-0 text-right">₺48.70</div>"#);
                    }
                    buf.push_str(r#"</div>"#);
                    buf.push_str(r#"<div class="row cart-summary-total-row">"#);
                    {
                        buf.push_str(r#"<div class="col pl-md-0">Total</div>"#);
                        buf.push_str(r#"<div class="col pr-md-0 text-right">₺671.46</div>"#);
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
                                buf.push_str(
                                    r#"<label for="credit_card_expiration_year">Year</label>"#,
                                );
                                buf.push_str(r#"<select name="credit_card_expiration_year" id="credit_card_expiration_year">"#);
                                {
                                    buf.push_str(r#"<option value="2024">2024</option>"#);
                                    buf.push_str(
                                        r#"<option value="2025" selected="selected">2025</option>"#,
                                    );
                                    buf.push_str(r#"<option value="2026">2026</option>"#);
                                    buf.push_str(r#"<option value="2027">2027</option>"#);
                                    buf.push_str(r#"<option value="2028">2028</option>"#);
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
    buf.push_str(r#"</main>"#);
    if let Err(e) = recommendation::get_recommendations(buf, page_props).await {
        return Err(e);
    }
    if let Err(e) = body_footer::write(buf, page_props) {
        return Err(e);
    }
    if let Err(e) = script::write(buf) {
        return Err(e);
    }

    Ok(())
}
