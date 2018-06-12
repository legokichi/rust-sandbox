#[macro_use]
extern crate log;
extern crate env_logger;
extern crate actix;
extern crate actix_web;
#[macro_use]
pub extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;

use futures::prelude::*;
use futures::future;
use mdo_future::future::{bind};
use actix::prelude::*;
use actix_web::dev::*;
use actix_web::http::{header, Method};
use actix_web::{ fs, http, pred, App };
use actix_web::{ HttpRequest, HttpResponse, FutureResponse, AsyncResponder, FromRequest, State, Path, Query, Json };
use actix_web::middleware::{Logger, Response};
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

struct Counter {
    count: i32
}

#[allow(non_snake_case)]
fn main() {
    ::std::env::set_var("RUST_LOG", "info,actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let API_SERVER_HOST = "localhost";
    let API_SERVER_PORT = "8000";

    let sys = System::new("pi server");
    actix_web::server::new(||{
        type Ctx = Arc<Mutex<Counter>>;
        let mut app: App<Ctx> = App::with_state(Arc::new(Mutex::new(Counter{count: 0})));
        {
            // middlewares
            app = app
                .middleware(Logger::default())
                ;
        }
        {
            // static file server
            app = app.handler("/", fs::StaticFiles::new("./public/"));
            app = app.route("/", Method::GET, move |req: HttpRequest<Ctx>|{
                HttpResponse::Found()
                    .header("location", "/index.html")
                    .finish()
            });
        }
        {
            // tracker
            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct Request { message: String }
            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct Response { message: String }
            app = app.route("/", Method::POST, move |(req, ctx, o): (HttpRequest<Ctx>, State<Ctx>, Json<serde_json::value::Value>)|-> FutureResponse<HttpResponse> {
                let count = {
                    // If another user of this mutex panicked while holding the mutex, then this call will return an error once the mutex is acquired.
                    let mut guard = ctx.lock().unwrap();
                    (*guard).count = (*guard).count + 1;
                    (*guard).count
                };
                let info = ConnectionInfo::new(&req);
                let o = o.into_inner();
                let ip = info.remote().unwrap_or("???");
                let fingerprint = ::serde_json::to_string_pretty(&o).unwrap_or(format!("{:?}", &o));
                info!("fingerprint: {}, {}, {}", count, ip, fingerprint);
                let fut = future::ok::<HttpResponse, actix_web::Error>( HttpResponse::Ok().json(Response{ message: "hi".to_string() }) );
                fut.responder()
            })
        }
        {
            // error handlings
            app = app.default_resource(|r| {
                // GET 404
                r.method(Method::GET).f(|_req| {
                    HttpResponse::NotFound()
                        .header(http::header::CONTENT_TYPE, "application/json")
                        .body("{\"message\":\"404 not found\"}")
                });
                // any others
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_req| HttpResponse::MethodNotAllowed());
            });
        }
        app
    })
        .bind(format!("{}:{}", API_SERVER_HOST, API_SERVER_PORT))
        .expect(&format!("Can not bind to {}:{}", API_SERVER_HOST, API_SERVER_PORT))
        .start();
    let _ = sys.run();
}
