#![feature(async_await)]
#![allow(unused_imports)]

use failure::ResultExt;
use serde::Deserialize;
use futures::compat::Future01CompatExt as _;
use std::error::Error;
use log::info;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    root_ca_path: String,
    client_cert_path: String,
    private_key_path: String,
    thing_name: String,
    host: String,
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), failure::Error> {
    dotenv::dotenv().ok();
    env_logger::try_init()?;
    let config = envy::from_env::<Config>()?;
    let root_ca = tokio_fs::read(config.root_ca_path).compat().await?;
    let client_cert = tokio_fs::read(config.client_cert_path).compat().await?;
    let private_key = tokio_fs::read(config.private_key_path).compat().await?;
    let opt = rumqtt::MqttOptions::new(&config.thing_name, config.host, 443)
        .set_connection_method(rumqtt::ConnectionMethod::Tls {
            ca: root_ca,
            cert_and_key: Some((client_cert, private_key)),
            alpn: vec!["x-amzn-mqtt-ca".to_string()]
        })
        .set_reconnect_opts(rumqtt::ReconnectOptions::Always(5));
    let (mut client, notifications) = rumqtt::MqttClient::start(opt)?;
    let th = std::thread::spawn(||{
        for notification in notifications {
            info!("!!!NOTIFICATION!!!: {:?}", notification);
        }
    });
    client.subscribe("legokichi/foo", rumqtt::QoS::AtMostOnce)?;
    // loop{
    //     client.publish("legokichi/foo", rumqtt::QoS::AtMostOnce, false, "hello")?;
    //     tokio_timer::sleep(std::time::Duration::from_secs(60*60)).compat().await?;
    // }
    th.join().unwrap();
    Ok(())
}

//  cargo watch -x "run --release" 2>&1 | tee log.txt
// aws iot describe-endpoint --endpoint-type iot:Data-ATS
