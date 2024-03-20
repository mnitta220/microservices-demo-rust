pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use crate::PageProps;
use hipstershop::currency_service_client::CurrencyServiceClient;
use hipstershop::Empty;
use std::env;

pub fn currency_logo(currency: &str) -> &'static str {
    match currency {
        "EUR" => "€",
        "CAD" => "$",
        "JPY" => "¥",
        "GBP" => "£",
        "TRY" => "₺",
        _ => "$",
    }
}

fn whitelisted_currencies(currency: &str) -> bool {
    match currency {
        "USD" | "EUR" | "CAD" | "JPY" | "GBP" | "TRY" => true,
        _ => false,
    }
}

pub async fn get_currency_service_client(
) -> Result<CurrencyServiceClient<tonic::transport::Channel>, &'static str> {
    let currency_service_addr = match env::var("CURRENCY_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get CURRENCY_SERVICE_ADDR");
        }
    };

    let currency_service_client =
        match CurrencyServiceClient::connect(format!("http://{}", currency_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err("get_currencies: connect failed");
            }
        };

    Ok(currency_service_client)
}

pub async fn select_currency_form(
    buf: &mut String,
    page_props: &PageProps,
) -> Result<(), &'static str> {
    /*
    let currency_service_addr = match env::var("CURRENCY_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get CURRENCY_SERVICE_ADDR");
        }
    };

    let mut currency_service_client =
        match CurrencyServiceClient::connect(format!("http://{}", currency_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err("get_currencies: connect failed");
            }
        };
        */

    let mut currency_service_client = match get_currency_service_client().await {
        Ok(client) => client,
        Err(_) => {
            return Err("get_currencies: connect failed");
        }
    };

    let currencies = match currency_service_client
        .get_supported_currencies(Empty {})
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err("get_currencies: get_supported_currencies failed");
        }
    };

    buf.push_str(r#"<span class="icon currency-icon"> "#);
    buf.push_str(currency_logo(page_props.user_currency.as_str()));
    buf.push_str(r#"</span>"#);

    buf.push_str(
        r#"<form method="POST" class="controls-form" action="/setCurrency" id="currency_form">"#,
    );
    {
        buf.push_str(r#"<select name="currency_code" onchange="document.getElementById('currency_form').submit();">"#);
        for currency_code in currencies.get_ref().currency_codes.iter() {
            if whitelisted_currencies(currency_code.as_str()) {
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
        buf.push_str(r#"</select>"#);
    }
    buf.push_str(r#"</form>"#);

    Ok(())
}
