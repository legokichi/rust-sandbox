extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate lapin_futures as lapin;
#[macro_use]
extern crate log;
pub extern crate tokio_core;
extern crate tokio_dns;
extern crate tokio_io;
extern crate env_logger;

use futures::future;
use futures::prelude::*;
use mdo_future::future::*;

use lapin::client::{Client as RabbitClient, ConnectionOptions};
use std::collections::BTreeMap;

use std::error::Error;

fn main() {
    let _ = env_logger::try_init();
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let remote = core.remote();
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
                let queue_name = "foo";
                let data = "hi";
                () =<< ch1.queue_declare(queue_name, &Default::default(), &BTreeMap::new());
                _ =<< ch1.basic_publish("", queue_name, data.as_bytes(), &Default::default(), Default::default());
                let () = println!("published");
                () =<< ch2.queue_declare(queue_name, &Default::default(), &BTreeMap::new());
                msg =<< futures::lazy(move ||{
                    future::poll_fn(move || {
                        let poll = ch2.basic_get(&queue_name, &Default::default()).poll();
                        println!("{:?}", poll);
                        if let Ok(_) = poll {
                            // has value
                            poll
                        }else{
                            // empty...
                            Ok(Async::NotReady)
                        }
                    })
                });
                msg =<< future::result(String::from_utf8(msg.delivery.data.to_vec())
                    .map_err(|err| ::std::io::Error::new(::std::io::ErrorKind::Other, err.description())));
                let () = println!("{}", msg);
                ret future::ok(())
            }
        })
    };
    core.run(fut).unwrap();
    println!("end");
}
