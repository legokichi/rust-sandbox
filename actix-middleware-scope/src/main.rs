use log::*;

use mdo_future::future::*;
use futures::future;
use futures::prelude::*;
use actix_web::{http, pred, App, HttpMessage, FromRequest, HttpRequest, HttpResponse, Query};
use actix_web::http::{header, Method};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized, ErrorBadRequest};
use actix_web::middleware::{Middleware, Started, Response, Finished, ErrorHandlers, Logger};



pub struct MyMiddleware;
impl<S> Middleware<S> for MyMiddleware {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started, ::actix_web::Error> {
        println!("start: {:?}", req);
        Ok(Started::Done)
    }
    fn response(&self, req: &HttpRequest<S>, resp: HttpResponse) -> Result<Response, ::actix_web::Error> {
        println!("response: {:?}, {:?}", req, resp);
        Ok(Response::Done(resp))
    }
    fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
        println!("finish: {:?}, {:?}", req, resp);
        Finished::Done
    }
}


fn main() -> Result<(), ::failure::Error> {
    ::env_logger::init();
    actix_web::server::new(||{
        App::new()
            .middleware(Logger::default())
            .scope("/foo", |scope|{
                scope
                    .middleware(MyMiddleware)
                    .route("/", Method::GET, |()|{ HttpResponse::Ok().body("foo") })
            })
            .route("/", Method::GET, |()|{ HttpResponse::Ok().body("ok") })
            .default_resource(|r| {
                // GET 404
                r.method(Method::GET).f(|_req| {
                    HttpResponse::NotFound()
                        .header(header::CONTENT_TYPE, "text/plain")
                        .body("404")
                });
                // any others
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_req| HttpResponse::MethodNotAllowed());
            })
    })
        .bind(format!("0.0.0.0:3000"))
        .unwrap()
        .run();
    Ok(())
}
