use crate::res;
use crate::Date;
use log::info;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface Request {
    now: number;
}

export type Response = 
    | { type: "ok", value: string; }
    | { type: "error", value: never; };

export function handler(req: Request): Promise<Response>;
"#;


#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub now: f64,
}
pub type Response = res::Response<SuccessResponse, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessResponse {
    pub iso8601: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum Error {}




async fn handler_impl(req: Request) -> Result<Response, JsValue> {
    let Request { now } = req;
    let iso8601 = Date::new(now).toISOString();
    info!("{}", iso8601);
    let res = Response::Ok(SuccessResponse { iso8601 });
    Ok(res)
}

#[wasm_bindgen]
pub async fn handler(req: JsValue) -> Result<JsValue, JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    info!("{} req: {:?}", Date::now(), req);
    let req: Request = serde_wasm_bindgen::from_value(req)?;
    info!("{} req: {:?}", Date::now(), req);
    let res: Response = handler_impl(req).await.map_err(JsValue::from)?;
    info!("{} res: {:?}", Date::now(), res);
    let res = serde_wasm_bindgen::to_value(&res)?;
    info!("{} res: {:?}", Date::now(), res);
    Ok(res)
}
