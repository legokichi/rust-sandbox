#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
async fn pass() -> () {
    use wasm_sandbox::to_iso_string::*;
    let req = JsValue::from_serde(&Request { now: 1_f64 }).unwrap();
    let res = handler(req).await.unwrap();
    let res: Response = res.into_serde().unwrap();
    let SuccessResponse { iso8601 } = Into::<Result<_, _>>::into(res).unwrap();
    assert_eq!(iso8601, "1970-01-01T00:00:00.001Z");
}
