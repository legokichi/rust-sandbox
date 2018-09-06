#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate actix;
extern crate actix_web;
extern crate actix_redis;
extern crate openssl;

use futures::prelude::*;
use actix::prelude::*;
use actix_web::http::Method;
use actix_web::middleware::session::{SessionStorage, RequestSession};
use actix_web::{HttpRequest, HttpResponse, Form};
use actix_redis::RedisSessionBackend;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[derive(Deserialize)]
struct Login {
    username: String,
}

fn main() {
    let _ = env_logger::try_init();


    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_builder.set_private_key_file("localhost-key.pem", SslFiletype::PEM).unwrap();
    ssl_builder.set_certificate_chain_file("localhost.pem").unwrap();

    let sys = actix::System::new("actix_redis_ex");

    ::actix_web::server::HttpServer::new(|| {
        ::actix_web::App::new()
            .middleware(::actix_web::middleware::Logger::default())
            .middleware(SessionStorage::new(
                RedisSessionBackend::new("localhost:6379", &[0; 32]).cookie_secure(true)
            ))
            .route("/", Method::GET, |req: HttpRequest| -> ::actix_web::Result<HttpResponse> {
                if let Some(username) = req.session().get::<String>("username")? {
                    return Ok(HttpResponse::Ok().body(format!(r##"<!DOCTYPE html>
<head>
<title>logined</title>
</head>
hello {}!
<form action="/logout" method="post">
<input type="submit" value="logout" />
</form>
</html>"##, username)));
                }
                Ok(HttpResponse::Ok().body(format!(r##"<!DOCTYPE html>
<html>
<head>
<title>login</title>
</head>
<form action="/login" method="post">
<input type="text" name="username" />
<input type="submit" value="login" />
</form>
</html>"##)))
            })
            .route("/login", Method::POST, |(req, form): (HttpRequest, Form<Login>)| -> ::actix_web::Result<HttpResponse>  {
                debug!("username {:?}", form.username);
                req.session().set("username", form.username.clone())?;
                Ok(HttpResponse::SeeOther().header("Location", "/").finish())
            })
            .route("/logout", Method::POST, |req: HttpRequest| -> ::actix_web::Result<HttpResponse>  {
                req.session().clear();
                Ok(HttpResponse::SeeOther().header("Location", "/").finish())
            })
    })
        .bind_ssl("localhost:8080", ssl_builder)
        .unwrap()
        .workers(2)
        .start();

    let _ = sys.run();
}
