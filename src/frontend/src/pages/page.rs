use crate::{
    components::{head::Head, Component},
    model,
};
use uuid::Uuid;

// buffer size for outputting HTML content.
// specify a sufficient size according to the characteristics of the system.
const PAGE_BUFFER_SIZE: usize = 20_000;

pub struct Props {
    pub session_id: String,
    pub request_id: String,
    pub user_currency: String,
    pub product_id: Option<String>,
    pub currency_codes: Option<model::currency::SupportedCurrencies>,
    pub cart: Option<model::cart::Cart>,
    pub hot_products: Option<model::hot_product::HotProducts>,
    pub product: Option<model::product::Product>,
    pub recommendations: Option<model::recommendation::RecommendationList>,
    pub ad: Option<model::ad::AdItem>,
    pub order: Option<model::order::Order>,
}

impl Props {
    pub fn new(session_id: &String, user_currency: &String) -> Self {
        Props {
            session_id: session_id.clone(),
            request_id: Uuid::new_v4().to_string(),
            user_currency: user_currency.clone(),
            product_id: None,
            currency_codes: None,
            cart: None,
            hot_products: None,
            product: None,
            recommendations: None,
            ad: None,
            order: None,
        }
    }
}

pub struct Page {
    pub head: Box<dyn Component + Send>,
    pub body: Option<Box<dyn Component + Send>>,
}

impl Page {
    pub fn new() -> Page {
        Page {
            head: Box::new(Head {}),
            body: None,
        }
    }

    // output HTML content to buffer.
    pub fn write(&mut self, props: &crate::pages::page::Props) -> String {
        // buffer for outputting HTML content.
        let mut buf = String::with_capacity(PAGE_BUFFER_SIZE);

        buf.push_str(r#"<!DOCTYPE html>"#);
        buf.push_str(r#"<html lang="en">"#);
        {
            self.head.write(props, &mut buf);

            if let Some(body) = &self.body {
                body.write(props, &mut buf);
            }
        }
        buf.push_str(r#"</html>"#);

        buf
    }
}
