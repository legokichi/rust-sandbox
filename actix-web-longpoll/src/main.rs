extern crate actix_web;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate failure;
extern crate tokio_timer;
use actix_web::middleware::{Logger};
use futures::prelude::*;
use actix_web::http::{StatusCode, Method};
use actix_web::{HttpRequest, HttpResponse};
use failure::Error;

fn main(){
    ::env_logger::init();

    let srv = ::actix_web::server::new(move ||{
        ::actix_web::App::new()
            .middleware(Logger::default())
            .resource("/", |r|{ r.method(Method::GET).with_async(get_index); })
    })
        .keep_alive(::actix_web::server::KeepAlive::Timeout(10))
        .bind("0.0.0.0:8081")
        .unwrap();
    srv.run();
    
}

/// GET /
pub fn get_index(_: HttpRequest) -> impl Future<Item=HttpResponse, Error=Error> {
    ::tokio_timer::sleep(::std::time::Duration::from_secs(0)).map(|()|{
        HttpResponse::Ok()
            .header("content-type", "text/plain; charset=utf-8")
            .body("🐲")
    }).map_err(Into::into)
}

// GET / HTTP/1.1