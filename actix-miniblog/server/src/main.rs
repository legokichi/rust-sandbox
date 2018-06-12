#[macro_use]
extern crate log;
extern crate env_logger;
extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
#[macro_use]
extern crate askama;
extern crate chrono;
extern crate jfs;

use futures::prelude::*;
use futures::future;
use mdo_future::future::{bind};
use actix::prelude::*;
use actix_web::dev::*;
use actix_web::http::{header, Method};
use actix_web::{ fs, http, pred, App, Form };
use actix_web::{ HttpRequest, HttpResponse, FutureResponse, AsyncResponder, FromRequest, State, Path, Query, Json };
use actix_web::middleware::{Logger, Response};
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use chrono::prelude::*;
use askama::Template;
use jfs::Store;

#[derive(Template)]
#[template(path = "index.html")]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct IndexTemplate {
    entries: Vec<Entry>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    timestamp: DateTime<Utc>,
    username: String,
    message: String,
}

struct Storage {
    pub entries: Vec<Entry>,
    db: jfs::Store,
}

impl Storage {
    fn new()-> Self {
        let mut cfg = jfs::Config::default();
        cfg.single = true;
        cfg.pretty = true;
        cfg.indent = 2;
        let db = jfs::Store::new_with_cfg("./storage.json", cfg).unwrap();
        let entries = match db.get::<Vec<Entry>>("entries") {
            Ok(entries) => entries,
            Err(_) => vec![],
        };
        Storage { entries, db }
    }
    fn insert(&mut self, username: String, message: String) {
        let entry = Entry{
            timestamp: Utc::now(),
            username,
            message
        };
        self.entries.push(entry);
        self.dump();
    }
    fn dump(&self) {
        let _ = self.db.save_with_id(&self.entries, "entries").unwrap();
    }
}

#[allow(non_snake_case)]
fn main() {
    ::std::env::set_var("RUST_LOG", "info,actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let API_SERVER_HOST = "localhost";
    let API_SERVER_PORT = "8000";

    let sys = System::new("server");
    type Ctx = Arc<Mutex<Storage>>;
    let ctx: Ctx = Arc::new(Mutex::new(Storage::new()));
    actix_web::server::new(move ||{
        let mut app: App<Ctx> = App::with_state(ctx.clone());
        {
            // middlewares
            app = app
                .middleware(Logger::default())
                ;
        }
        {
            // index
            app = app.route("/", Method::GET, move |state: State<Ctx>|{
                let mut entries = {
                    let mut ctx = state.lock().unwrap();
                    ctx.entries.clone()
                };
                entries.reverse();
                let tmp = IndexTemplate { entries };
                HttpResponse::Ok()
                    .content_type("text/html; charset=UTF-8")
                    .body(tmp.render().unwrap())
            });

            #[derive(Deserialize)]
            struct FormData {
                username: String,
                message: String,
            }
            app = app.route("/", Method::POST, move |(state, form): (State<Ctx>, Form<FormData>)|{
                if !form.message.is_empty() {
                    let mut username = if form.username.is_empty() { "annonymas".to_string() } else { form.username.clone() };
                    let mut message =  form.message.clone();
                    username.truncate(64);
                    message.truncate(256);
                    {
                        let mut ctx = state.lock().unwrap();
                        ctx.insert(username, message);
                    }
                }
                HttpResponse::SeeOther()
                    .header("location", "/")
                    .finish()
            });
        }
        {
            // error handlings
            app = app.default_resource(|r| {
                // GET 404
                r.method(Method::GET).f(|_req| {
                    HttpResponse::NotFound()
                        .header(http::header::CONTENT_TYPE, "application/json")
                        .body("{\"message\":\"404 not found\"}")
                });
                // any others
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_req| HttpResponse::MethodNotAllowed());
            });
        }
        app
    })
        .bind(format!("{}:{}", API_SERVER_HOST, API_SERVER_PORT))
        .expect(&format!("Can not bind to {}:{}", API_SERVER_HOST, API_SERVER_PORT))
        .start();
    let _ = sys.run();

}
