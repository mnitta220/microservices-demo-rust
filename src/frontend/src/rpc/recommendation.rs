pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use super::products;
use crate::PageProps;
use hipstershop::recommendation_service_client::RecommendationServiceClient;
use hipstershop::ListRecommendationsRequest;
use std::env;

async fn get_recommendation_service_client(
) -> Result<RecommendationServiceClient<tonic::transport::Channel>, &'static str> {
    let recommendation_service_addr = match env::var("RECOMMENDATION_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get RECOMMENDATION_SERVICE_ADDR");
        }
    };

    let recommendation_service_client = match RecommendationServiceClient::connect(format!(
        "http://{}",
        recommendation_service_addr
    ))
    .await
    {
        Ok(client) => client,
        Err(_) => {
            return Err("get_recommendation: connect failed");
        }
    };

    Ok(recommendation_service_client)
}

pub async fn get_recommendations(
    page_props: &PageProps,
) -> Result<Vec<products::hipstershop::Product>, &'static str> {
    let mut list: Vec<products::hipstershop::Product> = Vec::new();
    let mut recommendation_service_client = match get_recommendation_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };
    let mut product_catalog_service_client =
        match products::get_product_catalog_service_client().await {
            Ok(client) => client,
            Err(e) => {
                return Err(e);
            }
        };

    let product_id = match &page_props.product_id {
        Some(p) => (*p).clone(),
        None => {
            return Ok(list);
        }
    };
    let request = ListRecommendationsRequest {
        user_id: page_props.session_id.clone(),
        product_ids: vec![product_id],
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
        let request = products::hipstershop::GetProductRequest { id: id };

        if let Ok(response) = product_catalog_service_client.get_product(request).await {
            let product = response.into_inner();
            list.push(product);
        }
    }

    Ok(list)
}
