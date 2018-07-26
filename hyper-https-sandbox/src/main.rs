extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate flate2;


use std::collections::BTreeMap;
use futures::prelude::*;

fn main(){
    let https = ::hyper_tls::HttpsConnector::new(4).unwrap();
    let client = ::hyper::Client::builder().build::<::hyper_tls::HttpsConnector<::hyper::client::HttpConnector>, ::hyper::Body>(https);
    let mut map = BTreeMap::<&str, serde_json::Value>::new();
    map.insert("value1", serde_json::Value::String("メルキオール".to_string()));
    map.insert("value2", serde_json::Value::String("バルタザール".to_string()));
    map.insert("value3", serde_json::Value::String("カスパー".to_string()));
    let content = {
        use std::io::Write;
        use flate2::Compression;
        use flate2::write::GzEncoder;
        let content = serde_json::to_string(&map).unwrap();
        println!("{}", content);
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(&content.as_bytes()).unwrap();
        let content = e.finish().unwrap();
        content
    };
    let body = ::hyper::Body::from(content);
    let req = ::hyper::Request::post("https://maker.ifttt.com/trigger/..../with/key/...")
        .header("Content-Encoding", "gzip")
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap();
    let fut = client
        .request(req)
        .map(|o| { println!("{:?}", o); })
        .map_err(|e| { println!("err: {:?}", e); });
    println!("running");
    tokio::run(fut);
    println!("ok");
}