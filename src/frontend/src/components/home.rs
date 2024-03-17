use super::body;
use super::head;
use crate::PageProps;

pub struct HomePage {}

impl HomePage {
    pub async fn write_page(
        &self,
        buf: &mut String,
        page_props: &mut PageProps,
    ) -> Result<(), &'static str> {
        buf.push_str(r#"<!DOCTYPE html>"#);
        buf.push_str(r#"<html lang="en">"#);
        let head = head::Head {};
        if let Err(e) = head.write(buf) {
            return Err(e);
        }
        let body = body::Body {};
        if let Err(e) = body.write(buf, page_props).await {
            return Err(e);
        }
        buf.push_str(r#"</html>"#);

        Ok(())
    }
}
