use super::super::rpc::currency;
use crate::PageProps;

pub struct BodyHeader {}

impl BodyHeader {
    pub async fn write(
        &self,
        buf: &mut String,
        page_props: &mut PageProps,
    ) -> Result<(), &'static str> {
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
                                if let Err(e) =
                                    currency::select_currency_form(buf, page_props).await
                                {
                                    return Err(e);
                                }

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
