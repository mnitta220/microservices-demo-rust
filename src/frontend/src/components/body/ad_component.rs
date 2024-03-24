use crate::rpc::hipstershop::Ad;

pub struct AdComponent {}

impl AdComponent {
    pub fn write(ad: &Ad, buf: &mut String) {
        buf.push_str(r#"<div class="ad">"#);
        {
            buf.push_str(r#"<div class="container py-3 px-lg-5 py-lg-5">"#);
            {
                buf.push_str(r#"<div role="alert">"#);
                {
                    buf.push_str(r#"<strong>Ad</strong>"#);
                    buf.push_str(r#"<a href=""#);
                    buf.push_str(&ad.redirect_url);
                    buf.push_str(r#"" rel="nofollow" target="_blank">"#);
                    buf.push_str(&ad.text);
                    buf.push_str(r#"</a>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);
    }
}
