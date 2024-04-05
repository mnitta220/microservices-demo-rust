use crate::model;
use crate::{components::body::order::OrderBody, pages::page};
use anyhow::Result;

/// Component for rendering the order page
pub struct OrderPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl OrderPage {
    pub async fn new(
        session_id: String,
        currency: String,
        input: crate::handlers::PlaceOrderInput,
    ) -> Result<Self> {
        let mut props = page::Props::new(&session_id, &currency);

        // load and setting props
        {
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
        }

        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = OrderBody::new();
        page.body = Some(Box::new(body));

        Ok(OrderPage { props, page })
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
