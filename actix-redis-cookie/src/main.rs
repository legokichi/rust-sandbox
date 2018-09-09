#![allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate actix;
extern crate actix_web;
extern crate actix_redis;
extern crate openssl;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate url;
#[macro_use]
extern crate redis_async;
extern crate uuid;

use actix::prelude::*;
use actix_web::dev::*;
use actix_web::App;
use actix_web::http::{Method, header};
use actix_web::server::HttpServer;
use actix_web::middleware::Logger;
use actix_web::middleware::session::SessionStorage;
use actix_web::middleware::csrf::CsrfFilter;
use actix_web::middleware::cors::Cors;
use actix_web::client::ClientConnector;
use actix_redis::{RedisSessionBackend, RedisActor};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslConnector};
use std::sync::Arc;

mod middleware;
mod route;
mod logic;

#[macro_export]
macro_rules! import_env {
    ($env:ident : $default:expr ; $( $t:tt )*) => (
        #[allow(unused_variables, non_snake_case)]
        let $env = ::std::env::var(stringify!($env)).unwrap_or($default.into());
        debug!("using {}={}", stringify!($env), $env);
        import_env! { $( $t )* }
    );
    ($env:ident ; $( $t:tt )*) => (
        #[allow(unused_variables, non_snake_case)]
        let $env = ::std::env::var(stringify!($env))
            .expect(&format!("env {} is not set", stringify!($env)));
        debug!("using {}={}", stringify!($env), $env);
        import_env! { $( $t )* }
    );
    ()=>()
}

pub struct Ctx {
    redis_addr: Arc<Addr<RedisActor>>,
}

fn main() {
    let _ = ::env_logger::try_init();

    import_env!{
        GITHUB_CLIENT_ID;
        GITHUB_CLIENT_SECRET;
        // TWITTER_CLIENT_ID;
        // TWITTER_CLIENT_SECRET;
        ORIGIN: "https://localhost:8080";
        REDIS_HOST: "localhost:6379";
        COOKIE_KEY: "d6b68bde465f9ed9c77804f4618a8b73";
        PRIVATE_KEY_FILE: "localhost-key.pem";
        CERTIFICATE_CHAIN_FILE: "localhost.pem";
    };

    let sys = actix::System::new("actix_redis_ex");

    let redis_addr = Arc::new(RedisActor::start(REDIS_HOST.as_ref()));
    let connector = {
        let ssl_conn = SslConnector::builder(SslMethod::tls()).unwrap().build();
        ClientConnector::with_connector(ssl_conn).start()
    };
    let ssl_builder = {
        let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        ssl_builder.set_private_key_file(&PRIVATE_KEY_FILE, SslFiletype::PEM).unwrap();
        ssl_builder.set_certificate_chain_file(&CERTIFICATE_CHAIN_FILE).unwrap();
        ssl_builder
    };

    HttpServer::new(move || {
        let ctx = Ctx{
            redis_addr: redis_addr.clone(),
        };
        App::with_state(ctx)
            .middleware(CsrfFilter::new()
                .allowed_origin(ORIGIN.as_str())
            )
            .middleware(Logger::default())
            .middleware(SessionStorage::new(
                RedisSessionBackend::new(REDIS_HOST.as_str(), COOKIE_KEY.as_bytes())
                    .cookie_secure(true)
            ))

            .scope("/api", |app|{
                app
                    .middleware(
                        Cors::build()
                            .allowed_origin("*")
                            .allowed_methods(vec![Method::GET])
                            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                            .finish()
                    )
                    .resource("/counter", |r|{
                        // impl Future は with_async する必要あり
                        r.method(Method::GET).with_async(route::api::get_counter);
                        r.method(Method::POST).with_async(route::api::add_counter);
                    })
            })
            .route("/", Method::GET, route::index)
            .route("/logout", Method::GET, route::logout)
            .scope("/", |app|{
                app
                    .middleware(middleware::oauth2::OAuth2Middleware::new(
                        connector.clone(),
                        middleware::oauth2::OAuth2Config{
                            client_id: GITHUB_CLIENT_ID.clone(),
                            client_secret: GITHUB_CLIENT_SECRET.clone(),
                            origin: ::url::Url::parse(&ORIGIN).unwrap(),
                            scope: "user".into(),
                            authorize_endpoint: "https://github.com/login/oauth/authorize".into(),
                            access_token_endpoint: "https://github.com/login/oauth/access_token".into(),
                            login_path: "/auth/login/github".into(),
                            callback_path: "/auth/cb/github".into(),
                        }
                    ))
                    // MUST after OAuth2Middleware
                    .middleware(middleware::check_login::CheckLoginMiddleware::default())
                    // .middleware(middleware::oauth2::OAuth2Middleware::new(
                    //     connector.clone(),
                    //     middleware::oauth2::OAuth2Config{
                    //         client_id: TWITTER_CLIENT_ID.clone(),
                    //         client_secret: TWITTER_CLIENT_SECRET.clone(),
                    //         origin: ::url::Url::parse(&ORIGIN).unwrap(),
                    //         scope: "user".into(),
                    //         authorize_endpoint: "https://api.twitter.com/oauth/authorize".into(),
                    //         access_token_endpoint: "https://api.twitter.com/oauth/access_token".into(),
                    //         login_path: "/auth/login/twitter".into(),
                    //         callback_path: "/auth/cb/twitter".into(),
                    //     }
                    // ))
                    .route("/content", Method::GET, route::content::index)
                    .resource("/ws", |r| r.method(Method::GET).with_async(route::ws::index))
            })
    })
        .bind_ssl("localhost:8080", ssl_builder)
        .unwrap()
        .workers(2)
        .start();

    let _ = sys.run();
}
