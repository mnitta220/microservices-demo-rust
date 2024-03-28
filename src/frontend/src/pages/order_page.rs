use crate::{
    components::body::order::OrderBody,
    pages::page::Page,
    rpc::hipstershop::{Money, OrderResult},
};
use anyhow::Result;

/// Component for rendering the order page
pub struct OrderPage {}

impl OrderPage {
    /// Output the contents of the HTML page to a String.
    pub async fn generate(
        session_id: &String,
        currency: &String,
        order: OrderResult,
        total_cost: Money,
    ) -> Result<String> {
        // Construct the components of the HTML page.
        let mut page = Page::generate(session_id, currency, None).await?;

        // Construct the components of the HTML <body> tag.
        let body = match OrderBody::load(&page.props, order, total_cost).await {
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
