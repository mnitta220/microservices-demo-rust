use tonic::{Request, Response, Status};

use anyhow::Result;
use redis::Commands;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CartStoreItem {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct CartStoreItemList {
    pub items: Vec<CartStoreItem>,
}

#[derive(Default)]
pub struct CartServiceImpl {}

#[tonic::async_trait]
impl crate::CartService for CartServiceImpl {
    async fn get_cart(
        &self,
        request: Request<crate::GetCartRequest>,
    ) -> Result<Response<crate::Cart>, Status> {
        let cart = match get_cart_sub(request.into_inner()).await {
            Ok(c) => c,
            Err(_) => {
                return Err(tonic::Status::new(
                    tonic::Code::Internal,
                    "failed to get_cart",
                ));
            }
        };

        Ok(Response::new(cart))
    }

    async fn add_item(
        &self,
        request: tonic::Request<crate::AddItemRequest>,
    ) -> std::result::Result<tonic::Response<crate::Empty>, tonic::Status> {
        let req = request.into_inner();
        let item = match req.item {
            Some(i) => i,
            None => {
                return Ok(Response::new(crate::Empty {}));
            }
        };

        let get_cart_request = crate::GetCartRequest {
            user_id: req.user_id.clone(),
        };

        let cart = match get_cart_sub(get_cart_request).await {
            Ok(c) => c,
            Err(_) => {
                return Err(tonic::Status::new(
                    tonic::Code::Internal,
                    "failed to get_cart",
                ));
            }
        };

        let mut items = Vec::new();

        for item in cart.items {
            items.push(CartStoreItem {
                product_id: item.product_id,
                quantity: item.quantity,
            });
        }

        items.push(CartStoreItem {
            product_id: item.product_id,
            quantity: item.quantity,
        });

        let list = CartStoreItemList { items };

        let serialized = match serde_json::to_string(&list) {
            Ok(s) => s,
            Err(_) => {
                return Ok(Response::new(crate::Empty {}));
            }
        };

        if let Err(_) = set_redis_value(&req.user_id, &serialized) {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "failed to store cart item to redis",
            ));
        }

        Ok(Response::new(crate::Empty {}))
    }

    async fn empty_cart(
        &self,
        request: tonic::Request<crate::EmptyCartRequest>,
    ) -> std::result::Result<tonic::Response<crate::Empty>, tonic::Status> {
        let req = request.into_inner().user_id;

        if let Err(_) = set_redis_value(&req, &"".to_string()) {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "failed to store cart item to redis",
            ));
        }

        Ok(Response::new(crate::Empty {}))
    }
}

async fn get_cart_sub(request: crate::GetCartRequest) -> Result<crate::Cart> {
    let store = match get_redis_value(&request.user_id) {
        Ok(s) => s,
        Err(_) => {
            let list = CartStoreItemList { items: Vec::new() };
            serde_json::to_string(&list).unwrap()
        }
    };

    let deserialized: CartStoreItemList = match serde_json::from_str(&store) {
        Ok(d) => d,
        Err(_) => CartStoreItemList { items: Vec::new() },
    };

    let mut items = Vec::new();

    for item in deserialized.items {
        items.push(crate::CartItem {
            product_id: item.product_id,
            quantity: item.quantity,
        });
    }

    let cart = crate::Cart {
        user_id: request.user_id,
        items,
    };

    Ok(cart)
}

fn get_connection() -> redis::RedisResult<redis::Connection> {
    let redis_addr = crate::REDIS_ADDR.get().unwrap();
    let client = redis::Client::open(format!("redis://{}", redis_addr))?;
    client.get_connection()
}

fn get_redis_value(key: &str) -> redis::RedisResult<String> {
    let mut con = get_connection()?;
    con.get(key)
}

fn set_redis_value(key: &str, param: &String) -> redis::RedisResult<String> {
    let mut con = get_connection()?;
    con.set(key, param)
}
