use crate::model;
use crate::{components::body::product::ProductBody, pages::page};
use anyhow::Result;

pub struct ProductPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl ProductPage {
    pub async fn new(session_id: String, currency: String, product_id: String) -> Result<Self> {
        let mut props = page::Props::new(&session_id, &currency);

        // load and setting props
        {
            // fetch currency codes
            let currencies = model::currency::SupportedCurrencies::load().await?;
            props.currency_codes = Some(currencies);

            // fetch cart info
            let cart = model::cart::Cart::load(&session_id, &currency).await?;
            props.cart = Some(cart);

            // fetch product info
            let product = model::product::Product::load(&product_id, &currency).await?;
            props.product = Some(product);

            // fetch recommendation info
            let recommendations =
                model::recommendation::RecommendationList::load(Some(product_id), &session_id)
                    .await?;
            props.recommendations = Some(recommendations);

            // fetch advertisement info
            let ad = model::ad::AdItem::load().await;
            props.ad = ad;
        }

        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = ProductBody::new();
        page.body = Some(Box::new(body));

        Ok(ProductPage { props, page })
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
