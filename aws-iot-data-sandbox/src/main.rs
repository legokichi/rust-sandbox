use chrono::{DateTime, Utc};
use futures::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    root_ca_path: String,
    client_cert_path: String,
    private_key_path: String,
    data_endpoint: String,
    credentials_endpoint: String,
    roll_alias_name: String,
    aws_region: String,
    thing_name: String,
}

#[derive(Deserialize, Debug, Clone)]
enum Credentials {
    #[serde(rename = "credentials", rename_all = "camelCase")]
    Credentials {
        access_key_id: String,
        secret_access_key: String,
        session_token: String,
        expiration: DateTime<Utc>,
    },
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    dotenv::dotenv()?;
    env_logger::try_init()?;

    let Config {
        root_ca_path,
        client_cert_path,
        private_key_path,
        aws_region,
        data_endpoint,
        credentials_endpoint,
        roll_alias_name,
        thing_name,
    } = envy::from_env::<Config>()?;

    let mut cert_config = {
        use failure::err_msg;

        use rustls::{internal::pemfile, ClientConfig};
        use std::io::Cursor;

        let root_ca = std::fs::read(root_ca_path)?;
        let client_cert = std::fs::read(client_cert_path)?;
        let private_key = std::fs::read(private_key_path)?;
        let certs = pemfile::certs(&mut Cursor::new(client_cert)).map_err(|()| err_msg("certs"))?;
        let keys = pemfile::rsa_private_keys(&mut Cursor::new(private_key))
            .map_err(|()| err_msg("rsa_private_keys"))?;
        let mut config = ClientConfig::new();
        config.set_single_client_cert(certs, keys[0].clone());
        config
            .root_store
            .add_pem_file(&mut Cursor::new(root_ca))
            .map_err(|()| err_msg("root_store"))?;
        config
    };

    {
        println!("credentials_endpoint");
        use failure::SyncFailure;
        use hyper::client::HttpConnector;
        use hyper_rustls::HttpsConnector;

        let mut http_connector = HttpConnector::new();
        http_connector.enforce_http(false);
        let signed_https_connector = HttpsConnector::from((http_connector, cert_config.clone()));

        let credentials_url = format!(
            "https://{}/role-aliases/{}/credentials",
            credentials_endpoint, roll_alias_name
        )
        .parse::<hyper::Uri>()?;
        let signed_client =
            hyper::Client::builder().build::<_, hyper::Body>(signed_https_connector);
        let req = hyper::Request::get(credentials_url)
            .header("x-amzn-iot-thingname", &thing_name)
            .body(hyper::Body::empty())?;
        println!("{:?}", req);
        let res = signed_client.request(req).map_err(SyncFailure::new).await?;
        println!("{:?}", res);
        let body = res
            .into_body()
            .try_concat()
            .map_err(SyncFailure::new)
            .await?;
        let body = String::from_utf8(body.to_vec())?;
        println!("{}", body);
        let cred = serde_json::from_str::<Credentials>(&body)?;

        use failure::err_msg;
        use rusoto_core::region::Region;
        use rusoto_core::request::HttpClient;
        use rusoto_credential::StaticProvider;
        use rusoto_iot_data::{
            GetThingShadowRequest, GetThingShadowResponse, IotData, IotDataClient,
        };

        let region = Region::Custom {
            name: aws_region.clone(),
            endpoint: data_endpoint.clone(),
        };
        let cred = match cred {
            Credentials::Credentials {
                access_key_id,
                secret_access_key,
                session_token,
                expiration,
            } => StaticProvider::new(
                access_key_id,
                secret_access_key,
                Some(session_token),
                Some(expiration.timestamp_millis()),
            ),
        };
        let rusoto_http_client = HttpClient::new()?;
        let iotdata = IotDataClient::new_with(rusoto_http_client, cred, region);
        let GetThingShadowResponse { payload } = iotdata
            .get_thing_shadow(GetThingShadowRequest {
                thing_name: thing_name.clone(),
            })
            .sync()?; // rusoto の内部の hyper が tokio 0.2 に対応してないので await できない
        let shadow = payload.ok_or_else(|| err_msg("payload"))?;
        let shadow = String::from_utf8(shadow.to_vec())?;
        println!("{}", shadow);
    }

    {
        println!("alpn x-amzn-http-ca");
        use failure::SyncFailure;
        use hyper::client::HttpConnector;
        use hyper_rustls::HttpsConnector;

        cert_config.set_protocols(&[b"x-amzn-http-ca".to_vec()]);
        let mut http_connector = HttpConnector::new();
        http_connector.enforce_http(false);
        let signed_https_connector = HttpsConnector::from((http_connector, cert_config));

        let shadow_url = format!("https://{}/things/{}/shadow", data_endpoint, thing_name)
            .parse::<hyper::Uri>()?;
        let signed_client =
            hyper::Client::builder().build::<_, hyper::Body>(signed_https_connector.clone());
        let req = hyper::Request::get(shadow_url)
            .body(hyper::Body::empty())?;
        println!("{:?}", req);
        let res = signed_client.request(req).map_err(SyncFailure::new).await?;
        println!("{:?}", res);
        let body = res
            .into_body()
            .try_concat()
            .map_err(SyncFailure::new)
            .await?;
        let body = String::from_utf8(body.to_vec())?;
        println!("{}", body);
    }

    Ok(())
}
