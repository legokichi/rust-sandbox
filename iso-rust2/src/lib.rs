use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::window;


#[wasm_bindgen]
pub fn main() {
    let name = "💩";
    console::log(js_sys::Array::from(wasm_bindgen::JsValue::from_str((&format!"Hello, {}!", name))));
    // window().alert(&format!("Hello, {}!", name));
}