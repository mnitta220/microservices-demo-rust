use tonic::transport::Server;

pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use anyhow::Result;
use hipstershop::product_catalog_service_server::{
    ProductCatalogService, ProductCatalogServiceServer,
};
use hipstershop::{
    Empty, GetProductRequest, ListProductsResponse, Product, SearchProductsRequest,
    SearchProductsResponse,
};
use once_cell::sync::OnceCell;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod service;

const FILE_NAME: &'static str = "./products.json";

static PRODUCT_LIST: OnceCell<Vec<Product>> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "productcatalogservice=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Err(e) = load_products().await {
        tracing::error!("failed to load product list: {:?}", e);
        std::process::exit(0x0100);
    }

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ProductCatalogServiceServer<service::ProductCatalogServiceImpl>>()
        .await;

    let addr = "0.0.0.0:3550".parse().unwrap();
    let product_catalog_service = service::ProductCatalogServiceImpl::default();

    tracing::debug!(
        "HealthServer + ProductCatalogServiceServer listening on {}",
        addr
    );

    Server::builder()
        .add_service(health_service)
        .add_service(ProductCatalogServiceServer::new(product_catalog_service))
        .serve(addr)
        .await?;

    Ok(())
}

async fn load_products() -> Result<()> {
    let content = tokio::fs::read_to_string(FILE_NAME)
        .await
        .expect("Failed to load JSON");
    let product_list: ListProductsResponse = serde_json::from_str(&content).unwrap();

    if let Err(_) = PRODUCT_LIST.set(product_list.products) {
        return Err(anyhow::anyhow!("Failed to set AD_SERVICE_ADDR"));
    }

    Ok(())
}
