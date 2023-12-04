//#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    mqtt_log_init();
    env_logger::init();

    log::info!("connecting");
    let mut client = mqtt_connect()
        .await
        .unwrap();
    log::info!("connected");

    let mut strm = client.get_stream(25);
    let fut = async {
        client.subscribe("hello", 1).await.unwrap();
        loop{
            //let payload = make_payload();
            //let topic = format!("$aws/rules/actcast_dev_iot_syslog_rule/things/{THING_NAME}/syslog");
            //let msg = paho_mqtt::Message::new(topic, payload, 1);
            //if let Err(e) = client.try_publish(msg) {
            //    log::warn!("publish error = {:?}", e);
            //}
            //log::info!("done");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    };
    let fut2 = async {
        use futures::stream::StreamExt;
        while let Some(msg_opt) = strm.next().await {
            match msg_opt {
                Some(msg) => {
                    log::info!("{}:{:?}", msg.topic(), msg.qos());
                }
                None => {
                    // A "None" means we were disconnected.
                    // tx channel dropped or tcp connection disconnected
                    log::warn!("disconnected");
                    log::info!("is_connected: {}", client.is_connected());
                    log::warn!("reconnecting");
                    if let Err(err) = client.reconnect().await {
                        log::warn!("Error reconnecting: {:?}", err);
                        break;
                    }
                    log::warn!("reconnected");
                }
            }
        }
        log::warn!("client dropped");
    };
    tokio::select!{
        _ = fut => {},
        _ = fut2 => {},
    }
}

fn make_payload() -> Vec<u8> {
    let a = [0; 1000];
    a.to_vec()
}

fn make_payload_iot() -> Vec<u8> {
    let mut payload = Vec::new();
    for _ in 0..131073_usize {
        payload.push(0);
    }
    payload
}
pub async fn mqtt_connect(
) -> Result<paho_mqtt::AsyncClient, anyhow::Error> {
    let client = paho_mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("tcp://127.0.0.1:1883"))
        .client_id("hello")
        .persistence(paho_mqtt::PersistenceType::None)
        .mqtt_version(0)
        .max_buffered_messages(100)
        .create_client()?;
    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .finalize();
    client.connect(conn_opts).await?;
    Ok(client)

}

pub async fn mqtt_connect_test(
) -> Result<paho_mqtt::AsyncClient, anyhow::Error> {
    let client = paho_mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("tcp://test.mosquitto.org:1883"))
        .client_id("hello")
        .persistence(paho_mqtt::PersistenceType::None)
        .mqtt_version(4)
        .max_buffered_messages(100)
        .create_client()?;
    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .finalize();
    client.connect(conn_opts).await?;
    Ok(client)

}
pub async fn mqtt_connect_iot(
) -> Result<paho_mqtt::AsyncClient, anyhow::Error> {
    let data_endpoint: &str = DATA_ENDPOINT;
    let thing_name: &str = THING_NAME;
    let certificate_pem: &str = CERTIFICATE_PEM;
    let pkey_pkcs1_pem: &str = PRIVATE_KEY;
    use std::io::Write;
    use tempfile::NamedTempFile;
    let mut cert_file = NamedTempFile::new()?;
    cert_file.write_all(certificate_pem.as_bytes())?;
    let mut private_file = NamedTempFile::new()?;
    private_file.write_all(pkey_pkcs1_pem.as_bytes())?;
    let mut ca_file = NamedTempFile::new()?;
    ca_file.write_all(AMAZON_ROOT_CA1_PEM.as_bytes())?;
    let client = paho_mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("ssl://{data_endpoint}:443"))
        .client_id(thing_name)
        .persistence(paho_mqtt::PersistenceType::None)
        .mqtt_version(4)
        .max_buffered_messages(100)
        .create_client()?;
    let ssl_opts = paho_mqtt::SslOptionsBuilder::new()
        .key_store(cert_file.path())?
        .private_key(private_file.path())?
        .ssl_version(paho_mqtt::SslVersion::Tls_1_2)
        .verify(true)
        .trust_store(ca_file.path())?
        .alpn_protos(&["x-amzn-mqtt-ca"])
        .finalize();
    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .ssl_options(ssl_opts)
        .finalize();
    client.connect(conn_opts).await?;
    Ok(client)
}

async fn subscribe(
    mut client: paho_mqtt::AsyncClient,
    thing_name: &str,
    subscribes: Vec<String>,
) -> Result<
    (
        futures::channel::mpsc::Sender<(String, Vec<u8>)>,
        futures::stream::BoxStream<'static, Result<(String, Vec<u8>), anyhow::Error>>,
    ),
    anyhow::Error,
> {
    let thing_name = thing_name.to_string();
    let (tx, mut rx) = futures::channel::mpsc::channel::<(String, Vec<u8>)>(100);
    let mut strm = client.get_stream(25);

    for topic in subscribes {
        client.subscribe(topic.clone(), 1).await?;
    }
    use futures::stream::StreamExt;
    let _fut = tokio::spawn({
        let client = client.clone();
        async move {
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
        use futures::SinkExt;
        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                tx2.send((msg.topic().to_string(), msg.payload().to_vec()))
                    .await
                    .unwrap();
            } else {
                // tx channel dropped or tcp connection disconnected
                log::debug!("disconnected");
                // A "None" means we were disconnected. Try to reconnect...
                log::warn!("reconnecting");
                while let Err(err) = client.reconnect().await {
                    log::warn!("Error reconnecting: {:?}", err);
                }
            }
        }
    });
    Ok((tx, Box::pin(rx2.map(Ok))))
}


static AMAZON_ROOT_CA1_PEM: &str = r###"-----BEGIN CERTIFICATE-----
MIIDQTCCAimgAwIBAgITBmyfz5m/jAo54vB4ikPmljZbyjANBgkqhkiG9w0BAQsF
ADA5MQswCQYDVQQGEwJVUzEPMA0GA1UEChMGQW1hem9uMRkwFwYDVQQDExBBbWF6
b24gUm9vdCBDQSAxMB4XDTE1MDUyNjAwMDAwMFoXDTM4MDExNzAwMDAwMFowOTEL
MAkGA1UEBhMCVVMxDzANBgNVBAoTBkFtYXpvbjEZMBcGA1UEAxMQQW1hem9uIFJv
b3QgQ0EgMTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALJ4gHHKeNXj
ca9HgFB0fW7Y14h29Jlo91ghYPl0hAEvrAIthtOgQ3pOsqTQNroBvo3bSMgHFzZM
9O6II8c+6zf1tRn4SWiw3te5djgdYZ6k/oI2peVKVuRF4fn9tBb6dNqcmzU5L/qw
IFAGbHrQgLKm+a/sRxmPUDgH3KKHOVj4utWp+UhnMJbulHheb4mjUcAwhmahRWa6
VOujw5H5SNz/0egwLX0tdHA114gk957EWW67c4cX8jJGKLhD+rcdqsq08p8kDi1L
93FcXmn/6pUCyziKrlA4b9v7LWIbxcceVOF34GfID5yHI9Y/QCB/IIDEgEw+OyQm
jgSubJrIqg0CAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAOBgNVHQ8BAf8EBAMC
AYYwHQYDVR0OBBYEFIQYzIU07LwMlJQuCFmcx7IQTgoIMA0GCSqGSIb3DQEBCwUA
A4IBAQCY8jdaQZChGsV2USggNiMOruYou6r4lK5IpDB/G/wkjUu0yKGX9rbxenDI
U5PMCCjjmCXPI6T53iHTfIUJrU6adTrCC2qJeHZERxhlbI1Bjjt/msv0tadQ1wUs
N+gDS63pYaACbvXy8MWy7Vu33PqUXHeeE6V/Uq2V8viTO96LXFvKWlJbYK8U90vv
o/ufQJVtMVT8QtPHRh8jrdkPSHCa2XV4cdFyQzR1bldZwgJcJmApzyMZFo6IQ6XU
5MsI+yMRQ+hDKXJioaldXgjUkK642M4UwtBV8ob2xJNDd2ZhwLnoQdeXeGADbkpy
rqXRfboQnoZsG4q5WTP468SQvvG5
-----END CERTIFICATE-----
"###;
static CERTIFICATE_PEM: &str = r###"-----BEGIN CERTIFICATE-----
MIIDnzCCAoegAwIBAgIUHJXSyUOGeQWySxCxLaO6vST12hgwDQYJKoZIhvcNAQEL
BQAwTTFLMEkGA1UECwxCQW1hem9uIFdlYiBTZXJ2aWNlcyBPPUFtYXpvbi5jb20g
SW5jLiBMPVNlYXR0bGUgU1Q9V2FzaGluZ3RvbiBDPVVTMB4XDTIzMTEzMDAzMzY1
NVoXDTQ5MTIzMTIzNTk1OVowZDELMAkGA1UEBhMCSlAxDjAMBgNVBAgMBVRva3lv
MRAwDgYDVQQHDAdDaGl5b2RhMQ4wDAYDVQQKDAVJZGVpbjEQMA4GA1UECwwHYWN0
Y2FzdDERMA8GA1UEAwwIaWRlaW4uanAwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAw
ggEKAoIBAQDiO7WssYF6lDWGCX7shJfl4vE5l+dqdznj9Qm6eohB4O4Isag3Qryj
yCUylCqGKY8AjnzqDoWLfvuRQ9FqZI79pKmkgW3aJ7xOzZrADML3cQ1Jww1giO/a
iibp7v6gEH1nIkVfhfankTV5wsqMX87JoxdW5+gcRWEu29BKMKVN7Zu2HvPozrD5
h1ZjrVfP6XB1LflPE63ahgh18RGreeQUZ06FRmTATbfRLKSZHysllzKy+VyPmpXy
5cNNSwLMpHrpuruE0JphuBFCR92sx1onUxoy72ilhU91XNwI9Ki0zwIPTcUOEH/P
MmAkf2rilnueLK/rEX3mltGImkLSjdK1AgMBAAGjYDBeMB8GA1UdIwQYMBaAFIRe
2hhtAbpSWAnXKQzkoMOiPIq2MB0GA1UdDgQWBBTaFOLPiGEYdz2xDg0iJpYGNQfP
JzAMBgNVHRMBAf8EAjAAMA4GA1UdDwEB/wQEAwIHgDANBgkqhkiG9w0BAQsFAAOC
AQEAn3oWVN/qx+MzPuilSaJfZxLjamGkneVIcFd4+iX7LIWW07Xn1/w2eF1Y3MtD
FAuZNCSOCrJ3W5QG7X5Mql9xmsUWUw9Qy2vm6atEV7sWSfnE1hMqy/LvkNNUdEjw
s4Nky050v0/9Sr/TTNMMwiHwo5dMIIhXaWMJbiKp8hzwgVr22RW5C2wmMvDVWD3M
k3aMHbAn0ROAoOX9ixMxn0aKgPeSGnrDNFDDFzXGGW5HtIUqQLZlJTIunha0nc4Q
TgdcKYM1RKVDX7q/6xsw1t98+qAxd2480ZzSvZE4cTkRnAlu3oegocV5WSUfLuDF
tY5NafUxtXmxTHTj8Tga83w7ZA==
-----END CERTIFICATE-----
"###;
static DATA_ENDPOINT: &str = "a3588ryqztf6va-ats.iot.ap-northeast-1.amazonaws.com";
static CREDENTIALS_URL: &str = "https://c2kwx3drypfvs.credentials.iot.ap-northeast-1.amazonaws.com/role-aliases/actcast-dev-thing-role-alias/credentials";
static THING_NAME: &str = "05dae766-3f6a-47bc-92ab-48d232820f5c";
static PRIVATE_KEY: &str = r###"-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA4ju1rLGBepQ1hgl+7ISX5eLxOZfnanc54/UJunqIQeDuCLGo
N0K8o8glMpQqhimPAI586g6Fi377kUPRamSO/aSppIFt2ie8Ts2awAzC93ENScMN
YIjv2oom6e7+oBB9ZyJFX4X2p5E1ecLKjF/OyaMXVufoHEVhLtvQSjClTe2bth7z
6M6w+YdWY61Xz+lwdS35TxOt2oYIdfERq3nkFGdOhUZkwE230SykmR8rJZcysvlc
j5qV8uXDTUsCzKR66bq7hNCaYbgRQkfdrMdaJ1MaMu9opYVPdVzcCPSotM8CD03F
DhB/zzJgJH9q4pZ7niyv6xF95pbRiJpC0o3StQIDAQABAoIBACb62saynvib6Mz7
fd8KyZFWlPGzdrAlctgQNGjpC+kt9FWTJsS+vvoJVj9swBb1uioCYwuBDQmIC5hv
8vk98lVJteEeW+smxY9eZbeJe15fIEcBUKC9d3ZUL3hHMaAqtzC5+vIGhwVAQ9KC
+ZtONrcPXsoekyOr4J7OSQk/cHwIjcSt4YHVY3CKbbvlQFKuxkDsT7fNXgck2fyG
9VnRCSmNYBSAyBfz2Ff6S3ccJrKaTUU5NV4VDhlErwMN1PNZR/hUMvTtmN1LFu/A
sK3JogkJejmJl6Cjq+6O8pcnbPRlirpkKfDOsockmRDXbaK56nR33fEv3bFGLVIe
wangnNECgYEA9b/lPPOWZzvCnzDWEaOKJidgZN4IROGmtoHRpGy6gpgJlpnST8wu
wnVysfDOX/SZLdmTVDN+AsNzfMLmNy7d+TwW7IZduI3glWbs1F2zGeOoUgvtApIF
5uVa3RirGUg7eI9QYsLP+99Zi9SbW4H8/QCSeTnizPDHnJFnGWk1V9sCgYEA66tr
XBltfZGQKIAgXDYBUhwM3M34gaBj9vd05hve9j0xVk0w3ylah+g56khhh3qa0Fqo
Q4BrvNje/7j8axokj5tom7trxKcLh/GsGQgcsEnvcjTjW8c/pRayzY8ekt/HVlsb
TVLyFs1/aRdiPE6+vSIhGKbk5nANz3veSWiRjK8CgYBRiQmV33vXZIx+JMXhtFDx
t77TuJclw3h8tTXJSAnZqngD76VHtIcPHcigITVXwN/Rxo6LMUptwEtg6YlYZcRg
HP++GhaeVPRenXeWPkBeJpqCHlLUt34dzZsIIFckXELfDS2ultRKzR/4tOGWytoV
V0tBzWLifEC+Qb/jPnTkYQKBgGpB7vUmrdxY/mOUaYLKAsxvw0jnQW3I0Gmw5y89
T8k8i/s6cJeeAKiYh0xf+EFqgVUCGy3cQDd0nQ816/GqZuOtEXxuA8g1/Rf9Wjgr
FSxTGbgnqn9eFg8sbdALAKD+btU73NJ/XJH1c8YBehRw+MU0FzITRc+0pDVxxybY
FBAXAoGBANUMQRJ3HZ6tAsr9Sqj1zKyQ8Vzj9QZMI+E4lFt1etc8nRZfQ18K40ED
63heM3H0cpwS6cDuTdQMAUyULpn+VSCDFjymdUrC9wQiwWgqSfoG5T/S3RMZKQlB
GfdtV//67lc4mr7HIKzUOz3cQcxws2Hqp2cEDpW46M2PXP95KSzf
-----END RSA PRIVATE KEY-----
"###;



pub fn mqtt_log_init(){
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
}
