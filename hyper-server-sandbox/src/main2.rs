extern crate tokio; // 0.1.11
extern crate hyper; // 0.12.14;
extern crate futures; // 0.1.25
extern crate tokio_timer; // 0.2.7

use tokio_timer::sleep;
use hyper::{Body, Request, Response, Server, Client, Method, StatusCode};
use hyper::service::service_fn;
use futures::prelude::*;
use futures::future;
use futures::sync::oneshot::channel;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel::<()>();
    let server = Server::bind(&([0, 0, 0, 0], 3000).into())
        .serve(move || service_fn(|req: Request<Body>|-> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send + 'static>{
            let mut res = Response::new(Body::empty()); 
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => {
                    let _ = *res.body_mut() = Body::from("hello");
                }
                _ => {
                    *res.status_mut() = StatusCode::NOT_FOUND;
                }
            }
            Box::new(future::ok(res))
        }))
        .with_graceful_shutdown(rx)
        .map_err(|err| eprintln!("server error: {:?}", err));
    let client = sleep(Duration::from_secs(1))
        .map_err(|err| eprintln!("timer error: {:?}", err))
        .and_then(|()|{
            Client::new()
                .get("http://0.0.0.0:3000".parse().unwrap())
                .map_err(|err| eprintln!("client error: {:?}", err))
                .and_then(|res| {
                    println!("status: {}", res.status());
                    res.into_body()
                        .concat2()
                        .map_err(|err| eprintln!("concat error: {:?}", err))
                })
                .and_then(|body| {
                    ::std::str::from_utf8(&body)
                        .map(|a| a.to_string())
                        .into_future()
                        .map_err(|err| eprintln!("utf8 error: {:?}", err))
                })
                .and_then(|body| {
                    println!("body: {}", body);
                    Ok(()).into_future()
                })
        })
        .and_then(move |()| {
            println!("start shutdonwn");
            tx.send(())
                .into_future()
                .map_err(|err| eprintln!("tx error: {:?}", err))
        });

    let fut = server
        .join(client)
        .map(|_| ());
    tokio::run(fut);
}
