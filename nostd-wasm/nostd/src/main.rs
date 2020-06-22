#![no_std]
#![no_main]


// Replacing the allocator and using the `alloc` crate are still unstable.
// #![feature()]
#![feature(core_intrinsics, lang_items, alloc_error_handler)]
// use alloc;
extern crate alloc;
// extern crate wee_alloc;


// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Need to provide a tiny `panic` implementation for `#![no_std]`.
// This translates into an `unreachable` instruction that will
// raise a `trap` the WebAssembly execution if we panic at runtime.
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    // unsafe {
        ::core::intrinsics::abort();
    // }
}

// Need to provide an allocation error handler which just aborts
// the execution with trap.
#[alloc_error_handler]
#[no_mangle]
pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
    // unsafe {
        ::core::intrinsics::abort();
    // }
}

// Needed for non-wasm targets.
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}


use alloc::boxed::Box;
use heapless::Vec;

use heapless::consts::*;


// Box a `u8`!
#[no_mangle]
pub extern "C" fn hello() -> *mut u8 {
    let mut xs: Vec<_, U8> = Vec::new();

    xs.push(42).unwrap();
    assert_eq!(xs.pop(), Some(42));
    Box::into_raw(Box::new(42))
}
