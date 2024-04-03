use crate::PageProps;

pub mod body;
pub mod head;

pub trait Component {
    fn write(&self, props: &PageProps, buf: &mut String);
}
