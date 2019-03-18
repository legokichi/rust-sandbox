#![allow(unused_imports)]
use failure::{Error, Fail};
use futures::prelude::*;
use futures::sync::mpsc::UnboundedSender;
use log::*;
use mdo::*;
use mdo_future::future::*;
use serde_derive::*;
use servo_media::streams::*;
use servo_media::webrtc::*;
use servo_media::ServoMedia;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;
// use tokio_tungstenite::stream::PeerAddr;

fn default_stun_server_url() -> Url {
    "stun://stun.l.google.com:19302".parse().unwrap()
}
fn default_local_server() -> String {
    "127.0.0.1:8080".to_string()
}
fn default_local_server_url() -> Url {
    format!("ws://{}", default_local_server()).parse().unwrap()
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Config {
    #[serde(default = "default_stun_server_url")]
    #[serde(with = "::url_serde")]
    pub stun_server_url: Url,
    #[serde(default = "default_local_server")]
    pub local_server: String,
    #[serde(default = "default_local_server_url")]
    #[serde(with = "::url_serde")]
    pub local_server_url: Url,
}

fn main() {
    ::dotenv::dotenv().ok();
    ::std::env::set_var("RUST_LOG", "debug");
    let _ = ::env_logger::try_init();
    let config = ::envy::from_env::<Config>().unwrap();

    let srv = start_server(&config).map_err(|err: Error| error!("server: {:?}", err));
    let cli = start_client(&config).map_err(|err: Error| error!("client: {:?}", err));
    tokio::run(srv.join(cli).map(|_| ()).map_err(|_| ()));
}

fn start_client(config: &Config) -> impl Future<Item = (), Error = Error> + Send + 'static {
    let local_server_url = config.local_server_url.clone();
    mdo! {
        _ =<< tokio_timer::sleep(std::time::Duration::from_secs(1)).map_err(Into::into);
        (ws_stream, _res) =<< connect_async(local_server_url).map_err(Into::into);
        let (mut sink, stream) = ws_stream.split();
        let ws_reader = stream.for_each(move |message| {
            info!("client: Received a message: {}", message);
            Ok(())
        }).map(|_|()).map_err(Into::<Error>::into);
        let _ = sink.start_send(Message::Text("hello".to_string())).unwrap();
        ret ws_reader
    }
    // ServoMedia::init::<servo_media_auto::Backend>();
    // let servo_media = ServoMedia::get().unwrap();
    // let output: Box<MediaOutput> = servo_media.create_stream_output();
    // let signaller = Signaller::new();
    // let webrtc: WebRtcController = servo_media.create_webrtc(Box::new(signaller));
}

fn start_server(config: &Config) -> impl Future<Item = (), Error = Error> + Send + 'static {
    let connections: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let socket = TcpListener::bind(&config.local_server.parse().unwrap()).unwrap();
    socket
        .incoming()
        .map_err(Into::<Error>::into)
        .for_each(move |stream| {
            mdo! {
                let connections = connections.clone();
                let addr = stream.peer_addr().unwrap();
                let _ = info!("server: Peer address: {}", addr);
                ws_stream =<< accept_async(stream).map_err(Into::<Error>::into);
                let (sink, stream) = ws_stream.split();
                let (tx, rx) = futures::sync::mpsc::unbounded();
                let _ = connections.lock().unwrap().insert(addr, tx);
                let ws_reader = stream.for_each({
                    let connections = connections.clone();
                    move |message: Message| {
                        info!("server: Received a message from {}: {}", addr, message);
                        if let Message::Text(_) = message {
                            let mut conn = connections.lock().unwrap();
                            let mut tx = conn.get_mut(&addr).unwrap();
                            tx.unbounded_send(message.clone()).unwrap();
                        }
                        Ok(())
                    }
                }).map(|_|()).map_err(Into::<Error>::into);
                let ws_writer = rx.fold(sink, |mut sink, msg| {
                    info!("server: sending: {:?}", msg);
                    sink.start_send(msg).unwrap();
                    Ok(sink)
                });
                let connections = connections.clone();
                () =<< ws_reader.select2(ws_writer).then(move |_| {
                    connections.lock().unwrap().remove(&addr);
                    info!("Connection {} closed.", addr);
                    Ok(())
                });
                ret Ok(())
            }
        })
}

fn start_rtc(media: Arc<ServoMedia>) {
    /*
    let signaller = Signaller::new(
        self.send_msg_tx.clone(),
        self.peer_id.is_some(),
        media.create_stream_output(),
    );
    let s = signaller.clone();
    let webrtc = media.create_webrtc(Box::new(signaller));
    let webrtc = self.webrtc.as_ref().unwrap();

    let (video, audio) = if !self.peer_id.is_some() {
        (
            self.media
                .create_videoinput_stream(Default::default())
                .unwrap_or_else(|| self.media.create_videostream()),
            self.media
                .create_audioinput_stream(Default::default())
                .unwrap_or_else(|| self.media.create_audiostream()),
        )
    } else {
        (
            self.media.create_videostream(),
            self.media.create_audiostream(),
        )
    };
    webrtc.add_stream(video);
    webrtc.add_stream(audio);

    webrtc.configure(STUN_SERVER.into(), BundlePolicy::MaxBundle);
    */
}

#[derive(Clone)]
struct Signaller {
    // sender: mpsc::Sender<OwnedMessage>,
// initiate_negotiation: bool,
// output: Arc<Mutex<Box<MediaOutput>>>,
}

impl Signaller {
    fn send_sdp(&self, desc: SessionDescription) {
        unimplemented!()
        // let message = serde_json::to_string(&JsonMsg::Sdp {
        //     type_: desc.type_.as_str().into(),
        //     sdp: desc.sdp,
        // })
        // .unwrap();
        // self.sender.send(OwnedMessage::Text(message)).unwrap();
    }
    fn new() -> Self {
        unimplemented!()
        // Signaller {
        //     sender,
        //     initiate_negotiation,
        //     output: Arc::new(Mutex::new(output)),
        // }
    }
}

impl WebRtcSignaller for Signaller {
    fn close(&self) {
        unimplemented!()
        // let _ = self
        //     .sender
        //     .send(OwnedMessage::Close(Some(websocket::message::CloseData {
        //         status_code: 1011, //Internal Error
        //         reason: "explicitly closed".into(),
        //     })));
    }

    fn on_ice_candidate(&self, _: &WebRtcController, candidate: IceCandidate) {
        unimplemented!()
        // let message = serde_json::to_string(&JsonMsg::Ice {
        //     candidate: candidate.candidate,
        //     sdp_mline_index: candidate.sdp_mline_index,
        // })
        // .unwrap();
        // self.sender.send(OwnedMessage::Text(message)).unwrap();
    }

    fn on_negotiation_needed(&self, controller: &WebRtcController) {
        unimplemented!()
        // if !self.initiate_negotiation {
        //     return;
        // }
        // let c2 = controller.clone();
        // let s2 = self.clone();
        // controller.create_offer(
        //     (move |offer: SessionDescription| {
        //         c2.set_local_description(offer.clone(), (move || s2.send_sdp(offer)).into())
        //     })
        //     .into(),
        // );
    }

    fn on_add_stream(&self, stream: Box<MediaStream>) {
        unimplemented!()
        // println!("notified of stream!");
        // self.output.lock().unwrap().add_stream(stream);
    }
}
