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
extern crate actix;
extern crate actix_web;
extern crate mdo_future;
extern crate service;
extern crate tokio;

use askama::Template;
use chrono::{DateTime, Utc};
use futures::future;
use futures::prelude::*;
use mdo_future::future::*;
use actix_web::{server, App, http, HttpResponse, State, Form, Query};

pub mod error;
pub use error::Error;
pub use error::ErrorKind;

#[derive(Template)]
#[template(path = "index.html")]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct IndexTemplate {
    entries: Vec<Entry>,
    prev: u64,
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

fn main() {
    let _ = env_logger::try_init();
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host_url = std::env::var("HOST_URL").expect("HOST_URL is not set");
    println!("database_url: {}", database_url);
    println!("host_url: {}", host_url);
    let srv = service::Posts::new(&database_url)
        .map_err(|_| unimplemented!())
        .wait()
        .unwrap();
    server::new(move || {
        type Ret1 = Box<dyn Future<Item = HttpResponse, Error = error::Error> + 'static>;
        type Ret2 = Box<dyn Future<Item = HttpResponse, Error = actix_web::error::Error> + 'static>;
        App::with_state(srv.clone())
            .route("/", http::Method::GET, {
                #[derive(Deserialize)]
                struct QueryData {
                    offset: Option<u64>,
                    limit: Option<u64>,
                }
                |(ctx, path): (State<service::Posts>, Query<QueryData>)| -> Ret2 {
                    let fut = mdo!{
                        let offset = path.offset.unwrap_or(0);
                        let limit = path.limit.unwrap_or(40);
                        (_len, lst) =<< ctx.list(offset, limit).map_err(Into::into);
                        let entries = lst.iter().map(|o| Entry{
                            timestamp: DateTime::from_utc(o.timestamp, Utc),
                            username: o.author.to_string(),
                            message: o.body.to_string(),
                            soudane: o.soudane,
                            id: o.id,
                        }).collect();
                        body =<< future::result(IndexTemplate { entries, prev: offset - limit, offset: offset + limit, limit }.render()).map_err(Into::into);
                        ret future::ok::<_, error::Error>(HttpResponse::Ok().body(body))
                    };
                    Box::new(fut.map_err(actix_web::error::ErrorInternalServerError))
                }
            })
            .route("/", http::Method::POST, {
                #[derive(Deserialize)]
                struct FormData {
                    username: String,
                    message: String,
                }
                |(ctx, body): (State<service::Posts>, Form<FormData>)| -> Ret2 {
                    let fut = mdo!{
                        _ =<< ctx.create(&body.username, &body.message).map_err(Into::into);
                        ret future::ok::<_, error::Error>(HttpResponse::SeeOther().header(http::header::LOCATION, "/").body(""))
                    };
                    Box::new(fut.map_err(actix_web::error::ErrorInternalServerError))
                }
            })
            .route("/soudane", http::Method::POST, {
                #[derive(Deserialize)]
                struct FormData {
                    id: i32,
                }
                |(ctx, body): (State<service::Posts>, Form<FormData>)| -> Ret2 {
                    let fut = mdo!{
                        _ =<< ctx.soudane(body.id).map_err(Into::into);
                        ret future::ok::<_, error::Error>(HttpResponse::SeeOther().header(http::header::LOCATION, "/").body(""))
                    };
                    Box::new(fut.map_err(actix_web::error::ErrorInternalServerError))
                }
            })
    }).bind(&host_url)
        .unwrap()
        .run();
}
