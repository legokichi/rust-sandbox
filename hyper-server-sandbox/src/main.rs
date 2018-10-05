use futures::future;
use futures::prelude::*;
use hyper::service::service_fn;	
use hyper::{Body, Method, Request, Response, StatusCode};

fn handler(req: Request<Body>) -> impl Future<Item=Response<Body>, Error=::hyper::Error> + Send + 'static {
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

/// https://github.com/legokichi/rust-sandbox/commit/838934784a57a2b1c1c80101182169c3e50d76ee
fn main() {
    let _ = env_logger::try_init();
    let (tx, rx) = ::futures::sync::oneshot::channel::<()>();
    let server = ::hyper::Server::bind(&([127, 0, 0, 1], 3000).into())
        .serve(move || service_fn(handler))
        .with_graceful_shutdown(rx);
    tokio::run(
        future::ok(())
            .join(
                server
                    .map_err(|err| eprintln!("server error: {:?}", err))
            )
            .join(
                ::hyper::Client::new()
                    .get("http://localhost:3000".parse().unwrap())
                    .map_err(|err| eprintln!("client error: {:?}", err))
                    .and_then(|res| {
                        println!("status: {}", res.status());
                        res.into_body().concat2()
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
            )
            .join(
                ::tokio_timer::sleep(::std::time::Duration::from_millis(10))
                    .map_err(|err| eprintln!("timer error: {:?}", err))
                    .and_then(move |()|{
                        println!("start shutdonwn");
                        tx.send(())
                            .into_future()
                            .map_err(|err| eprintln!("tx error: {:?}", err))
                    })
            )
            .map(|_| ())
    );
    println!("shutdonwn");
}