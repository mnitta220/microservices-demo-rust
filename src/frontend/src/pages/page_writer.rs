use super::super::components::body;
use super::super::components::head;
use crate::PageProps;

pub async fn write(buf: &mut String, page_props: &PageProps) -> Result<(), &'static str> {
    buf.push_str(r#"<!DOCTYPE html>"#);
    buf.push_str(r#"<html lang="en">"#);
    if let Err(e) = head::write(buf) {
        return Err(e);
    }
    if let Err(e) = body::write(buf, page_props).await {
        return Err(e);
    }
    buf.push_str(r#"</html>"#);

    Ok(())
}
