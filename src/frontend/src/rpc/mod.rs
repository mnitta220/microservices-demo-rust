pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use hipstershop::{
    ad_service_client::AdServiceClient, cart_service_client::CartServiceClient,
    checkout_service_client::CheckoutServiceClient, currency_service_client::CurrencyServiceClient,
    product_catalog_service_client::ProductCatalogServiceClient,
    recommendation_service_client::RecommendationServiceClient,
    shipping_service_client::ShippingServiceClient, Ad, AdRequest, AddItemRequest, CartItem,
    CurrencyConversionRequest, Empty, EmptyCartRequest, GetCartRequest, GetProductRequest,
    GetQuoteRequest, ListRecommendationsRequest, Money, PlaceOrderRequest, PlaceOrderResponse,
    Product,
};

pub mod ad;
pub mod cart;
pub mod checkout;
pub mod currency;
pub mod product;
pub mod recommendation;
pub mod shipping;
