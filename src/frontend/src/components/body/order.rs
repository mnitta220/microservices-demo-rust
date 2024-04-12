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

            *buf += r#"<body>"#;
            {
                self.header.write(props, buf);

                *buf += r#"<main role="main" class="order">"#;
                {
                    *buf += r#"<section class="container order-complete-section">"#;
                    {
                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<div class="col-12 text-center">"#;
                            {
                                *buf += r#"<h3>Your order is complete!</h3>"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-12 text-center">"#;
                            {
                                *buf += r#"<p>We've sent you a confirmation email.</p>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row border-bottom-solid padding-y-24">"#;
                        {
                            *buf += r#"<div class="col-6 pl-md-0">"#;
                            {
                                *buf += r#"Confirmation #"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-6 pr-md-0 text-right">"#;
                            {
                                *buf += &order.order_id;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row border-bottom-solid padding-y-24">"#;
                        {
                            *buf += r#"<div class="col-6 pl-md-0">"#;
                            {
                                *buf += r#"Tracking #"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-6 pr-md-0 text-right">"#;
                            {
                                *buf += &order.shipping_tracking_id;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row padding-y-24">"#;
                        {
                            *buf += r#"<div class="col-6 pl-md-0">"#;
                            {
                                *buf += r#"Total Paid"#;
                            }
                            *buf += r#"</div>"#;

                            *buf += r#"<div class="col-6 pr-md-0 text-right">"#;
                            {
                                *buf += &total_cost.money_for_display();
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;

                        *buf += r#"<div class="row">"#;
                        {
                            *buf += r#"<div class="col-12 text-center">"#;
                            {
                                *buf +=
                                    r#"<a class="cymbal-button-primary" href="/" role="button">"#;
                                {
                                    *buf += r#"Continue Shopping"#;
                                }
                                *buf += r#"</a>"#;
                            }
                            *buf += r#"</div>"#;
                        }
                        *buf += r#"</div>"#;
                    }
                    *buf += r#"</section>"#;

                    if let Some(recommendations) = &props.recommendations {
                        recommendations.write(props, buf)
                    }
                }
                *buf += r#"</main>"#;

                self.footer.write(props, buf);
            }
            *buf += r#"</body>"#;
        }
    }
}
