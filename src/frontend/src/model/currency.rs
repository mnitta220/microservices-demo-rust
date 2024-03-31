use crate::rpc::currency;
use anyhow::Result;

pub struct SupportedCurrencies {
    pub currency_codes: Vec<String>,
}

impl SupportedCurrencies {
    pub async fn load() -> Result<Self> {
        let currencies = match currency::get_supported_currencies().await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        Ok(SupportedCurrencies {
            currency_codes: currencies,
        })
    }
}
