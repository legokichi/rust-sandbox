#![feature(async_await, async_closure)]
#![allow(unused_imports)]

use failure::ResultExt;
use futures::compat::Future01CompatExt as _;
use log::{info, error};
use serde::Deserialize;
use std::error::Error;
use structopt::StructOpt;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    root_ca_path: String,
    client_cert_path: String,
    private_key_path: String,
    thing_name: String,
    host: String,
}
#[derive(StructOpt, Debug)]
struct Opt {
    /// 262144 = 2^18 = 256 * 1024: rumqtt default max_packet_size
    /// 131072 = 2^17 = 128 * 1024: AWS IoT acceptable packet size
    #[structopt(short, long, default_value = "131072")]
    payload_size: usize,
    #[structopt(short, long, default_value = "1000")]
    wait_millis: u64,
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), failure::Error> {
    dotenv::dotenv().ok();
    env_logger::try_init()?;
    let config = envy::from_env::<Config>()?;
    let Opt{payload_size, wait_millis} = Opt::from_args();

    let (root_ca, client_cert, private_key) = futures::future::try_join3(
        tokio_fs::read(config.root_ca_path).compat(),
        tokio_fs::read(config.client_cert_path).compat(),
        tokio_fs::read(config.private_key_path).compat(),
    )
    .await?;

    let opt = rumqtt::MqttOptions::new(&config.thing_name, config.host, 443)
        .set_ca(root_ca)
        .set_client_auth(client_cert, private_key)
        .set_alpn(vec!["x-amzn-mqtt-ca".as_bytes().to_vec()])
        // .set_max_packet_size(128)
        .set_reconnect_opts(rumqtt::ReconnectOptions::Always(5));
    let (mut client, notifications) = rumqtt::MqttClient::start(opt)?;
    client.subscribe("legokichi/foo", rumqtt::QoS::AtMostOnce)?;
    std::thread::spawn(|| {
        for notification in notifications {
            match notification {
                rumqtt::client::Notification::Publish(_) => {
                    info!("recieved");
                    // std::process::exit(0);
                },
                rumqtt::client::Notification::Disconnection => {
                    error!("!!!Disconnection!!!");
                    // std::process::exit(1);
                },
                _ => {
                    info!("!!!NOTIFICATION!!!: {:?}", notification);
                }
            }
        }
    });
    loop {
        let o = vec![0_u8; payload_size];
        client.publish("legokichi/foo", rumqtt::QoS::AtMostOnce, false, o)?;
        info!("published");
        tokio_timer::sleep(std::time::Duration::from_millis(wait_millis))
            .compat()
            .await?;
    }
}
