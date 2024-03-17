use super::body_header;
use super::body_main;
use crate::PageProps;

pub struct Body {}

impl Body {
    pub async fn write(
        &self,
        buf: &mut String,
        page_props: &mut PageProps,
    ) -> Result<(), &'static str> {
        buf.push_str(r#"<body>"#);
        {
            let header = body_header::BodyHeader {};
            if let Err(e) = header.write(buf, page_props).await {
                return Err(e);
            }

            buf.push_str(r#"<div class="local">"#);
            {
                buf.push_str(r#"<span class="platform-flag">"#);
                buf.push_str(r#"local"#);
                buf.push_str(r#"</span>"#);
            }
            buf.push_str(r#"</div>"#);

            let main = body_main::BodyMain {};
            if let Err(e) = main.write(buf).await {
                return Err(e);
            }
        }
        buf.push_str(r#"</body>"#);

        Ok(())
    }
}
