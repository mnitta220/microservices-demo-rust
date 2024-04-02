use super::{
    product, GetProductRequest, ListRecommendationsRequest, Product, RecommendationServiceClient,
};
use anyhow::Result;
use tonic::transport::Channel;

async fn get_recommendation_service_client() -> Result<RecommendationServiceClient<Channel>> {
    let recommendation_service_addr = crate::RECOMMENDATION_SERVICE_ADDR.get().unwrap();

    let recommendation_service_client = match RecommendationServiceClient::connect(format!(
        "http://{}",
        recommendation_service_addr
    ))
    .await
    {
        Ok(client) => client,
        Err(_) => {
            return Err(anyhow::anyhow!("get_recommendation_service_client failed"));
        }
    };

    Ok(recommendation_service_client)
}

pub async fn get_recommendations(
    product_id: Option<String>,
    session_id: &String,
) -> Result<Vec<Product>> {
    let mut list: Vec<Product> = Vec::new();
    let mut recommendation_service_client = get_recommendation_service_client().await?;

    let mut product_catalog_service_client = product::get_product_catalog_service_client().await?;

    let product_ids = match product_id {
        Some(p) => vec![p],
        None => vec![],
    };

    let request = ListRecommendationsRequest {
        user_id: session_id.clone(),
        product_ids,
    };

    let recommendations = match recommendation_service_client
        .list_recommendations(request)
        .await
    {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return Ok(list);
        }
    };

    let mut idx = 0;

    for id in recommendations.product_ids {
        if idx > 3 {
            break;
        }

        idx += 1;
        let request = GetProductRequest { id: id };

        if let Ok(response) = product_catalog_service_client.get_product(request).await {
            let product = response.into_inner();
            list.push(product);
        }
    }

    Ok(list)
}
