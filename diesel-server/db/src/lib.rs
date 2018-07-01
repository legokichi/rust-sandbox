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
pub mod error;
pub use error::Error;
pub use error::ErrorKind;

use futures::prelude::*;
use futures::future;
use transaction::prelude::*;
use transaction::mdo::*;
use diesel::prelude::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use std::sync::{Arc, Mutex};


#[derive(Clone)]
pub struct DB {
    conn: Arc<Mutex<SqliteConnection>>
}

impl DB {
    pub fn new(database_url: &str) -> Box<Future<Item=Self, Error=Error> + Send + 'static> {
        let o = SqliteConnection::establish(&database_url)
            .map_err(Into::into)
            .map(Mutex::new)
            .map(Arc::new)
            .map(|conn| Self { conn });
        Box::new(future::result(o))
    }
    pub fn run<'a, T: 'static + Send>(&self, tx: impl Transaction<Ctx=SqliteConnection, Item=T, Err=Error>) -> Box<Future<Item=T, Error=Error> + Send + 'static> {
        let o = {
            let mut conn = self.conn.lock().unwrap();
            tx.run(&mut conn)
        };
        Box::new(future::result(o))
    }
    pub fn create<'a>(&self, author: &'a str, body: &'a str) -> impl Transaction<Ctx=SqliteConnection, Item=usize, Err=Error> + 'a{
        with_ctx(move |conn|{
            use schema::posts;
            let now = Utc::now();
            let new_post = models::NewPost {
                timestamp: NaiveDateTime::from_timestamp(now.timestamp(), 0),
                author: author,
                body: body,
                soudane: 0,
            };
            diesel::insert_into(posts::table)
                .values(&new_post)
                .execute(conn)
                .map_err(Into::into)
        })
    }
    pub fn create_with_time<'a>(&self, author: &'a str, body: &'a str, timestamp: DateTime<Utc>) -> impl Transaction<Ctx=SqliteConnection, Item=usize, Err=Error> + 'a{
        with_ctx(move |conn|{
            use schema::posts;
            let now = timestamp;
            let new_post = models::NewPost {
                timestamp: NaiveDateTime::from_timestamp(now.timestamp(), 0),
                author: author,
                body: body,
                soudane: 0,
            };
            diesel::insert_into(posts::table)
                .values(&new_post)
                .execute(conn)
                .map_err(Into::into)
        })
    }
    pub fn list<'a>(&self, offset: u64, limit: u64) -> impl Transaction<Ctx=SqliteConnection, Item=Vec<models::Post>, Err=Error> + 'a {
        with_ctx(move |conn|{
            use schema::posts::dsl;
            dsl::posts
                .order(dsl::timestamp.desc())
                .limit(limit as i64)
                .offset(offset as i64)
                .get_results::<models::Post>(conn)
                .map_err(Into::into)
        })
    }
    pub fn soudane<'a>(&self, id: i32) -> impl Transaction<Ctx=SqliteConnection, Item=Option<()>, Err=Error> + 'a {
        with_ctx(move |conn|{
            use schema::posts::dsl;
            let post_opt = dsl::posts
                .find(id)
                .get_result::<models::Post>(conn)
                .optional()?;
            if let Some(post) = post_opt {
                diesel::update(dsl::posts
                    .find(id))
                    .set(&models::UpdatePost {
                        soudane: Some(post.soudane + 1)
                    })
                    .execute(conn)
                    .map(|rows| if rows == 1 { Some(()) } else { None })
                    .map_err(Into::into)
            }else{
                Ok(None)
            }
        })
    }
    pub fn count<'a>(&self) -> impl Transaction<Ctx=SqliteConnection, Item=u64, Err=Error> + 'a {
        with_ctx(move |conn|{
            use schema::posts::dsl;
            dsl::posts
                .count()
                .get_result::<i64>(conn)
                .map(|o| o as u64)
                .map_err(Into::into)
        })
    }
}

