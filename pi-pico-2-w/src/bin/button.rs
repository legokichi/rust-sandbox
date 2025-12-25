#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::Pio;
use embassy_time::Timer;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use cyw43_pio::PioSpi;

use embassy_rp::bind_interrupts;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());

    // GP15 uses an internal pull-up; wire the tact switch to GND.
    // GP15 --- SWITCH --- GND
    let mut button = Input::new(p.PIN_15, Pull::Up);

    // Pico 2 W onboard LED is on the CYW43 (WL_GPIO0).
    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        cyw43_pio::DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());

    let fw = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/cyw43-firmware/43439A0.bin"
    ));
    let clm = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/cyw43-firmware/43439A0_clm.bin"
    ));

    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(cyw43_task(runner)).unwrap();
    control.init(clm).await;

    control.gpio_set(0, false).await;

    loop {
        button.wait_for_falling_edge().await;
        Timer::after_millis(20).await;
        if !button.is_low() {
            continue;
        }

        defmt::info!("button: pressed");
        control.gpio_set(0, true).await;
        Timer::after_millis(50).await;
        control.gpio_set(0, false).await;

        button.wait_for_rising_edge().await;
        Timer::after_millis(20).await;
    }
}
