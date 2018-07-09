extern crate hyper; // 0.12.5
extern crate serde_json; // 1.0.22
extern crate futures; // 0.1.21
use futures::prelude::*;
use std::borrow::Cow;
use std::collections::BTreeMap;

fn main(){
    let client = hyper::Client::new();
    let mut req = hyper::Request::post("http://example.com");
    let mut map = BTreeMap::<&str, Cow<serde_json::Value>>::new();
    let body = hyper::Body::from(serde_json::to_string(&map).unwrap());
    let req = req.body(body).unwrap();
    let fut = client.request(req);
    fut.wait();
}