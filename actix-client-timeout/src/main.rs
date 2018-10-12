use futures::prelude::*;
use actix_web::client::ClientRequest;
use actix_web::error::ResponseError;

fn main() {
    ::env_logger::init();
    ::actix::run(||{
        ClientRequest::get("http://localhost:8888")
            .timeout(::std::time::Duration::from_secs(10))
            .finish().unwrap()
            .send()
            .map(|res|{
                println!("res: {:?}", res);
            })
            .map_err(|err|{
                println!("err: {:?}", err);
                println!("res: {:?}", err.error_response());
            })
    });
}
