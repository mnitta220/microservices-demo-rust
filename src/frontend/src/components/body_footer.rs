use crate::PageProps;

pub struct BodyFooter {}

impl BodyFooter {
    pub fn write(&self, buf: &mut String, page_props: &mut PageProps) -> Result<(), &'static str> {
        buf.push_str(r#"<footer class="py-5">"#);
        {
            buf.push_str(r#"<div class="footer-top">"#);
            {
                buf.push_str(r#"<div class="container footer-social">"#);
                {
                    buf.push_str(r#"<p class="footer-text">"#);
                    {
                        buf.push_str(r#"This website is hosted for demo purposes only. It is not an actual shop. This is not a Google product."#);
                    }
                    buf.push_str(r#"</p>"#);

                    buf.push_str(r#"<p class="footer-text">"#);
                    {
                        buf.push_str(r#"© 2020 Google Inc ("#);
                        buf.push_str(
                        r#"<a href="https://github.com/GoogleCloudPlatform/microservices-demo">"#,
                        );
                        {
                            buf.push_str(r#"Source Code"#);
                        }
                        buf.push_str(r#"</a>)"#);
                    }
                    buf.push_str(r#"</p>"#);

                    buf.push_str(r#"<p class="footer-text">"#);
                    {
                        buf.push_str(r#"<small>"#);
                        {
                            buf.push_str(r#"session-id: "#);
                            buf.push_str(&page_props.session_id);
                            buf.push_str(r#" — request-id: "#);
                            buf.push_str(&page_props.request_id);
                        }
                        buf.push_str(r#"</small>"#);
                        /* don't get hostname
                        buf.push_str(r#"<br />"#);
                        buf.push_str(r#"<small>"#);
                        {
                            buf.push_str(r#"<b>Pod: </b>"#);
                            buf.push_str(r#"frontend-55dc66b9dd-hc7vm"#);
                        }
                        buf.push_str(r#"</small>"#);
                        */
                    }
                    buf.push_str(r#"</p>"#);
                }
                buf.push_str(r#"</div>"#);
            }
            buf.push_str(r#"</div>"#);
        }
        buf.push_str(r#"</footer>"#);

        Ok(())
    }
}
