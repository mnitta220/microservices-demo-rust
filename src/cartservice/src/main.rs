pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use anyhow::Result;
use hipstershop::cart_service_server::{CartService, CartServiceServer};
use hipstershop::{AddItemRequest, Cart, CartItem, Empty, EmptyCartRequest, GetCartRequest};
use once_cell::sync::OnceCell;
use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod service;

static REDIS_ADDR: OnceCell<String> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cartservice=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Err(e) = get_environment_values() {
        tracing::error!("failed to get environment values: {:?}", e);
        std::process::exit(0x0100);
    }

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<CartServiceServer<service::CartServiceImpl>>()
        .await;

    let addr = "0.0.0.0:7070".parse().unwrap();
    let cart_service = service::CartServiceImpl::default();

    tracing::debug!("HealthServer + CartServiceServer listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(CartServiceServer::new(cart_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn get_environment_values() -> Result<()> {
    // get REDIS_ADDR env
    let addr = match std::env::var("REDIS_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get REDIS_ADDR"));
        }
    };
    // set REDIS_ADDR static
    if let Err(_) = REDIS_ADDR.set(addr) {
        return Err(anyhow::anyhow!("Failed to set REDIS_ADDR"));
    }

    Ok(())
}
