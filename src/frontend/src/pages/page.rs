use crate::{components, rpc, CartInfo, Component};
use anyhow::Result;
use uuid::Uuid;

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

        let head = components::head::Head {};

        let page = Page {
            props,
            lang: Some("en".to_string()),
            head: Box::new(head),
            body: None,
        };

        Ok(page)
    }

    pub fn write(&self, buf: &mut String) -> Result<()> {
        buf.push_str(r#"<!DOCTYPE html>"#);
        if let Some(lang) = &self.lang {
            buf.push_str(r#"<html lang=""#);
            buf.push_str(lang);
            buf.push_str(r#"">"#);
        } else {
            buf.push_str(r#"<html>"#);
        }

        if let Err(e) = self.head.write(&self.props, buf) {
            return Err(e);
        }

        if let Some(b) = &self.body {
            if let Err(e) = b.write(&self.props, buf) {
                return Err(e);
            }
        }

        buf.push_str(r#"</html>"#);

        Ok(())
    }
}
