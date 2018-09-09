use mdo_future::future::*;
use futures::future;
use futures::prelude::*;
use actix_web::dev::*;
use actix::prelude::*;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, AsyncResponder, Responder};
use actix_web::error::ErrorInternalServerError;
use actix_web::middleware::session::RequestSession;
use actix_redis::{Command};
use redis_async::resp::RespValue;

use Ctx;
use logic;

pub mod content;
pub mod api;
pub mod ws;

/// GET /
pub fn index(req: HttpRequest<Ctx>) -> Result<HttpResponse, ::actix_web::Error> {
    if let Ok(Some(_username)) = req.session().get::<String>("username") {
        return Ok(HttpResponse::SeeOther().header("location", "/content").finish());
    }
    req.session().clear();
    let body = format!(r##"<!DOCTYPE html>
<html>
<head>
<title>login</title>
</head>
<body>
<form action="/auth/login/github" method="post">
<input type="submit" value="login with github" />
</form>
<form action="/auth/login/twitter method="post">
<input type="submit" value="login with twitter" />
</form>
</body>
</html>"##);
    Ok(HttpResponse::Ok().body(body))
}

pub fn logout(req: HttpRequest<Ctx>) -> Result<HttpResponse, ::actix_web::Error> {
    req.session().clear();
    Ok(HttpResponse::SeeOther().header("location", "/").finish())
}