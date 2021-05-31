// use panic_halt as _;
use core::panic::PanicInfo;

// #[no_panic::no_panic]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    uprintln!("{}", info);
    loop {}
}
