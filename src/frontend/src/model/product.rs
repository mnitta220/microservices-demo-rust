use crate::rpc;
use anyhow::Result;

pub struct Product {
    pub product: rpc::hipstershop::Product,
}

impl Product {
    pub async fn load(product_id: &String, currency: &String) -> Result<Self> {
        let product = match rpc::product::get_product(product_id.clone(), currency.clone()).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        Ok(Product { product })
    }
}
