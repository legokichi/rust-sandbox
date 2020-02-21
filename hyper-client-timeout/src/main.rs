use std::net::SocketAddr;
use std::time::Duration;
use std::error::Error;
use futures::prelude::*;
use warp::Filter;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let srvfut = server();
    let clifut = client();
    let ((), ()) = futures::future::try_join(srvfut, clifut).await?;
    unreachable!()
}
async fn server() -> Result<(), Box<dyn Error>> {
    let routes = warp::path::end()
        .and_then(handler);
    warp::serve(routes)
        .run(SocketAddr::from_str("127.0.0.1:8088").unwrap())
        .await;
    // unreachable!()
    Ok(())
}

async fn handler() -> Result<String, warp::Rejection> {
    println!("1");
    tokio::time::delay_for(Duration::from_secs(1)).await;
    println!("2");
    tokio::time::delay_for(Duration::from_secs(1)).await;
    println!("3");
    unreachable!()
}

async fn client() -> Result<(), Box<dyn Error>> {
    tokio::time::delay_for(Duration::from_secs(1)).await;
    let cli = reqwest::ClientBuilder::new().build()?;
    let res: reqwest::Response = cli
        .get("http://localhost:8088/")
        .timeout(Duration::from_millis(1500))
        .send()
        .map_err(|err|{ println!("{:?}", err); err })
        .await?;
    unreachable!()
}
