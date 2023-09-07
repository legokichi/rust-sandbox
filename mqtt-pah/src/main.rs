#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Hello, world!");
    let (tx, rx, fut) = mqtt_connect("127.0.0.1:1883", "rust", vec![])
        .await
        .unwrap();
    tokio::spawn(fut);
    tokio::time::sleep(std::time::Duration::from_secs(100)).await;
}

#[allow(clippy::type_complexity)]
pub async fn mqtt_connect(
    data_endpoint: &str,
    thing_name: &str,
    //certificate_pem: &str,
    //pkey_pkcs1_pem: &str,
    subscribes: Vec<String>,
) -> Result<
    (
        futures::channel::mpsc::Sender<(String, Vec<u8>)>,
        futures::stream::BoxStream<'static, Result<(String, Vec<u8>), anyhow::Error>>,
        futures::future::BoxFuture<'static, ()>,
    ),
    anyhow::Error,
> {
    use futures::stream::StreamExt;
    use futures::FutureExt;
    unsafe extern "C" fn on_trace_callback(
        level: paho_mqtt_sys::MQTTASYNC_TRACE_LEVELS,
        message: *mut std::os::raw::c_char,
    ) {
        // MQTTASYNC_TRACE_MAXIMUM  = 1,
        // MQTTASYNC_TRACE_MEDIUM   = 2,
        // MQTTASYNC_TRACE_MINIMUM  = 3,
        // MQTTASYNC_TRACE_PROTOCOL = 4,
        // MQTTASYNC_TRACE_ERROR    = 5,
        // MQTTASYNC_TRACE_SEVERE   = 6,
        // MQTTASYNC_TRACE_FATAL    = 7,
        let msg = std::ffi::CStr::from_ptr(message);
        let msg = msg
            .to_str()
            .unwrap_or_else(|e| panic!("invalid msg: {msg:?}, {e:?}"));
        if level <= 3 {
            log::trace!("{msg}");
        } else if level <= 4 {
            log::debug!("{msg}");
        } else if level <= 5 {
            log::info!("{msg}");
        } else if level <= 6 {
            log::warn!("{msg}");
        } else if level <= 7 {
            log::error!("{msg}");
        }
    }
    unsafe {
        paho_mqtt_sys::MQTTAsync_setTraceCallback(Some(on_trace_callback));
    }
    //use std::io::Write;
    //use tempfile::NamedTempFile;
    //let mut cert_file = NamedTempFile::new()?;
    //cert_file.write_all(certificate_pem.as_bytes())?;
    //let mut private_file = NamedTempFile::new()?;
    //private_file.write_all(pkey_pkcs1_pem.as_bytes())?;
    //let mut ca_file = NamedTempFile::new()?;
    //ca_file.write_all(AMAZON_ROOT_CA1_PEM.as_bytes())?;
    let mut client = paho_mqtt::CreateOptionsBuilder::new()
        //.server_uri(format!("ssl://{data_endpoint}:443"))
        .server_uri(format!("mqtt://{data_endpoint}"))
        .client_id(thing_name)
        .persistence(paho_mqtt::PersistenceType::None)
        .mqtt_version(4)
        .max_buffered_messages(100)
        .create_client()?;

    //let ssl_opts = paho_mqtt::SslOptionsBuilder::new()
    //    .key_store(cert_file.path())?
    //    .private_key(private_file.path())?
    //    .ssl_version(paho_mqtt::SslVersion::Tls_1_2)
    //    .verify(true)
    //    .trust_store(ca_file.path())?
    //    .alpn_protos(&["x-amzn-mqtt-ca"])
    //    .finalize();
    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        //    .ssl_options(ssl_opts)
        .finalize();
    let (tx, mut rx) = futures::channel::mpsc::channel::<(String, Vec<u8>)>(100);
    let thing_name = thing_name.to_string();

    let strm = client.get_stream(25);
    client.connect(conn_opts).await?;
    log::info!("=======");
    if let Err(e) = client.reconnect().await {
        dbg!(e);
    }

    for topic in subscribes {
        client.subscribe(topic.clone(), 1).await?;
    }
    let fut1 = {
        let client = client.clone();
        async move {
            while let Some((topic, payload)) = rx.next().await {
                log::debug!("{thing_name}:{topic}:{payload:?}");
                let msg = paho_mqtt::Message::new(topic, payload, 1);
                while let Err(e) = client.try_publish(msg.clone()) {
                    log::warn!("{thing_name}: publish error = {e:?}");
                }
            }
            // tx ga drop sareta node channel wo close suru
            client.disconnect(None).await.unwrap();
        }
    };
    let (mut tx2, rx2) = futures::channel::mpsc::channel::<(String, Vec<u8>)>(100);
    let fut2 = async move {
        loop {
            match strm.recv().await {
                Ok(Some(msg)) => {
                    use futures::SinkExt;
                    tx2.send((msg.topic().to_string(), msg.payload().to_vec()))
                        .await
                        .unwrap();
                }
                Ok(None) => {
                    // tx channel dropped or tcp connection disconnected
                    log::info!("disconnected");
                    // A "None" means we were disconnected. Try to reconnect...
                    loop {
                        log::info!("reconnecting");
                        match client.reconnect().await {
                            Ok(resp) => {
                                log::info!("reconnect succ: {resp:?}");
                                break;
                            }
                            Err(err) => {
                                log::warn!("Error reconnecting: {:?}", err);
                                continue;
                            }
                        }
                    }
                }
                Err(err) => {
                    // paho client error
                    panic!("unexpected err: {err:?}");
                }
            }
        }
    };

    Ok((
        tx,
        rx2.map(Ok).boxed(),
        futures::future::join(fut1, fut2).map(|_| ()).boxed(),
    ))
}
