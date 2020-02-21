use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Promise, Error, Function, Reflect};
use web_sys::console;
use std::convert::{TryFrom, TryInto};

pub struct Request {
    pub sleep: Box<dyn Fn(f64) -> Result<JsFuture, JsValue>>,
    pub periodic: Box<dyn Fn(f64, Box<dyn FnMut(f64)>) -> Result<Box<dyn FnOnce() -> Result<JsValue, JsValue>>, JsValue>>,
}

impl TryFrom<JsValue> for Request {
    type Error = JsValue;
    fn try_from(o: JsValue) -> Result<Self, Self::Error> {
        #[derive(Deserialize)]
        pub struct _Request {
        }
        let sleep = {
            let cb = Reflect::get(&o, &JsValue::from("sleep"))?;
            if !Function::instanceof(&cb) {
                return Err(JsValue::from(Error::new("sleep is not function")));
            }
            Function::unchecked_from_js(cb)
        };
        let periodic = {
            let cb = Reflect::get(&o, &JsValue::from("periodic"))?;
            if !Function::instanceof(&cb) {
                return Err(JsValue::from(Error::new("periodic is not function")));
            }
            Function::unchecked_from_js(cb)
        };
        let _req: _Request = o.into_serde()
            .map_err(|err| Error::new(&format!("{:?}", err)))?;
        Ok(Request{
            sleep: Box::new(move |ms|{
                let prm = sleep.call1(&JsValue::NULL, &JsValue::from(ms))?;
                if !Promise::instanceof(&prm) {
                    return Err(JsValue::from(Error::new("return value is not instanceof Promise")));
                }
                Ok(JsFuture::from(Promise::unchecked_from_js(prm)))
            }),
            periodic: Box::new(move |wait, cb|{
                let cb = Closure::<dyn FnMut(f64)>::wrap(Box::new(cb) as Box<dyn FnMut(f64)>);
                let stopfn = periodic.call2(&JsValue::NULL, &JsValue::from(wait), AsRef::<JsValue>::as_ref(&cb))?;
                if !Function::instanceof(&stopfn) {
                    return Err(JsValue::from(Error::new("return value is not instanceof Function")));
                }
                let stopfn = Function::unchecked_from_js(stopfn);
                Ok(Box::new(move ||{
                    let ret = stopfn.call0(&JsValue::NULL);
                    cb.forget();
                    ret
                }))
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
pub async fn handler2(req: JsValue) -> Result<JsValue, JsValue> {
    set_panic_hook();

    let Request{periodic, sleep} = Request::try_from(req)?;

    let stop = periodic(10.0, Box::new(|i|{
        console::log_1(&JsValue::from(format!("{}", i)));
    }))?;
    sleep(3000.0)?.await?;
    stop()?;
    

    Response{}.try_into()
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
