
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate futures;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate tokio;
extern crate bytes;

use futures::prelude::*;
use futures::future;
use mdo_future::future::*;
use actix::prelude::*;
use actix_web::dev::*;
use actix_web::http::{ header, Method };
use actix_web::{ http, pred, App };
use actix_web::{ HttpRequest, HttpResponse, HttpContext };
struct EventSource{
    counter: usize
}
impl EventSource {
    fn new() -> Self {
        Self { counter: 5 }
    }
}
impl Stream for EventSource {
    type Item = bytes::Bytes;
    type Error = actix_web::Error;
    // https://developer.mozilla.org/ja/docs/Server-sent_events/Using_server-sent_events#Event_stream_format
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        /*
        Ok(Async::NotReady), // pending
        Ok(Async::Ready(Some(bytes::Bytes::from(payload)))), // has value
        Err(_) => Ok(Async::Ready(None)) // finish
        */
        if self.counter == 0 {
            return Ok(Async::Ready(None));
        }
        let payload = format!("data: {}\n\n", self.counter);
        self.counter -= 1;
        Ok(Async::Ready(Some(bytes::Bytes::from(payload))))
    }
}


fn main() {
    ::std::env::set_var("RUST_LOG", "info,actix_web=trace");
    ::std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let sys = System::new("system");
    let server = actix_web::server::new(||{
        let handle = ::actix::Arbiter::handle();
        let mut app = App::new();
        app = app.route("/", Method::GET, move |_req: HttpRequest| -> HttpResponse {
            HttpResponse::Ok().body(r#"
<script>
window.es = new EventSource("/sse");
es.onmessage = console.log.bind(console);
</script>
            "#)
        });
        app = app.route("/sse", Method::GET, move |req: HttpRequest| -> HttpResponse {
            println!("sse");
            HttpResponse::build(http::StatusCode::OK)
                .content_encoding(header::ContentEncoding::Identity)
                .no_chunking()
                .force_close()
                .content_type("text/event-stream")
                .streaming(EventSource::new())
        });
        app
    });
    server
        .bind(format!("{}:{}", "localhost", "8080"))
        .expect(&format!("Can not bind to {}:{}", "localhost", "8080"))
        .start();
    let _ = sys.run();
}
