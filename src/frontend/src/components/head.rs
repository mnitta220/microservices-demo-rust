use crate::{components::Component, Props};

pub struct Head {}

impl Component for Head {
    fn write(&self, _props: &Props, buf: &mut String) {
        *buf += r#"<head>"#;
        {
            *buf += r#"<meta charset="UTF-8">"#;
            *buf += r#"<meta name="viewport" content="width=device-width, initial-scale=1.0, shrink-to-fit=no">"#;
            *buf += r#"<meta http-equiv="X-UA-Compatible" content="ie=edge">"#;
            *buf += r#"<title>Online Boutique</title>"#;
            *buf += r#"<link href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/css/bootstrap.min.css" rel="stylesheet" "#;
            *buf += r#"integrity="sha384-WskhaSGFgHYWDcbwN70/dfYBj47jz9qbsMId/iRN3ewGhXQFZCSftd1LZCfmhktB" crossorigin="anonymous">"#;
            *buf += r#"<link rel="preconnect" href="https://fonts.googleapis.com">"#;
            *buf += r#"<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>"#;
            *buf += r#"<link href="https://fonts.googleapis.com/css2?family=DM+Sans:ital,wght@0,400;0,700;1,400;1,700&display=swap" "#;
            *buf += r#"rel="stylesheet">"#;
            *buf += r#"<link rel="stylesheet" type="text/css" href="/static/styles/styles.css">"#;
            *buf += r#"<link rel="stylesheet" type="text/css" href="/static/styles/cart.css">"#;
            *buf += r#"<link rel="stylesheet" type="text/css" href="/static/styles/order.css">"#;
            *buf += r#"<link rel="icon" type="image/x-icon" href="/static/favicon.ico">"#;
        }
        *buf += r#"</head>"#;
    }
}
