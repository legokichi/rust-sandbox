#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate mdo;
extern crate db;
extern crate futures;
extern crate mdo_future;
extern crate transaction;

pub mod error;
pub use error::Error;
pub use error::ErrorKind;

use futures::future;
use futures::prelude::*;
use mdo_future::future::*;
use transaction::prelude::*;

#[derive(Clone)]
pub struct Posts {
    db: db::DB,
}

impl Posts {
    pub fn new(database_url: &str) -> Box<Future<Item = Self, Error = Error> + Send + 'static> {
        let fut = mdo!{
            db =<< db::DB::new(database_url);
            ret future::ok(Self { db })
        }.map_err(Into::into);
        Box::new(fut)
    }
    pub fn list(
        &self,
        offset: u64,
        limit: u64,
    ) -> Box<Future<Item = (u64, Vec<db::models::Post>), Error = Error> + Send + 'static> {
        debug!("list: {} {}", offset, limit);
        use transaction::mdo::*;
        let db = &self.db;
        let fut =
            db.run(mdo!{
                list =<< db.list(offset, limit);
                (count, list) =<< db.count().join(transaction::ok(list));
                ret transaction::ok((count, list))
            }).map_err(Into::into);
        Box::new(fut)
    }
    pub fn create(
        &self,
        author: &str,
        body: &str,
    ) -> Box<Future<Item = (), Error = Error> + Send + 'static> {
        debug!("create: {} {}", author, body);
        use transaction::mdo::*;
        let db = &self.db;
        let fut =
            db.run(mdo!{
                _ =<< db.create(author, body);
                ret transaction::ok(())
            }).map_err(Into::into);
        Box::new(fut)
    }
    pub fn soudane(
        &self,
        id: i32,
    ) -> Box<Future<Item = Option<()>, Error = Error> + Send + 'static> {
        debug!("soudane: {}", id);
        let fut = self.db.run(self.db.soudane(id)).map_err(Into::into);
        Box::new(fut)
    }
}
