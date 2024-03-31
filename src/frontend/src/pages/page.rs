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
    pub product_id: Option<String>,
    pub cart: Option<model::cart::Cart>,
    pub hot_products: Option<model::hot_product::HotProducts>,
    pub product: Option<model::product::Product>,
    pub recommendations: Option<model::recommendation::RecommendationList>,
    pub ad: Option<model::ad::AdItem>,
    pub order: Option<model::order::Order>,
}

impl PageProps {
    pub fn new(session_id: String, user_currency: String) -> Self {
        PageProps {
            session_id,
            request_id: Uuid::new_v4().to_string(),
            user_currency,
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
    pub lang: Option<String>,
    pub head: Box<dyn Component + Send>,
    pub body: Option<Box<dyn Component + Send>>,
    pub buf: String,
}

impl Page {
    pub fn new() -> Page {
        let head = Head {};

        Page {
            lang: Some("en".to_string()),
            head: Box::new(head),
            body: None,
            buf: String::with_capacity(PAGE_BUFFER_SIZE),
        }
    }

    pub fn write(&mut self, props: &crate::pages::page::PageProps) -> Result<()> {
        self.buf.push_str(r#"<!DOCTYPE html>"#);
        if let Some(lang) = &self.lang {
            self.buf.push_str(r#"<html lang=""#);
            self.buf.push_str(lang);
            self.buf.push_str(r#"">"#);
        } else {
            self.buf.push_str(r#"<html>"#);
        }

        if let Err(e) = self.head.write(props, &mut self.buf) {
            return Err(e);
        }

        if let Some(b) = &self.body {
            if let Err(e) = b.write(props, &mut self.buf) {
                return Err(e);
            }
        }

        self.buf.push_str(r#"</html>"#);

        Ok(())
    }
}
