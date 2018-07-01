extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate tokio;
extern crate tokio_timer;
extern crate actix;

use std::time::Duration;
use futures::prelude::*;
use mdo_future::future::*;
use actix::prelude::*;

fn main(){
    println!("actix 0.6 runtime");
    System::run(|| {
        tokio::spawn(mdo!{
            _ =<< ::tokio_timer::sleep(Duration::from_secs(1)).map_err(|_| ());
            let _ = println!("1");
            _ =<< ::tokio_timer::sleep(Duration::from_secs(1)).map_err(|_| ());
            let _ = println!("2");
            _ =<< ::tokio_timer::sleep(Duration::from_secs(1)).map_err(|_| ());
            let _ = println!("3");
            ret ret(System::current().stop())
        });
    });
    println!("end");
}