use futures::prelude::*;
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    env_logger::init();
    let client: hyper::Client<hyperlocal::UnixConnector> =
        hyper::Client::builder().build(hyperlocal::UnixConnector);
    let _resp = client
        .get(hyperlocal::Uri::new("/var/run/docker.sock", "//events").into()) // this uri can be "//"
        .await;
    let resp = client
        .get(hyperlocal::Uri::new("/var/run/docker.sock", "/events").into())
        .await
        .unwrap();
    tokio::spawn(resp.into_body().into_future());
    let _resp = client
        .get(hyperlocal::Uri::new("/var/run/docker.sock", "//events").into()) // this uri can be "//", too
        .await;
    println!("ok: {}", args[1]);
}
