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

pub mod models;
pub mod schema;

use transaction::mdo::*;
use diesel::prelude::*;
use chrono::{NaiveDateTime, Utc};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Connection(#[cause] diesel::ConnectionError),
    #[fail(display = "{}", _0)]
    Query(#[cause] diesel::result::Error),
}

pub fn establish_connection(database_url: &str) -> Result<SqliteConnection, Error> {
    SqliteConnection::establish(&database_url).map_err(Error::Connection)
}

pub fn create_post(conn: &SqliteConnection, author: &str, body: &str) -> Result<usize, Error> {
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
        .map_err(Error::Query)
}

pub fn list_post(conn: &SqliteConnection, offset: i64, limit: i64) -> Result<(i64, Vec<models::Post>), Error> {
    use schema::posts::dsl;

    let a = dsl::posts
        .count()
        .get_result::<i64>(conn)
        .map_err(Error::Query);
    let b = dsl::posts
        .order(dsl::timestamp.desc())
        .limit(limit)
        .offset(offset)
        .get_results::<models::Post>(conn)
        .map_err(Error::Query);

    match (a, b) {
        (Ok(r1), Ok(r2)) => Ok((r1, r2)),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}
