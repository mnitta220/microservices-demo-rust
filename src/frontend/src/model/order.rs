use crate::rpc::{checkout, hipstershop};
use anyhow::Result;

pub struct Order {
    pub order: hipstershop::OrderResult,
    pub total_cost: hipstershop::Money,
}

impl Order {
    pub async fn place_order(
        input: crate::handlers::PlaceOrderInput,
        session_id: &String,
        currency: &String,
    ) -> Result<Self> {
        let request = hipstershop::PlaceOrderRequest {
            user_id: session_id.clone(),
            user_currency: currency.clone(),
            address: Some(hipstershop::Address {
                street_address: input.street_address,
                city: input.city,
                state: input.state,
                country: input.country,
                zip_code: input.zip_code,
            }),
            email: input.email,
            credit_card: Some(hipstershop::CreditCardInfo {
                credit_card_number: input.credit_card_number,
                credit_card_cvv: input.credit_card_cvv,
                credit_card_expiration_year: input.credit_card_expiration_year,
                credit_card_expiration_month: input.credit_card_expiration_month,
            }),
        };

        let (order, total_cost) = checkout::place_order(request, currency).await?;

        Ok(Order { order, total_cost })
    }
}
