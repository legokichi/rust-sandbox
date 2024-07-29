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
