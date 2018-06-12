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
use actix_web::{ http, pred, App };
use actix_web::{ HttpRequest, HttpResponse, FutureResponse, AsyncResponder, FromRequest, State, Path, Query, Json };
use actix_web::middleware::{ErrorHandlers, Logger, Response};

/// https://boats.gitlab.io/failure/custom-fail.html
#[derive(Debug, Fail)]
pub enum APIError {
    #[fail(display = "{}", _0)]
    Io(#[cause] ::std::io::Error),
    #[fail(display = "{}", _0)]
    Something(String),
}

/// https://actix.rs/book/actix-web/sec-5-errors.html
impl ::actix_web::error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
       match self {
          &APIError::Io(ref o) => HttpResponse::with_body(http::StatusCode::INTERNAL_SERVER_ERROR, format!("{}", o)),
          &APIError::Something(ref o) => HttpResponse::with_body(http::StatusCode::BAD_REQUEST, format!("{}", o)),
       }
    }
}




#[allow(non_snake_case)]
fn main() {
    ::std::env::set_var("RUST_LOG", "info,actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let API_SERVER_HOST = "localhost";
    let API_SERVER_PORT = "3000";

    let sys = System::new("myserver");
    actix_web::server::new(||{
        let handle = ::actix::Arbiter::handle();
        let mut app = App::new();
        app = app
            .middleware(Logger::default())
            ;
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
        {
            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct Response { message: String }
            app = app.route("/ok", Method::GET, move |_res: HttpRequest|-> FutureResponse<HttpResponse> {
                let fut = future::ok::<HttpResponse, actix_web::Error>( HttpResponse::Ok().json(Response{ message: "hi".to_string() }) );
                fut.responder()
            })
        }
        {
            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct Response { message: String }
            app = app.route("/failure", Method::GET, move |_res: HttpRequest|-> FutureResponse<HttpResponse> {
                let err: failure::Error = APIError::Io(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")).into();
                let fut = future::err::<HttpResponse, actix_web::Error>(actix_web::error::ErrorInternalServerError(err));
                fut.responder()
            })
        }
        {
            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct Response { message: String }
            app = app.route("/api", Method::GET, move |_res: HttpRequest|-> FutureResponse<HttpResponse> {
                let fut = future::err::<HttpResponse, actix_web::Error>(APIError::Io(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")).into());
                fut.responder()
            })
        }
        app
    })
        .bind(format!("{}:{}", API_SERVER_HOST, API_SERVER_PORT))
        .expect(&format!("Can not bind to {}:{}", API_SERVER_HOST, API_SERVER_PORT))
        .start();
    let _ = sys.run();
}
