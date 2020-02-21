#![allow(clippy::useless_format)]

use actix_web::client::PayloadError;
use actix_web::dev::{MessageBody, Payload, ResponseBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use actix_web::HttpMessage;
use futures::future::{Either, FutureResult};
use futures::prelude::*;
use log::{debug, error};
use mdo::mdo;
use mdo_future::future::*;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SlackParams {
    pub slack_error_notify_token: String,
    pub slack_error_notify_channel: String,
}

#[derive(Debug, Clone)]
pub struct SentryParams {
    pub sentry_dsn: String,
}

#[derive(Debug, Clone)]
pub struct ErrorLogger<C: Clone + hyper::client::connect::Connect + 'static> {
    pub sentry: Option<SentryParams>,
    pub slack: Option<SlackParams>,
    pub error_status: HashSet<StatusCode>,
    pub connector: C,
}

impl<S, B, C> Transform<S> for ErrorLogger<C>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>
        + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
    C: Clone + hyper::client::connect::Connect + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = ErrorLoggerMiddleware<S, C>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(ErrorLoggerMiddleware {
            service: Rc::new(RefCell::new(service)),
            sentry: self.sentry.clone(),
            slack: self.slack.clone(),
            error_status: self.error_status.clone(),
            connector: self.connector.clone(),
        })
    }
}

pub struct ErrorLoggerMiddleware<S, C: Clone + hyper::client::connect::Connect + 'static> {
    service: Rc<RefCell<S>>,
    slack: Option<SlackParams>,
    sentry: Option<SentryParams>,
    error_status: HashSet<StatusCode>,
    pub connector: C,
}

impl<S, B, C> Service for ErrorLoggerMiddleware<S, C>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>
        + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
    C: Clone + hyper::client::connect::Connect + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut service = self.service.clone();
        let sentry = self.sentry.clone();
        let slack = self.slack.clone();
        let error_status = self.error_status.clone();
        let connector = self.connector.clone();
        Box::new(mdo! {
            req_body: Bytes =<< HttpMessage::take_payload(&mut req)
                .concat2()
                .from_err();
            let stream: Box<dyn Stream<Item = Bytes, Error = PayloadError> + 'static> = Box::new(futures::stream::once(Ok(req_body.clone())));
            let _ = req.set_payload(Payload::Stream(stream));
            let req_info = format!("{} {}", req.method(), req.path());
            let req_text = format!("{:?}", req);
            mut res =<< service.call(req);
            let res_info = format!("{} {}", req_info, res.status());
            ret if error_status.contains(&res.status()) {
                Either::A(mdo!{
                    res_body: Bytes =<< res
                        .take_body()
                        .concat2()
                        .from_err();
                    let res = res.map_body(|_, _| ResponseBody::Body(res_body.clone().into()).into_body());
                    let res_text = format!("{:?}", res.response());
                    let opt_res_error_text = res.response().error().map(|err|{
                        let mut texts: Vec<String> = vec![];
                        texts.push(format!("{}", err));
                        let mut source = std::error::Error::source(err);
                        while let Some(err) = source {
                            texts.push(format!("{}", err));
                            source = err.source();
                        }
                        texts.join("\n")
                    });
                    let is_server_error = res.status().is_server_error();
                    let _ = actix_rt::spawn(mdo!{
                        req_body_text =<< String::from_utf8(req_body.to_vec())
                            .into_future()
                            .map_err(|err| error!("{}", err));
                        res_body_text =<< String::from_utf8(res_body.to_vec())
                            .into_future()
                            .map_err(|err| error!("{}", err));
                        let text = {
                            let mut texts: Vec<String> = vec![];
                            texts.push(format!("<<<<<<<<<<<< request"));
                            texts.push(format!("{}", req_text));
                            texts.push(format!("req.body: {}", req_body_text));
                            texts.push(format!("============ error"));
                            if let Some(error_text) = opt_res_error_text.clone() {
                                texts.push(error_text);
                            }
                            texts.push(format!("============"));
                            texts.push(format!("{}", res_text));
                            texts.push(format!("res.body: {}", res_body_text));
                            texts.push(format!(">>>>>>>>>>>> response"));
                            texts.join("\n")
                        };
                        // CloudWatch
                        let _ = error!("{}", text);
                        // sentry
                        let _ = if let Some(sentry) = sentry {
                            post_sentry(&sentry, &res_info, &text, opt_res_error_text, is_server_error);
                        };
                        // slack
                        ret match (is_server_error, slack) {
                            (true, Some(ref slack)) => {
                                Either::A(post_slack(connector, &slack, &text).map_err(|err| error!("{}", err)))
                            },
                            _ => Either::B(Ok(()).into_future()),
                        }
                    });
                    ret ret(res)
                })
            }else{
                Either::B(ret(res))
            }
        })
    }
}

// 送信そのものは別にスレッドを作成して行なっているのでブロッキングしない
// https://github.com/getsentry/sentry-rust/blob/0.17.0/src/transport.rs#L244
fn post_sentry(
    _sentry: &SentryParams,
    res_info: &str,
    text: &str,
    error_text: Option<String>,
    is_server_error: bool,
) {
    use sentry::{Hub, Level};

    let hub = Hub::new_from_top(Hub::main());
    hub.configure_scope(|scope| {
        scope.set_fingerprint(Some(&[res_info]));

        scope.set_extra("text", text.into());
    });

    if is_server_error {
        let message = error_text.unwrap_or_else(|| format!("server error: {}", res_info));
        hub.capture_message(message.as_str(), Level::Error);
    } else {
        hub.capture_message(format!("client error: {}", res_info).as_str(), Level::Info);
    }
}

#[derive(Debug, Clone, Default, Serialize)]
struct FilesUploadRequest {
    token: String,
    channels: Option<String>,
    content: Option<String>,
    file: Option<String>,
    filename: Option<String>,
    filetype: Option<String>,
    initial_comment: Option<String>,
    thread_ts: Option<String>,
    title: Option<String>,
}

pub fn post_slack<C: hyper::client::connect::Connect + 'static>(
    https_connector: C,
    slack: &SlackParams,
    text: &str,
) -> impl Future<Item = (), Error = failure::Error> {
    let client = hyper::Client::builder().build::<_, hyper::Body>(https_connector);
    mdo! {
        let body = FilesUploadRequest {
            token: slack.slack_error_notify_token.clone(),
            channels: Some(slack.slack_error_notify_channel.clone(),),
            content: Some(text.to_string()),
            filetype: Some("text".to_string()),
            ..Default::default()
        };
        url =<< "https://slack.com/api/files.upload"
            .parse::<::hyper::Uri>()
            .into_future()
            .from_err();
        body =<< serde_urlencoded::to_string(body)
            .into_future()
            .from_err();
        let body = hyper::Body::from(body);
        req =<< ::hyper::Request::post(url)
            .header(hyper::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(hyper::header::ACCEPT, "application/json")
            .body(body)
            .into_future()
            .from_err();
        resp =<< client
            .request(req)
            .from_err();
        let (head, body) = resp.into_parts();
        let _ = debug!("{:?}", head);
        body =<< body.concat2().from_err();
        body =<< String::from_utf8(body.to_vec())
            .into_future()
            .from_err();
        let _ = debug!("{}", body);
        ret ret(())
    }
}
