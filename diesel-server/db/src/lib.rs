#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate chrono;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate transaction;
extern crate transaction_diesel;
#[macro_use]
extern crate log;

pub mod error;
pub mod models;
pub mod schema;
pub use error::Error;
pub use error::ErrorKind;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::debug_query;
use diesel::prelude::*;
use futures::future;
use futures::prelude::*;
use transaction::prelude::*;
use std::sync::{Arc, Mutex};

type DBBackend = <SqliteConnection as Connection>::Backend;

#[derive(Clone)]
pub struct DB {
    conn: Arc<Mutex<SqliteConnection>>,
}

impl DB {
    pub fn new(database_url: &str) -> Box<Future<Item = Self, Error = Error> + Send + 'static> {
        let o = SqliteConnection::establish(&database_url)
            .map_err(Into::into)
            .map(Mutex::new)
            .map(Arc::new)
            .map(|conn| Self { conn });
        Box::new(future::result(o))
    }
    pub fn run<T: 'static + Send>(
        &self,
        tx: impl Transaction<Ctx = SqliteConnection, Item = T, Err = Error>,
    ) -> Box<Future<Item = T, Error = Error> + Send + 'static> {
        let o = {
            let mut conn = self.conn.lock().unwrap();
            tx.run(&mut conn)
        };
        Box::new(future::result(o))
    }
    pub fn create<'a>(
        &self,
        author: &'a str,
        body: &'a str,
    ) -> impl Transaction<Ctx = SqliteConnection, Item = usize, Err = Error> + 'a {
        with_ctx(move |conn| {
            use schema::posts;
            let now = Utc::now();
            let new_post = models::NewPost {
                timestamp: NaiveDateTime::from_timestamp(now.timestamp(), 0),
                author,
                body,
                soudane: 0,
            };
            let query = diesel::insert_into(posts::table).values(&new_post);
            debug!("{}", debug_query::<DBBackend, _>(&query));
            query.execute(conn).map_err(Into::into)
        })
    }
    pub fn create_with_time<'a>(
        &self,
        author: &'a str,
        body: &'a str,
        timestamp: DateTime<Utc>,
    ) -> impl Transaction<Ctx = SqliteConnection, Item = usize, Err = Error> + 'a {
        with_ctx(move |conn| {
            use schema::posts;
            let now = timestamp;
            let new_post = models::NewPost {
                timestamp: NaiveDateTime::from_timestamp(now.timestamp(), 0),
                author,
                body,
                soudane: 0,
            };
            let query = diesel::insert_into(posts::table).values(&new_post);
            debug!("{}", debug_query::<DBBackend, _>(&query));
            query.execute(conn).map_err(Into::into)
        })
    }
    pub fn list<'a>(
        &self,
        offset: u64,
        limit: u64,
    ) -> impl Transaction<Ctx = SqliteConnection, Item = Vec<models::Post>, Err = Error> + 'a {
        with_ctx(move |conn| {
            use schema::posts::dsl;
            let query = dsl::posts
                .order(dsl::timestamp.desc())
                .limit(limit as i64)
                .offset(offset as i64);
            debug!("{}", debug_query::<DBBackend, _>(&query));
            query.get_results::<models::Post>(conn).map_err(Into::into)
        })
    }
    pub fn soudane<'a>(
        &self,
        id: i32,
    ) -> impl Transaction<Ctx = SqliteConnection, Item = Option<()>, Err = Error> + 'a {
        with_ctx(move |conn| {
            use schema::posts::dsl;
            let query = dsl::posts.find(id);
            debug!("{}", debug_query::<DBBackend, _>(&query));
            let post_opt = query.get_result::<models::Post>(conn).optional()?;
            if let Some(post) = post_opt {
                let tmp = models::UpdatePost {
                    soudane: Some(post.soudane + 1),
                };
                let query = diesel::update(dsl::posts.find(id)).set(&tmp);
                debug!("{}", debug_query::<DBBackend, _>(&query));
                query
                    .execute(conn)
                    .map(|rows| if rows == 1 { Some(()) } else { None })
                    .map_err(Into::into)
            } else {
                Ok(None)
            }
        })
    }
    pub fn count<'a>(
        &self,
    ) -> impl Transaction<Ctx = SqliteConnection, Item = u64, Err = Error> + 'a {
        with_ctx(move |conn| {
            use schema::posts::dsl;
            let query = dsl::posts.count();
            debug!("{}", debug_query::<DBBackend, _>(&query));
            query
                .get_result::<i64>(conn)
                .map(|o| o as u64)
                .map_err(Into::into)
        })
    }
}
