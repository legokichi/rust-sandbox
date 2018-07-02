extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate tokio_core;
extern crate actix;

use std::time::Duration;
use futures::prelude::*;
use mdo_future::future::*;
use actix::prelude::*;
use tokio_core::reactor::Timeout;

fn main(){
    println!("actix 0.5");
    let sys = System::new("system");
    Arbiter::handle().spawn(mdo!{
        _ =<< Timeout::new(Duration::from_secs(1), Arbiter::handle()).unwrap();
        let _ = println!("1");
        _ =<< Timeout::new(Duration::from_secs(1), Arbiter::handle()).unwrap();
        let _ = println!("2");
        _ =<< Timeout::new(Duration::from_secs(1), Arbiter::handle()).unwrap();
        let _ = println!("3");
        ret ret(Arbiter::system().do_send(actix::msgs::SystemExit(0)))
    }.map_err(|err| println!("{:?}", err) ) );
    sys.run();
    println!("end");
}