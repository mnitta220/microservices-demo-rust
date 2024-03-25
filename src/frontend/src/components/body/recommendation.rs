use super::super::Component;
use crate::{
    rpc::{hipstershop::Product, recommendation},
    PageProps,
};
use anyhow::Result;

pub struct RecommendationList {
    pub items: Vec<RecommendationItem>,
}

pub struct RecommendationItem {
    pub product: Product,
}

impl RecommendationList {
    pub async fn load(props: &PageProps) -> Result<Self> {
        let recommendation_list = match recommendation::get_recommendations(&props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let mut items = Vec::new();
        for product in recommendation_list {
            items.push(RecommendationItem { product: product });
        }

        Ok(RecommendationList { items })
    }
}

impl Component for RecommendationList {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
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
                        for item in &self.items {
                            item.write(props, buf)?;
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

        Ok(())
    }
}

impl Component for RecommendationItem {
    fn write(&self, _props: &PageProps, buf: &mut String) -> Result<()> {
        buf.push_str(r#"<div class="col-md-3">"#);
        {
            buf.push_str(r#"<div>"#);
            {
                buf.push_str(r#"<a href="/product/"#);
                buf.push_str(&self.product.id);
                buf.push_str(r#"">"#);
                {
                    buf.push_str(r#"<img alt src=""#);
                    buf.push_str(&self.product.picture);
                    buf.push_str(r#"">"#);
                }
                buf.push_str(r#"</a>"#);

                buf.push_str(r#"<div>"#);
                {
                    buf.push_str(r#"<h5>"#);
                    buf.push_str(&self.product.name);
                    buf.push_str(r#"</h5>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);

        Ok(())
    }
}
