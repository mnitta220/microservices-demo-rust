use crate::rpc;

pub struct AdItem {
    pub ad: rpc::hipstershop::Ad,
}

impl AdItem {
    pub async fn load() -> Option<AdItem> {
        let ad: rpc::hipstershop::Ad = match rpc::ad::get_ad().await {
            Some(a) => a,
            None => return None,
        };

        Some(AdItem { ad })
    }
}
