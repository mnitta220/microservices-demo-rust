use crate::{components::body::order::OrderBody, pages::page};

/// Component for rendering the order page
pub struct OrderPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl OrderPage {
    pub fn new(props: page::Props) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = OrderBody::new();
        page.body = Some(Box::new(body));

        OrderPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
