async fn handle(_req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
    Ok(hyper::Response::new(hyper::Body::from("Hello World")))
}
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "hyper=trace");
    env_logger::init();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));
    let make_service = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, std::convert::Infallible>(hyper::service::service_fn(handle))
    });
    let server = hyper::Server::bind(&addr).serve(make_service);
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let server = server.with_graceful_shutdown(async {rx.await.ok();});
    let fut = async {
        let client = reqwest::Client::builder().http2_prior_knowledge().build().unwrap();
        let a = (0_u32..10).into_iter().map(|_|async{
            let res = client.get("http://127.0.0.1:8000/").send().await.unwrap();
            dbg!(&res);
            dbg!(&res.text().await);
        }).collect::<Vec<_>>();
        futures::future::join_all(a).await;
        tx.send(()).unwrap();
    };
    // tx.send(()).unwrap();
    if let (_, Err(e)) = futures::future::join(fut, server).await {
        eprintln!("server error: {}", e);
    }
}
