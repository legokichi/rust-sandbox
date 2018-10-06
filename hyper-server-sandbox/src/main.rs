extern crate env_logger;
extern crate futures;     // 0.1.24
extern crate hyper;       // 0.12.10
extern crate tokio;       // 0.1.8
extern crate tokio_timer; // 0.2.6 // 0.5.13

use futures::future;
use futures::prelude::*;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};

fn handler(
    req: Request<Body>,
) -> impl Future<Item = Response<Body>, Error = ::hyper::Error> + Send + 'static {
    let mut res = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let _ = *res.body_mut() = Body::from("hello");
            future::ok(res)
        }
        _ => {
            *res.status_mut() = StatusCode::NOT_FOUND;
            future::ok(res)
        }
    }
}

fn main() {
    let _ = env_logger::try_init();
    let (tx, rx) = ::futures::sync::oneshot::channel::<()>();
    let server = ::hyper::Server::bind(&([127, 0, 0, 1], 3000).into())
        .serve(move || service_fn(handler))
        .with_graceful_shutdown(rx);
    tokio::run(
        future::ok(())
            // server
            .join(server.map_err(|err| eprintln!("server error: {:?}", err)))
            // client
            .join(
                // https://github.com/hyperium/hyper/issues/1668
                future::lazy(move ||{
                        println!("start request");
                        ::hyper::Client::new()
                            .get("http://localhost:3000".parse().unwrap())
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
            )
            // shutdown timer
            .join(
                ::tokio_timer::sleep(::std::time::Duration::from_secs(4))
                    .map_err(|err| eprintln!("timer error: {:?}", err))
                    .and_then(move |()| {
                        println!("start shutdonwn");
                        tx.send(())
                            .into_future()
                            .map_err(|err| eprintln!("tx error: {:?}", err))
                    }),
            )
            .map(|_| ()),
    );
    println!("shutdonwn");
}