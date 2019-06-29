#![feature(async_await)]

use log::*;
use err_derive::Error;
use futures::compat::*;
use std::time::Duration;
use runtime::prelude::*;
use std::error::Error;
use tracing_futures::Instrument;
use tracing::{field, Level, span};
use tracing_log::*;

#[derive(Debug, Error)]
pub enum MyError {
    // #[error(display = "error: {:?}", _0)]
    // Unknwon(#[error] Box<dyn Send + Sync + std::error::Error>),
    #[error(display = "io: {:?}", _0)]
    Io(#[error(cause)] std::io::Error),
    #[error(display = "timer: {}", _0)]
    Timer(#[error(cause)] tokio_timer::Error),
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), MyError> {
    std::env::set_var("RUST_LOG", "info");
    tracing_env_logger::try_init().unwrap();
    info!("hi");
    let err = MyError::Io(std::io::Error::new(std::io::ErrorKind::Other, "unknown"));
    // コンパイルエラー！！
    // err.source() -> Option<dyn &Error> は同期ブロック文の中で実行しないといけない！！
    // let mut cause = err.source();
    // while let Some(e) = cause {
    //     eprintln!("caused by: {}", e);
    //     cause = e.source();
    // }
    {
        let mut cause = err.source();
        while let Some(e) = cause {
            eprintln!("caused by: {}", e);
            cause = e.source();
        }
    }
    // async クロージャ
    let a = async || -> Result<(), MyError>{
        for _ in 0..10 {
            // 非同期ブロック文
            let fut = async {
                info!("hello");
                let res: Result<std::time::Instant, std::io::Error> = runtime::time::Delay::new(Duration::from_secs(1))
                    .timeout(Duration::from_millis(2000))
                    .await;
                res.map_err(MyError::Io)
            };
            fut.await?;
            info!("world");
            let res: Result<Result<(), tokio_timer::Error>, std::io::Error> = tokio_timer::sleep(Duration::from_secs(1))
                .instrument(span!(Level::TRACE, "client_to_server"))
                .compat()
                .timeout(Duration::from_millis(1800)).await;
            let () = res.map_err(MyError::Io)?.map_err(MyError::Timer)?;
        }
        Ok(())
    };
    let handle = runtime::spawn(a());
    let (a,b) = futures::future::join(handle, a()).await;
    a?;
    b?;
    Ok(())
}
