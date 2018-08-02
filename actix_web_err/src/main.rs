extern crate actix_web;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use actix_web::error::{ ErrorInternalServerError };
use actix_web::http::Method;
use actix_web::{ AsyncResponder, FutureResponse, HttpRequest, HttpResponse };
use futures::prelude::*;
use futures::future;
#[derive(Serialize, Debug)]
struct Error {
    error: String,
}

fn error_to_res(result: Result<HttpResponse, actix_web::Error>) -> Box<dyn Future<Item=HttpResponse, Error=actix_web::Error>> {
    Box::new(future::ok(match result {
        Ok(res) => res,
        Err(err) => {
            HttpResponse::InternalServerError().json(Error{ error: err.to_string() })
        }
    }))
}
fn main() {
    println!("ho?");
    actix_web::server::new(move ||{
        actix_web::App::new()
            .route("/", Method::GET, |_req: HttpRequest| -> FutureResponse<HttpResponse> {
                Box::new(future::err(ErrorInternalServerError("korosu")).then(error_to_res).responder())
            })
    })
        .bind("127.0.0.1:4000")
        .unwrap()
        .run();
    println!("ha?");
}
