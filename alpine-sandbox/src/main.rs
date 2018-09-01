extern crate futures;
extern crate tokio;
extern crate hyper;
#[cfg(feature = "openssl")]
extern crate hyper_tls;
#[cfg(feature = "rustls")]
extern crate hyper_rustls;

use futures::prelude::*;

fn main() {
    let mut runtime = ::tokio::runtime::current_thread::Runtime::new().unwrap();
    #[cfg(feature = "openssl")]
    let https_connector = ::hyper_tls::HttpsConnector::new(1).unwrap();
    #[cfg(feature = "rustls")]
    let https_connector = ::hyper_rustls::HttpsConnector::new(1);

    let client = ::hyper::Client::builder().build::<_, ::hyper::Body>(https_connector);
    let res = runtime.block_on(client.get("https://hyper.rs/".parse().unwrap())).unwrap();
    println!("{:?}", res);
}
