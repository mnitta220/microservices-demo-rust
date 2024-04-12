use super::super::Component;
use super::parts::{footer::Footer, header::BodyHeader};
use crate::Props;

pub struct HomeBody {
    pub body_header: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl HomeBody {
    pub fn new() -> Self {
        let body_header = BodyHeader::new();
        let footer = Footer {};

        HomeBody {
            body_header: Box::new(body_header),
            footer: Box::new(footer),
        }
    }
}

impl Component for HomeBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            self.body_header.write(props, buf);

            *buf += r#"<main role="main" class="home">"#;
            {
                *buf += r#"<div class="home-mobile-hero-banner d-lg-none"></div>"#;

                *buf += r#"<div class="container-fluid">"#;
                {
                    *buf += r#"<div class="row">"#;
                    {
                        *buf += r#"<div class="col-4 d-none d-lg-block home-desktop-left-image"></div>"#;
                        *buf += r#"<div class="col-12 col-lg-8">"#;
                        {
                            *buf += r#"<div class="row hot-products-row px-xl-6">"#;
                            {
                                *buf += r#"<div class="col-12">"#;
                                {
                                    *buf += r#"<h3>Hot Products</h3>"#;
                                }
                                *buf += r#"</div>"#;

                                if let Some(hot_products) = &props.hot_products {
                                    hot_products.write(props, buf);
                                }
                            }
                            *buf += r#"</div>"#;

                            *buf +=
                                r#"<div class="row d-none d-lg-block home-desktop-footer-row">"#;
                            {
                                *buf += r#"<div class="col-12 p-0">"#;
                                {
                                    self.footer.write(props, buf);
                                }
                                *buf += r#"</div>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</main>"#;
        }
        *buf += r#"</body>"#;
    }
}
