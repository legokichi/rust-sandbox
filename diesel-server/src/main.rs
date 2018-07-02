extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate askama;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate chrono;
#[macro_use]
extern crate mdo;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate mdo_future;
extern crate serde_urlencoded;
extern crate service;
extern crate tokio;

use askama::Template;
use chrono::{DateTime, Utc};
use failure::{Fail, SyncFailure};
use futures::future;
use futures::prelude::*;
use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::header::{HeaderValue, LOCATION};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use mdo_future::future::*;

pub mod error;
pub use error::Error;
pub use error::ErrorKind;

#[derive(Template)]
#[template(path = "index.html")]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct IndexTemplate {
    entries: Vec<Entry>,
    limit: u64,
    offset: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    timestamp: DateTime<Utc>,
    username: String,
    message: String,
    soudane: i32,
    id: i32,
}

/// ex.
/// ```json
/// {"message":"service error\n\tcaused by: db error\n\tcaused by: diesel query result error\n\tcaused by: attempt to write a readonly database"}
/// ```
fn error_handler(
    ret: Result<Response<Body>, Error>,
) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send + 'static> {
    match ret {
        Ok(res) => Box::new(future::ok(res)),
        Err(err) => {
            let mut fail: &Fail = &err;
            let mut message = err.to_string();

            while let Some(cause) = fail.cause() {
                message.push_str(&format!("\n\tcaused by: {}", cause.to_string()));
                fail = cause;
            }
            let status_code = match *err.kind() {
                ErrorKind::UrlParse | ErrorKind::Hyper => StatusCode::BAD_REQUEST,
                ErrorKind::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            let body = json!({
                "message": message,
            }).to_string();

            let res: Response<Body> = Response::builder()
                .status(status_code)
                .header(CONTENT_TYPE, "application/json")
                .header(CONTENT_LENGTH, body.len().to_string().as_str())
                .body(body.into())
                .expect("response builder failure");

            Box::new(future::ok(res.map(Into::into)))
        }
    }
}

fn handler(
    ctx: service::Posts,
    req: Request<Body>,
) -> Box<Future<Item = Response<Body>, Error = Error> + Send + 'static> {
    let mut res = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            #[derive(Deserialize)]
            struct Query {
                offset: u64,
                limit: u64,
            }
            let fut = mdo!{
                let query = req.uri().query().unwrap_or("offset=0&limit=40");
                Query{ offset, limit } =<< future::result(serde_urlencoded::from_str(query)).map_err(Into::into);
                (_len, lst) =<< ctx.list(offset, limit).map_err(Into::into);
                let entries = lst.iter().map(|o| Entry{
                    timestamp: DateTime::from_utc(o.timestamp, Utc),
                    username: o.author.to_string(),
                    message: o.body.to_string(),
                    soudane: o.soudane,
                    id: o.id,
                }).collect();
                tmp =<< future::result(IndexTemplate { entries, offset: offset + limit, limit }.render()).map_err(SyncFailure::new).map_err(Into::into);
                let _ = *res.body_mut() = Body::from(tmp);
                ret future::ok(res)
            };
            Box::new(fut)
        }
        (&Method::POST, "/") => {
            #[derive(Deserialize)]
            struct FormData {
                username: String,
                message: String,
            }
            let fut = mdo!{
                let body = req.into_body();
                buf =<< body.concat2().map_err(Into::into);
                FormData{ username, message } =<< future::result(serde_urlencoded::from_bytes(&buf)).map_err(Into::into);
                _ =<< ctx.create(&username, &message).map_err(Into::into);
                let _ = res.headers_mut().insert(LOCATION, HeaderValue::from_static("/"));
                let _ = *res.status_mut() = StatusCode::SEE_OTHER;
                ret future::ok(res)
            };
            Box::new(fut)
        }
        (&Method::POST, "/soudane") => {
            #[derive(Deserialize)]
            struct FormData {
                id: i32,
            }
            let fut = mdo!{
                let body = req.into_body();
                buf =<< body.concat2().map_err(Into::into);
                FormData{ id } =<< future::result(serde_urlencoded::from_bytes(&buf)).map_err(Into::into);
                _ =<< ctx.soudane(id).map_err(Into::into);
                let _ = res.headers_mut().insert(LOCATION, HeaderValue::from_static("/"));
                let _ = *res.status_mut() = StatusCode::SEE_OTHER;
                ret future::ok(res)
            };
            Box::new(fut)
        }
        _ => {
            *res.status_mut() = StatusCode::NOT_FOUND;
            Box::new(future::ok(res))
        }
    }
}

fn main() {
    let _ = env_logger::try_init();
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    println!("database_url: {}", database_url);

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(move || {
            let fut = mdo!{
                srv =<< service::Posts::new(&database_url).map_err(|_| unimplemented!());
                ret service_fn(move |req| handler(srv.clone(), req).then(error_handler) )
            };
            Box::new(fut)
        })
        .map_err(|err| error!("server error: {}", err));
    let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();
    rt.spawn(server);
    rt.run().unwrap();
}
