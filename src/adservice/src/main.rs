use tonic::{transport::Server, Request, Response, Status};

pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use hipstershop::ad_service_server::{AdService, AdServiceServer};
use hipstershop::{AdRequest, AdResponse};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Default)]
pub struct AdServiceImpl {}

#[tonic::async_trait]
impl AdService for AdServiceImpl {
    async fn get_ads(&self, request: Request<AdRequest>) -> Result<Response<AdResponse>, Status> {
        println!("Got a request: {:?}", request);

        let ads = AdResponse { ads: Vec::new() };

        Ok(Response::new(ads))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "adservice=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<AdServiceServer<AdServiceImpl>>()
        .await;

    let addr = "0.0.0.0:9555".parse().unwrap();
    let ad_service = AdServiceImpl::default();

    tracing::debug!("HealthServer + AdServiceServer listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(AdServiceServer::new(ad_service))
        .serve(addr)
        .await?;

    Ok(())
}
