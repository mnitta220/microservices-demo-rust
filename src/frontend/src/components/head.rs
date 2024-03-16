pub struct Head {}

impl Head {
    pub fn write(&self, buf: &mut String) -> Result<(), &'static str> {
        buf.push_str(r#"<head>"#);
        {
            buf.push_str(r#"<meta charset="UTF-8">"#);
            buf.push_str(r#"<meta name="viewport" content="width=device-width, initial-scale=1.0, shrink-to-fit=no">"#);
            buf.push_str(r#"<meta http-equiv="X-UA-Compatible" content="ie=edge">"#);
            buf.push_str(r#"<title>Online Boutique</title>"#);
            buf.push_str(r#"<link href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/css/bootstrap.min.css" rel="stylesheet" "#);
            buf.push_str(r#"integrity="sha384-WskhaSGFgHYWDcbwN70/dfYBj47jz9qbsMId/iRN3ewGhXQFZCSftd1LZCfmhktB" crossorigin="anonymous">"#);
            buf.push_str(r#"<link rel="preconnect" href="https://fonts.googleapis.com">"#);
            buf.push_str(r#"<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>"#);
            buf.push_str(r#"<link href="https://fonts.googleapis.com/css2?family=DM+Sans:ital,wght@0,400;0,700;1,400;1,700&display=swap" "#);
            buf.push_str(r#"rel="stylesheet">"#);
            buf.push_str(
                r#"<link rel="stylesheet" type="text/css" href="/static/styles/styles.css">"#,
            );
            buf.push_str(
                r#"<link rel="stylesheet" type="text/css" href="/static/styles/cart.css">"#,
            );
            buf.push_str(
                r#"<link rel="stylesheet" type="text/css" href="/static/styles/order.css">"#,
            );
            buf.push_str(
                r#"<link rel='shortcut icon' type='image/x-icon' href='/static/favicon.ico' />"#,
            );
        }
        buf.push_str(r#"</head>"#);

        Ok(())
    }
}
