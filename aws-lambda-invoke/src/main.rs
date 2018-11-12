extern crate rusoto_core;
extern crate rusoto_lambda;
extern crate tokio;
extern crate futures;
#[macro_use]
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate env_logger;

use rusoto_core::request::HttpClient;
use rusoto_core::credential::StaticProvider;
use rusoto_core::region::Region;
use rusoto_lambda::Lambda;
use rusoto_lambda::LambdaClient;
use rusoto_lambda::{InvocationRequest, ListFunctionsRequest};
use futures::prelude::*;
use futures::future;

fn main() {
    env_logger::init();
    ::tokio::run(future::lazy(||{
        let body_json = json!({});
        let body_json_str = ::serde_json::to_string(&body_json).unwrap();
        let body_base64: String = ::base64::encode(body_json_str.as_bytes());
        println!("{}", body_base64);
        let json = json!({
            "url":"https://example.com",
            "method":"get",
            "body": body_base64
        });
        let json_str = ::serde_json::to_string(&json).unwrap();
        println!("{}", json_str);
        let payload: Vec<u8> = json_str.as_bytes().to_vec();
        let client = HttpClient::new().unwrap();
        let provider = StaticProvider::new(
            "xxxxxx".to_string(),
            "xxxxxx".to_string(),
            None,
            None,
        );
        
        // let client = LambdaClient::new(Region::ApNortheast1);
        let client = LambdaClient::new_with(client, provider, Region::ApNortheast1);
        // let fut = client.list_functions(
        //     ListFunctionsRequest{
        //         ..Default::default()
        let fut = client.invoke(InvocationRequest{
            client_context: None,
            // function_name: "arn%253Aaws%253Alambda%253Aap-northeast-1%253Axxxxxx%253Afunction%253Axxxxxx-xxxxxx-xxxxxx-webhook-sender".to_string(),
            // function_name: "arn%3Aaws%3Alambda%3Aap-northeast-1%3Axxxxxx%3Afunction%3Axxxxxx-xxxxxx-xxxxxx-webhook-sender".to_string(),
            function_name: "arn:aws:lambda:ap-northeast-1:xxxxxx:function:xxxxxx-xxxxxx-xxxxxx-webhook-sender".to_string(),
            // invocation_type: None, 
            invocation_type: Some("RequestResponse".to_string()),
            log_type: None,
            // payload: None,
            payload: Some(payload),
            qualifier: None,
        }).map(|o|{
            println!("status_code: {:?}", o.status_code);
            println!("payload: {:?}", ::serde_json::from_slice::<serde_json::Value>(&o.payload.unwrap()));
        }).map_err(|err|{
            println!("err: {:?}", err);
        });
        fut
    }));
}
