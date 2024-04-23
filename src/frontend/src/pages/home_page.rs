use crate::{components::body::home::HomeBody, pages::page};

/// Component for rendering the homepage
pub struct HomePage {
    pub props: page::Props,
    pub page: page::Page,
}

impl HomePage {
    pub fn new(props: page::Props) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = HomeBody::new();
        page.body = Some(Box::new(body));

        HomePage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
