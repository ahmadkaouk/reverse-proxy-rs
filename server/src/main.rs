/// A simple reverse proxy with cache support implemented in Rust.
use clap::Parser;
use proxy::proxy_server::ProxyServer;
use reverse_proxy::ReverseProxy;
use tonic::transport::Server;

mod cache;
mod error;
mod reverse_proxy;

pub mod proxy {
    tonic::include_proto!("proxy");
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// IP of the Proxy Server
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,
    /// Port of the Proxy Server
    #[arg(short, long, default_value_t = 50051)]
    port: u16,
    /// Time To Live for cache entries (in seconds)
    #[arg(short, long, default_value_t = 30)]
    ttl: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Addr of the socket the proxy server will bind to
    let addr = [args.ip, args.port.to_string()]
        .join(":")
        .parse()
        .expect("Invalid Address {addr}");

    // Construct and run the server
    Server::builder()
        .add_service(ProxyServer::new(ReverseProxy::new(args.ttl)))
        .serve(addr)
        .await?;
    Ok(())
}
