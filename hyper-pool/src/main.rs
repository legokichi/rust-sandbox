
use futures::prelude::*;
use hyper::Uri;
use hyper::Response;
use hyper::Error;
use hyper::client::Client;
use hyper::Body;
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "hyper=trace");
    env_logger::init();
    let https_conn = hyper_tls::HttpsConnector::new();
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let client = Client::builder().build::<_, Body>(https_conn);
    let server = Server::bind(&([127, 0, 0, 1], 9000).into())
        .serve(
            make_service_fn(|_| async {
                Ok::<_, Error>(service_fn(|_req| async {
                    Ok::<_, Error>(Response::new(Body::from("Hello World")))
                }))
            })
        )
        .with_graceful_shutdown(async {
            println!("wait shutdown");
            rx.await.ok();
            println!("shutdownning");
        });
    
    let (a,b) = future::join(
        async move {
            use hyper::body::Buf;
            println!("before get");
            let res = client.get(Uri::from_static("http://localhost:9000/")).await.unwrap();
            println!("get body");
            dbg!(&res);
            let body = hyper::body::aggregate(res).await.unwrap();
            println!("after get");
            dbg!(&std::str::from_utf8(body.bytes()));
            tokio::time::delay_for(std::time::Duration::from_secs(100)).await;
            println!("waiting ....");
            println!("you must be shutdown");
            tx.send(()).unwrap();
        },
        server
    ).await;
    println!("the end");
}
