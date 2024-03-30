use tonic::transport::Server;

pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use anyhow::Result;
use hipstershop::ad_service_server::{AdService, AdServiceServer};
use hipstershop::{Ad, AdRequest, AdResponse};
use once_cell::sync::OnceCell;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod service;

static AD_MAP: OnceCell<Vec<Ad>> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "adservice=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Err(e) = load_map() {
        tracing::error!("failed to load ad map: {:?}", e);
        std::process::exit(0x0100);
    }

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<AdServiceServer<service::AdServiceImpl>>()
        .await;

    let addr = "0.0.0.0:9555".parse().unwrap();
    let ad_service = service::AdServiceImpl::default();

    tracing::debug!("HealthServer + AdServiceServer listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(AdServiceServer::new(ad_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn load_map() -> Result<()> {
    let map = vec![
        Ad {
            redirect_url: "/product/2ZYFJ3GM2N".to_string(),
            text: "Hairdryer for sale. 50% off.".to_string(),
        },
        Ad {
            redirect_url: "/product/66VCHSJNUP".to_string(),
            text: "Tank top for sale. 20% off.".to_string(),
        },
        Ad {
            redirect_url: "/product/0PUK6V6EV0".to_string(),
            text: "Candle holder for sale. 30% off.".to_string(),
        },
        Ad {
            redirect_url: "/product/9SIQT8TOJO".to_string(),
            text: "Bamboo glass jar for sale. 10% off.".to_string(),
        },
        Ad {
            redirect_url: "/product/1YMWWN1N4O".to_string(),
            text: "Watch for sale. Buy one, get second kit for free".to_string(),
        },
        Ad {
            redirect_url: "/product/6E92ZMYYFZ".to_string(),
            text: "Mug for sale. Buy two, get third one for free".to_string(),
        },
        Ad {
            redirect_url: "/product/L9ECAV7KIM".to_string(),
            text: "Loafers for sale. Buy one, get second one for free".to_string(),
        },
    ];

    if let Err(_) = AD_MAP.set(map) {
        return Err(anyhow::anyhow!("Failed to set AD_SERVICE_ADDR"));
    }

    Ok(())
}
