use crate::{components::Component, Props};

pub struct Footer {}

impl Component for Footer {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<footer class="py-5">"#;
        {
            *buf += r#"<div class="footer-top">"#;
            {
                *buf += r#"<div class="container footer-social">"#;
                {
                    *buf += r#"<p class="footer-text">"#;
                    {
                        *buf += r#"This website is hosted for demo purposes only. It is not an actual shop. This is not a Google product."#;
                    }
                    *buf += r#"</p>"#;

                    *buf += r#"<p class="footer-text">"#;
                    {
                        *buf += r#"<small>"#;
                        {
                            *buf += r#"Original : &nbsp;&nbsp;"#;
                        }
                        *buf += r#"</small>"#;
                        *buf += r#"© 2020 Google Inc ("#;
                        *buf += r#"<a href="https://github.com/GoogleCloudPlatform/microservices-demo">"#;
                        {
                            *buf += r#"Source Code"#;
                        }
                        *buf += r#"</a>)<br/>"#;
                        *buf += r#"<small>"#;
                        {
                            *buf += r#"Rewrote in Rust : &nbsp;&nbsp;"#;
                        }
                        *buf += r#"</small>"#;
                        *buf += r#"© 2024 Masahiro Nitta ("#;
                        *buf +=
                            r#"<a href="https://github.com/mnitta220/microservices-demo-rust">"#;
                        {
                            *buf += r#"Source Code"#;
                        }
                        *buf += r#"</a>)"#;
                    }
                    *buf += r#"</p>"#;

                    *buf += r#"<p class="footer-text">"#;
                    {
                        *buf += r#"<small>"#;
                        {
                            *buf += r#"session-id: "#;
                            *buf += &props.session_id;
                            *buf += r#" — request-id: "#;
                            *buf += &props.request_id;
                        }
                        *buf += r#"</small>"#;

                        if let Some(hostname) = crate::HOST_NAME.get() {
                            *buf += r#"<br />"#;
                            *buf += r#"<small>"#;
                            {
                                *buf += r#"<b>Pod: </b>"#;
                                *buf += hostname;
                            }
                            *buf += r#"</small>"#;
                        }
                    }
                    *buf += r#"</p>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</div>"#;
        }
        *buf += r#"</footer>"#;

        *buf += r#"<script src="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/js/bootstrap.min.js" "#;
        *buf += r#"integrity="sha384-smHYKdLADwkXOn1EmN1qk/HfnUcbVRZyYmZ4qpPea6sjB/pTJ0euyQp0Mk8ck+5T" "#;
        *buf += r#"crossorigin="anonymous">"#;
        *buf += r#"</script>"#;
    }
}
