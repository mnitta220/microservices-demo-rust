pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use super::super::{CURRENCY_LOGO, WHITELISTED_CURRENCIES};
use crate::PageProps;
use hipstershop::currency_service_client::CurrencyServiceClient;
use hipstershop::Empty;
use std::env;

pub async fn select_currency_form(
    buf: &mut String,
    page_props: &mut PageProps,
) -> Result<(), &'static str> {
    let addr = match env::var("CURRENCY_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get CURRENCY_SERVICE_ADDR");
        }
    };

    let mut client = match CurrencyServiceClient::connect(format!("http://{}", addr)).await {
        Ok(client) => client,
        Err(_) => {
            return Err("get_currencies: connect failed");
        }
    };

    let response = match client.get_supported_currencies(Empty {}).await {
        Ok(response) => response,
        Err(_) => {
            return Err("get_currencies: get_supported_currencies failed");
        }
    };

    buf.push_str(r#"<span class="icon currency-icon"> "#);
    if let Some(c) = CURRENCY_LOGO.get(page_props.user_currency.as_str()) {
        buf.push_str(c);
    } else {
        buf.push_str("$");
    }
    buf.push_str(r#"</span>"#);

    buf.push_str(
        r#"<form method="POST" class="controls-form" action="/setCurrency" id="currency_form">"#,
    );
    {
        buf.push_str(r#"<select name="currency_code" onchange="document.getElementById('currency_form').submit();">"#);
        for currency_code in response.get_ref().currency_codes.iter() {
            if let Some(c) = WHITELISTED_CURRENCIES.get(currency_code.as_str()) {
                if *c == true {
                    buf.push_str(r#"<option value=""#);
                    buf.push_str(currency_code);
                    buf.push_str(r#"""#);
                    if currency_code == &page_props.user_currency {
                        buf.push_str(r#" selected="selected""#);
                    }
                    buf.push_str(r#">"#);
                    buf.push_str(currency_code);
                    buf.push_str(r#"</option>"#);
                }
            }
        }
        buf.push_str(r#"</select>"#);
    }
    buf.push_str(r#"</form>"#);

    Ok(())
}
