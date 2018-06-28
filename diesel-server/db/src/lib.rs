#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate chrono;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate mdo;
extern crate transaction;
extern crate transaction_diesel;
extern crate futures;

pub mod models;
pub mod schema;

use futures::prelude::*;
use futures::future;
use transaction::prelude::*;
use transaction::mdo::*;
use diesel::prelude::*;
use chrono::{NaiveDateTime, Utc};
use std::sync::{Arc, Mutex};

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "{}", _0)]
    Connection(#[cause] diesel::ConnectionError),
    #[fail(display = "{}", _0)]
    Query(#[cause] diesel::result::Error),
}

#[derive(Clone)]
pub struct DB {
    conn: Arc<Mutex<SqliteConnection>>
}

impl DB {
    pub fn new(database_url: &str) -> Box<Future<Item=Self, Error=ErrorKind> + Send + 'static> {
        let o = SqliteConnection::establish(&database_url)
            .map_err(ErrorKind::Connection)
            .map(Mutex::new)
            .map(Arc::new)
            .map(|conn| Self { conn });
        Box::new(future::result(o))
    }
    pub fn run<'a, T: 'static + Send>(&self, tx: impl Transaction<Ctx=SqliteConnection, Item=T, Err=ErrorKind>) -> Box<Future<Item=T, Error=ErrorKind> + Send + 'static> {
        let o = {
            let mut conn = self.conn.lock().unwrap();
            tx.run(&mut conn)
        };
        Box::new(future::result(o))
    }
    pub fn create<'a>(&self, author: &'a str, body: &'a str) -> impl Transaction<Ctx=SqliteConnection, Item=usize, Err=ErrorKind> + 'a{
        with_ctx(move |conn|{
            use schema::posts;
            let now = Utc::now();
            let new_post = models::NewPost {
                timestamp: NaiveDateTime::from_timestamp(now.timestamp(), 0),
                author: author,
                body: body,
            };
            diesel::insert_into(posts::table)
                .values(&new_post)
                .execute(conn)
                .map_err(ErrorKind::Query)
        })
    }
    pub fn list<'a>(&self, offset: u64, limit: u64) -> impl Transaction<Ctx=SqliteConnection, Item=Vec<models::Post>, Err=ErrorKind> + 'a {
        with_ctx(move |conn|{
            use schema::posts::dsl;
            dsl::posts
                .order(dsl::timestamp.desc())
                .limit(limit as i64)
                .offset(offset as i64)
                .get_results::<models::Post>(conn)
                .map_err(ErrorKind::Query)
        })
    }
    pub fn count<'a>(&self) -> impl Transaction<Ctx=SqliteConnection, Item=u64, Err=ErrorKind> + 'a {
        with_ctx(move |conn|{
            use schema::posts::dsl;
            dsl::posts
                .count()
                .get_result::<i64>(conn)
                .map(|o| o as u64)
                .map_err(ErrorKind::Query)
        })
    }
}

