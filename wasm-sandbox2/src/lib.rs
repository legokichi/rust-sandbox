use js_sys::Promise;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::console;

mod res;
pub mod to_iso_string;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(inline_js = r##"
module.exports.sleep = function sleep(ms) {
    return new Promise((resolve)=> setTimeout(resolve, ms));
}
"##)]
extern "C" {
    fn sleep(ms: f64) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    type Date;

    #[wasm_bindgen(constructor)]
    fn new(now: f64) -> Date;
    #[wasm_bindgen(method)]
    fn getTime(this: &Date) -> f64;
    #[wasm_bindgen(method)]
    fn toISOString(this: &Date) -> String;
    #[wasm_bindgen(static_method_of = Date)]
    pub fn now() -> f64;
}

// #[wasm_bindgen(start)]
// pub fn main() {
//     console_log::init_with_level(log::Level::Debug).unwrap();

//     // wasm_logger::init(wasm_logger::Config::default());
    
//     // log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
//     // log::set_max_level(log::LevelFilter::Info);
// }
