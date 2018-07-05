extern crate futures;
extern crate tokio;
extern crate tokio_timer;


use std::time::Duration;
use futures::prelude::*;


fn foo(n: i32) -> Box<dyn Future<Item=(),Error=()> + Send + 'static> {
    let fut = (0..n).fold(Box::new(futures::future::ok(())) as Box<dyn Future<Item=(),Error=()> + Send + 'static>, move |o, i|{
        let fut = o.and_then(move |_|{
            println!("foo {} foo foo foo foo foo foo foo foo foo foo foo foo foo foo foo foo foo foo foo foo", i);
            let fut = tokio_timer::sleep(Duration::from_micros(1)).map(|_| ()).map_err(|_| ());
            Box::new(fut)
        });
        Box::new(fut)
    });
    Box::new(fut)
}

fn tokio_example() {
    let fut = foo(100).join(foo(100)).join(foo(100)).join(foo(100)).join(foo(100)).join(foo(100)).join(foo(100)).join(foo(100)).map(|_|())
    tokio::run(fut);
}
