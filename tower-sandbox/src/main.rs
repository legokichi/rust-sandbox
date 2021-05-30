// pub trait Service<Request> {
//     type Response;
//     type Error;
//     type Future: Future;
//     pub fn poll_ready(
//         &mut self, 
//         cx: &mut Context<'_>
//     ) -> Poll<Result<(), Self::Error>>;
//     pub fn call(&mut self, req: Request) -> Self::Future;
// }
use std::error::Error;
use std::future::Future;
use tower::Service;
use std::pin::Pin;
use std::task::{Context, Poll};

struct AReq{}
struct ARes{}
struct HogeReq{}
struct HogeRes{}
type BoxErr = Box<dyn Error+Send+Sync+'static>;
type BoxFut<T> = Pin<Box<dyn Future<Output = T>+Send+Sync+'static>>;
async fn hoge_user<T>(mut ctx: T, AReq{}: AReq) -> Result<ARes, BoxErr>
where
    T: Service<HogeReq, Future=BoxFut<Result<HogeRes, BoxErr>>>
     + Service<(), Future=BoxFut<Result<(), BoxErr>>>
{
    let HogeRes{} = ctx.call(HogeReq{}).await?;
    let () = ctx.call(()).await?;
    Ok(ARes{})
}


struct Ctx{}

fn hoge(&mut Ctx, HogeReq{}: HogeReq) -> BoxFut<Result<HogeRes, BoxErr>> {
    unimplemented!()
}
fn unit(&mut Ctx, (): ()) -> BoxFut<Result<(), BoxErr>> {
    unimplemented!()
}

macro_rules! svc(
    ($ctx:ty, $req:ty, $res:ty, $err:ty, $mtd:ident)=> {
        impl Service<$req> for $ctx {
            type Response = $res;
            type Error = $err;
            type Future = BoxFut<Result<Self::Response, Self::Error>>;
            fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: $req) -> Self::Future {
                self.$mtd(req)
            }
        }
    };
);
svc!(Ctx, HogeReq, HogeRes, BoxErr, hoge);
svc!(Ctx, (), (), BoxErr, unit);

#[tokio::main]
async fn main() -> Result<(), BoxErr> {
    let ARes{} = hoge_user(Ctx{}, AReq{}).await?;
    // use hello_world_service::*;
    // let mut o = HelloWorld{ count: 0 };
    // o.add(Request{}).await;
    // println!("Hello, world!");
    Ok(())
}

// mod hello_world_service {
//     use tower_service::Service;
//     use std::task::{Context, Poll};
//     use futures::prelude::*;
//     use err_derive::Error;
//     use std::pin::Pin;

//     #[derive(Debug)]
//     pub struct HelloWorld{
//         pub count: i64,
//     }
//     impl HelloWorld {
//         pub async fn add(&mut self, req: Request) -> Result<Response, Error> {
//             self.count += 1;
//             futures::future::ready(()).await;
//             self.count += 1;
//             futures::future::ready(()).await;
//             Ok(Response{})
//         }
//     }
//     pub struct Request {}
//     pub struct Response {}
    
//     #[derive(Debug, Error)]
//     pub enum Error {
//         #[error(display = "unknown")]
//         Unknown
//     }

//     impl Service<Request> for HelloWorld {
//         type Response = Response;
//         type Error = Error;
//         type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

//         fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//             Poll::Ready(Ok(()))
//         }

//         fn call(&mut self, req: Request) -> Self::Future {
//             async fn run2(hw: &mut HelloWorld, req: Request) -> Result<Response, Error> {
//                 hw.add(req).await;
//                 Ok(Response{})
//             }
            
//             run2(self, req).boxed()
//             // Box::pin(futures::future::ok(unimplemented!()))
//             // Box::pin(futures::future::ok(Response{}))
//         }
//     }
// }
