use super::super::Component;
use super::{footer::Footer, header::BodyHeader, hot_product::HotProduct, Body};
use crate::{
    rpc::{hipstershop::Product, product},
    PageProps,
};
use anyhow::Result;

pub struct HomeBody {
    pub body_header: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub product_list: Vec<Product>,
}

impl Body for HomeBody {
    async fn load(props: &PageProps) -> Result<Box<Self>> {
        let product_list = match product::get_product_list(&props.user_currency).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let body_header = match BodyHeader::load(props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let footer = Footer {};

        let body = HomeBody {
            body_header: Box::new(body_header),
            footer: Box::new(footer),
            product_list,
        };

        Ok(Box::new(body))
    }
}

impl Component for HomeBody {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        buf.push_str(r#"<body>"#);
        {
            self.body_header.write(props, buf)?;

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
                            buf.push_str(r#"<div class="row hot-products-row px-xl-6">"#);
                            {
                                buf.push_str(r#"<div class="col-12">"#);
                                {
                                    buf.push_str(r#"<h3>Hot Products</h3>"#);
                                }
                                buf.push_str(r#"</div>"#);
                                for product in self.product_list.iter() {
                                    HotProduct::write(product, buf);
                                }
                            }
                            buf.push_str(r#"</div>"#);

                            buf.push_str(
                                r#"<div class="row d-none d-lg-block home-desktop-footer-row">"#,
                            );
                            {
                                buf.push_str(r#"<div class="col-12 p-0">"#);
                                {
                                    self.footer.write(props, buf)?;
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
