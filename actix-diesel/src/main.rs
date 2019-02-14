#![allow(proc_macro_derive_resolution_fallback)] 
#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate env_logger;
extern crate dotenv;
extern crate futures;
extern crate actix;
extern crate transaction;
extern crate failure;


use std::ops::Deref;
use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, HttpResponse,
    HttpRequest
};
use actix_web::error::ErrorInternalServerError;
use futures::prelude::*;
use transaction::prelude::*;
use transaction::Transaction;
use transaction::diesel::DieselContext;
use diesel::pg::PgConnection;
use db::{DbExecutor, TxMessage};
use failure::Error;

mod db;
mod models;
mod schema;


#[derive(Clone)]
struct DB(pub Addr<DbExecutor>);
#[derive(Clone)]
struct AppState {
    db: DB,
}

#[derive(Clone)]
struct ToDao<T: HaveDao>(HttpRequest<T>);
impl<T: HaveDao> Deref for ToDao<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.state()
    }
}
impl<T: HaveDao> From<HttpRequest<T>> for ToDao<T> {
    fn from(req: HttpRequest<T>) -> ToDao<T> {
        ToDao(req)
    }
}

pub trait DBDao: Clone + Send + Sync + 'static {
    type Ctx: 'static + Send;

    fn run<Tx>(
        &self,
        tx: Tx,
    ) -> Box<dyn Future<Item = Tx::Item, Error = Tx::Err> + Send + 'static>
    where
        Tx::Ctx: Send + 'static,
        Tx::Item: Send + 'static,
        Tx::Err: Send + 'static,
        Tx: Transaction<Ctx = Self::Ctx, Err = Error> + Send + 'static;

    fn category_count(
        &self,
    ) -> Box<dyn Transaction<Ctx = Self::Ctx, Item = u64, Err = Error> + Send + 'static>;
}

pub trait HaveDao{
    type Ctx: Send + 'static;
    type DBDao: DBDao<Ctx = Self::Ctx>;
    fn db(&self) -> Self::DBDao;
}

impl DBDao for DB {
    type Ctx = DieselContext<PgConnection>;
    fn run<Tx>(
        &self,
        tx: Tx,
    ) -> Box<dyn Future<Item = Tx::Item, Error = Tx::Err> + Send + 'static>
    where
        Tx::Ctx: Send + 'static,
        Tx::Item: Send + 'static,
        Tx::Err: Send + 'static,
        Tx: Transaction<Ctx = Self::Ctx, Err = Error> + Send + 'static,
    {
        Box::new(
            self.0.send(TxMessage(tx))
                .then(|ret|{
                    ret.unwrap()
                })
        )
    }
    fn category_count(
        &self,
    ) -> Box<dyn Transaction<Ctx = Self::Ctx, Item = u64, Err = Error> + Send + 'static>{
        Box::new(db::count())
    }
}


impl HaveDao for AppState {
    type Ctx = <Self::DBDao as DBDao>::Ctx;
    type DBDao = DB;
    fn db(&self) -> Self::DBDao
    {
        DB(self.db.0.clone())
    }
}

pub fn list_categories(
    dao: impl Deref<Target = impl HaveDao> + Clone + 'static,
) -> impl Future<Item = String, Error = Error> + 'static {
    let db = dao.db();
    let db2 = dao.db();
    db.run(with_ctx(move |ctx| {
        let db = db2.clone();
        let total = db.category_count().run(ctx)?;
        Ok(format!("hello world!{:?}", total))
    }))
}

/// Async request handler
fn index(
    req: HttpRequest<AppState>,
) -> impl Future<Item=HttpResponse, Error = actix_web::Error> {
    println!("korosuzo");
    list_categories(ToDao(req.clone()))
        .and_then(|hello| futures::future::ok(HttpResponse::Ok().body(hello)) )
        .map_err(ErrorInternalServerError)
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let sys = actix::System::new("diesel-example");

    // Start 3 db executor actors
    let db = DbExecutor::new(::std::env::var("DATABASE_URL").unwrap());
    let addr = SyncArbiter::start(4, move || db.clone());

    // Start http server
    server::new(move || {
        App::with_state(AppState{db: DB(addr.clone())})
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with_async(index) )
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
