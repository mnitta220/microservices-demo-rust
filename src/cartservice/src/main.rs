use tonic::{transport::Server, Request, Response, Status};

pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use hipstershop::cart_service_server::{CartService, CartServiceServer};
use hipstershop::{AddItemRequest, Cart, CartItem, Empty, EmptyCartRequest, GetCartRequest};
//use redis::AsyncCommands;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
impl CartService for CartServiceImpl {
    async fn get_cart(&self, request: Request<GetCartRequest>) -> Result<Response<Cart>, Status> {
        println!("Got a request: {:?}", request);
        let redis_addr = match env::var("REDIS_ADDR") {
            Ok(addr) => addr,
            Err(_) => {
                return Err(tonic::Status::new(
                    tonic::Code::Internal,
                    "Failed to get REDIS_ADDR",
                ));
            }
        };

        let user_id = request.into_inner().user_id;

        let store = match get_string(&redis_addr, &user_id) {
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
            items.push(CartItem {
                product_id: item.product_id,
                quantity: item.quantity,
            });
        }

        let cart = Cart { user_id, items };

        Ok(Response::new(cart))
    }

    async fn add_item(
        &self,
        request: tonic::Request<AddItemRequest>,
    ) -> std::result::Result<tonic::Response<Empty>, tonic::Status> {
        let redis_addr = match env::var("REDIS_ADDR") {
            Ok(addr) => addr,
            Err(_) => {
                return Err(tonic::Status::new(
                    tonic::Code::Internal,
                    "Failed to get REDIS_ADDR",
                ));
            }
        };

        let req = request.into_inner();
        let item = match req.item {
            Some(i) => i,
            None => {
                return Ok(Response::new(Empty {}));
            }
        };

        let store = CartStoreItem {
            product_id: item.product_id,
            quantity: item.quantity,
        };

        //let stores = vec![store];
        let list = CartStoreItemList { items: vec![store] };

        let serialized = match serde_json::to_string(&list) {
            Ok(s) => s,
            Err(_) => {
                return Ok(Response::new(Empty {}));
            }
        };

        if let Err(_) = set_string(&redis_addr, &req.user_id, &serialized) {
            return Err(tonic::Status::new(
                tonic::Code::Internal,
                "failed to store cart item to redis",
            ));
        }

        Ok(Response::new(Empty {}))
    }

    async fn empty_cart(
        &self,
        _request: tonic::Request<EmptyCartRequest>,
    ) -> std::result::Result<tonic::Response<Empty>, tonic::Status> {
        Ok(Response::new(Empty {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cartservice=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<CartServiceServer<CartServiceImpl>>()
        .await;

    let addr = "0.0.0.0:7070".parse().unwrap();
    let cart_service = CartServiceImpl::default();

    tracing::debug!("HealthServer + CartServiceServer listening on {}", addr);
    //println!("HealthServer + CartServiceServer listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(CartServiceServer::new(cart_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn get_string(redis_addr: &String, key: &str) -> redis::RedisResult<String> {
    // connect to redis REDIS_ADDR
    let client = redis::Client::open(format!("redis://{}", redis_addr))?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    //let _: () = con.set("my_key", param)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get(key)
    //con.set(key, param)
}

fn set_string(redis_addr: &String, key: &str, param: &String) -> redis::RedisResult<String> {
    // connect to redis REDIS_ADDR
    let client = redis::Client::open(format!("redis://{}", redis_addr))?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    //let _: () = con.set("my_key", param)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    //con.get("my_key")
    con.set(key, param)
}
