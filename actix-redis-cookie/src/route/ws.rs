use super::*;
use actix_web::ws;
use std::sync::Arc;
use actix_redis::RedisActor;

pub struct WsActor{
    stopped: bool,
    redis_addr: Arc<Addr<RedisActor>>,
}
impl WsActor {
}
impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self, Ctx>;
    fn started<'a>(&'a mut self, ctx: &'a mut Self::Context){
        let redis_addr = self.redis_addr.clone();
        let addr = ctx.address();
        Arbiter::spawn(mdo!{
            // http://d.hatena.ne.jp/hiroe_orz17/20120814/1344963552
            resp =<< redis_addr.send(Command(resp_array!["SUBSCRIBE", "bloadcast"])).from_err();
            _ =<< match resp {
                Ok(RespValue::Array(arr)) =>{
                    // actix-redis は pubsub できなかった
                    // https://mitsuhiko.github.io/redis-rs/redis/#pubsub
                    println!("started: {:?}", arr);
                    Box::new(future::ok(()))
                },
                Ok(RespValue::SimpleString(text)) =>{
                    Box::new(addr.send(Message(text.into())).map_err(|err| format_err!("{:?}", err))) as Box<dyn Future<Item=_, Error=_>>
                },
                _ => Box::new(future::err(format_err!("redis cache failed: {:?}", resp))) as Box<dyn Future<Item=_, Error=_>>,
            };
            ret ret(())
        }.map_err(|err| error!("redis publish error {:?}", err)));
    }
}
#[derive(Message)]
pub struct Message(pub String);
impl ::actix::Handler<Message> for WsActor {
    type Result = ();
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context){
        ctx.text(msg.0);
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => {
                let redis = ctx.state().redis_addr.clone();
                Arbiter::spawn(mdo!{
                    // http://d.hatena.ne.jp/hiroe_orz17/20120814/1344963552
                    _ =<< redis.send(Command(resp_array!["PUBLISH", "bloadcast", text]));
                    ret ret(())
                }.map_err(|err| error!("redis publish error {:?}", err)));
            },
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

pub fn index(req: HttpRequest<Ctx>) -> impl Future<Item=HttpResponse, Error=::actix_web::Error> {
    future::result(ws::start(&req, WsActor{ stopped: false, redis_addr: req.state().redis_addr.clone() }))
}