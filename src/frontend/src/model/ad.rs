use crate::rpc::{ad, hipstershop::Ad};

pub struct AdItem {
    pub ad: Ad,
}

impl AdItem {
    pub async fn load() -> Option<AdItem> {
        let ad: Ad = match ad::get_ad().await {
            Some(a) => a,
            None => return None,
        };

        Some(AdItem { ad })
    }
}
