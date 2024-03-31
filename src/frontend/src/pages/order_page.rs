use crate::{components::body::order::OrderBody, pages::page::Page};
use anyhow::Result;

/// Component for rendering the order page
pub struct OrderPage {}

impl OrderPage {
    /// Output the contents of the HTML page to a String.
    pub async fn generate(props: &crate::pages::page::PageProps) -> Result<String> {
        // Construct the components of the HTML page.
        let mut page = Page::new();

        // Construct the components of the HTML <body> tag.
        let body = match OrderBody::load(&props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        page.body = Some(body);

        // Output the contents of the HTML page to a buffer.
        if let Err(e) = page.write(props) {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(page.buf)
    }
}
