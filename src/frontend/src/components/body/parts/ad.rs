use crate::{components::Component, model, Props};

impl Component for model::ad::AdItem {
    fn write(&self, _props: &Props, buf: &mut String) {
        buf.push_str(r#"<div class="ad">"#);
        {
            buf.push_str(r#"<div class="container py-3 px-lg-5 py-lg-5">"#);
            {
                buf.push_str(r#"<div role="alert">"#);
                {
                    buf.push_str(r#"<strong>Ad</strong>"#);
                    buf.push_str(r#"<a href=""#);
                    buf.push_str(&self.ad.redirect_url);
                    buf.push_str(r#"" rel="nofollow" target="_blank">"#);
                    buf.push_str(&self.ad.text);
                    buf.push_str(r#"</a>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);
    }
}
