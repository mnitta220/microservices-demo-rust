//use super::super::rpc::products;

use super::body;
use super::head;

pub struct HomePage {}

impl HomePage {
    pub async fn write_page(&self, buf: &mut String) -> Result<(), &'static str> {
        buf.push_str(r#"<!DOCTYPE html>"#);
        buf.push_str(r#"<html lang="en">"#);
        let head = head::Head {};
        if let Err(e) = head.write(buf) {
            return Err(e);
        }
        let body = body::Body {};
        if let Err(e) = body.write(buf).await {
            return Err(e);
        }
        buf.push_str(r#"</html>"#);
        Ok(())
    }
}

/*
pub async fn generate_page(buf: &mut String) -> Result<(), &'static str> {
    let head = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0, shrink-to-fit=no">
      <meta http-equiv="X-UA-Compatible" content="ie=edge">
      <title>
        Online Boutique
      </title>
      <link href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/css/bootstrap.min.css" rel="stylesheet"
        integrity="sha384-WskhaSGFgHYWDcbwN70/dfYBj47jz9qbsMId/iRN3ewGhXQFZCSftd1LZCfmhktB" crossorigin="anonymous">
      <link rel="preconnect" href="https://fonts.googleapis.com">
      <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
      <link href="https://fonts.googleapis.com/css2?family=DM+Sans:ital,wght@0,400;0,700;1,400;1,700&display=swap"
        rel="stylesheet">
      <link rel="stylesheet" type="text/css" href="/static/styles/styles.css">
      <link rel="stylesheet" type="text/css" href="/static/styles/cart.css">
      <link rel="stylesheet" type="text/css" href="/static/styles/order.css">
      <link rel='shortcut icon' type='image/x-icon' href='/static/favicon.ico' />
    </head>
    <body>
      <header>
        <div class="navbar sub-navbar">
          <div class="container d-flex justify-content-between">
            <a href="/" class="navbar-brand d-flex align-items-center">
              <img src="/static/icons/Hipster_NavLogo.svg" alt="" class="top-left-logo" />
            </a>
            <div class="controls">
              <div class="h-controls">
                <div class="h-control">
                  <span class="icon currency-icon"> $</span>
                  <form method="POST" class="controls-form" action="/setCurrency" id="currency_form">
                    <select name="currency_code" onchange="document.getElementById('currency_form').submit();">
                      <option value="EUR">EUR</option>
                      <option value="USD" selected="selected">USD</option>
                      <option value="JPY">JPY</option>
                      <option value="GBP">GBP</option>
                      <option value="TRY">TRY</option>
                      <option value="CAD">CAD</option>
                    </select>
                  </form>
                  <img src="/static/icons/Hipster_DownArrow.svg" alt="" class="icon arrow" />
                </div>
              </div>
              <a href="/cart" class="cart-link">
                <img src="/static/icons/Hipster_CartIcon.svg" alt="Cart icon" class="logo" title="Cart" />
              </a>
            </div>
          </div>
        </div>
      </header>
      <div class="local">
        <span class="platform-flag">
          local
        </span>
      </div>
      <main role="main" class="home">
        <div class="home-mobile-hero-banner d-lg-none"></div>
        <div class="container-fluid">
          <div class="row">
            <div class="col-4 d-none d-lg-block home-desktop-left-image"></div>
            <div class="col-12 col-lg-8">
            "#;
    buf.push_str(head);

    if let Err(e) = products::get_products(buf).await {
        return Err(e);
    }

    let contents = r#"
              <div class="row d-none d-lg-block home-desktop-footer-row">
                <div class="col-12 p-0">
                  <footer class="py-5">
                    <div class="footer-top">
                      <div class="container footer-social">
                        <p class="footer-text">This website is hosted for demo purposes only. It is not an actual shop. This
                          is not a Google product.</p>
                        <p class="footer-text">© 2020 Google Inc (<a
                            href="https://github.com/GoogleCloudPlatform/microservices-demo">Source Code</a>)</p>
                        <p class="footer-text">
                          <small>
                            session-id: c6400be8-95e4-4719-9300-d86712a76acf —
                            request-id: ff5c663c-d516-4aab-b5a3-5af7197df163
                          </small>
                          <br />
                          <small>
                            <b>Pod: </b>frontend-55dc66b9dd-hc7vm
                          </small>
                        </p>
                      </div>
                    </div>
                  </footer>
                  <script src="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/js/bootstrap.min.js"
                    integrity="sha384-smHYKdLADwkXOn1EmN1qk/HfnUcbVRZyYmZ4qpPea6sjB/pTJ0euyQp0Mk8ck+5T"
                    crossorigin="anonymous">
                    </script>
                </div>
              </div>
            </div>
            </div>
            </div>
          </main>
        </body>
        </html>
                    "#;
    buf.push_str(contents);

    Ok(())
}
*/
