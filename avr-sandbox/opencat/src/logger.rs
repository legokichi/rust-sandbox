use atmega328p_hal::clock::MHz16;
use atmega328p_hal::pac::USART0;
use atmega328p_hal::port::mode::Floating;
use atmega328p_hal::port::mode::{Input, Output};
use atmega328p_hal::port::portd::{PD0, PD1};
use atmega328p_hal::usart::UsartWriter;
use avr_device::interrupt::Mutex;
use core::cell::RefCell;

type Tx = UsartWriter<USART0, PD0<Input<Floating>>, PD1<Output>, MHz16>;

// https://github.com/Rahix/avr-hal/issues/115
static LOGGER: Mutex<RefCell<Option<Tx>>> = Mutex::new(RefCell::new(None));

pub fn init(tx: Tx) {
    avr_device::interrupt::free(move |cs| {
        *LOGGER.borrow(cs).borrow_mut() = Some(tx);
    });
}

pub fn _print(s: &str) {
    // use core::fmt::Write;
    avr_device::interrupt::free(move |cs| {
        if let Some(tx) = LOGGER.borrow(cs).borrow_mut().as_mut() {
            ufmt::uwriteln!(tx, "{}", s).unwrap();
        }
    });
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // uprintln!("{:?}", info);
    loop {
        avr_device::interrupt::free(move |cs| {
            if let Some(tx) = LOGGER.borrow(cs).borrow_mut().as_mut() {
                ufmt::uwriteln!(tx, "dead").unwrap();
            }
        });
    }
}
