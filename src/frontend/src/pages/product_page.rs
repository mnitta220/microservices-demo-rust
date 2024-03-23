use crate::{components::product_body::ProductBody, pages::page::Page, Body};
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

        // Allocate a buffer with sufficient capacity to output the HTML content.
        let mut buf = String::with_capacity(100000);

        // Output the contents of the HTML page to a buffer.
        if let Err(e) = page.write(&mut buf) {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(buf)
    }
}
