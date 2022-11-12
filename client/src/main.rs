use crate::proxy::{proxy_client::ProxyClient, ProxyRequest};

pub mod proxy {
    tonic::include_proto!("proxy");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ProxyClient::connect("http://0.0.0.0:50051").await?;

    let request = tonic::Request::new(ProxyRequest {
        url: "https://blockstream.info/api/blocks/0".into(),
    });

    let response = client.get(request).await?.get_ref().response.clone();
    println!("{response}");
    Ok(())
}
