pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use hipstershop::product_catalog_service_client::ProductCatalogServiceClient;
use hipstershop::{Empty, GetProductRequest, SearchProductsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ProductCatalogServiceClient::connect("http://127.0.0.1:3550").await?;

    let response1 = client.list_products(Empty {}).await?;

    println!("RESPONSE={:?}\n", response1);

    let request2 = tonic::Request::new(GetProductRequest {
        id: "OLJCESPC7Z".into(),
    });

    let response2 = client.get_product(request2).await?;

    println!("RESPONSE={:?}\n", response2);

    let request3 = tonic::Request::new(SearchProductsRequest {
        query: "Sunglasses".into(),
    });

    let response3 = client.search_products(request3).await?;

    println!("RESPONSE={:?}\n", response3);

    Ok(())
}
