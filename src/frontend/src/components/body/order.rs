use super::super::Component;
use super::parts::{footer::Footer, header::BodyHeader, recommendation::RecommendationList};
use crate::{
    rpc::hipstershop::{Money, OrderResult},
    PageProps,
};
use anyhow::Result;

pub struct OrderBody {
    pub header: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub recommendation_list: RecommendationList,
    pub order: OrderResult,
    pub total_cost: Money,
}

impl OrderBody {
    pub async fn load(
        props: &PageProps,
        order: OrderResult,
        total_cost: Money,
    ) -> Result<Box<Self>> {
        let recommendation_list = match RecommendationList::load(&props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let body_header = match BodyHeader::load(props).await {
            Ok(response) => response,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let footer = Footer {};

        let body = OrderBody {
            header: Box::new(body_header),
            footer: Box::new(footer),
            recommendation_list,
            order,
            total_cost,
        };

        Ok(Box::new(body))
    }
}

impl Component for OrderBody {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()> {
        buf.push_str(r#"<body>"#);
        {
            self.header.write(props, buf)?;

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
                            buf.push_str(&self.order.order_id);
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
                            buf.push_str(&self.order.shipping_tracking_id);
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
                            buf.push_str(&self.total_cost.money_for_display());
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

                self.recommendation_list.write(props, buf)?;
            }
            buf.push_str(r#"</main>"#);
            self.footer.write(props, buf)?;
        }
        buf.push_str(r#"</body>"#);

        Ok(())
    }
}
