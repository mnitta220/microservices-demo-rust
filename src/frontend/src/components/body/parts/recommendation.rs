use crate::{components::Component, model, Props};

impl Component for model::recommendation::RecommendationList {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<section class="recommendations">"#;
        {
            *buf += r#"<div class="container">"#;
            {
                *buf += r#"<div class="row">"#;
                {
                    *buf += r#"<div class="col-xl-10 offset-xl-1">"#;
                    {
                        *buf += r#"<h2>You May Also Like</h2>"#;

                        *buf += r#"<div class="row">"#;
                        for item in &self.items {
                            item.write(props, buf);
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</section>"#;
    }
}

impl Component for model::recommendation::RecommendationItem {
    fn write(&self, _props: &Props, buf: &mut String) {
        *buf += r#"<div class="col-md-3">"#;
        {
            *buf += r#"<div>"#;
            {
                *buf += r#"<a href="/product/"#;
                *buf += &self.product.id;
                *buf += r#"">"#;
                {
                    *buf += r#"<img alt src=""#;
                    *buf += &self.product.picture;
                    *buf += r#"">"#;
                }
                *buf += r#"</a>"#;

                *buf += r#"<div>"#;
                {
                    *buf += r#"<h5>"#;
                    *buf += &self.product.name;
                    *buf += r#"</h5>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</div>"#;
    }
}
