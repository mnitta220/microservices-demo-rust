use super::super::rpc::products;
use super::body_footer;
use crate::PageProps;

pub struct BodyMain {}

impl BodyMain {
    pub async fn write(
        &self,
        buf: &mut String,
        page_props: &mut PageProps,
    ) -> Result<(), &'static str> {
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
                        if let Err(e) =
                            products::get_products(buf, page_props.user_currency.clone()).await
                        {
                            return Err(e);
                        }
                        buf.push_str(
                            r#"<div class="row d-none d-lg-block home-desktop-footer-row">"#,
                        );
                        {
                            buf.push_str(r#"<div class="col-12 p-0">"#);
                            {
                                let footer = body_footer::BodyFooter {};
                                if let Err(e) = footer.write(buf, page_props) {
                                    return Err(e);
                                }
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

        Ok(())
    }
}
