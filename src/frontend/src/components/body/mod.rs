use crate::PageProps;
use anyhow::Result;

pub mod ad;
pub mod cart;
pub mod currency;
pub mod footer;
pub mod header;
pub mod home;
pub mod product;
pub mod recommendation;

pub trait Body {
    fn load(props: &PageProps) -> impl std::future::Future<Output = Result<Box<Self>>> + Send;
}
