use crate::PageProps;
use anyhow::Result;

pub mod ad_component;
pub mod cart;
pub mod cart_item;
pub mod currency;
pub mod footer;
pub mod header;
pub mod home;
pub mod hot_product;
pub mod product;
pub mod recommendations;

pub trait Body {
    fn load(props: &PageProps) -> impl std::future::Future<Output = Result<Box<Self>>> + Send;
}
