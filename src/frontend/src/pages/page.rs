use crate::{
    components::{head::Head, Component},
    rpc, CartInfo,
};
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
    pub cart_info: CartInfo,
}

pub struct Page {
    pub props: PageProps,
    pub lang: Option<String>,
    pub head: Box<dyn Component + Send>,
    pub body: Option<Box<dyn Component + Send>>,
    pub buf: String,
}

impl Page {
    pub async fn generate(
        session_id: &String,
        currency: &String,
        product_id: Option<String>,
    ) -> Result<Page> {
        let cart_info = match rpc::cart::get_cart_info(session_id.clone(), currency.clone()).await {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let props = PageProps {
            session_id: session_id.clone(),
            request_id: Uuid::new_v4().to_string(),
            user_currency: currency.clone(),
            product_id: product_id,
            cart_info,
        };

        let head = Head {};

        let page = Page {
            props,
            lang: Some("en".to_string()),
            head: Box::new(head),
            body: None,
            buf: String::with_capacity(PAGE_BUFFER_SIZE),
        };

        Ok(page)
    }

    pub fn write(&mut self) -> Result<()> {
        self.buf.push_str(r#"<!DOCTYPE html>"#);
        if let Some(lang) = &self.lang {
            self.buf.push_str(r#"<html lang=""#);
            self.buf.push_str(lang);
            self.buf.push_str(r#"">"#);
        } else {
            self.buf.push_str(r#"<html>"#);
        }

        if let Err(e) = self.head.write(&self.props, &mut self.buf) {
            return Err(e);
        }

        if let Some(b) = &self.body {
            if let Err(e) = b.write(&self.props, &mut self.buf) {
                return Err(e);
            }
        }

        self.buf.push_str(r#"</html>"#);

        Ok(())
    }
}
