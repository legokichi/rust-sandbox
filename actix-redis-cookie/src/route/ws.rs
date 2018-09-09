use super::*;
use actix_web::ws;
use std::sync::Arc;
use actix_redis::RedisActor;

/// redis_async と actix-redis の都合で、 
/// publish は actix-redis を、
/// subscribe は redis_async を使っている
/// redis-rs や r2d2-redis に非同期 IO なんてなかった。
pub struct WsActor{
    stopped: bool,
    conn: ::redis_async::client::pubsub::PubsubConnection,
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self, Ctx>;
    fn started(&mut self, ctx: &mut Self::Context){
        let addr = ctx.address();
        Arbiter::spawn(mdo!{
            stream =<< self.conn.subscribe("broadcast").map_err(|_| ());
            () =<< stream.for_each(move |resp|{
                let addr = addr.clone();
                match resp {
                    RespValue::BulkString(buf) => {
                        Arbiter::spawn(mdo!{
                            text: String =<< future::result(::std::str::from_utf8(&buf).map(Into::into)).map_err(|_| ());
                            _ =<< addr.send(Message(text)).map_err(|_| ());
                            ret ret(())
                        });
                        Ok(())
                    },
                    err @ _ => {
                        error!("{:?}", err);
                        Ok(())
                    },
                }
            }).map_err(|_| ());
            ret ret(())
        });
    }
}
#[derive(Message)]
struct Message(pub String);
impl ::actix::Handler<Message> for WsActor {
    type Result = ();
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
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
                    _ =<< redis.send(Command(resp_array!["PUBLISH", "broadcast", text]));
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
    mdo!{
        conn =<< ::redis_async::client::pubsub_connect(&req.state().redis_socket_addr)
            .map_err(ErrorInternalServerError);
        ret future::result(
            ws::start(&req, WsActor{ stopped: false, conn })
                .map_err(ErrorInternalServerError)
        )
    }
}