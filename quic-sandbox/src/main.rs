// use tokio::prelude::*;
use tracing::instrument;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let span = span!(Level::TRACE, "main");
    
    // let mut socket = tokio::net::UdpSocket("").await?;
    // let addr = socket.local_addr()?;
    
    println!("{:?}", addr);
}
