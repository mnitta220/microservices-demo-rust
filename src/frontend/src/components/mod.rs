use crate::Props;

pub mod body;
pub mod head;

pub trait Component {
    fn write(&self, props: &Props, buf: &mut String);
}
