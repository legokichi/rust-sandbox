#[macro_use]
extern crate log;
extern crate env_logger;
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


use actix::prelude::*;
use actix_web::App;
use actix_web::http::Method;
use actix_web::server::HttpServer;
use actix_web::middleware::session::SessionStorage;
use actix_web::middleware::Logger;
use actix_web::client::ClientConnector;
use actix_redis::RedisSessionBackend;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslConnector};

mod route;

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

type Conn = ::actix::Addr<actix_web::client::ClientConnector>;

pub struct Ctx {
    conn: Conn,
    client_id: String,
    client_secret: String,
}




fn main() {
    let _ = ::env_logger::try_init();

    import_env!{
        GITHUB_CLIENT_ID;
        GITHUB_CLIENT_SECRET;
        COOKIE_KEY: "d6b68bde465f9ed9c77804f4618a8b73";
    };

    let connector = {
        let ssl_conn = SslConnector::builder(SslMethod::tls()).unwrap().build();
        ClientConnector::with_connector(ssl_conn).start()
    };
    let ssl_builder = {
        let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        ssl_builder.set_private_key_file("localhost-key.pem", SslFiletype::PEM).unwrap();
        ssl_builder.set_certificate_chain_file("localhost.pem").unwrap();
        ssl_builder
    };

    let sys = actix::System::new("actix_redis_ex");

    HttpServer::new(move || {
        let ctx = Ctx{
            conn: connector.clone(),
            client_id: GITHUB_CLIENT_ID.clone(),
            client_secret: GITHUB_CLIENT_SECRET.clone(),
        };
        App::with_state(ctx)
            .middleware(Logger::default())
            .middleware(SessionStorage::new(
                RedisSessionBackend::new("localhost:6379", &[0; 32]).cookie_secure(true)
            ))
            .route("/", Method::GET, route::index)
            .route("/auth/sign/github", Method::POST, route::auth::github::post_auth_sign_github)
            .route("/auth/cb/github", Method::GET, route::auth::github::get_auth_cb_github)
            .route("/auth/logout", Method::POST, route::auth::logout)
            .route("/auth/logout", Method::GET, route::auth::logout)
            .route("/auth/login", Method::GET, route::auth::login)
            .route("/content", Method::GET, route::check_auth(route::content::index))
    })
        .bind_ssl("localhost:8080", ssl_builder)
        .unwrap()
        .workers(2)
        .start();

    let _ = sys.run();
}
