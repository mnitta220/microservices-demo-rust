use crate::rpc;
use anyhow::Result;

pub struct Order {
    pub order: rpc::hipstershop::OrderResult,
    pub total_cost: rpc::hipstershop::Money,
}

impl Order {
    pub async fn place_order(
        input: crate::handlers::PlaceOrderInput,
        session_id: &String,
        currency: &String,
    ) -> Result<Self> {
        let request = rpc::hipstershop::PlaceOrderRequest {
            user_id: session_id.clone(),
            user_currency: currency.clone(),
            address: Some(rpc::hipstershop::Address {
                street_address: input.street_address,
                city: input.city,
                state: input.state,
                country: input.country,
                zip_code: input.zip_code,
            }),
            email: input.email,
            credit_card: Some(rpc::hipstershop::CreditCardInfo {
                credit_card_number: input.credit_card_number,
                credit_card_cvv: input.credit_card_cvv,
                credit_card_expiration_year: input.credit_card_expiration_year,
                credit_card_expiration_month: input.credit_card_expiration_month,
            }),
        };

        let (order, total_cost) = rpc::checkout::place_order(request, currency).await?;

        Ok(Order { order, total_cost })
    }
}
