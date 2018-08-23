#![type_length_limit="2097152"]
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

    type BoxFut<T> = Box<dyn Future<Item=T, Error=failure::Error> + Send + 'static>;

    let option = ConnectionOptions {
        username: username.to_string(),
        password: password.to_string(),
        ..Default::default()
    };
    let fut: BoxFut<_> = Box::new(mdo!{
        stream =<< DnsTcpStream::connect(rabbit_addr.to_string().as_str()).map_err(Into::into);
        (client, heartbeat) =<< Client::connect(stream, option).map_err(Into::into);
        channel =<< client.create_channel().map_err(Into::into);
        () =<< channel.close_ok().map_err(Into::into);
        channel =<< client.create_channel().map_err(Into::into);
        () =<< channel.close_ok().map_err(Into::into);
        ret ret(())
    });
    tokio::run(fut.map_err(|err|{ error!("err reciver: {:?}", err); }));

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
        std::thread::spawn(move ||{
            let fut: BoxFut<_> = Box::new(mdo!{
                stream =<< DnsTcpStream::connect(rabbit_addr.to_string().as_str()).map_err(Into::into);
                (client, heartbeat) =<< {
                    let option = ConnectionOptions {
                        username: username.to_string(),
                        password: password.to_string(),
                        heartbeat: 60,
                        ..Default::default()
                    };
                    Client::connect(stream, option).map_err(Into::into)
                };
                ret ret((client, heartbeat))
            });
            let (client, mut heartbeat) = fut.wait().unwrap();
            let handle = heartbeat.handle().unwrap();
            std::thread::spawn(move||{ tokio::run(heartbeat.map_err(|err|{ error!("heartbeat error: {:?}", err); })); info!("heartbeat end"); });
            for i in 0..10 {
                let queue_name = queue_name.clone();
                let dec_opts = dec_opts.clone();
                let client = client.clone();
                let data = format!("{:?}", i);
                let fut: BoxFut<_> = Box::new(mdo!{
                    () =<< Delay::new(Instant::now() + Duration::from_secs(6*60)).map_err(Into::into);
                    produce =<< client.create_channel().map_err(Into::into);
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
            handle.stop();
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
                    (client, heartbeat) =<< {
                        let option = ConnectionOptions {
                            username: username.to_string(),
                            password: password.to_string(),
                            heartbeat: 60,
                            ..Default::default()
                        };
                        Client::connect(stream, option).map_err(Into::into)
                    };
                    ret ret((client, heartbeat))
                });
                let (client, mut heartbeat) = fut.wait().unwrap();
                let handle = heartbeat.handle().unwrap();
                std::thread::spawn(move||{ tokio::run(heartbeat.map_err(|err|{ error!("heartbeat error: {:?}", err); })); info!("heartbeat end"); });
                let fut: BoxFut<_> = Box::new(mdo!{
                    consume =<< client.create_channel().map_err(Into::into);
                    queue =<< consume.queue_declare(&queue_name, dec_opts.clone(), BTreeMap::new()).map_err(Into::into);
                    let () = info!("reciv: basic_consume");
                    consumer =<< consume.basic_consume(&queue, "", BasicConsumeOptions{ no_ack: false, ..Default::default() }, BTreeMap::new()).map_err(Into::into);
                    let () = info!("reciv: consume");
                    msg =<< consumer.into_future().map(|(a,_)| a.unwrap()).map_err(|(err, _st)| err).map_err(Into::into);
                    let () = info!("reciv: ack");
                    () =<< consume.basic_ack(msg.delivery_tag, false).map_err(Into::into);
                    let () = info!("reciv: close");
                    () =<< consume.close_ok().map_err(Into::into);
                    let () = info!("reciv: {:?}", std::str::from_utf8(&msg.data));
                    ret ret(())
                });
                tokio::run(fut.map_err(|err|{ error!("err reciver: {:?}", err); }));
                handle.stop();
            }
        })
    };
    info!("ready");
    th1.join().unwrap();
    th2.join().unwrap();
    info!("ok");

}
