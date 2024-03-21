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

pub async fn write_recommendations(
    buf: &mut String,
    page_props: &PageProps,
) -> Result<(), &'static str> {
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
            return Ok(());
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
            return Err("get_recommendations: list_recommendations failed");
        }
    };

    buf.push_str(r#"<section class="recommendations">"#);
    {
        buf.push_str(r#"<div class="container">"#);
        {
            buf.push_str(r#"<div class="row">"#);
            {
                buf.push_str(r#"<div class="col-xl-10 offset-xl-1">"#);
                {
                    buf.push_str(r#"<h2>You May Also Like</h2>"#);
                    buf.push_str(r#"<div class="row">"#);
                    let mut idx = 0;
                    for id in recommendations.product_ids {
                        if idx > 3 {
                            break;
                        }
                        idx += 1;
                        let request = products::hipstershop::GetProductRequest { id: id };

                        if let Ok(response) =
                            product_catalog_service_client.get_product(request).await
                        {
                            let product = response.into_inner();
                            buf.push_str(r#"<div class="col-md-3">"#);
                            {
                                buf.push_str(r#"<div>"#);
                                {
                                    buf.push_str(r#"<a href="/product/"#);
                                    buf.push_str(&product.id);
                                    buf.push_str(r#"">"#);
                                    {
                                        buf.push_str(r#"<img alt src=""#);
                                        buf.push_str(&product.picture);
                                        buf.push_str(r#"">"#);
                                    }
                                    buf.push_str(r#"</a>"#);
                                    buf.push_str(r#"<div>"#);
                                    {
                                        buf.push_str(r#"<h5>"#);
                                        buf.push_str(&product.name);
                                        buf.push_str(r#"</h5>"#);
                                    }
                                    buf.push_str(r#"</div>"#);
                                }
                                buf.push_str(r#"</div>"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                    }
                    buf.push_str(r#"</div>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</div>"#);
    }
    buf.push_str(r#"</section>"#);

    Ok(())
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
