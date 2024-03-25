use super::super::Component;
use super::{ad, footer::Footer, header::BodyHeader, recommendation::RecommendationList, Body};
use crate::{
    rpc::{hipstershop::Product, product},
    PageProps,
};
use anyhow::Result;

pub struct HotProductItem {
    pub product: Product,
}

pub struct HotProductList {
    pub items: Vec<HotProductItem>,
}

pub struct ProductBody {
    pub body_header: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub product: Product,
    pub recommendation_list: RecommendationList,
    pub ad: Option<ad::AdItem>,
}

impl HotProductList {
    pub async fn load(props: &PageProps) -> Result<Self> {
        let product_list = match product::get_product_list(&props.user_currency).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let mut items = Vec::new();
        for product in product_list {
            items.push(HotProductItem { product: product });
        }

        Ok(HotProductList { items })
    }
}

impl Component for HotProductList {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        for item in &self.items {
            item.write(props, buf)?;
        }
        Ok(())
    }
}

impl Component for HotProductItem {
    fn write(&self, _props: &PageProps, buf: &mut String) -> Result<()> {
        let money = self.product.price_usd.as_ref().unwrap();
        buf.push_str(r#"<div class="col-md-4 hot-product-card">"#);
        {
            buf.push_str(r#"<a href="/product/"#);
            buf.push_str(&self.product.id);
            buf.push_str(r#"">"#);
            {
                buf.push_str(r#"<img alt="" src=""#);
                buf.push_str(&self.product.picture);
                buf.push_str(r#"">"#);
                buf.push_str(r#"<div class="hot-product-card-img-overlay"></div>"#);
            }
            buf.push_str(r#"</a>"#);

            buf.push_str(r#"<div>"#);
            {
                buf.push_str(r#"<div class="hot-product-card-name">"#);
                buf.push_str(&self.product.name);
                buf.push_str(r#"</div>"#);

                buf.push_str(r#"<div class="hot-product-card-price">"#);
                buf.push_str(&money.money_for_display());
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);

        Ok(())
    }
}

impl Body for ProductBody {
    async fn load(props: &PageProps) -> Result<Box<Self>> {
        let product = match product::get_product(&props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let recommendation_list = match RecommendationList::load(&props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let ad = ad::AdItem::load().await;

        let body_header = match BodyHeader::load(props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let footer = Footer {};

        let body = ProductBody {
            body_header: Box::new(body_header),
            footer: Box::new(footer),
            product,
            recommendation_list,
            ad,
        };

        Ok(Box::new(body))
    }
}

impl Component for ProductBody {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        let money = self.product.price_usd.as_ref().unwrap();
        buf.push_str(r#"<body>"#);
        {
            self.body_header.write(props, buf)?;

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
                                buf.push_str(&money.money_for_display());
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
                {
                    self.recommendation_list.write(props, buf)?
                }
                buf.push_str(r#"</div>"#);

                if let Some(a) = &self.ad {
                    a.write(props, buf)?;
                }
            }
            buf.push_str(r#"</main>"#);
            self.footer.write(props, buf)?;
        }
        buf.push_str(r#"</body>"#);

        Ok(())
    }
}
