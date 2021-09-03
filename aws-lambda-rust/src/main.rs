use std::task::Poll;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDbClient, DynamoDb};
use aws_lambda_events::event::kinesis::KinesisEvent;
use tower::Service;
use tower::Layer;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    env_logger::init();
    let env = envy::from_env::<Config>().unwrap();
    let client = Client::new(env);
    let layer = tower::layer::layer_fn(|service| {
        LogService {
            service
        }
    });
    lambda_runtime::run(lambda_runtime::handler_fn(move |e, _|{
        layer.layer(client.clone()).call(e)
    })).await?;
    Ok(())
}

#[derive(serde::Deserialize, Debug, Clone)]
struct Config {
    permanent_table_name: String,
    log_table_name: String,
}

#[derive(Clone)]
struct Client {
    env: Config,
    client: DynamoDbClient,
}
impl Client {
    pub fn new(env: Config) -> Self {
        let client = DynamoDbClient::new(Region::default());
        Self{
            env,
            client
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, smart_default::SmartDefault, Clone)]
pub struct Request{
}
#[derive(serde::Deserialize, serde::Serialize, Debug, smart_default::SmartDefault, Clone)]
pub struct Response{
}
impl tower::Service<Request> for Client {
    type Response = Response;
    type Error = void::Void;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send + Sync + 'static>>;
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn call(&mut self, Request{}: Request) -> Self::Future {
        Box::pin(async {
            Ok(Response{})
        })
    }
}

pub struct LogService<S> {
    service: S,
}

impl<S, Req> Service<Req> for LogService<S>
where
    S: Service<Req>,
    Req: Clone + std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize + Sync + Send + 'static,
    S::Response: Clone + std::fmt::Debug + serde::Serialize + Sync + Send + 'static,
    S::Error: std::error::Error + std::fmt::Display + std::fmt::Debug + Sync + Send + 'static,
    S::Future: Sync + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send + Sync + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        log::debug!("request: {:?}", req);
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await;
            log::debug!("response: {:?}", res);
            res
        })
    }
}

// pub struct LogService<S> {
//     service: S,
// }

// impl<S> Service<S::Request> for LogService<S>
// where
//     S: Service<Request>
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = S::Future;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }

//     fn call(&mut self, request: KinesisEvent) -> Self::Future {
//         // KinesisEvent::int
//         let request: Request = unimplemented!();
//         self.service.call(request)
//     }
// }
