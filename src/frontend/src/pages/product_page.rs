use crate::model;
use crate::{components::body::product::ProductBody, pages::page};
use anyhow::Result;

pub struct ProductPage {
    pub props: page::PageProps,
    pub page: page::Page,
}

impl ProductPage {
    pub async fn new(session_id: String, currency: String, product_id: String) -> Result<Self> {
        let mut props = page::PageProps::new(&session_id, &currency);
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
            model::recommendation::RecommendationList::load(Some(product_id), &session_id).await?;
        props.recommendations = Some(recommendations);
        // fetch advertisement info
        let ad = model::ad::AdItem::load().await;
        props.ad = ad;

        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML <body> tag.
        let body = match ProductBody::load() {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        page.body = Some(body);

        Ok(ProductPage { props, page })
    }

    pub fn write(&mut self) -> Result<String> {
        self.page.write(&self.props)
    }
}
