use super::currency::CurrencyForm;
use crate::{components::Component, PageProps};
use anyhow::Result;

pub struct BodyHeader {
    pub currency_form: Box<dyn Component + Send>,
}

impl BodyHeader {
    pub async fn load(props: &PageProps) -> Result<Self> {
        let currency_form = match CurrencyForm::load(props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let body_header = BodyHeader {
            currency_form: Box::new(currency_form),
        };

        Ok(body_header)
    }
}

impl Component for BodyHeader {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
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
                                self.currency_form.write(props, buf)?;

                                buf.push_str(r#"<img src="/static/icons/Hipster_DownArrow.svg" alt="" class="icon arrow" />"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);

                        buf.push_str(r#"<a href="/cart" class="cart-link">"#);
                        {
                            buf.push_str(r#"<img src="/static/icons/Hipster_CartIcon.svg" alt="Cart icon" class="logo" title="Cart" />"#);
                            let cart_size = props.cart_info.cart_size();
                            if cart_size > 0 {
                                buf.push_str(r#"<span class="cart-size-circle">"#);
                                buf.push_str(&cart_size.to_string());
                                buf.push_str(r#"</span>"#);
                            }
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

        buf.push_str(r#"<div class=""#);
        buf.push_str(&crate::PLATFORM_CSS.get().unwrap());
        buf.push_str(r#"">"#);
        {
            buf.push_str(r#"<span class="platform-flag">"#);
            buf.push_str(&crate::PLATFORM_NAME.get().unwrap());
            buf.push_str(r#"</span>"#);
        }
        buf.push_str(r#"</div>"#);

        Ok(())
    }
}
