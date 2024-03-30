use crate::rpc::{
    cart,
    hipstershop::{Money, Product},
};
use anyhow::Result;

pub struct CartItem {
    pub product: Product,
    pub quantity: i32,
    pub price: Money,
}

pub struct Cart {
    pub items: Vec<CartItem>,
    pub shipping_cost: Money,
    pub total_price: Money,
    pub total_quantity: i32,
}

impl Cart {
    pub async fn load(session_id: &String, currency: &String) -> Result<Self> {
        let cart = match cart::get_cart(session_id.clone(), currency.clone()).await {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        Ok(cart)
    }

    pub fn cart_size(&self) -> i32 {
        let mut size = 0;
        for item in &self.items {
            size += item.quantity;
        }
        size
    }
}
