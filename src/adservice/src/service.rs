use rand::{thread_rng, Rng};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct AdServiceImpl {}

#[tonic::async_trait]
impl crate::AdService for AdServiceImpl {
    async fn get_ads(
        &self,
        _request: Request<crate::AdRequest>,
    ) -> Result<Response<crate::AdResponse>, Status> {
        let map = crate::AD_MAP.get().unwrap();

        let mut rng = thread_rng();
        let n = rng.gen_range(0..map.len());
        let ad = map.get(n).unwrap();

        let ads = crate::AdResponse {
            ads: vec![crate::Ad {
                redirect_url: ad.redirect_url.clone(),
                text: ad.text.clone(),
            }],
        };

        Ok(Response::new(ads))
    }
}
