pub mod hipstershop {
    tonic::include_proto!("hipstershop");
}

use hipstershop::{
    cart_service_client::CartServiceClient, currency_service_client::CurrencyServiceClient,
    product_catalog_service_client::ProductCatalogServiceClient,
    recommendation_service_client::RecommendationServiceClient,
    shipping_service_client::ShippingServiceClient, AddItemRequest, CartItem,
    CurrencyConversionRequest, Empty, EmptyCartRequest, GetCartRequest, GetProductRequest,
    GetQuoteRequest, ListRecommendationsRequest, Money, Product,
};
pub mod cart;
pub mod currency;
pub mod product;
pub mod recommendation;
pub mod shipping;
