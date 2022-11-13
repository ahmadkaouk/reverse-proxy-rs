/// A lightweight client to communicate with the Reverse Proxy Server via gRPC
use crate::proxy::{proxy_client::ProxyClient, ProxyRequest};
use clap::Parser;

pub mod proxy {
    tonic::include_proto!("proxy");
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// IP of the Proxy Server
    #[arg(long, default_value = "0.0.0.0")]
    server_ip: String,
    /// Port of the Proxy Server
    #[arg(long, default_value_t = 50051)]
    server_port: u16,
    /// URL to query
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse Command line arguments
    let args = Args::parse();

    // Addr of the proxy server
    let addr = "http://".to_owned() + &args.server_ip + ":" + &args.server_port.to_string();
    // Create a new client and connect to the server endpoint
    let mut client = ProxyClient::connect(addr).await?;
    // Create the proxy request
    let request = tonic::Request::new(ProxyRequest { url: args.url });

    let response = client.get(request).await?.get_ref().response.clone();
    println!("{response}");
    Ok(())
}
