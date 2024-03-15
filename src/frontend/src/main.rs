use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "frontend=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(handler))
        .route("/_healthz", get(health))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    tracing::debug!("GET /");
    let buf = r#"
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
              <div class="row hot-products-row px-xl-6">
                <div class="col-12">
                  <h3>Hot Products</h3>
                </div>
                <div class="col-md-4 hot-product-card">
                  <a href="/product/OLJCESPC7Z">
                    <img alt="" src="/static/img/products/sunglasses.jpg">
                    <div class="hot-product-card-img-overlay"></div>
                  </a>
                  <div>
                    <div class="hot-product-card-name">Sunglasses1</div>
                    <div class="hot-product-card-price">$19.99</div>
                  </div>
                </div>
                <div class="col-md-4 hot-product-card">
                  <a href="/product/66VCHSJNUP">
                    <img alt="" src="/static/img/products/tank-top.jpg">
                    <div class="hot-product-card-img-overlay"></div>
                  </a>
                  <div>
                    <div class="hot-product-card-name">Tank Top2</div>
                    <div class="hot-product-card-price">$18.99</div>
                  </div>
                </div>
                <div class="col-md-4 hot-product-card">
                  <a href="/product/1YMWWN1N4O">
                    <img alt="" src="/static/img/products/watch.jpg">
                    <div class="hot-product-card-img-overlay"></div>
                  </a>
                  <div>
                    <div class="hot-product-card-name">Watch3</div>
                    <div class="hot-product-card-price">$109.99</div>
                  </div>
                </div>
                <div class="col-md-4 hot-product-card">
                  <a href="/product/L9ECAV7KIM">
                    <img alt="" src="/static/img/products/loafers.jpg">
                    <div class="hot-product-card-img-overlay"></div>
                  </a>
                  <div>
                    <div class="hot-product-card-name">Loafers</div>
                    <div class="hot-product-card-price">$89.99</div>
                  </div>
                </div>
                <div class="col-md-4 hot-product-card">
                  <a href="/product/2ZYFJ3GM2N">
                    <img alt="" src="/static/img/products/hairdryer.jpg">
                    <div class="hot-product-card-img-overlay"></div>
                  </a>
                  <div>
                    <div class="hot-product-card-name">Hairdryer</div>
                    <div class="hot-product-card-price">$24.99</div>
                  </div>
                </div>
                <div class="col-md-4 hot-product-card">
                  <a href="/product/0PUK6V6EV0">
                    <img alt="" src="/static/img/products/candle-holder.jpg">
                    <div class="hot-product-card-img-overlay"></div>
                  </a>
                  <div>
                    <div class="hot-product-card-name">Candle Holder</div>
                    <div class="hot-product-card-price">$18.99</div>
                  </div>
                </div>
                <div class="col-md-4 hot-product-card">
                  <a href="/product/LS4PSXUNUM">
                    <img alt="" src="/static/img/products/salt-and-pepper-shakers.jpg">
                    <div class="hot-product-card-img-overlay"></div>
                  </a>
                  <div>
                    <div class="hot-product-card-name">Salt &amp; Pepper Shakers</div>
                    <div class="hot-product-card-price">$18.49</div>
                  </div>
                </div>
                <div class="col-md-4 hot-product-card">
                  <a href="/product/9SIQT8TOJO">
                    <img alt="" src="/static/img/products/bamboo-glass-jar.jpg">
                    <div class="hot-product-card-img-overlay"></div>
                  </a>
                  <div>
                    <div class="hot-product-card-name">Bamboo Glass Jar</div>
                    <div class="hot-product-card-price">$5.49</div>
                  </div>
                </div>
                <div class="col-md-4 hot-product-card">
                  <a href="/product/6E92ZMYYFZ">
                    <img alt="" src="/static/img/products/mug.jpg">
                    <div class="hot-product-card-img-overlay"></div>
                  </a>
                  <div>
                    <div class="hot-product-card-name">Mug</div>
                    <div class="hot-product-card-price">$8.99</div>
                  </div>
                </div>
              </div>
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
      <div class="d-lg-none">
        <footer class="py-5">
          <div class="footer-top">
            <div class="container footer-social">
              <p class="footer-text">This website is hosted for demo purposes only. It is not an actual shop. This is not a
                Google product.</p>
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
          integrity="sha384-smHYKdLADwkXOn1EmN1qk/HfnUcbVRZyYmZ4qpPea6sjB/pTJ0euyQp0Mk8ck+5T" crossorigin="anonymous">
          </script>
      </div>
    </body>
    </html>
    "#;
    //Html("<h1>Hello, World!</h1>")
    Html(buf)
}

async fn health() -> &'static str {
    "OK"
}
