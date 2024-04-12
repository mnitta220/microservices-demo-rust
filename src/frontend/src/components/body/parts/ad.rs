use crate::{components::Component, model, Props};

impl Component for model::ad::AdItem {
    fn write(&self, _props: &Props, buf: &mut String) {
        *buf += r#"<div class="ad">"#;
        {
            *buf += r#"<div class="container py-3 px-lg-5 py-lg-5">"#;
            {
                *buf += r#"<div role="alert">"#;
                {
                    *buf += r#"<strong>Ad</strong>"#;
                    *buf += r#"<a href=""#;
                    *buf += &self.ad.redirect_url;
                    *buf += r#"" rel="nofollow" target="_blank">"#;
                    *buf += &self.ad.text;
                    *buf += r#"</a>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</div>"#;
    }
}
