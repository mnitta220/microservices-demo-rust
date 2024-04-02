use crate::rpc;
use anyhow::Result;

pub struct HotProducts {
    pub products: Vec<rpc::hipstershop::Product>,
}

impl HotProducts {
    pub async fn load(currency: &String) -> Result<Self> {
        let products = match rpc::product::get_product_list(currency).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        Ok(HotProducts { products })
    }
}
