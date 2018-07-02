extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate tokio_core;

use std::time::Duration;
use futures::prelude::*;
use mdo_future::future::*;
use tokio_core::reactor::Core;
use tokio_core::reactor::Timeout;

fn main() {
    println!("tokio_core");
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    core.run(mdo!{
        _ =<< Timeout::new(Duration::from_secs(1), &handle).unwrap();
        let _ = println!("1");
        _ =<< Timeout::new(Duration::from_secs(1), &handle).unwrap();
        let _ = println!("2");
        _ =<< Timeout::new(Duration::from_secs(1), &handle).unwrap();
        let _ = println!("3");
        ret ret(())
    }.map_err(|err| println!("{:?}", err) ) ).unwrap();
    println!("end");
}