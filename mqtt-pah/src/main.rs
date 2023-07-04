#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Hello, world!");
    let (tx, rx) = mqtt_connect("127.0.0.1:1883", "rust", vec![]).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(100)).await;
}

//static AMAZON_ROOT_CA1_PEM: &str = r###"-----BEGIN CERTIFICATE-----
//MIIDQTCCAimgAwIBAgITBmyfz5m/jAo54vB4ikPmljZbyjANBgkqhkiG9w0BAQsF
//ADA5MQswCQYDVQQGEwJVUzEPMA0GA1UEChMGQW1hem9uMRkwFwYDVQQDExBBbWF6
//b24gUm9vdCBDQSAxMB4XDTE1MDUyNjAwMDAwMFoXDTM4MDExNzAwMDAwMFowOTEL
//MAkGA1UEBhMCVVMxDzANBgNVBAoTBkFtYXpvbjEZMBcGA1UEAxMQQW1hem9uIFJv
//b3QgQ0EgMTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALJ4gHHKeNXj
//ca9HgFB0fW7Y14h29Jlo91ghYPl0hAEvrAIthtOgQ3pOsqTQNroBvo3bSMgHFzZM
//9O6II8c+6zf1tRn4SWiw3te5djgdYZ6k/oI2peVKVuRF4fn9tBb6dNqcmzU5L/qw
//IFAGbHrQgLKm+a/sRxmPUDgH3KKHOVj4utWp+UhnMJbulHheb4mjUcAwhmahRWa6
//VOujw5H5SNz/0egwLX0tdHA114gk957EWW67c4cX8jJGKLhD+rcdqsq08p8kDi1L
//93FcXmn/6pUCyziKrlA4b9v7LWIbxcceVOF34GfID5yHI9Y/QCB/IIDEgEw+OyQm
//jgSubJrIqg0CAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAOBgNVHQ8BAf8EBAMC
//AYYwHQYDVR0OBBYEFIQYzIU07LwMlJQuCFmcx7IQTgoIMA0GCSqGSIb3DQEBCwUA
//A4IBAQCY8jdaQZChGsV2USggNiMOruYou6r4lK5IpDB/G/wkjUu0yKGX9rbxenDI
//U5PMCCjjmCXPI6T53iHTfIUJrU6adTrCC2qJeHZERxhlbI1Bjjt/msv0tadQ1wUs
//N+gDS63pYaACbvXy8MWy7Vu33PqUXHeeE6V/Uq2V8viTO96LXFvKWlJbYK8U90vv
//o/ufQJVtMVT8QtPHRh8jrdkPSHCa2XV4cdFyQzR1bldZwgJcJmApzyMZFo6IQ6XU
//5MsI+yMRQ+hDKXJioaldXgjUkK642M4UwtBV8ob2xJNDd2ZhwLnoQdeXeGADbkpy
//rqXRfboQnoZsG4q5WTP468SQvvG5
//-----END CERTIFICATE-----
//"###;

#[cfg(feature = "paho")]
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
    ),
    anyhow::Error,
> {
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

    let mut strm = client.get_stream(25);
    client.connect(conn_opts).await?;

    for topic in subscribes {
        client.subscribe(topic.clone(), 1).await?;
    }
    let _fut = tokio::spawn({
        let client = client.clone();
        async move {
        use futures::stream::StreamExt;
            while let Some((topic, payload)) = rx.next().await {
                log::debug!("{thing_name}:{topic}:{payload:?}");
                let msg = paho_mqtt::Message::new(topic, payload, 1);
                while let Err(e) = client.try_publish(msg.clone()) {
                    log::warn!("{}: publish error = {:?}", thing_name, e);
                }
            }
            client.disconnect(None).await.unwrap();
        }
    });
    let (mut tx2, rx2) = futures::channel::mpsc::channel::<(String, Vec<u8>)>(100);
    let _fut2 = tokio::spawn(async move {
        loop {
            match strm.recv().await {
                Ok(Some(msg)) =>{
                    use futures::SinkExt;
                    tx2.send((msg.topic().to_string(), msg.payload().to_vec()))
                        .await
                        .unwrap();
                },
                Ok(None) => {
                    // tx channel dropped or tcp connection disconnected
                    log::debug!("disconnected");
                    // A "None" means we were disconnected. Try to reconnect...
                    log::warn!("reconnecting");
                    loop {
                        match client.reconnect().await {
                            Ok(resp)=>{
                                log::info!("reconnect succ: {resp:?}");
                                break;
                            },
                            Err(err)=>{
                                log::warn!("Error reconnecting: {:?}", err);
                                continue;
                            },
                        }
                    }
                },
                Err(err) => {
                    panic!("{err:?}");
                },
            }
        }
    });
    Ok((tx, Box::pin({
        use futures::stream::StreamExt;
        rx2.map(Ok)
    })))
}

#[cfg(feature = "rumqtt")]
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
    ),
    anyhow::Error,
> {
    let mut mqttoptions = rumqttc::MqttOptions::new(thing_name, data_endpoint.to_string(), 443);
    //let mut cert = std::io::Cursor::new(certificate_pem.as_bytes().to_vec());
    //let mut key = std::io::Cursor::new(pkey_pkcs1_pem.as_bytes().to_vec());
    //let mut ca_cert = std::io::Cursor::new(AMAZON_ROOT_CA1_PEM.as_bytes().to_vec());
    //let keys = rustls_pemfile::rsa_private_keys(&mut key).unwrap();
    //let certs = rustls_pemfile::certs(&mut cert).unwrap();
    //let ca_certs = rustls_pemfile::certs(&mut ca_cert).unwrap();
    //let key = keys.into_iter().map(rustls::PrivateKey).next().unwrap();
    //let cert = certs
    //    .into_iter()
    //    .map(rustls::Certificate)
    //    .collect::<Vec<_>>();
    //let ca_cert = {
    //    let ca_cert = rustls::Certificate(ca_certs.into_iter().next().unwrap());
    //    let mut ca_cert_ = rustls::RootCertStore::empty();
    //    ca_cert_.add(&ca_cert).unwrap();
    //    ca_cert_
    //};
    let mut conf = rumqttc::tokio_rustls::rustls::ClientConfig::builder()
        .with_safe_defaults()
    //    .with_root_certificates(ca_cert)
    //    .with_single_cert(cert, key)?;
        ;
    conf.alpn_protocols = vec![b"x-amzn-mqtt-ca".to_vec()];
    mqttoptions.set_transport(rumqttc::Transport::Tls(conf.into()));
    // この数字は subscribe 数より十分に大きくないといけない
    let (client, mut eventloop) = rumqttc::AsyncClient::new(mqttoptions, 100);
    let (tx, mut rx) = futures::channel::mpsc::channel::<(String, Vec<u8>)>(100);
    let thing_name = thing_name.to_string();
    for topic in subscribes {
        client
            .subscribe(topic.clone(), rumqttc::QoS::AtMostOnce)
            .await?;
    }
    use futures::StreamExt;
    let _fut = tokio::spawn(async move {
        while let Some((topic, payload)) = rx.next().await {
            log::debug!("{thing_name}:{topic}:{payload:?}");
            while let Err(e) = client
                .publish(
                    topic.clone(),
                    rumqttc::QoS::AtLeastOnce,
                    false,
                    payload.clone(),
                )
                .await
            {
                log::warn!("{}: publish error = {:?}", thing_name, e);
            }
        }
        // tx channel dropped
        client.disconnect().await.unwrap();
    });
    let (mut tx2, rx2) = futures::channel::mpsc::channel::<(String, Vec<u8>)>(100);
    let _fut2 = tokio::spawn(async move {
        use futures::SinkExt;
        loop {
            let o = eventloop.poll().await;
            log::debug!("{o:?}");
            #[allow(clippy::single_match)]
            match o {
                Ok(rumqttc::Event::Incoming(rumqttc::Incoming::Publish(
                    rumqttc::v4::Publish { payload, topic, .. },
                ))) => {
                    tx2.send((topic.clone(), payload.to_vec())).await.unwrap();
                }
                Ok(rumqttc::Event::Outgoing(rumqttc::Outgoing::Disconnect)) => {
                    log::debug!("disconnected");
                    break;
                }
                _ => {}
            }
        }
    });
    // fut2 内での mqtt connect と subscribe 待つ
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    Ok((tx, Box::pin(rx2.map(Ok))))
}
