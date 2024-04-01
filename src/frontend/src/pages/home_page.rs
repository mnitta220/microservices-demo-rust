use crate::model;
use crate::{components::body::home::HomeBody, pages::page};
use anyhow::Result;

/// Component for rendering the homepage
pub struct HomePage {
    pub props: page::PageProps,
    pub page: page::Page,
}

impl HomePage {
    pub async fn new(session_id: String, currency: String) -> Result<Self> {
        let mut props = page::PageProps::new(&session_id, &currency);
        // fetch currency codes
        let currencies = model::currency::SupportedCurrencies::load().await?;
        props.currency_codes = Some(currencies);
        // fetch cart info
        let cart = model::cart::Cart::load(&session_id, &currency).await?;
        props.cart = Some(cart);
        // fetch hot product info
        let hot_products = model::hot_product::HotProducts::load(&currency).await?;
        props.hot_products = Some(hot_products);

        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML <body> tag.
        let body = HomeBody::new();
        page.body = Some(Box::new(body));

        Ok(HomePage { props, page })
    }

    pub fn write(&mut self) -> Result<String> {
        self.page.write(&self.props)
    }
}
