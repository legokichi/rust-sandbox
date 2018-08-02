extern crate failure;
extern crate futures;
extern crate env_logger;
extern crate uuid;
#[macro_use]
extern crate mdo;
extern crate lapin_futures;
extern crate mdo_future;
#[macro_use]
extern crate log;
extern crate tokio;
extern crate tokio_dns;
extern crate tokio_timer;

use futures::prelude::*;
use mdo_future::future::*;

use lapin_futures::channel::*;
use lapin_futures::client::*;

use std::collections::BTreeMap;

use tokio_dns::TcpStream as DnsTcpStream;
use futures::stream::Stream;
use tokio_timer::Delay;
use std::time::{Duration, Instant};


fn main() {
    let _ = env_logger::try_init();

    let rabbit_addr = "localhost:5672";
    let username = "rabbitmq";
    let password = "rabbitmq";
    let queue_name = uuid::Uuid::new_v4().to_string();
    let max_waiting_sec = 60_u64;
    type BoxFut<T> = Box<dyn Future<Item=T, Error=failure::Error> + Send + 'static>;
    
    let dec_opts = QueueDeclareOptions{
        passive: false,
        durable: true,
        exclusive: false,
        auto_delete: false,
        nowait: false,
        ..Default::default()
    };
    let th1 = {
        let queue_name = queue_name.clone();
        let dec_opts = dec_opts.clone();
        let fut: BoxFut<_> = Box::new(mdo!{
            stream =<< DnsTcpStream::connect(rabbit_addr.to_string().as_str()).map_err(Into::into);
            (client, _heartbeater) =<< {
                let option = ConnectionOptions {
                    username: username.to_string(),
                    password: password.to_string(),
                    ..Default::default()
                };
                Client::connect(stream, option).map_err(Into::into)
            };
            ch =<< client.create_channel().map_err(Into::into);
            ret ret(ch)
        });
        let ch = fut.wait().unwrap();
        std::thread::spawn(move ||{
            for i in 0..100 {
                let queue_name = queue_name.clone();
                let dec_opts = dec_opts.clone();
                let produce = ch.clone();
                let data = format!("{:?}", i);
                let fut: BoxFut<_> = Box::new(mdo!{
                    () =<< Delay::new(Instant::now() + Duration::from_secs(1)).map_err(Into::into);
                    _queue =<< produce.queue_declare(&queue_name, dec_opts, BTreeMap::new()).map_err(Into::into);
                    let publish_opts = BasicPublishOptions{
                        ..Default::default()
                    };
                    let props = BasicProperties::default().with_expiration(format!("{}", max_waiting_sec * 1000));
                    opt =<< produce.basic_publish("",  &queue_name, data.as_bytes().to_vec(), publish_opts.clone(), props).map_err(Into::into);
                    let _ = info!("sended: {:?}", opt);
                    ret ret(())
                });
                tokio::run(fut.map_err(|err|{ error!("err sender: {:?}", err); }));
            }
        })
    };
    
    let th2 = {
        let queue_name = queue_name.clone();
        let dec_opts = dec_opts.clone();
        std::thread::spawn(move ||{
            loop{
                let queue_name = queue_name.clone();
                let dec_opts = dec_opts.clone();
                let fut: BoxFut<_> = Box::new(mdo!{
                    stream =<< DnsTcpStream::connect(rabbit_addr.to_string().as_str()).map_err(Into::into);
                    (client, _heartbeater) =<< {
                        let option = ConnectionOptions {
                            username: username.to_string(),
                            password: password.to_string(),
                            ..Default::default()
                        };
                        Client::connect(stream, option).map_err(Into::into)
                    };
                    consume =<< client.create_channel().map_err(Into::into);
                    queue =<< consume.queue_declare(&queue_name, dec_opts.clone(), BTreeMap::new()).map_err(Into::into);
                    consumer =<< consume.basic_consume(&queue, "", BasicConsumeOptions{ no_ack: false, ..Default::default() }, BTreeMap::new()).map_err(Into::into);
                    msg =<< consumer.into_future().map(|(a,_)| a.unwrap()).map_err(|(err, _st)| err).map_err(Into::into);
                    () =<< consume.basic_ack(msg.delivery_tag, false).map_err(Into::into);
                    () =<< consume.close_ok().map_err(Into::into);
                    let () = info!("{:?}", std::str::from_utf8(&msg.data));
                    ret ret(())
                });
                tokio::run(fut.map_err(|err|{ error!("err reciver: {:?}", err); }));    
            }
        })
    };
    info!("ready");
    th1.join().unwrap();
    th2.join().unwrap();
    info!("ok");
}
