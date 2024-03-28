use super::{CheckoutServiceClient, PlaceOrderRequest, PlaceOrderResponse};
use anyhow::Result;
use tonic::transport::Channel;

async fn get_checkout_service_client() -> Result<CheckoutServiceClient<Channel>> {
    let checkout_service_addr = crate::CHECKOUT_SERVICE_ADDR.get().unwrap();

    let checkout_service_client =
        match CheckoutServiceClient::connect(format!("http://{}", checkout_service_addr)).await {
            Ok(client) => client,
            Err(_) => {
                return Err(anyhow::anyhow!("get_recommendation_service_client failed"));
            }
        };

    Ok(checkout_service_client)
}

pub async fn place_order(request: PlaceOrderRequest) -> Result<PlaceOrderResponse> {
    let mut checkout_service_client = get_checkout_service_client().await?;

    let request: tonic::Request<PlaceOrderRequest> = tonic::Request::new(request);

    let order = match checkout_service_client.place_order(request).await {
        Ok(response) => response.into_inner(),
        Err(_) => {
            return Err(anyhow::anyhow!("failed place_order"));
        }
    };

    Ok(order)
}
