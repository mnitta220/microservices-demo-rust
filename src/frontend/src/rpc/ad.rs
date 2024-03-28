use super::{Ad, AdRequest, AdServiceClient};
use anyhow::Result;
use rand::{thread_rng, Rng};
use std::env;
use tonic::transport::Channel;

async fn get_ad_service_client() -> Result<AdServiceClient<Channel>> {
    let ad_service_addr = match env::var("AD_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get AD_SERVICE_ADDR"));
        }
    };

    let ad_service_client =
        match AdServiceClient::connect(format!("http://{}", ad_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err(anyhow::anyhow!("get_ad_service_client failed"));
            }
        };

    Ok(ad_service_client)
}

pub async fn get_ad() -> Option<Ad> {
    let mut ad_service_client = match get_ad_service_client().await {
        Ok(client) => client,
        Err(_) => {
            return None;
        }
    };

    let request = AdRequest {
        context_keys: Vec::new(),
    };

    let ads: Vec<Ad> = match ad_service_client.get_ads(request).await {
        Ok(response) => response.into_inner().ads,
        Err(_) => {
            return None;
        }
    };

    if ads.len() == 0 {
        return None;
    }

    let mut rng = thread_rng();
    let n = rng.gen_range(0..ads.len());

    Some(Ad {
        redirect_url: ads[n].redirect_url.clone(),
        text: ads[n].text.clone(),
    })
}
