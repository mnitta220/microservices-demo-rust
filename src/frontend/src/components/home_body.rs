use crate::{components, rpc::product, Component, PageProps};
use anyhow::Result;

pub struct HomeBody {
    pub body_header: Box<dyn Component>,
    pub footer: Box<dyn Component>,
    pub product_list: Vec<crate::rpc::hipstershop::Product>,
}

impl HomeBody {
    pub async fn load(props: &PageProps) -> Result<Self> {
        let product_list = match product::get_product_list(&props.user_currency).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let body_header = match components::body_header::BodyHeader::load(props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let footer = components::body_footer::Footer {};
        let body = components::home_body::HomeBody {
            body_header: Box::new(body_header),
            footer: Box::new(footer),
            product_list,
        };

        Ok(body)
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
                                    let money = product.price_usd.as_ref().unwrap();
                                    buf.push_str(r#"<div class="col-md-4 hot-product-card">"#);
                                    {
                                        buf.push_str(r#"<a href="/product/"#);
                                        buf.push_str(&product.id);
                                        buf.push_str(r#"">"#);
                                        {
                                            buf.push_str(r#"<img alt="" src=""#);
                                            buf.push_str(&product.picture);
                                            buf.push_str(r#"">"#);
                                            buf.push_str(r#"<div class="hot-product-card-img-overlay"></div>"#);
                                        }
                                        buf.push_str(r#"</a>"#);
                                        buf.push_str(r#"<div>"#);
                                        {
                                            buf.push_str(r#"<div class="hot-product-card-name">"#);
                                            buf.push_str(&product.name);
                                            buf.push_str(r#"</div>"#);

                                            buf.push_str(r#"<div class="hot-product-card-price">"#);
                                            buf.push_str(&money.money_for_display());
                                            buf.push_str(r#"</div>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
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
