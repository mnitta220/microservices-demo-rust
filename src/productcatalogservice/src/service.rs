use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct ProductCatalogServiceImpl {}

#[tonic::async_trait]
impl crate::ProductCatalogService for ProductCatalogServiceImpl {
    async fn list_products(
        &self,
        _request: Request<crate::Empty>,
    ) -> Result<Response<crate::ListProductsResponse>, Status> {
        let product_list = crate::PRODUCT_LIST.get().unwrap();
        let mut products = Vec::new();
        for product in product_list {
            products.push(product.clone());
        }
        let reply: crate::ListProductsResponse = crate::ListProductsResponse { products };
        Ok(Response::new(reply))
    }

    async fn get_product(
        &self,
        request: Request<crate::GetProductRequest>,
    ) -> Result<Response<crate::Product>, Status> {
        let products = crate::PRODUCT_LIST.get().unwrap();
        let key_id = request.into_inner().id;

        match products.iter().find(|p| p.id == key_id) {
            Some(product) => Ok(Response::new(product.clone())),
            None => Err(Status::new(
                tonic::Code::NotFound,
                format!("no product with ID {}", key_id),
            )),
        }
    }

    async fn search_products(
        &self,
        request: Request<crate::SearchProductsRequest>,
    ) -> Result<Response<crate::SearchProductsResponse>, Status> {
        let products = crate::PRODUCT_LIST.get().unwrap();
        let query = request.into_inner().query;

        let mut results: Vec<crate::Product> = vec![];

        for product in products {
            match product.name.find(&query) {
                Some(_) => results.push(product.clone()),
                None => match product.description.find(&query) {
                    Some(_) => results.push(product.clone()),
                    None => (),
                },
            }
        }

        Ok(Response::new(crate::SearchProductsResponse {
            results: results,
        }))
    }
}
