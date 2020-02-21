use std::collections::HashSet;
use failure::format_err;
use log::error;
use futures::prelude::*;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpResponse, HttpServer};
use actix_web::http::StatusCode;

mod logger;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::try_init().unwrap();
    let https_connector = ::hyper_tls::HttpsConnector::new(4).expect("https connector failed");
    let error_status = vec![
        StatusCode::BAD_REQUEST,
        StatusCode::UNAUTHORIZED,
        StatusCode::FORBIDDEN,
        StatusCode::NOT_FOUND,
        StatusCode::METHOD_NOT_ALLOWED,
        StatusCode::NOT_ACCEPTABLE,
        StatusCode::INTERNAL_SERVER_ERROR,
        StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
        StatusCode::UNPROCESSABLE_ENTITY,
    ]
    .into_iter()
    .collect::<HashSet<StatusCode>>();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(logger::ErrorLogger {
                slack: None,
                sentry: None,
                error_status: error_status.clone(),
                connector: https_connector.clone(),
            })
            .service(
                web::resource("/")
                    .to(|| {
                        actix_rt::spawn(futures::future::ok(()).and_then(|()|{
                            error!("hello");
                            Ok(())
                        }));
                        HttpResponse::from_error(format_err!("dummy error").into())
                    }),
            )
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .unwrap();
}
