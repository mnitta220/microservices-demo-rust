use super::super::Component;
use crate::{rpc::currency, PageProps};
use anyhow::Result;

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

pub struct CurrencyForm {
    pub currency_codes: Vec<String>,
}

impl CurrencyForm {
    pub async fn load(_props: &PageProps) -> Result<Self> {
        let currencies = match currency::get_supported_currencies().await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let currency_form = CurrencyForm {
            currency_codes: currencies,
        };

        Ok(currency_form)
    }
}

impl Component for CurrencyForm {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        buf.push_str(r#"<span class="icon currency-icon"> "#);
        buf.push_str(currency_logo(props.user_currency.as_str()));
        buf.push_str(r#"</span>"#);

        buf.push_str(
          r#"<form method="POST" class="controls-form" action="/setCurrency" id="currency_form">"#,
        );
        {
            buf.push_str(r#"<select name="currency_code" onchange="document.getElementById('currency_form').submit();">"#);
            for currency_code in self.currency_codes.iter() {
                if whitelisted_currencies(currency_code.as_str()) {
                    buf.push_str(r#"<option value=""#);
                    buf.push_str(currency_code);
                    buf.push_str(r#"""#);
                    if currency_code == &props.user_currency {
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
}