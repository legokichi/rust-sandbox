extern crate tokio; // 0.1.13
extern crate hyper; // 0.12.20

use tokio::prelude::*;
use tokio::runtime::current_thread::Runtime;
use hyper::Client;
use std::str::from_utf8;

fn main() {
    let mut rt = Runtime::new().unwrap();
    let client = Client::new();

    let res = rt.block_on(client.get("http://localhost:8080/".parse().unwrap())).unwrap();
    let mut body: hyper::Body = res.into_body();
    let buf = rt.block_on(body.by_ref().take_while(|bytes|{ println!("{:?}", bytes); Ok(true)}).concat2()).unwrap();
    println!("buf2: {:?}", buf);
}

