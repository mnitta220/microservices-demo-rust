use tonic::{transport::Server, Request, Response, Status};

use hipstershop::product_catalog_service_server::{
    ProductCatalogService, ProductCatalogServiceServer,
};
use hipstershop::{
    Empty, GetProductRequest, ListProductsResponse, Product, SearchProductsRequest,
    SearchProductsResponse,
};

pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

const FILE_NAME: &'static str = "./products.json";

async fn read_catalog_file() -> Vec<Product> {
    let content = tokio::fs::read_to_string(FILE_NAME)
        .await
        .expect("Failed to load JSON");
    let list_product_response: ListProductsResponse = serde_json::from_str(&content).unwrap();
    list_product_response.products
}

#[derive(Default)]
pub struct ProductCatalogServiceImpl {}

// サーバ用のコードを実装
#[tonic::async_trait]
impl ProductCatalogService for ProductCatalogServiceImpl {
    async fn list_products(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ListProductsResponse>, Status> {
        let products = read_catalog_file().await;
        let reply: ListProductsResponse = ListProductsResponse { products: products };
        Ok(Response::new(reply))
    }

    async fn get_product(
        &self,
        request: Request<GetProductRequest>,
    ) -> Result<Response<Product>, Status> {
        let products = read_catalog_file().await;
        let key_id = request.into_inner().id;

        for product in products {
            if key_id == product.id {
                return Ok(Response::new(product));
            }
        }

        Err(Status::new(
            tonic::Code::NotFound,
            format!("no product with ID {}", key_id),
        ))
    }

    async fn search_products(
        &self,
        request: Request<SearchProductsRequest>,
    ) -> Result<Response<SearchProductsResponse>, Status> {
        let products = read_catalog_file().await;
        let query = request.into_inner().query;

        let mut results: Vec<Product> = vec![];

        for product in products {
            match product.name.find(&query) {
                Some(_) => results.push(product),
                None => match product.description.find(&query) {
                    Some(_) => results.push(product),
                    None => (),
                },
            }
        }

        Ok(Response::new(SearchProductsResponse { results: results }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ProductCatalogServiceServer<ProductCatalogServiceImpl>>()
        .await;

    let addr = "0.0.0.0:3550".parse().unwrap();
    let product_catalog_service = ProductCatalogServiceImpl::default();

    println!(
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
