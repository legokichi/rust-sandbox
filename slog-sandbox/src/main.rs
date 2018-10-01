#![feature(await_macro, async_await, futures_api, nll)]

use tokio::await;

use slog::{trace, debug, info, warn, error, o, Drain, Logger};
use slog_atomic::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;

use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::time::Duration;


fn main() {
    let decorator = ::slog_term::PlainDecorator::new(std::io::stdout());
    let drain = ::slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = ::slog_async::Async::new(drain).build().fuse();
    let tmp = AtomicSwitch::new(drain);
    // 競合フリーでログレベルを動的に変更できる
    let ctrl = tmp.ctrl();
    let drain = tmp.fuse();
    let root_log = Logger::root(
        drain,
        o!("version" => env!("CARGO_PKG_VERSION")),
    );
    {
        let root_log = root_log.clone();
        tokio::run_async(async move {
            let log1 = root_log.new(o!("log" => "1"));
            tokio::spawn_async(async move {
                for i in 0..10_i32 {
                    await!(tokio_timer::sleep(::std::time::Duration::from_millis(1000)));
                    info!(log1, "hello");
                }
            });
            let log2 = root_log.new(o!("log" => "2"));
            tokio::spawn_async(async move {
                
                for i in 0..10_i32 {
                    await!(tokio_timer::sleep(::std::time::Duration::from_millis(100)));
                    info!(log2, "hello2");
                }
            });
            let log3 = root_log.new(o!("log" => "3"));
            tokio::spawn_async(foo(log3));
        });
    }
    info!(root_log, "end");
}

use std::future::Future;
async fn foo(log: Logger) -> () {
    for i in 0..10_i32 {
        await!(tokio_timer::sleep(::std::time::Duration::from_millis(10)));
        info!(log, "hello3");
    }
}