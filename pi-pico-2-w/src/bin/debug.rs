#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    // RP2350初期化（クロック/割り込みなど）
    let _p = embassy_rp::init(Default::default());

    let mut n: u32 = 0;
    loop {
        defmt::info!("tick: {}", n);
        n = n.wrapping_add(1);
        Timer::after(Duration::from_secs(1)).await;
    }
}
