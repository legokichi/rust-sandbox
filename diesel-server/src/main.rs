#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
#[macro_use]
extern crate askama;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate futures;
extern crate tokio;
extern crate hyper;
extern crate serde_urlencoded;
#[macro_use]
extern crate failure;
extern crate db;

use mdo_future::future::*;
use futures::prelude::*;
use futures::future;
use futures::Stream;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::service_fn;
use hyper::header::{HeaderValue, LOCATION};
use chrono::{DateTime, Utc};
use askama::Template;
use std::sync::{Arc, Mutex};


#[derive(Template)]
#[template(path = "index.html")]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct IndexTemplate {
    entries: Vec<Entry>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    timestamp: DateTime<Utc>,
    username: String,
    message: String,
}

#[derive(Fail, Debug)]
enum Error {
    #[fail(display = "{}", _0)]
    DB(#[cause] ::db::Error),
    #[fail(display = "{}", _0)]
    UrlEncoded(#[cause] serde_urlencoded::de::Error),
    #[fail(display = "{}", _0)]
    Hyper(#[cause] ::hyper::Error),
    #[fail(display = "{}", _0)]
    Io(#[cause] ::std::io::Error),
    #[fail(display = "{}", _0)]
    Other(String),
}

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send + 'static>;
type BoxFut2 = Box<Future<Item = Response<Body>, Error = Error> + Send>;

fn main() {
    let _ = env_logger::try_init();
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(move ||{
            let conn = Arc::new(Mutex::new(db::establish_connection(&database_url).unwrap()));
            service_fn(move |req: Request<Body>| -> BoxFut {
                let conn = conn.clone();
                let mut res = Response::new(Body::empty());
                let fut: BoxFut2 = match (req.method(), req.uri().path()) {
                    // Serve some instructions at /
                    (&Method::GET, "/") => {
                        #[derive(Deserialize)]
                        struct Query {
                            offset: u64,
                            limit: u64,
                        }
                        let fut = mdo!{
                            let query = req.uri().query().unwrap_or("offset=0&limit=100");
                            Query{offset, limit} =<< future::result(serde_urlencoded::from_str(query)).map_err(Error::UrlEncoded);
                            (_len, lst) =<< future::result({
                                let conn = conn.lock().unwrap();
                                db::list_post(&conn, offset as i64, limit as i64)
                            }).map_err(Error::DB);
                            let entries = lst.iter().map(|db::models::Post{ id: _, timestamp, author, body }| Entry{
                                timestamp: DateTime::from_utc(*timestamp, Utc), username: author.to_string(), message: body.to_string()
                            }).collect();
                            tmp =<< future::result(IndexTemplate { entries }.render()).map_err(|err| Error::Other(err.description().to_string()) );
                            let _ = *res.body_mut() = Body::from(tmp);
                            ret future::ok(res)
                        };
                        Box::new(fut) as BoxFut2
                    },
                    (&Method::POST, "/") => {
                        #[derive(Deserialize)]
                        struct FormData {
                            username: String,
                            message: String,
                        }
                        let fut = mdo!{
                            let body = req.into_body();
                            buf =<< body.concat2().map_err(Error::Hyper);
                            FormData{username, message} =<< future::result(serde_urlencoded::from_bytes(&buf)).map_err(Error::UrlEncoded);
                            _ =<< future::result({
                                let conn = conn.lock().unwrap();
                                db::create_post(&conn, &username, &message)
                            }).map_err(Error::DB);
                            let _ = res.headers_mut().insert(LOCATION, HeaderValue::from_static("/"));
                            let _ = *res.status_mut() = StatusCode::SEE_OTHER;
                            ret future::ok(res)
                        };
                        Box::new(fut) as BoxFut2
                    },
                    _ => {
                        *res.status_mut() = StatusCode::NOT_FOUND;
                        Box::new(future::ok(res)) as BoxFut2
                    }
                };
                Box::new(fut.then(|o|{
                    match o {
                        Ok(res) => future::ok(res),
                        Err(err) =>{
                            let mut res = Response::new(format!("{}", err));
                            match err{
                                Error::UrlEncoded(_) |
                                Error::Hyper(_) => *res.status_mut() = StatusCode::BAD_REQUEST,
                                _ => *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
                            }
                            
                            future::ok(res.map(Into::into))
                        }
                    }
                })) as BoxFut
            })
        })
        .map_err(|err| eprintln!("server error: {}", err) );
    let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();
    rt.spawn(server);
    rt.run().unwrap();
}
