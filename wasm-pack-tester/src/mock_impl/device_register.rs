use crate::model::device_register::*;
use crate::mock_impl::client::*;

use tower_service::Service;
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

impl Service<Request> for Client {
    type Response = Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unimplemented!()
    }
    fn call(&mut self, _req: Request) -> Self::Future {
        unimplemented!()
    }
}
