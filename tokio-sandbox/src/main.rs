#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate futures;
extern crate tokio;
extern crate tokio_timer;
extern crate tokio_reactor;
extern crate tokio_threadpool;
use mdo_future::future::{bind};
use futures::prelude::*;
use tokio::prelude::*;
use tokio_threadpool::{blocking};


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse::<u64>().unwrap_or(4) as usize;
    let m = args[2].parse::<u64>().unwrap_or(200) as usize;
    println!("pool_size: {}, max_blocking:{}", n, m);
    let mut builder = tokio::executor::thread_pool::Builder::new();
    builder.pool_size(n).max_blocking(m);
    let mut core = tokio::runtime::Builder::new().threadpool_builder(builder).build().unwrap();
    let now = tokio::clock::now();
    for i in 0..3 {
        let fut = Box::new(future::ok(()));
        let fut = (0..3).fold(fut as Box<Future<Item=(), Error=()> + Send>, move |o, j|{
            let fut = mdo!{
                () =<< o;
                let (sender, receiver) = futures::sync::mpsc::channel::<()>(10);
                let sender = std::sync::Arc::new(std::sync::Mutex::new(sender));
                () =<< futures::lazy(move ||{
                    future::poll_fn(move || {
                        let sender = sender.clone();
                        blocking(move ||{
                            println!("blocking {}:{}, thread:{:?}, {}s", i, j, std::thread::current().id(), now.elapsed().as_secs());
                            std::thread::sleep(std::time::Duration::from_secs(1));
                            {
                                let mut sender = (*sender).lock().unwrap();
                                let _ = sender.try_send(());
                            }
                        }).map_err(|_|())
                    })
                });
                (_,_) =<< receiver.into_future().map_err(|_|());
                ret future::ok(())
            };
            Box::new(fut)
        });
        let fut = fut.map(|_|());
        core.spawn(fut);
        let fut = Box::new(future::ok(()));
        let fut = (0..3).fold(fut as Box<Future<Item=(), Error=()> + Send>, move |o, j|{
            Box::new(o.and_then(move |_|{
                println!("non-blocking {}:{}, thread:{:?}, {}s", i, j, std::thread::current().id(), now.elapsed().as_secs());
                tokio_timer::sleep(std::time::Duration::from_secs(1)).map_err(|_|())
            }))
        });
        core.spawn(fut);
    }
    core.shutdown_on_idle().wait().unwrap();
    println!("finish");
}
