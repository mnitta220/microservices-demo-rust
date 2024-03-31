use crate::components::{head::Head, Component};
use crate::model;
use anyhow::Result;
use uuid::Uuid;

// buffer size for outputting HTML content.
// specify a sufficient size according to the characteristics of the system.
const PAGE_BUFFER_SIZE: usize = 20_000;

pub struct PageProps {
    pub session_id: String,
    pub request_id: String,
    pub user_currency: String,
    pub currency_codes: Option<Vec<String>>,
    pub product_id: Option<String>,
    pub cart: Option<model::cart::Cart>,
    pub hot_products: Option<model::hot_product::HotProducts>,
    pub product: Option<model::product::Product>,
    pub recommendations: Option<model::recommendation::RecommendationList>,
    pub ad: Option<model::ad::AdItem>,
    pub order: Option<model::order::Order>,
}

impl PageProps {
    pub fn new(session_id: &String, user_currency: &String) -> Self {
        PageProps {
            session_id: session_id.clone(),
            request_id: Uuid::new_v4().to_string(),
            user_currency: user_currency.clone(),
            currency_codes: None,
            product_id: None,
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
    pub fn write(&mut self, props: &crate::pages::page::PageProps) -> Result<String> {
        // buffer for outputting HTML content.
        let mut buf = String::with_capacity(PAGE_BUFFER_SIZE);

        buf.push_str(r#"<!DOCTYPE html>"#);
        buf.push_str(r#"<html lang="en">"#);
        {
            if let Err(e) = self.head.write(props, &mut buf) {
                return Err(e);
            }

            if let Some(b) = &self.body {
                if let Err(e) = b.write(props, &mut buf) {
                    return Err(e);
                }
            }
        }
        buf.push_str(r#"</html>"#);

        Ok(buf)
    }
}
