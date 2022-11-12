use proxy::proxy_server::ProxyServer;
use reverse_proxy::ReverseProxy;
use tonic::transport::Server;

mod error;
mod reverse_proxy;

pub mod proxy {
    tonic::include_proto!("proxy");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    Server::builder()
        .add_service(ProxyServer::new(ReverseProxy::default()))
        .serve(addr)
        .await?;
    Ok(())
}
