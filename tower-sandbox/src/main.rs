


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use hello_world_service::*;
    let mut o = HelloWorld{ count: 0 };
    o.add(Request{}).await;
    println!("Hello, world!");
    Ok(())
}

mod hello_world_service {
    use tower_service::Service;
    use std::task::{Context, Poll};
    use futures::prelude::*;
    use err_derive::Error;
    use std::pin::Pin;

    #[derive(Debug)]
    pub struct HelloWorld{
        pub count: i64,
    }
    impl HelloWorld {
        pub async fn add(&mut self, req: Request) -> Result<Response, Error> {
            self.count += 1;
            futures::future::ready(()).await;
            self.count += 1;
            futures::future::ready(()).await;
            Ok(Response{})
        }
    }
    pub struct Request {}
    pub struct Response {}
    
    #[derive(Debug, Error)]
    pub enum Error {
        #[error(display = "unknown")]
        Unknown
    }

    impl Service<Request> for HelloWorld {
        type Response = Response;
        type Error = Error;
        type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: Request) -> Self::Future {
            async fn run2(hw: &mut HelloWorld, req: Request) -> Result<Response, Error> {
                hw.add(req).await;
                Ok(Response{})
            }
            
            run2(self, req).boxed()
            // Box::pin(futures::future::ok(unimplemented!()))
            // Box::pin(futures::future::ok(Response{}))
        }
    }
}
