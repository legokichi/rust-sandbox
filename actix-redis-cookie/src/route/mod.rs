use mdo_future::future::*;
use futures::future;
use futures::future::*;
#[allow(unused_imports)]
use actix_web::{HttpRequest, HttpResponse, Query, State, Form, AsyncResponder, HttpMessage};
use actix_web::middleware::session::RequestSession;
use actix_web::client::ClientRequest;
use actix_web::error::ErrorInternalServerError;
use Ctx;

pub mod auth;
pub mod content;


pub fn index(req: HttpRequest<Ctx>) -> Result<HttpResponse, ::actix_web::Error> {
    if let Ok(Some(_username)) = req.session().get::<String>("username") {
        return Ok(HttpResponse::SeeOther().header("location", "/content").finish());
    }
    return Ok(HttpResponse::SeeOther().header("location", "/auth/login").finish());
}

pub fn check_auth(callback: impl Fn(HttpRequest<Ctx>, String) -> Box<dyn Future<Item=HttpResponse, Error=::actix_web::Error>>) -> impl Fn(HttpRequest<Ctx>) -> Box<dyn Future<Item=HttpResponse, Error=::actix_web::Error>> {
    move |req: HttpRequest<Ctx>|{
        if let Ok(Some(username)) = req.session().get::<String>("username") {
            return callback(req, username);
        }
        Box::new(future::result(Ok(HttpResponse::SeeOther().header("location", "/auth/logout").finish())))
    }
}