use crate::{
    cache::{Cache, TTLCache, TTLCacheEntry},
    error::Result,
    proxy::{proxy_server::Proxy, ProxyRequest, ProxyResponse},
};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct ReverseProxy {
    cache: TTLCache<String, TTLCacheEntry<String>>,
}

impl ReverseProxy {
    pub fn new(ttl: u64) -> ReverseProxy {
        ReverseProxy {
            cache: TTLCache::new(ttl),
        }
    }
    /// Handle the proxy request, search if in the cache or get it via the HTTP Client
    async fn handle_request(&self, url: &str) -> Result<String> {
        println!("== Received a request for {url} ==");
        let resp = match self.cache.fetch(&url.into()) {
            Some(entry) => {
                println!("Found an entry for the request in the cache");
                entry.value()
            },
            None=> {
                // Get the response from the original server (does not block the cache)
                println!("Not present in the Cache, Request from original server");
                let res = reqwest::get(url).await?.text().await?;
                self.cache.insert(url.to_string(), TTLCacheEntry::new(res.clone()));
                res
            }
        };
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
            response: self.handle_request(&request.get_ref().url).await?,
        }))
    }
}
