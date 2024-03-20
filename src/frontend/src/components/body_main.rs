use super::super::rpc::products;
use super::super::PageType;
use super::body_footer;
use crate::PageProps;

pub async fn write(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
    match page_props.page_type {
        PageType::Home => write_main_home(buf, page_props).await,
        PageType::Product => write_main_product(buf, page_props).await,
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

async fn write_main_product(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
    buf.push_str(r#"<main role="main">"#);
    {
        if let Err(e) = products::get_product(buf, page_props).await {
            return Err(e);
        }

        buf.push_str(r#"<div>"#);
        {
            buf.push_str(r#"<section class="recommendations">"#);
            {
                buf.push_str(r#"<div class="container">"#);
                {
                    buf.push_str(r#"<div class="row">"#);
                    {
                        buf.push_str(r#"<div class="col-xl-10 offset-xl-1">"#);
                        {
                            buf.push_str(r#"<h2>You May Also Like</h2>"#);
                            buf.push_str(r#"<div class="row">"#);
                            {
                                buf.push_str(r#"<div class="col-md-3">"#);
                                {
                                    buf.push_str(r#"<div>"#);
                                    {
                                        buf.push_str(r#"<a href="/product/6E92ZMYYFZ">"#);
                                        {
                                            buf.push_str(
                                                r#"<img alt src="/static/img/products/mug.jpg">"#,
                                            );
                                        }
                                        buf.push_str(r#"</a>"#);
                                        buf.push_str(r#"<div>"#);
                                        {
                                            buf.push_str(r#"<h5>"#);
                                            buf.push_str(r#"Mug"#);
                                            buf.push_str(r#"</h5>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                }
                                buf.push_str(r#"</div>"#);

                                buf.push_str(r#"<div class="col-md-3">"#);
                                {
                                    buf.push_str(r#"<div>"#);
                                    {
                                        buf.push_str(r#"<a href="/product/6E92ZMYYFZ">"#);
                                        {
                                            buf.push_str(
                                                r#"<img alt src="/static/img/products/mug.jpg">"#,
                                            );
                                        }
                                        buf.push_str(r#"</a>"#);
                                        buf.push_str(r#"<div>"#);
                                        {
                                            buf.push_str(r#"<h5>"#);
                                            buf.push_str(r#"Tank Top2"#);
                                            buf.push_str(r#"</h5>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                }
                                buf.push_str(r#"</div>"#);
                                buf.push_str(r#"<div class="col-md-3">"#);
                                {
                                    buf.push_str(r#"<div>"#);
                                    {
                                        buf.push_str(r#"<a href="/product/6E92ZMYYFZ">"#);
                                        {
                                            buf.push_str(
                                                r#"<img alt src="/static/img/products/mug.jpg">"#,
                                            );
                                        }
                                        buf.push_str(r#"</a>"#);
                                        buf.push_str(r#"<div>"#);
                                        {
                                            buf.push_str(r#"<h5>"#);
                                            buf.push_str(r#"Hairdryer"#);
                                            buf.push_str(r#"</h5>"#);
                                        }
                                        buf.push_str(r#"</div>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                }
                                buf.push_str(r#"</div>"#);
                                buf.push_str(r#"<div class="col-md-3">"#);
                                {
                                    buf.push_str(r#"<div>"#);
                                    {
                                        buf.push_str(r#"<a href="/product/6E92ZMYYFZ">"#);
                                        {
                                            buf.push_str(
                                                r#"<img alt src="/static/img/products/mug.jpg">"#,
                                            );
                                        }
                                        buf.push_str(r#"</a>"#);
                                        buf.push_str(r#"<div>"#);
                                        {
                                            buf.push_str(r#"<h5>"#);
                                            buf.push_str(r#"Watch3"#);
                                            buf.push_str(r#"</h5>"#);
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
                    buf.push_str(r#"</div>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</section>"#);
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
    buf.push_str(
        r#"<script src="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/js/bootstrap.min.js" "#,
    );
    buf.push_str(
        r#"integrity="sha384-smHYKdLADwkXOn1EmN1qk/HfnUcbVRZyYmZ4qpPea6sjB/pTJ0euyQp0Mk8ck+5T" "#,
    );
    buf.push_str(r#"crossorigin="anonymous">"#);
    buf.push_str(r#"</script>"#);

    Ok(())
}
