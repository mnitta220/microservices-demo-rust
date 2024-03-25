use super::{CheckoutServiceClient, PlaceOrderRequest, PlaceOrderResponse};
use std::env;
use tonic::transport::Channel;

async fn get_checkout_service_client() -> Result<CheckoutServiceClient<Channel>, &'static str> {
    let checkout_service_addr = match env::var("CHECKOUT_SERVICE_ADDR") {
        Ok(addr) => addr,
        Err(_) => {
            return Err("Failed to get CHECKOUT_SERVICE_ADDR");
        }
    };

    let checkout_service_client =
        match CheckoutServiceClient::connect(format!("http://{}", checkout_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err("get_recommendation_service_client failed");
            }
        };

    Ok(checkout_service_client)
}

pub async fn place_order(request: PlaceOrderRequest) -> Result<PlaceOrderResponse, &'static str> {
    let mut checkout_service_client = match get_checkout_service_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };

    let request: tonic::Request<PlaceOrderRequest> = tonic::Request::new(request);

    let order = match checkout_service_client.place_order(request).await {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return Err("failed place_order");
        }
    };

    Ok(order)
}
