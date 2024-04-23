use crate::{components::body::product::ProductBody, pages::page};

pub struct ProductPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl ProductPage {
    pub fn new(props: page::Props) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = ProductBody::new();
        page.body = Some(Box::new(body));

        ProductPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
