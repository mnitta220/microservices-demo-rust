use super::super::Component;
use super::parts::{footer::Footer, header::BodyHeader};
use crate::Props;

pub struct OrderBody {
    pub header: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl OrderBody {
    pub fn new() -> Self {
        let body_header = BodyHeader::new();
        let footer = Footer {};

        OrderBody {
            header: Box::new(body_header),
            footer: Box::new(footer),
        }
    }
}

impl Component for OrderBody {
    fn write(&self, props: &Props, buf: &mut String) {
        if let Some(o) = &props.order {
            let order = &o.order;
            let total_cost = &o.total_cost;

            buf.push_str(r#"<body>"#);
            {
                self.header.write(props, buf);

                buf.push_str(r#"<main role="main" class="order">"#);
                {
                    buf.push_str(r#"<section class="container order-complete-section">"#);
                    {
                        buf.push_str(r#"<div class="row">"#);
                        {
                            buf.push_str(r#"<div class="col-12 text-center">"#);
                            {
                                buf.push_str(r#"<h3>Your order is complete!</h3>"#);
                            }
                            buf.push_str(r#"</div>"#);

                            buf.push_str(r#"<div class="col-12 text-center">"#);
                            {
                                buf.push_str(r#"<p>We've sent you a confirmation email.</p>"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);

                        buf.push_str(r#"<div class="row border-bottom-solid padding-y-24">"#);
                        {
                            buf.push_str(r#"<div class="col-6 pl-md-0">"#);
                            {
                                buf.push_str(r#"Confirmation #"#);
                            }
                            buf.push_str(r#"</div>"#);

                            buf.push_str(r#"<div class="col-6 pr-md-0 text-right">"#);
                            {
                                buf.push_str(&order.order_id);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);

                        buf.push_str(r#"<div class="row border-bottom-solid padding-y-24">"#);
                        {
                            buf.push_str(r#"<div class="col-6 pl-md-0">"#);
                            {
                                buf.push_str(r#"Tracking #"#);
                            }
                            buf.push_str(r#"</div>"#);

                            buf.push_str(r#"<div class="col-6 pr-md-0 text-right">"#);
                            {
                                buf.push_str(&order.shipping_tracking_id);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);

                        buf.push_str(r#"<div class="row padding-y-24">"#);
                        {
                            buf.push_str(r#"<div class="col-6 pl-md-0">"#);
                            {
                                buf.push_str(r#"Total Paid"#);
                            }
                            buf.push_str(r#"</div>"#);

                            buf.push_str(r#"<div class="col-6 pr-md-0 text-right">"#);
                            {
                                buf.push_str(&total_cost.money_for_display());
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);

                        buf.push_str(r#"<div class="row">"#);
                        {
                            buf.push_str(r#"<div class="col-12 text-center">"#);
                            {
                                buf.push_str(
                                    r#"<a class="cymbal-button-primary" href="/" role="button">"#,
                                );
                                {
                                    buf.push_str(r#"Continue Shopping"#);
                                }
                                buf.push_str(r#"</a>"#);
                            }
                            buf.push_str(r#"</div>"#);
                        }
                        buf.push_str(r#"</div>"#);
                    }
                    buf.push_str(r#"</section>"#);

                    if let Some(recommendations) = &props.recommendations {
                        recommendations.write(props, buf)
                    }
                }
                buf.push_str(r#"</main>"#);

                self.footer.write(props, buf);
            }
            buf.push_str(r#"</body>"#);
        }
    }
}
