extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate tokio_core;
extern crate tokio;
extern crate tokio_timer;
extern crate actix;

use futures::future;
use futures::prelude::*;
use mdo_future::future::*;
use actix::prelude::*;

fn tokio_core_exmple() {
    println!("tokio_core");
    let mut core = ::tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    core.run(mdo!{
        _ =<< tokio_core::reactor::Timeout::new(::std::time::Duration::from_secs(1), &handle).unwrap();
        let _ = println!("1");
        _ =<< tokio_core::reactor::Timeout::new(::std::time::Duration::from_secs(1), &handle).unwrap();
        let _ = println!("2");
        _ =<< tokio_core::reactor::Timeout::new(::std::time::Duration::from_secs(1), &handle).unwrap();
        let _ = println!("3");
        ret ret(())
    }).unwrap();
    println!("end");
}

fn tokio_example(){
    println!("tokio runtime");
    tokio::run(mdo!{
        _ =<< ::tokio_timer::sleep(::std::time::Duration::from_secs(1));
        let _ = println!("1");
        _ =<< ::tokio_timer::sleep(::std::time::Duration::from_secs(1));
        let _ = println!("2");
        _ =<< ::tokio_timer::sleep(::std::time::Duration::from_secs(1));
        let _ = println!("3");
        ret ret(())
    }.map_err(|err| println!("{:?}", err) ));
    println!("end");
}


struct MyActor {}
impl MyActor {
    fn new() -> Self { Self {} }
}
impl Actor for MyActor {
    type Context = Context<Self>;
}
enum MyMessage {
    Ping
}
impl Message for MyMessage {
    type Result = MyResponse;
}
#[derive(Debug)]
enum MyResponse {
    Pong
}
impl ::actix::dev::MessageResponse<MyActor, MyMessage> for MyResponse {
    fn handle<R: ::actix::dev::ResponseChannel<MyMessage>>(self, ctx: &mut <MyActor as Actor>::Context, tx: Option<R>){
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
impl Handler<MyMessage> for MyActor {
    type Result = MyResponse;
    fn handle(&mut self, msg: MyMessage, _ctx: &mut Context<Self>) -> Self::Result {
        ::std::thread::sleep(std::time::Duration::from_secs(1));
        MyResponse::Pong
    }
}


fn actix_example(){
    println!("actix 0.5");
    let sys = System::new("system");
    let addr: Addr<Unsync, _> = MyActor::new().start();
    Arbiter::handle().spawn(mdo!{
        _ =<< addr.send(MyMessage::Ping).map_err(|_| ());
        let _ = println!("1");
        _ =<< addr.send(MyMessage::Ping).map_err(|_| ());
        let _ = println!("2");
        _ =<< addr.send(MyMessage::Ping).map_err(|_| ());
        let _ = println!("3");
        ret ret(Arbiter::system().do_send(actix::msgs::SystemExit(0)))
    });
    sys.run();
    println!("end");
}


fn main(){
    //tokio_core_exmple();
    //tokio_example();
    actix_example();
}