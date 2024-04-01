use crate::model;
use crate::{components::body::cart::CartBody, pages::page};
use anyhow::Result;

/// Component for rendering the cart page
pub struct CartPage {
    pub props: page::PageProps,
    pub page: page::Page,
}

impl CartPage {
    pub async fn new(session_id: String, currency: String) -> Result<Self> {
        let mut props = page::PageProps::new(&session_id, &currency);
        // fetch currency codes
        let currencies = model::currency::SupportedCurrencies::load().await?;
        props.currency_codes = Some(currencies);
        // fetch cart info
        let cart = model::cart::Cart::load(&session_id, &currency).await?;
        props.cart = Some(cart);
        // fetch recommendation info
        let recommendations =
            model::recommendation::RecommendationList::load(None, &session_id).await?;
        props.recommendations = Some(recommendations);

        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML <body> tag.
        let body = match CartBody::load() {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        page.body = Some(body);

        Ok(CartPage { props, page })
    }

    pub fn write(&mut self) -> Result<String> {
        self.page.write(&self.props)
    }
}
