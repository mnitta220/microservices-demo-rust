use super::body_header;
use super::body_main;
use crate::PageProps;

pub async fn write(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
    buf.push_str(r#"<body>"#);
    {
        if let Err(e) = body_header::write(buf, page_props).await {
            return Err(e);
        }

        buf.push_str(r#"<div class="local">"#);
        {
            buf.push_str(r#"<span class="platform-flag">"#);
            buf.push_str(r#"local"#);
            buf.push_str(r#"</span>"#);
        }
        buf.push_str(r#"</div>"#);

        if let Err(e) = body_main::write(buf, page_props).await {
            return Err(e);
        }
    }
    buf.push_str(r#"</body>"#);

    Ok(())
}
