
use super::*;

pub fn get_counter(req: HttpRequest<Ctx>) -> impl Future<Item=HttpResponse, Error=::actix_web::Error> + 'static {
    mdo!{
        // http://redis.shibu.jp/commandreference/
        // https://docs.rs/redis-async/0.4.2/redis_async/macro.resp_array.html
        // https://qiita.com/rubytomato@github/items/d66d932959d596876ab5
        // https://redis.io/topics/data-types-intro
        // https://github.com/actix/examples/blob/master/actix_redis/src/main.rs
        // https://actix.rs/actix/actix/struct.Addr.html#method.send
        // https://actix.rs/actix-redis/actix_redis/struct.Command.html
        // https://github.com/actix/examples/blob/master/actix_redis/src/main.rs
        let redis = req.state().redis_addr.clone();

        resp =<< redis.send(Command(resp_array!["SET", "counter", "0"])).from_err();
        () =<< future::result(match resp {
            Ok(RespValue::SimpleString(ref ret)) if ret.as_str() == "OK" => Ok(()),
            _ => Err(ErrorInternalServerError(format!("redis cache failed: {:?}", resp))),
        });

        resp =<< redis.send(Command(resp_array!["INCR", "counter"])).from_err();
        n =<< future::result(match resp {
            Ok(RespValue::Integer(n)) => Ok(n),
            _ => Err(ErrorInternalServerError(format!("redis cache failed: {:?}", resp))),
        });

        resp =<< redis.send(Command(resp_array!["GET", "counter"])).from_err();
        n =<< future::result(match resp {
            Ok(RespValue::Nil) => Ok(1),
            Ok(RespValue::BulkString(bulk)) => Ok(1),
            _ => Err(ErrorInternalServerError(format!("redis cache failed: {:?}", resp)))
        });
        ret ret(HttpResponse::Ok().body(format!("{}", n)))
    }
}

pub fn add_counter(req: HttpRequest<Ctx>) -> impl Future<Item=HttpResponse, Error=::actix_web::Error> {
    mdo!{
        // let redis = req.state().redis_addr.clone();
        // counter =<< redis.send(Command(resp_array!["GET", "counter"]));
        // counter =<< redis.send(Command(resp_array!["SET", "counter", counter+1]));
        // counter =<< redis.send(Command(resp_array!["GET", "counter"]));
        ret ret(HttpResponse::Ok().body("1"))
    }
}


// TODO: redis queue
// http://d.hatena.ne.jp/hiroe_orz17/20120814/1344963552