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
extern crate tokio;

use askama::Template;
use chrono::{DateTime, Utc};
use futures::future;
use futures::prelude::*;
use mdo_future::future::*;
use actix_web::{server, App, http, HttpRequest, HttpResponse, State, Form, Query};

pub mod error;
pub use error::Error;
pub use error::ErrorKind;

#[derive(Template)]
#[template(path = "index.html")]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct IndexTemplate {
}

fn main() {
    let _ = env_logger::try_init();
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host_url = std::env::var("HOST_URL").expect("HOST_URL is not set");
    println!("database_url: {}", database_url);
    println!("host_url: {}", host_url);
    server::new(move || {
        type Ret1 = Box<dyn Future<Item = HttpResponse, Error = error::Error> + 'static>;
        type Ret2 = Box<dyn Future<Item = HttpResponse, Error = actix_web::error::Error> + 'static>;
        App::new()
            .route("/", http::Method::GET, {
                |req: HttpRequest| -> Ret2 {
                    let fut = mdo!{
                        body =<< future::result(IndexTemplate {}.render()).map_err(Into::into);
                        ret future::ok::<_, error::Error>(HttpResponse::Ok().body(body))
                    };
                    Box::new(fut.map_err(actix_web::error::ErrorInternalServerError))
                }
            })
    }).bind(&host_url)
        .unwrap()
        .run();
}
