use super::super::Component;
use crate::{rpc::hipstershop::Product, PageProps};
use anyhow::Result;

pub struct Recommendations {
    pub recommendation_list: Vec<Product>,
}

impl Component for Recommendations {
    fn write(&self, _props: &PageProps, buf: &mut String) -> Result<()> {
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

        Ok(())
    }
}
