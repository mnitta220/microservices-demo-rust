use crate::{components::body::product::ProductBody, pages::page::Page};
use anyhow::Result;

/// Component for rendering the product page
pub struct ProductPage {}

impl ProductPage {
    /// Output the contents of the HTML page to a String.
    pub async fn generate(
        session_id: &String,
        currency: &String,
        product_id: String,
    ) -> Result<String> {
        // Construct the components of the HTML page.
        let mut page = Page::generate(session_id, currency, Some(product_id)).await?;

        // Construct the components of the HTML <body> tag.
        let body = match ProductBody::load(&page.props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        page.body = Some(body);

        // Output the contents of the HTML page to a buffer.
        if let Err(e) = page.write() {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(page.buf)
    }
}
