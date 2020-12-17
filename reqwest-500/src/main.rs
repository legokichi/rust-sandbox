async fn handle(_req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
    let mut builder = hyper::Response::builder()
        .status(hyper::StatusCode::INTERNAL_SERVER_ERROR);

    let resp = builder.body(
        hyper::Body::from("Hello World")
    ).unwrap();

    Ok(resp)
}

#[tokio::main]
async fn main() {
    let make_service = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, std::convert::Infallible>(hyper::service::service_fn(handle))
    });

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = hyper::Server::bind(&addr).serve(make_service);

    let fut2 = async move{
        tokio
        let res = reqwest::get("http://localhost:3000").await;
        println!("{:?}", res);
    };
    let fut1 = async move {
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    };
    let (_, _) = futures::future::join(
        fut1, fut2
    ).await;
    
}
