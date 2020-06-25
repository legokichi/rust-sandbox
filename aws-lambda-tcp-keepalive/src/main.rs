use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::request::HttpClient;
use rusoto_core::region::Region;
use rusoto_core::credential::DefaultCredentialsProvider;
use hyper_tls::HttpsConnector;
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};

fn main() {
    std::env::set_var("RUST_LOG", "hyper=trace,hyper-tls=trace,rusoto_lambda=trace,rusoto_core=trace,lambda_runtime_client=trace");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    lambda!(handler);
}

fn handler(ev: serde_json::Value, _c: Context) -> Result<serde_json::Value, HandlerError> {
    println!("{:?}", ev);
    let mut rt = tokio::runtime::Runtime::new()
        .unwrap();
    let https_conn = HttpsConnector::new();
    rt.block_on(async move{
        let cred_provider = DefaultCredentialsProvider::new().unwrap();
        let region = Region::ApNortheast1;
        let http_client = HttpClient::from_connector(https_conn.clone());
        let client = LambdaClient::new_with(http_client, cred_provider, region);
        let req = serde_json::to_string(&serde_json::json!({})).unwrap();
        let req = InvocationRequest {
            function_name: "nop".to_string(),
            invocation_type: Some("RequestResponse".to_string()),
            payload: Some(req.into()),
            ..Default::default()
        };
        println!("{:?}", req);
        let resp = client.invoke(req).await.unwrap();
        println!("{:?}", resp);

        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;

        let cred_provider = DefaultCredentialsProvider::new().unwrap();
        let region = Region::ApNortheast1;
        let http_client = HttpClient::from_connector(https_conn.clone());
        let client = LambdaClient::new_with(http_client, cred_provider, region);
        let req = serde_json::to_string(&serde_json::json!({})).unwrap();
        let req = InvocationRequest {
            function_name: "nop".to_string(),
            invocation_type: Some("RequestResponse".to_string()),
            payload: Some(req.into()),
            ..Default::default()
        };
        println!("{:?}", req);
        let resp = client.invoke(req).await.unwrap();
        println!("{:?}", resp);

        tokio::time::delay_for(std::time::Duration::from_secs(91)).await;

        let cred_provider = DefaultCredentialsProvider::new().unwrap();
        let region = Region::ApNortheast1;
        let http_client = HttpClient::from_connector(https_conn.clone());
        let client = LambdaClient::new_with(http_client, cred_provider, region);
        let req = serde_json::to_string(&serde_json::json!({})).unwrap();
        let req = InvocationRequest {
            function_name: "nop".to_string(),
            invocation_type: Some("RequestResponse".to_string()),
            payload: Some(req.into()),
            ..Default::default()
        };
        println!("{:?}", req);
        let resp = client.invoke(req).await.unwrap();
        println!("{:?}", resp);
    });
    Ok(serde_json::json!({
        "type": "ok"
    }))
}
