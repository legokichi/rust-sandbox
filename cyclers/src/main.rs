use futures::prelude::*;
// use futures::future::Either;
// use futures::stream::BoxStream;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum FooState {
    Init,
    High,
    Low
}
#[derive(Debug)]
enum FooEvent {
    Tick,
    Tack
}
#[derive(Debug)]
enum FooCommand {
    High,
    Low,
}
fn foo(source: impl Stream<Item=FooEvent> + Send + 'static) -> impl Stream<Item=FooCommand> + Send + 'static {
    source.filter_map({
        let state = Arc::new(Mutex::new(FooState::Init));
        move|o| {
            let mut st = state.lock().unwrap();
            let ret = match (&*st, o) {
                (FooState::Init, FooEvent::Tick) => {
                    *st = FooState::High;
                    Some(FooCommand::High)
                },
                (FooState::Init, FooEvent::Tack) => {
                    *st = FooState::Low;
                    Some(FooCommand::Low)
                },
                (FooState::Low, FooEvent::Tick) => {
                    *st = FooState::High;
                    Some(FooCommand::High)
                },
                (FooState::High, FooEvent::Tack) => {
                    *st = FooState::Low;
                    Some(FooCommand::Low)
                },
                _ => {
                    // not permitted, ignore
                    None
                }
            };
            future::ready(ret)
        }
    })
}
#[tokio::main]
async fn main(){
    let rx = futures::stream::iter(vec![
        FooEvent::Tick,
        FooEvent::Tack,
        FooEvent::Tick,
        FooEvent::Tack
    ]);
    let rx = foo(rx);
    let coms = rx.collect::<Vec<_>>().await;
    println!("{:?}", coms);
}

