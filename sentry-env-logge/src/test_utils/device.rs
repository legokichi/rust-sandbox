use crate::test_utils::device_api::*;
use crate::test_utils::*;

pub static AMAZON_ROOT_CA1_PEM: &str = r###"-----BEGIN CERTIFICATE-----
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


pub struct DeviceApiClient {
    get_provisioning_token_arn: String,
    invoke_arn: String,
    post_device_arn: String,
    client: rusoto_lambda::LambdaClient,
}
impl DeviceApiClient {
    pub fn new() -> Result<Self, anyhow::Error> {
        use rusoto_core::Region;
        use rusoto_lambda::LambdaClient;
        let get_provisioning_token_arn = std::env::var("GET_PROVISIONING_TOKEN_LAMBDA_ARN")?;
        let invoke_arn = std::env::var("INVOKE_LAMBDA_ARN")?;
        let post_device_arn = std::env::var("POST_DEVICE_LAMBDA_ARN")?;
        let client = LambdaClient::new(Region::default());
        Ok(Self {
            get_provisioning_token_arn,
            invoke_arn,
            post_device_arn,
            client,
        })
    }
    async fn invoke<T, U>(&self, function_name: String, req: T) -> Result<U, anyhow::Error>
    where
        T: serde::Serialize + std::fmt::Debug,
        U: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        use rusoto_lambda::{InvocationRequest, InvocationResponse, Lambda};
        log::debug!("lambda {} req: {:?}", function_name, req);
        let req = serde_json::to_vec(&req)?;
        let InvocationResponse { payload, .. } = self
            .client
            .invoke(InvocationRequest {
                function_name: function_name.clone(),
                payload: Some(req.into()),
                ..Default::default()
            })
            .await?;
        let o = serde_json::from_slice::<serde_json::Value>(payload.unwrap().as_ref())?;
        log::debug!("lambda {} res: {:?}", function_name, o);
        let o = serde_json::from_value::<LambdaResult<U>>(o)?;
        match o {
            LambdaResult::Ok(o) => Ok(o),
            e => Err(anyhow::anyhow!("{:?}", e)),
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Device {
    pub thing_name: String,
    pub certificate_pem: String,
    pub data_endpoint: String,
    pub credentials_url: String,
    pub pkey_pkcs1_pem: String,
}

impl Device {
    fn create_csr_str() -> Result<(String, String), anyhow::Error> {
        let rsa = openssl::rsa::Rsa::generate(2048)?;
        let pkey = openssl::pkey::PKey::from_rsa(rsa)?;
        let x509req = {
            let mut names = openssl::x509::X509NameBuilder::new()?;
            names.append_entry_by_text("C", "JP")?;
            names.append_entry_by_text("ST", "Tokyo")?;
            names.append_entry_by_text("L", "Chiyoda")?;
            names.append_entry_by_text("O", "Idein")?;
            names.append_entry_by_text("OU", "actcast")?;
            names.append_entry_by_text("CN", "idein.jp")?;
            let mut x509req = openssl::x509::X509ReqBuilder::new()?;
            x509req.set_subject_name(names.build().as_ref())?;
            x509req.set_pubkey(&pkey)?;
            x509req.sign(&pkey, openssl::hash::MessageDigest::sha256())?;
            x509req.build()
        };
        let pem = &x509req.to_pem()?;
        let csr = String::from_utf8_lossy(pem).to_string();
        let rsa = pkey.rsa()?;
        let pem = rsa.private_key_to_pem()?;
        let pkey_pkcs1_pem = String::from_utf8(pem)?;
        Ok((csr, pkey_pkcs1_pem))
    }
    pub async fn register(group_id: i64, client: impl DeviceApi) -> Result<Self, anyhow::Error> {
        let post_provisioning_token::Response {
            provisioning_token, ..
        } = client
            .post_provisioning_token(post_provisioning_token::Request {
                group_id,
                ..Default::default()
            })
            .await
            .unwrap();
        log::debug!("provisioning_token: {}", provisioning_token);
        let (csr, pkey_pkcs1_pem) = Self::create_csr_str()?;
        let post_device::Response {
            thing_name,
            certificate_pem,
            data_endpoint,
            credentials_url,
            ..
        } = client
            .post_device(post_device::Request {
                provisioning_token,
                hostname: "hoge".to_string(),
                certificate_signing_request: csr,
                device_type: "Unknown".to_string(),
                target_triple: "x86_64-unknown-linux".to_string(),
                access_points: vec![] as Vec<serde_json::Value>,
                host_version: "0.0.0-legokichi".to_string(),
            })
            .await?;
        let o = Device {
            thing_name,
            pkey_pkcs1_pem,
            certificate_pem,
            data_endpoint,
            credentials_url,
        };
        log::debug!("device: {:?}", o);
        Ok(o)
    }
    pub fn mqtt(&self) -> Result<(rumqttc::AsyncClient, rumqttc::EventLoop), anyhow::Error> {
        use rumqttc::{AsyncClient, ClientConfig, MqttOptions, Transport};
        use rustls::internal::pemfile;
        let mut mqttoptions =
            MqttOptions::new(&self.thing_name.clone(), self.data_endpoint.clone(), 443);
        let mut cert = std::io::Cursor::new(self.certificate_pem.as_bytes().to_vec());
        let mut key = std::io::Cursor::new(self.pkey_pkcs1_pem.as_bytes().to_vec());
        let mut ca_cert = std::io::Cursor::new(AMAZON_ROOT_CA1_PEM.as_bytes().to_vec());
        let keys = pemfile::rsa_private_keys(&mut key).unwrap();
        let certs = pemfile::certs(&mut cert).unwrap();
        let ca_certs = pemfile::certs(&mut ca_cert).unwrap();
        let mut conf = ClientConfig::new();
        conf.set_single_client_cert(certs, keys.into_iter().next().unwrap())?;
        conf.root_store.add(ca_certs.get(0).unwrap())?;
        conf.set_protocols(&[b"x-amzn-mqtt-ca".to_vec()]);
        mqttoptions.set_transport(Transport::Tls(conf.into()));
        Ok(AsyncClient::new(mqttoptions, 10))
    }
    pub async fn start(
        &self,
        subscribes: Vec<String>,
        mut outgoing: impl futures::stream::Stream<Item = (String, Vec<u8>)>
            + Unpin
            + Send
            + Sync
            + 'static,
    ) -> Result<
        impl futures::stream::Stream<Item = Vec<u8>> + Unpin + Send + Sync + 'static,
        anyhow::Error,
    > {
        use rumqttc::QoS;
        let (mut tx, incomming) = futures::channel::mpsc::channel(10);
        let (client, mut eventloop) = self.mqtt()?;
        let _fut = tokio::spawn({
            let thing_name = self.thing_name.clone();
            async move {
                while let Ok(notification) = eventloop.poll().await {
                    use rumqttc::Event::*;
                    match notification {
                        Incoming(o) => {
                            use rumqttc::v4::Publish;
                            match o {
                                rumqttc::Incoming::Publish(Publish { payload, .. }) => {
                                    while let Err(e) = tx.try_send(payload.to_vec()) {
                                        log::warn!("{}: send error = {:?}", thing_name, e);
                                    }
                                }
                                o => {
                                    log::trace!("{}: incomming = {:?}", thing_name, o);
                                }
                            }
                        }
                        Outgoing(o) => {
                            log::trace!("{}: outgoing = {:?}", thing_name, o);
                        }
                    }
                }
            }
        });
        // @todo shutdown 方法
        let _fut = tokio::spawn({
            let thing_name = self.thing_name.clone();
            async move {
                for topic in subscribes {
                    client.subscribe(topic, QoS::AtMostOnce).await.unwrap();
                }
                use futures::stream::StreamExt;
                while let Some((topic, payload)) = outgoing.next().await {
                    while let Err(e) = client
                        .publish(topic.clone(), QoS::AtLeastOnce, false, payload.clone())
                        .await
                    {
                        log::warn!("{}: publish error = {:?}", thing_name, e);
                    }
                }
            }
        });
        Ok(incomming)
    }
}

#[cfg(feature = "e2e")]
#[rstest]
#[tokio::test]
async fn e2e_mqtt() {
    dotenv::dotenv().ok();
    let group_id = std::env::var("E2E_GROUP_ID").unwrap().parse().unwrap();
    let client = DeviceApiClient::new().unwrap();
    let device = device::Device::register(group_id, client).await.unwrap();
    let (mut tx, rx) = futures::channel::mpsc::channel::<(String, Vec<u8>)>(10);
    let topic = format!("things/{}/service/firmware/response", device.thing_name);
    let mut rx = device.start(vec![topic.clone()], rx).await.unwrap();
    use futures::SinkExt;
    tx.send((topic, vec![1, 2, 3])).await.unwrap();
    use futures::stream::StreamExt;
    let o = rx.next().await.unwrap();
    assert_eq!(o, vec![1, 2, 3]);
}
