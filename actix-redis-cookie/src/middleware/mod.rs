use mdo_future::future::*;
use futures::future;
use futures::future::*;
use actix_web::{HttpMessage, FromRequest, HttpRequest, HttpResponse, Query};
use actix_web::http::Method;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized, ErrorBadRequest};
use actix_web::middleware::{Middleware, Started};
use actix_web::middleware::session::RequestSession;

pub mod oauth2;
pub mod check_login;