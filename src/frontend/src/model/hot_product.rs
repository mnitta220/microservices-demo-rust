use crate::rpc::{hipstershop::Product, product};
use anyhow::Result;

pub struct HotProductItem {
    pub product: Product,
}

pub struct HotProducts {
    pub items: Vec<HotProductItem>,
}

impl HotProducts {
    pub async fn load(currency: &String) -> Result<Self> {
        let product_list = match product::get_product_list(currency).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let mut items = Vec::new();
        for product in product_list {
            items.push(HotProductItem { product: product });
        }

        Ok(HotProducts { items })
    }
}
