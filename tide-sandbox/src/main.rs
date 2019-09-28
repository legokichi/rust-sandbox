#![feature(async_await, async_closure)]
#![allow(unused_imports)]

mod middleware {
    use tide::Context;
    use tide::middleware::Next;
    use tide::Response;
    use futures::future::Future;
    use futures::future::BoxFuture;

    pub struct ArmorMiddleware;
    impl ArmorMiddleware {
        pub fn new()->ArmorMiddleware{ArmorMiddleware}
    }
    impl<State: Send + Sync + 'static> tide::middleware::Middleware<State> for ArmorMiddleware {
        fn handle<'a>(
            &'a self, 
            cx: Context<State>, 
            next: Next<'a, State>
        ) -> BoxFuture<'a, Response> {
            Box::pin(async move {
                let mut res = next.run(cx).await;
                let mut headers = res.headers_mut();
                armor::armor(&mut headers);
                res
            })
        }
    }
}

use crate::middleware::ArmorMiddleware;
use failure::ResultExt;
use futures::compat::Future01CompatExt as _;
use serde::Deserialize;
use structopt::StructOpt;
use async_log::span;
use log::info;

#[derive(Deserialize, Debug, Clone)]
struct Config {
}

#[derive(StructOpt, Debug)]
struct Opt {
}


// #[runtime::main(runtime_tokio::Tokio)]
fn main() -> Result<(), failure::Error> {
    dotenv::dotenv().ok();
    // env_logger::try_init()?;

    let logger = env_logger::Builder::from_default_env()
        .build();
    async_log::Logger::wrap(logger, || 12)
        .start(log::LevelFilter::Trace)?;

    let config = envy::from_env::<Config>()?;
    let Opt{} = Opt::from_args();
    let mut app = tide::App::new();
    app.middleware(ArmorMiddleware::new());
    app.at("/").get(|req| async move {
        span!("level I", {
            info!("1");
            tokio_timer::sleep(std::time::Duration::from_millis(10000)).compat().await.unwrap();
            info!("2");
            tokio_timer::sleep(std::time::Duration::from_millis(10000)).compat().await.unwrap();
            info!("3");
            "Hello, world!"
        })
    });
    Ok(app.run("127.0.0.1:8000")?)
}
