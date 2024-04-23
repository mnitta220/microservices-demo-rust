use anyhow::Result;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub mod cart;
pub mod currency;
pub mod home;
pub mod product;

const COOKIE_SESSION_ID: &str = "shop_session-id";
const COOKIE_CURRENCY: &str = "shop_currency";

fn session_info(cookies: Cookies, should_exist: bool) -> Result<(String, String)> {
    let session_id = match cookies.get(COOKIE_SESSION_ID) {
        Some(s) => s.value().to_string(),
        None => {
            if should_exist {
                return Err(anyhow::anyhow!("failed to get session"));
            } else {
                let id = Uuid::new_v4().to_string();
                cookies.add(Cookie::new(COOKIE_SESSION_ID, id.clone()));
                id
            }
        }
    };

    let currency = match cookies.get(COOKIE_CURRENCY) {
        Some(s) => s.value().to_string(),
        None => {
            if should_exist {
                return Err(anyhow::anyhow!("failed to get currency"));
            } else {
                cookies.add(Cookie::new(COOKIE_CURRENCY, "USD".to_string()));
                "USD".to_string()
            }
        }
    };

    Ok((session_id, currency))
}

pub async fn health() -> &'static str {
    "OK"
}
