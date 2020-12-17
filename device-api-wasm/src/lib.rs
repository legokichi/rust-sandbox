#[macro_use]
extern crate load_file;

pub mod api;
pub mod model;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start, skip_typescript)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

