use super::{CurrencyServiceClient, Empty};
use anyhow::Result;
use tonic::transport::Channel;

impl super::Money {
    pub fn money_for_display(&self) -> String {
        format!(
            "{}{}.{:.2}",
            currency_logo(&self.currency_code),
            self.units,
            self.nanos / 10000000
        )
    }
}

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

pub async fn get_currency_service_client() -> Result<CurrencyServiceClient<Channel>> {
    let currency_service_addr = crate::CURRENCY_SERVICE_ADDR.get().unwrap();

    let currency_service_client =
        match CurrencyServiceClient::connect(format!("http://{}", currency_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err(anyhow::anyhow!("get_currency_service_client failed"));
            }
        };

    Ok(currency_service_client)
}

pub async fn get_supported_currencies() -> Result<Vec<String>> {
    let mut currency_service_client = match get_currency_service_client().await {
        Ok(client) => client,
        Err(_) => {
            return Err(anyhow::anyhow!("get_currencies: connect failed"));
        }
    };

    let currencies = match currency_service_client
        .get_supported_currencies(Empty {})
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return Err(anyhow::anyhow!(
                "get_currencies: get_supported_currencies failed"
            ));
        }
    };

    let mut list: Vec<String> = Vec::new();
    for currency_code in currencies.get_ref().currency_codes.iter() {
        if whitelisted_currencies(currency_code.as_str()) {
            list.push(currency_code.to_string());
        }
    }

    Ok(list)
}
