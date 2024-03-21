use crate::rpc::currency;
use crate::rpc::products::hipstershop::Product;
use crate::Component;
use crate::PageProps;
use anyhow::Result;

pub struct ProductBody {
    pub body_header: Box<dyn Component>,
    pub footer: Box<dyn Component>,
    pub product: Product,
    pub recommendation_list: Vec<Product>,
}

impl Component for ProductBody {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        let money = self.product.price_usd.as_ref().unwrap();
        buf.push_str(r#"<body>"#);
        {
            self.body_header.write(props, buf)?;

            buf.push_str(r#"<div class="local">"#);
            {
                buf.push_str(r#"<span class="platform-flag">"#);
                buf.push_str(r#"local"#);
                buf.push_str(r#"</span>"#);
            }
            buf.push_str(r#"</div>"#);

            buf.push_str(r#"<main role="main">"#);
            {
                buf.push_str(r#"<div class="h-product container">"#);
                {
                    buf.push_str(r#"<div class="row">"#);
                    {
                        buf.push_str(r#"<div class="col-md-6">"#);
                        {
                            buf.push_str(r#"<img class="product-image" alt src=""#);
                            buf.push_str(&self.product.picture);
                            buf.push_str(r#"" />"#);
                        }
                        buf.push_str(r#"</div>"#);
                        buf.push_str(r#"<div class="product-info col-md-5">"#);
                        {
                            buf.push_str(r#"<div class="product-wrapper">"#);
                            {
                                buf.push_str(r#"<h2>"#);
                                buf.push_str(&self.product.name);
                                buf.push_str(r#"</h2>"#);
                                buf.push_str(r#"<p class="product-price">"#);
                                buf.push_str(currency::currency_logo(money.currency_code.as_str()));
                                buf.push_str(money.units.to_string().as_str());
                                buf.push_str(".");
                                buf.push_str(format!("{:.2}", money.nanos / 10000000).as_str());
                                buf.push_str(r#"</p>"#);
                                buf.push_str(r#"<p>"#);
                                buf.push_str(&self.product.description);
                                buf.push_str(r#"</p>"#);
                                buf.push_str(r#"<form method="POST" action="/cart">"#);
                                {
                                    buf.push_str(
                                        r#"<input type="hidden" name="product_id" value=""#,
                                    );
                                    buf.push_str(&self.product.id);
                                    buf.push_str(r#"" />"#);
                                    buf.push_str(r#"<div class="product-quantity-dropdown">"#);
                                    {
                                        buf.push_str(r#"<select name="quantity" id="quantity">"#);
                                        {
                                            buf.push_str(r#"<option>1</option>"#);
                                            buf.push_str(r#"<option>2</option>"#);
                                            buf.push_str(r#"<option>3</option>"#);
                                            buf.push_str(r#"<option>4</option>"#);
                                            buf.push_str(r#"<option>5</option>"#);
                                            buf.push_str(r#"<option>10</option>"#);
                                        }
                                        buf.push_str(r#"</select>"#);
                                        buf.push_str(r#"<img src="/static/icons/Hipster_DownArrow.svg" alt>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                    buf.push_str(r#"<button type="submit" class="cymbal-button-primary">Add To Cart</button>"#);
                                }
                                buf.push_str(r#"</form>"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);
                    }
                    buf.push_str(r#"</div>"#);
                }
                buf.push_str(r#"</div>"#);

                buf.push_str(r#"<div>"#);
                //if let Err(e) = recommendation::get_recommendations(buf, page_props).await {
                //    return Err(e);
                //}
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
                                for recommendation in self.recommendation_list.iter() {
                                    //let product = response.into_inner();
                                    buf.push_str(r#"<div class="col-md-3">"#);
                                    {
                                        buf.push_str(r#"<div>"#);
                                        {
                                            buf.push_str(r#"<a href="/product/"#);
                                            buf.push_str(&recommendation.id);
                                            buf.push_str(r#"">"#);
                                            {
                                                buf.push_str(r#"<img alt src=""#);
                                                buf.push_str(&recommendation.picture);
                                                buf.push_str(r#"">"#);
                                            }
                                            buf.push_str(r#"</a>"#);
                                            buf.push_str(r#"<div>"#);
                                            {
                                                buf.push_str(r#"<h5>"#);
                                                buf.push_str(&recommendation.name);
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
            self.footer.write(props, buf)?;
        }
        buf.push_str(r#"</body>"#);

        Ok(())
    }
}
