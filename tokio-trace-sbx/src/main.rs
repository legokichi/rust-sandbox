#[macro_use]
extern crate tokio_trace;
use tokio::prelude::*;

use tokio_trace::Level;


fn main() {
    event!(Level::ERROR, "something has happened!");
    let mut span = span!("my span");
    info!("ha?");
    // tokio::run(
        span.enter(|| {
            info!("hello");
            // future::ok(())
        })
    // );
}
