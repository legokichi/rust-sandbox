#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    #[cfg(feature = "console_error_panic_hook")]
    #[wasm_bindgen(start, skip_typescript)]
    pub fn init() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
}

mod hoge;
mod fuga;
mod piyo;

pub use hoge::*;
pub use fuga::*;
pub use piyo::*;
