use crate::{
    error::Result,
    proxy::{proxy_server::Proxy, ProxyRequest, ProxyResponse},
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ReverseProxy {}

impl ReverseProxy {
    /// Handle the proxy request, sending it through the HTTP Client
    async fn handle_request(url: &str) -> Result<String> {
        let resp = reqwest::get(url).await?.text().await?;
        Ok(resp)
    }
}

#[tonic::async_trait]
impl Proxy for ReverseProxy {
    async fn get(
        &self,
        request: Request<ProxyRequest>,
    ) -> std::result::Result<Response<ProxyResponse>, Status> {
        Ok(Response::new(ProxyResponse {
            response: Self::handle_request(&request.get_ref().url).await?,
        }))
    }
}
