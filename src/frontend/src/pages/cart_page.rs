use crate::{components::body::cart::CartBody, pages::page};

/// Component for rendering the cart page
pub struct CartPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl CartPage {
    pub fn new(props: page::Props) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = CartBody::new();
        page.body = Some(Box::new(body));

        CartPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
