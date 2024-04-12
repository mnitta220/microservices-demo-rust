use super::currency::CurrencyForm;
use crate::{components::Component, Props};

pub struct BodyHeader {
    pub currency_form: Box<dyn Component + Send>,
}

impl BodyHeader {
    pub fn new() -> Self {
        let body_header = BodyHeader {
            currency_form: Box::new(CurrencyForm {}),
        };

        body_header
    }
}

impl Component for BodyHeader {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<header>"#;
        {
            *buf += r#"<div class="navbar sub-navbar">"#;
            {
                *buf += r#"<div class="container d-flex justify-content-between">"#;
                {
                    *buf += r#"<a href="/" class="navbar-brand d-flex align-items-center">"#;
                    {
                        *buf += r#"<img src="/static/icons/Hipster_NavLogo.svg" alt="" class="top-left-logo" />"#;
                    }
                    *buf += r#"</a>"#;

                    *buf += r#"<div class="controls">"#;
                    {
                        *buf += r#"<div class="h-controls">"#;
                        {
                            *buf += r#"<div class="h-control">"#;
                            {
                                self.currency_form.write(props, buf);

                                *buf += r#"<img src="/static/icons/Hipster_DownArrow.svg" alt="" class="icon arrow" />"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<a href="/cart" class="cart-link">"#;
                        {
                            *buf += r#"<img src="/static/icons/Hipster_CartIcon.svg" alt="Cart icon" class="logo" title="Cart" />"#;

                            if let Some(cart) = &props.cart {
                                let cart_size = cart.cart_size();
                                if cart_size > 0 {
                                    *buf += r#"<span class="cart-size-circle">"#;
                                    *buf += &cart_size.to_string();
                                    *buf += r#"</span>"#;
                                }
                            }
                        }
                        *buf += r#"</a>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</header>"#;

        *buf += r#"<div class=""#;
        *buf += &crate::PLATFORM_CSS.get().unwrap();
        *buf += r#"">"#;
        {
            *buf += r#"<span class="platform-flag">"#;
            *buf += &crate::PLATFORM_NAME.get().unwrap();
            *buf += r#"</span>"#;
        }
        *buf += r#"</div>"#;
    }
}
