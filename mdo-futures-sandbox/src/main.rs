#[macro_use]
extern crate mdo;
extern crate mdo_future;

use futures::future::ok;
use futures::prelude::*;
use mdo_future::future::{bind};

fn main() {
    tokio::run(mdo!{
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        () =<< ok(());
        ret ok(())
    });
}
