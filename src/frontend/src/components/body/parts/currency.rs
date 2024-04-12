use crate::{components::Component, Props};

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

pub struct CurrencyForm {}

impl Component for CurrencyForm {
    fn write(&self, props: &Props, buf: &mut String) {
        if let Some(currency_codes) = &props.currency_codes {
            *buf += r#"<span class="icon currency-icon"> "#;
            *buf += currency_logo(props.user_currency.as_str());
            *buf += r#"</span>"#;

            *buf += r#"<form method="POST" class="controls-form" action="/setCurrency" id="currency_form">"#;
            {
                *buf += r#"<select name="currency_code" onchange="document.getElementById('currency_form').submit();">"#;

                for currency_code in currency_codes.currency_codes.iter() {
                    if whitelisted_currencies(currency_code.as_str()) {
                        *buf += r#"<option value=""#;
                        *buf += currency_code;
                        *buf += r#"""#;
                        if *currency_code == props.user_currency {
                            *buf += r#" selected="selected""#;
                        }
                        *buf += r#">"#;
                        *buf += currency_code;
                        *buf += r#"</option>"#;
                    }
                }
                *buf += r#"</select>"#;
            }
            *buf += r#"</form>"#;
        }
    }
}
