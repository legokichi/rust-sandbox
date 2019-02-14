#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use std::sync::Arc;
use diesel::prelude::*;
use diesel::result::{ConnectionError, Error as DieselError};
use failure::Fail;
use try_from::{Void, TryFromIntError};

pub mod models;
pub mod schema;
pub mod tx;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "connection: {}", _0)]
    Connection(#[cause] ConnectionError),
    #[fail(display = "diesel: {}", _0)]
    Diesel(#[cause] DieselError),
    #[fail(display = "no such post id: {}", _0)]
    NotFound(i32),
    #[fail(display = "out of range: {}", _0)]
    TryFromIntError(#[cause] TryFromIntError),
    #[fail(display = "out of range: {:?}", _0)]
    TryFromVoid(Void),
}

impl From<DieselError> for Error {
    fn from(o: DieselError) -> Self {
        Error::Diesel(o)
    }
}

#[derive(Clone)]
pub struct Db {
    conn: Arc<SqliteConnection>,
}

impl Db {
    pub fn new(database_url: &str) -> Result<Self, Error> {
        SqliteConnection::establish(&database_url)
            .map(Arc::new)
            .map(|conn| Self { conn })
            .map_err(Error::Connection)
    }
    pub fn transaction<T>(&self, callback: impl FnOnce(&SqliteConnection)-> Result<T, Error>) -> Result<T, Error> {
        let conn = &*self.conn;
        conn.transaction(|| {
            callback(conn)
        })   
    }
    pub fn test_transaction<T>(&self, callback: impl FnOnce(&SqliteConnection)-> Result<T, Error>) -> T {
        let conn = &*self.conn;
        conn.test_transaction(|| {
            callback(conn)
        })
    }
}

