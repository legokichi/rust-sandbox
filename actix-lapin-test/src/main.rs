extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate lapin_futures as lapin;
extern crate actix;
extern crate tokio_core;
extern crate tokio_dns;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate env_logger;

use futures::future;
use futures::prelude::*;
use mdo_future::future::*;

use lapin::client::{Client as RabbitClient, ConnectionOptions};
use std::collections::BTreeMap;

use std::error::Error;

fn main() {
    let mut sys = actix::System::new("System");
    let remote = actix::Arbiter::handle().remote().to_owned();
    let _ = env_logger::try_init();
    let addr = "127.0.0.1:5672";
    let username = "rabbitmq".to_string();
    let password = "rabbitmq".to_string();

    let fut = mdo!{
        stream =<< tokio_dns::tcp_connect(addr, remote);
        let option = ConnectionOptions {
            username,
            password,
            ..Default::default()
        };
        let () = debug!("using rabbitmq option {:?}", option);
        (client, _heartbeater) =<< RabbitClient::connect(stream, &option);
        (ch1, ch2) =<< client.create_channel().join(client.create_channel());
        let ch1 = ch1.clone();
        let ch2 = ch2.clone();
        // https://github.com/sozu-proxy/lapin/issues/67
        ret future::lazy(||{
            println!("ready");
            mdo!{
                let queue_name = uuid::Uuid::new_v4().hyphenated().to_string();
                let data = "hi";
                _ =<< tokio_core::reactor::Timeout::new(::std::time::Duration::from_secs(1), ::actix::Arbiter::handle()).unwrap();
                ((), ()) =<< ch1.queue_declare(&queue_name, &Default::default(), &BTreeMap::new())
                    .join( ch2.queue_declare(&queue_name, &Default::default(), &BTreeMap::new()) );
                (_, msg) =<< ch1.basic_publish("", &queue_name, data.as_bytes(), &Default::default(), Default::default())
                    .join(ch2.basic_get(&queue_name, &Default::default()));
                let () = println!("{:?}", msg);
                ret future::ok(())
            }
        })
    };
    sys.run_until_complete(fut.map_err(|err|{ println!("{:?}", err) })).unwrap();
    println!("end");
}
