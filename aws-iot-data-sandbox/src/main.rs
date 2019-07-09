#![feature(async_await)]

use futures::compat::Future01CompatExt as _;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use rusoto_core::region::Region;
use rusoto_core::request::HttpClient;
use rusoto_core::DefaultCredentialsProvider;
use rusoto_iot_data::{IotData, IotDataClient, PublishRequest};
use rustls::{internal::pemfile, ClientConfig};
use serde::Deserialize;
use std::io::Cursor;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    root_ca_path: String,
    client_cert_path: String,
    private_key_path: String,
    host: String,
    aws_region: String,
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), failure::Error> {
    dotenv::dotenv()?;
    env_logger::try_init()?;
    let config = envy::from_env::<Config>()?;

    let root_ca = tokio_fs::read(config.root_ca_path).compat().await?;
    let client_cert = tokio_fs::read(config.client_cert_path).compat().await?;
    let private_key = tokio_fs::read(config.private_key_path).compat().await?;
    let region = Region::Custom {
        name: config.aws_region,
        endpoint: config.host,
    };
    let certs = pemfile::certs(&mut Cursor::new(client_cert)).unwrap();
    let keys = pemfile::rsa_private_keys(&mut Cursor::new(private_key)).unwrap();
    let mut config = ClientConfig::new();
    config.set_single_client_cert(certs, keys[0].clone());
    config
        .root_store
        .add_pem_file(&mut std::io::Cursor::new(root_ca))
        .unwrap();
    let mut http = HttpConnector::new(4);
    http.enforce_http(false);
    let https_connector = HttpsConnector::from((http, config));
    let client = HttpClient::from_connector(https_connector);
    let iotdata = IotDataClient::new_with(client, DefaultCredentialsProvider::new()?, region);
    iotdata
        .publish(PublishRequest {
            topic: "legokichi/foo".to_string(),
            payload: Some("hello".to_string().into()),
            qos: None,
        })
        .compat()
        .await?;
    Ok(())
}
