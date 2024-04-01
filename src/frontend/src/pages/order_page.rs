use crate::model;
use crate::{components::body::order::OrderBody, pages::page};
use anyhow::Result;

/// Component for rendering the order page
pub struct OrderPage {
    pub props: page::PageProps,
    pub page: page::Page,
}

impl OrderPage {
    pub async fn new(
        session_id: String,
        currency: String,
        input: crate::handlers::PlaceOrderInput,
    ) -> Result<Self> {
        let mut props = page::PageProps::new(&session_id, &currency);
        // place order
        let order = model::order::Order::place_order(input, &session_id, &currency).await?;
        props.order = Some(order);
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
        let body = match OrderBody::load() {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        page.body = Some(body);

        Ok(OrderPage { props, page })
    }

    pub fn write(&mut self) -> Result<String> {
        self.page.write(&self.props)
    }
}
