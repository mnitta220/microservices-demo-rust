use super::super::rpc::products;

pub struct Body {}

impl Body {
    pub async fn write(&self, buf: &mut String) -> Result<(), &'static str> {
        buf.push_str(r#"<body>"#);
        {
            buf.push_str(r#"<header>"#);
            {
                buf.push_str(r#"<div class="navbar sub-navbar">"#);
                {
                    buf.push_str(r#"<div class="container d-flex justify-content-between">"#);
                    {
                        buf.push_str(
                            r#"<a href="/" class="navbar-brand d-flex align-items-center">"#,
                        );
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
            buf.push_str(r#"<div class="local">"#);
            {
                buf.push_str(r#"<span class="platform-flag">"#);
                buf.push_str(r#"local"#);
                buf.push_str(r#"</span>"#);
            }
            buf.push_str(r#"</div>"#);
            buf.push_str(r#"<main role="main" class="home">"#);
            {
                buf.push_str(r#"<div class="home-mobile-hero-banner d-lg-none"></div>"#);
                buf.push_str(r#"<div class="container-fluid">"#);
                {
                    buf.push_str(r#"<div class="row">"#);
                    {
                        buf.push_str(r#"<div class="col-4 d-none d-lg-block home-desktop-left-image"></div>"#);
                        buf.push_str(r#"<div class="col-12 col-lg-8">"#);
                        {
                            if let Err(e) = products::get_products(buf).await {
                                return Err(e);
                            }
                            buf.push_str(
                                r#"<div class="row d-none d-lg-block home-desktop-footer-row">"#,
                            );
                            {
                                buf.push_str(r#"<div class="col-12 p-0">"#);
                                {
                                    buf.push_str(r#"<footer class="py-5">"#);
                                    {
                                        buf.push_str(r#"<div class="footer-top">"#);
                                        {
                                            buf.push_str(
                                                r#"<div class="container footer-social">"#,
                                            );
                                            {
                                                buf.push_str(r#"<p class="footer-text">"#);
                                                buf.push_str(r#"This website is hosted for demo purposes only. It is not an actual shop. This is not a Google product."#);
                                                buf.push_str(r#"</p>"#);
                                                buf.push_str(r#"<p class="footer-text">"#);
                                                buf.push_str(r#"© 2020 Google Inc ("#);
                                                buf.push_str(r#"<a href="https://github.com/GoogleCloudPlatform/microservices-demo">"#);
                                                buf.push_str(r#"Source Code"#);
                                                buf.push_str(r#"</a>"#);
                                                buf.push_str(r#")</p>"#);
                                                buf.push_str(r#"<p class="footer-text">"#);
                                                {
                                                    buf.push_str(r#"<small>"#);
                                                    buf.push_str(r#"session-id: c6400be8-95e4-4719-9300-d86712a76acf — "#);
                                                    buf.push_str(r#"request-id: ff5c663c-d516-4aab-b5a3-5af7197df163"#);
                                                    buf.push_str(r#"</small>"#);
                                                    buf.push_str(r#"<br />"#);
                                                    buf.push_str(r#"<small>"#);
                                                    buf.push_str(r#"<b>Pod: </b>"#);
                                                    buf.push_str(r#"frontend-55dc66b9dd-hc7vm"#);
                                                    buf.push_str(r#"</small>"#);
                                                }
                                                buf.push_str(r#"</p>"#);
                                            }
                                            buf.push_str(r#"</div>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</footer>"#);
                                    buf.push_str(r#"<script src="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/js/bootstrap.min.js" "#);
                                    buf.push_str(r#"integrity="sha384-smHYKdLADwkXOn1EmN1qk/HfnUcbVRZyYmZ4qpPea6sjB/pTJ0euyQp0Mk8ck+5T" "#);
                                    buf.push_str(r#"crossorigin="anonymous">"#);
                                    buf.push_str(r#"</script>"#);
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
        }
        buf.push_str(r#"</body>"#);

        Ok(())
    }
}
