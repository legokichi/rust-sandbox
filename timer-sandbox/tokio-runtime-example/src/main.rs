extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate tokio;
extern crate tokio_timer;

use std::time::Duration;
use futures::prelude::*;
use mdo_future::future::*;

fn main(){
    println!("tokio runtime");
    tokio::run(mdo!{
        _ =<< ::tokio_timer::sleep(Duration::from_secs(1));
        let _ = println!("1");
        _ =<< ::tokio_timer::sleep(Duration::from_secs(1));
        let _ = println!("2");
        _ =<< ::tokio_timer::sleep(Duration::from_secs(1));
        let _ = println!("3");
        ret ret(())
    }.map_err(|err| println!("{:?}", err) ));
    println!("end");
}