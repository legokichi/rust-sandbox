use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Promise, Error, Function, Reflect};
use web_sys::console;
use std::convert::{TryFrom, TryInto};

pub struct Request {
    pub count: u32,
    pub wait: f64,
    pub sleep: Box<dyn Fn(f64) -> Result<JsFuture, JsValue>>,
}

impl TryFrom<JsValue> for Request {
    type Error = JsValue;
    fn try_from(o: JsValue) -> Result<Self, Self::Error> {
        #[derive(Deserialize)]
        pub struct _Request {
            pub count: u32,
            pub wait: f64,
        }
        let sleep = {
            let cb = Reflect::get(&o, &JsValue::from("sleep"))?;
            if !Function::instanceof(&cb) {
                return Err(JsValue::from(Error::new("sleep is not function")));
            }
            Function::unchecked_from_js(cb)
        };
        let _req: _Request = o.into_serde()
            .map_err(|err| Error::new(&format!("{:?}", err)))?;
        Ok(Request{
            count: _req.count,
            wait: _req.wait,
            sleep: Box::new(move |ms|{
                let prm = sleep.call1(&JsValue::NULL, &JsValue::from(ms))?;
                if !Promise::instanceof(&prm) {
                    return Err(JsValue::from(Error::new("return value is not instanceof Promise")));
                }
                Ok(JsFuture::from(Promise::unchecked_from_js(prm)))
            })
        })
    }
}

pub struct Response {
}
impl TryInto<JsValue> for Response {
    type Error = JsValue;
    fn try_into(self) -> Result<JsValue, Self::Error> {
        #[derive(Serialize)]
        pub struct _Response {
        }
        let Response {} = self;
        JsValue::from_serde(&_Response{})
            .map_err(|err| JsValue::from(Error::new(&format!("{:?}", err))))
    }
}

#[wasm_bindgen]
pub async fn handler(req: JsValue) -> Result<JsValue, JsValue> {
    set_panic_hook();

    let Request{sleep, count, wait} = Request::try_from(req)?;

    for i in 0_u32..count {
        sleep(wait)?.await?;
        console::log_1(&JsValue::from(format!("{}", i)));
    }

    Response{}.try_into()
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
