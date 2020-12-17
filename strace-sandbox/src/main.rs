use rweb::*;
use futures::prelude::*;

#[get("/")]
fn index() -> String {
    String::from("root")
}

#[get("/hello")]
async fn hello() -> Result<impl rweb::Reply, rweb::Rejection> {
    Ok(warp::reply::with_status(String::from("hello"), warp::http::StatusCode::CREATED))
}

#[tokio::main]
async fn main() {
    let (spec, filter) = openapi::spec().build(|| {
        index().or(hello())
    });

    let (tx, rx) = futures::channel::oneshot::channel();
    let (_, _) = futures::future::join(
        async move {
            let svc = warp::service(filter);
            let make_svc = hyper::service::make_service_fn(|_|
                futures::future::ok::<_,  std::convert::Infallible>(svc.clone())
            );
            hyper::server::Server::bind(&([127, 0, 0, 1], 3030).into())
                .http1_keepalive(true)
                .tcp_keepalive(Some(std::time::Duration::from_secs(10)))
                .serve(make_svc)
                .with_graceful_shutdown(rx.map(|_| ()))
                .await
                .unwrap();
        },
        async move {
            tokio::time::delay_for(std::time::Duration::from_secs(2)).await;
            let res = reqwest::Client::new().get("http://localhost:3030/").send().await.unwrap();
            println!("{:?}", res);
            let body = res.text().await.unwrap();
            println!("{}", body);
            tx.send(()).unwrap();
        }
    ).await;
    
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());
}
