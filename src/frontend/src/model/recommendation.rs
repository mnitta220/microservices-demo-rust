use crate::rpc;
use anyhow::Result;

pub struct RecommendationList {
    pub items: Vec<RecommendationItem>,
}

pub struct RecommendationItem {
    pub product: rpc::hipstershop::Product,
}

impl RecommendationList {
    pub async fn load(product_id: Option<String>, session_id: &String) -> Result<Self> {
        let recommendation_list =
            match rpc::recommendation::get_recommendations(product_id, session_id).await {
                Ok(response) => response,
                Err(e) => {
                    return Err(anyhow::anyhow!(e));
                }
            };

        let mut items = Vec::new();
        for product in recommendation_list {
            items.push(RecommendationItem { product: product });
        }

        Ok(RecommendationList { items })
    }
}
