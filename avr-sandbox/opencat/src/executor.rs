/// https://blog.rahix.de/005-avr-hal-millis/
use avr_device::interrupt::Mutex;
use core::cell::Cell;
use core::cell::RefCell;
use core::cell::UnsafeCell;
use core::future::Future;
use core::pin::Pin;
use core::ptr;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use futures::stream::Stream;
use atmega328p_hal::clock::MHz16;
use atmega328p_hal::pac::USART0;
use atmega328p_hal::port::mode::Floating;
use atmega328p_hal::port::mode::{Input, Output};
use atmega328p_hal::port::portd::{PD0, PD1};
use atmega328p_hal::usart::{UsartWriter, UsartReader};

type Tx = UsartWriter<USART0, PD0<Input<Floating>>, PD1<Output>, MHz16>;
type Rx = UsartReader<USART0, PD0<Input<Floating>>, PD1<Output>, MHz16>;

// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: u32 = 1024;
const TIMER_COUNTS: u32 = 125;
const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000;
static MILLIS_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static TIMERS: Mutex<RefCell<[Option<u32>; 7]>> = Mutex::new(RefCell::new([None; 7]));
static READER: Mutex<RefCell<Option<Rx>>> = Mutex::new(RefCell::new(None));

pub fn init(tc0: arduino_uno::pac::TC0) {
    // https://rahix.github.io/avr-hal/src/avr_hal_generic/usart.rs.html#439
    // https://github.com/Rahix/avr-hal/blob/master/chips/atmega328p-hal/src/lib.rs#L210
    // https://rahix.github.io/avr-hal/atmega328p_hal/pac/usart0/index.html
    // https://docs.rs/avr-device/0.3.0/avr_device/atmega328p/usart0/ucsr0b/struct.W.html
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(TIMER_COUNTS as u8) });
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    });
}

#[avr_device::interrupt(atmega328p)]
fn USART_RX(){
    crate::logger::_print("serial");
}
#[avr_device::interrupt(atmega328p)]
fn USART_TX(){
}

pub fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

#[derive(Debug)]
#[repr(transparent)]
struct Volatile<T: Copy>(UnsafeCell<T>);

impl<T: Copy> Volatile<T> {
    pub fn new(value: T) -> Volatile<T> {
        Volatile(UnsafeCell::new(value))
    }

    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.0.get()) }
    }

    pub fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.0.get(), value) };
    }
}

// NOTE `*const ()` is &Volatile<bool>
static VTABLE: RawWakerVTable = {
    unsafe fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    unsafe fn wake(p: *const ()) {
        wake_by_ref(p)
    }
    unsafe fn wake_by_ref(p: *const ()) {
        (*(p as *const Volatile<bool>)).write(true)
    }
    unsafe fn drop(_: *const ()) {
        // no-op
    }

    RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};

/// Spawns a task and blocks until the future resolves, returning its result.
pub fn block_on<T>(task: impl Future<Output = T>) -> T {
    let ready = Volatile::new(true);
    let waker = unsafe { Waker::from_raw(RawWaker::new(&ready as *const _ as *const _, &VTABLE)) };
    let mut context = Context::from_waker(&waker);
    futures::pin_mut!(task);
    let mut task = task;
    loop {
        // crate::logger::_print("timer");
        let timers = avr_device::interrupt::free(|cs| TIMERS.borrow(cs).borrow().clone());
        let now = millis();
        for timer in timers.iter() {
            if let Some(tick) = timer {
                if *tick <= now {
                    ready.write(true);
                    break;
                }
            }
        }
        while ready.read() {
            // crate::logger::_print("ready");
            // poll の中で wake できるようにここで false にする
            ready.write(false);
            match task.as_mut().poll(&mut context) {
                Poll::Ready(val) => {
                    return val;
                }
                Poll::Pending => {}
            }
        }
    }
}

pub fn delay_ms(tick: u32) -> Timeout {
    Timeout::new(tick)
}

#[derive(Clone)]
pub struct Timeout(u32);

impl Timeout {
    fn new(tick: u32) -> Self {
        let tick = millis() + tick;
        avr_device::interrupt::free(|cs| {
            let mut times = TIMERS.borrow(cs).borrow_mut();
            for timer in times.iter_mut() {
                if timer.is_none() {
                    *timer = Some(tick);
                    break;
                }
            }
        });
        Self(tick)
    }
}

impl Drop for Timeout {
    fn drop(&mut self) {
        avr_device::interrupt::free(|cs| {
            let mut times = TIMERS.borrow(cs).borrow_mut();
            for timer in times.iter_mut() {
                if *timer == Some(self.0) {
                    *timer = None;
                    break;
                }
            }
        });
    }
}

impl Future for Timeout {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 <= millis() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[derive(derive_more::From)]
pub struct AsyncUsartReader<Rx: embedded_hal::serial::Read<u8> + Unpin>(Rx);

impl<Rx: embedded_hal::serial::Read<u8> + Unpin> Stream for AsyncUsartReader<Rx> {
    type Item = u8;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.0.read() {
            Ok(byte) => {
                cx.waker().wake_by_ref();
                Poll::Ready(Some(byte))
            },
            Err(nb::Error::Other(_void)) => {
                Poll::Ready(None)
            },
            Err(nb::Error::WouldBlock) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}
