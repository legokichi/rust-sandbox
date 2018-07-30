#![allow(unused_imports)]
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
extern crate tokio_io;
extern crate tokio_timer;

use futures::future;
use futures::prelude::*;
use mdo_future::future::*;

use lapin_futures::channel::*;
use lapin_futures::client::*;

use std::collections::BTreeMap;

use tokio::net::TcpStream;
use tokio_dns::TcpStream as DnsTcpStream;

use tokio_timer::Delay;
use std::time::{Duration, Instant};


fn main() {
    let rabbit_addr = "localhost:5672".to_string();
    let username = "rabbitmq".to_string();
    let password = "rabbitmq".to_string();
    let queue_name = uuid::Uuid::new_v4().to_string();
    let max_waiting_sec = 60_u64;
    let data = "hi";
    type BoxFut<T> = Box<dyn Future<Item=T, Error=failure::Error> + Send + 'static>;

    let fut: BoxFut<_> = Box::new(mdo!{
        stream =<< DnsTcpStream::connect(rabbit_addr.as_str()).map_err(Into::into);
        let () = println!("stream");
        (client, _heartbeater) =<< {
            let option = ConnectionOptions {
                username,
                password,
                ..Default::default()
            };
            Client::connect(stream, option).map_err(Into::into)
        };
        let () = println!("client");
        produce =<< client.create_channel().map_err(Into::into);
        let () = println!("produce");
        consume =<< client.create_channel().map_err(Into::into);
        let () = println!("consume");
        // https://mariuszwojcik.wordpress.com/2014/05/19/how-to-choose-prefetch-count-value-for-rabbitmq/
        // https://www.rabbitmq.com/tutorials/tutorial-two-go.html
        // https://www.rabbitmq.com/amqp-0-9-1-reference.html
        // https://stackoverflow.com/questions/29841690/how-to-consume-one-message
        // https://github.com/rabbitmq/rabbitmq-java-client/blob/b248a7a2799234793b10e07a573386b17e2bddbf/src/main/java/com/rabbitmq/client/impl/ChannelN.java#L660
        let qos_opts = BasicQosOptions {
            prefetch_size: 0,
            prefetch_count: 1,
            global: false,
            ..Default::default()
        };
        () =<< produce.basic_qos(qos_opts.clone()).map_err(Into::into);
        () =<< consume.basic_qos(qos_opts.clone()).map_err(Into::into);
        let () = println!("qos");
        let dec_opts = QueueDeclareOptions{
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            nowait: false,
            ..Default::default()
        };
        ((), (), (), ()) =<< future::ok(()).join4(
            Box::new(mdo!{
                let produce = produce.clone();
                let consume = consume.clone();
                let queue_name = queue_name.clone();
                let dec_opts = dec_opts.clone();
                () =<< Delay::new(Instant::now() + Duration::from_secs(3)).map_err(Into::into);
                queue =<< consume.queue_declare(&queue_name, dec_opts.clone(), BTreeMap::new()).map_err(Into::into);
                let publish_opts = BasicPublishOptions{
                    ..Default::default()
                };
                let props = BasicProperties::default().with_expiration(format!("{}", max_waiting_sec * 1000));
                opt =<< produce.basic_publish("",  &queue_name, data.as_bytes().to_vec(), publish_opts.clone(), props).map_err(Into::into);
                let _ = println!("sended: {:?}", opt);
                ret future::ok(())
            }) as BoxFut<_>,
            Box::new(mdo!{
                let produce = produce.clone();
                let consume = consume.clone();
                let queue_name = queue_name.clone();
                let dec_opts = dec_opts.clone();
                () =<< Delay::new(Instant::now() + Duration::from_secs(2)).map_err(Into::into);
                queue =<< consume.queue_declare(&queue_name, dec_opts.clone(), BTreeMap::new()).map_err(Into::into);
                let publish_opts = BasicPublishOptions{
                    ..Default::default()
                };
                let props = BasicProperties::default().with_expiration(format!("{}", max_waiting_sec * 1000));
                opt =<< produce.basic_publish("",  &queue_name, data.as_bytes().to_vec(), publish_opts.clone(), props).map_err(Into::into);
                let _ = println!("sended: {:?}", opt);
                ret future::ok(())
            }) as BoxFut<_>,
            Box::new(mdo!{
                let produce = produce.clone();
                let consume = consume.clone();
                let queue_name = queue_name.clone();
                let dec_opts = dec_opts.clone();
                let consume_opts = lapin_futures::channel::BasicConsumeOptions{
                    no_ack: false,
                    ..Default::default()
                };

                queue =<< consume.queue_declare(&queue_name, dec_opts.clone(), BTreeMap::new()).map_err(Into::into);
                consumer =<< consume.basic_consume(&queue, "", consume_opts.clone(), BTreeMap::new()).map_err(Into::into);
                let () = println!("waiting1... csm: {}, msg: {}", queue.consumer_count(), queue.message_count());
                opt =<< consumer.into_future().map(|(a,_)| a).map_err(|(err, _st)| err.into() );
                let val = opt.unwrap();
                let _ = println!("recv: {:?}", val);
                () =<< consume.basic_ack(val.delivery_tag, false).map_err(Into::into);
                let _ = println!("ack");

                
                queue =<< consume.queue_declare(&queue_name, dec_opts.clone(), BTreeMap::new()).map_err(Into::into);
                consumer =<< consume.basic_consume(&queue, "", consume_opts.clone(), BTreeMap::new()).map_err(Into::into);
                let () = println!("waiting2... csm: {}, msg: {}", queue.consumer_count(), queue.message_count());
                opt =<< consumer.into_future().map(|(a,_)| a).map_err(|(err, _st)| err.into() );
                let val = opt.unwrap();
                let _ = println!("recv: {:?}", val);
                () =<< consume.basic_ack(val.delivery_tag, false).map_err(Into::into);
                let _ = println!("ack");

                ret future::ok(())
            }) as BoxFut<_>,
        );
        ret future::ok(())
    });

    println!("ready");
    tokio::run(fut.map_err(|err|{ println!("err: {:?}", err); }));
    println!("ok");
}
