use crate::{Component, PageProps};
use anyhow::Result;

pub struct Footer {}

impl Component for Footer {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
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
                        buf.push_str(r#"<small>"#);
                        {
                            buf.push_str(r#"Original : &nbsp;&nbsp;"#);
                        }
                        buf.push_str(r#"</small>"#);
                        buf.push_str(r#"© 2020 Google Inc ("#);
                        buf.push_str(
                          r#"<a href="https://github.com/GoogleCloudPlatform/microservices-demo">"#,
                        );
                        {
                            buf.push_str(r#"Source Code"#);
                        }
                        buf.push_str(r#"</a>)<br/>"#);
                        buf.push_str(r#"<small>"#);
                        {
                            buf.push_str(r#"Rewrite in Rust : &nbsp;&nbsp;"#);
                        }
                        buf.push_str(r#"</small>"#);
                        buf.push_str(r#"© 2024 Masahiro Nitta ("#);
                        buf.push_str(
                            r#"<a href="https://github.com/mnitta220/microservices-demo-rust">"#,
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
                            buf.push_str(&props.session_id);
                            buf.push_str(r#" — request-id: "#);
                            buf.push_str(&props.request_id);
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
        buf.push_str(
            r#"<script src="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/js/bootstrap.min.js" "#,
        );
        buf.push_str(
            r#"integrity="sha384-smHYKdLADwkXOn1EmN1qk/HfnUcbVRZyYmZ4qpPea6sjB/pTJ0euyQp0Mk8ck+5T" "#,
        );
        buf.push_str(r#"crossorigin="anonymous">"#);
        buf.push_str(r#"</script>"#);

        Ok(())
    }
}
