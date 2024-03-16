pub struct BodyHeader {}

impl BodyHeader {
    pub fn write(&self, buf: &mut String) -> Result<(), &'static str> {
        buf.push_str(r#"<header>"#);
        {
            buf.push_str(r#"<div class="navbar sub-navbar">"#);
            {
                buf.push_str(r#"<div class="container d-flex justify-content-between">"#);
                {
                    buf.push_str(r#"<a href="/" class="navbar-brand d-flex align-items-center">"#);
                    {
                        buf.push_str(r#"<img src="/static/icons/Hipster_NavLogo.svg" alt="" class="top-left-logo" />"#);
                    }
                    buf.push_str(r#"</a>"#);

                    buf.push_str(r#"<div class="controls">"#);
                    {
                        buf.push_str(r#"<div class="h-controls">"#);
                        {
                            buf.push_str(r#"<div class="h-control">"#);
                            {
                                buf.push_str(r#"<span class="icon currency-icon"> $</span>"#);

                                buf.push_str(r#"<form method="POST" class="controls-form" action="/setCurrency" id="currency_form">"#);
                                {
                                    buf.push_str(r#"<select name="currency_code" onchange="document.getElementById('currency_form').submit();">"#);
                                    {
                                        buf.push_str(r#"<option value="EUR">EUR</option>"#);
                                        buf.push_str(r#"<option value="USD" selected="selected">USD</option>"#);
                                        buf.push_str(r#"<option value="JPY">JPY</option>"#);
                                        buf.push_str(r#"<option value="GBP">GBP</option>"#);
                                        buf.push_str(r#"<option value="TRY">TRY</option>"#);
                                        buf.push_str(r#"<option value="CAD">CAD</option>"#);
                                    }
                                    buf.push_str(r#"</select>"#);
                                }
                                buf.push_str(r#"</form>"#);

                                buf.push_str(r#"<img src="/static/icons/Hipster_DownArrow.svg" alt="" class="icon arrow" />"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);

                        buf.push_str(r#"<a href="/cart" class="cart-link">"#);
                        {
                            buf.push_str(r#"<img src="/static/icons/Hipster_CartIcon.svg" alt="Cart icon" class="logo" title="Cart" />"#);
                        }
                        buf.push_str(r#"</a>"#);
                    }
                    buf.push_str(r#"</div>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</header>"#);

        Ok(())
    }
}
