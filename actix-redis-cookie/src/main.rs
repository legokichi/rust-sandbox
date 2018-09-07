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

#[allow(unused_imports)]
use failure::Fail;
use mdo_future::future::*;
use futures::future;
use futures::prelude::*;
use actix::prelude::*;
use actix_web::{HttpRequest, HttpResponse, Query, State, Form, AsyncResponder, HttpMessage};
use actix_web::http::Method;
use actix_web::error::ErrorInternalServerError;
use actix_web::middleware::session::{SessionStorage, RequestSession};
use actix_web::client::{ClientConnector, ClientRequest};
use actix_redis::RedisSessionBackend;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslConnector};

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

fn main() {
    let _ = ::env_logger::try_init();

    import_env!{
        GITHUB_CLIENT_ID;
        GITHUB_CLIENT_SECRET;
        COOKIE_KEY: "d6b68bde465f9ed9c77804f4618a8b73";
    };

    type Conn = ::actix::Addr<actix_web::client::ClientConnector>;
    struct Ctx {
        conn: Conn,
        client_id: String,
        client_secret: String,
    }

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

    ::actix_web::server::HttpServer::new(move || {
        let ctx = Ctx{
            conn: connector.clone(),
            client_id: GITHUB_CLIENT_ID.clone(),
            client_secret: GITHUB_CLIENT_SECRET.clone(),
        };
        ::actix_web::App::with_state(ctx)
            .middleware(::actix_web::middleware::Logger::default())
            .middleware(SessionStorage::new(
                RedisSessionBackend::new("localhost:6379", &[0; 32]).cookie_secure(true)
            ))
            .route("/", Method::GET, move |req: HttpRequest<_>| -> ::actix_web::Result<HttpResponse> {
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
<body>
<form action="/auth/sign/github" method="post">
<input type="submit" value="login with github" />
</form>
</body>
</html>"##)))
            })
            .route("/auth/sign/github", Method::POST, move |req: HttpRequest<Ctx>| -> ::actix_web::Result<HttpResponse> {
                let mut url = ::url::Url::parse("https://github.com/login/oauth/authorize")?;
                url.query_pairs_mut()
                    .append_pair("client_id", &req.state().client_id)
                    .append_pair("scope", "user") // user:email
                    .append_pair("state", "1234")
                    .append_pair("redirect_uri", "https://localhost:8080/auth/cb/github");
                Ok(HttpResponse::Found().header("Location", url.as_str()).finish())
            })
            .route("/auth/cb/github", Method::GET, {
                #[derive(Serialize, Deserialize, Debug)]
                #[serde(untagged)]
                enum CallbackResp {
                    Success{ code: String, state: String },
                    Error{ error: String, error_description: String, error_uri: String, state: String }
                }
                #[derive(Serialize, Deserialize, Debug)]
                struct AccessToken { access_token: String, token_type: String }
                #[derive(Serialize, Deserialize, Debug)]
                struct GitHubUserData { id: u64, login: String }
                move |(req, ctx, query): (HttpRequest<Ctx>, State<Ctx>, Query<CallbackResp>)| -> Box<dyn Future<Item=_, Error=_>>{
                    let ( code, state ) = match query.into_inner() {
                        CallbackResp::Error{ error, error_description, error_uri, .. } => {
                            return Box::new(future::ok(HttpResponse::InternalServerError().body(format!(r##"{} {} {}"##, error, error_description, error_uri))));
                        },
                        CallbackResp::Success{ code, state } => ( code, state ),
                    };
                    let connector = ctx.conn.to_owned();
                    let connector2 = ctx.conn.to_owned();
                    let session = req.session();
                    let encoded: String = ::url::form_urlencoded::Serializer::new(String::new())
                        .append_pair("client_id", &ctx.client_id)
                        .append_pair("client_secret", &ctx.client_secret)
                        .append_pair("code", &code)
                        .append_pair("state", &state)
                        .finish();
                    let fut = mdo!{
                        req =<< future::result(ClientRequest::post("https://github.com/login/oauth/access_token")
                            .with_connector(connector)
                            .header("content-type", "application/x-www-form-urlencoded")
                            .header("accept", "application/json")
                            .body(encoded)
                        ).map_err(ErrorInternalServerError);
                        resp =<< req.send().from_err();
                        token =<< resp.json::<AccessToken>().from_err();
                        let _ = println!("{:?}", token);
                        // https://developer.github.com/v3/users/#get-the-authenticated-user
                        req =<< future::result(ClientRequest::post("https://api.github.com/user")
                            .with_connector(connector2)
                            .header("authorization", format!("{} {}", token.token_type, token.access_token))
                            .header("content-type", "application/json")
                            .header("accept", "application/json")
                            .body("{}")
                        ).from_err();
                        resp =<< req.send().from_err();
                        // data =<< resp.body().from_err();
                        // data =<< future::result(std::str::from_utf8(&data).map(|s|s.to_owned())).from_err();
                        data =<< resp.json::<GitHubUserData>().from_err();
                        let _ = debug!("{:?}", data);
                        () =<< future::result(session.set::<String>("username", data.login)).from_err();
                        ret ret(HttpResponse::SeeOther().header("Location", "/").finish())
                    };
                    fut.responder()
                }
            })
            .route("/logout", Method::POST, |req: HttpRequest<_>| -> ::actix_web::Result<HttpResponse>  {
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
