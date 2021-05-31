#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(unused_variables)]
#![allow(dead_code)]

use arduino_uno::prelude::*;
use arduino_uno::{Peripherals, Pins, Serial};

#[macro_use]
mod logger;
mod aio;
mod executor;
mod millis;
mod panic;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    millis::millis_init(dp.TC0);
    let mut pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let serial = Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );
    let (rx, mut tx) = serial.split();
    // TWBR = 24
    // 400kHz I2C clock (200kHz if CPU is 8MHz)
    let mut i2c = arduino_uno::I2cMaster::new(
        dp.TWI,
        pins.a4.into_pull_up_input(&mut pins.ddr),
        pins.a5.into_pull_up_input(&mut pins.ddr),
        400000,
    );
    // -    0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
    // 00:       -- -- -- -- -- -- -- -- -- -- -- -- -- --
    // 10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    // 20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    // 30: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    // 40: 40 -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    // 50: -- -- -- -- 54 -- -- -- -- -- -- -- -- -- -- --
    // 60: -- -- -- -- -- -- -- -- 68 -- -- -- -- -- -- --
    // 70: 70 -- -- -- -- -- -- --
    i2c.i2cdetect(&mut tx, atmega328p_hal::i2c::Direction::Write)
        .unwrap();
    logger::logger_init(tx);

    // let delay = atmega328p_hal::delay::Delay::<atmega328p_hal::clock::MHz16>::new();
    // let i2cdev = i2cdev::I2cDev::new(i2c, delay);
    // let delay = atmega328p_hal::delay::Delay::<atmega328p_hal::clock::MHz16>::new();
    // let mut mpu = mpu6050::Mpu6050::new(i2cdev, delay, None);
    // mpu.initialize().unwrap();
    // if mpu.test_connection().unwrap() {
    //     ufmt::uwriteln!(&mut serial, "MPU successful").void_unwrap();
    // } else {
    //     ufmt::uwriteln!(&mut serial, "MPU failed").void_unwrap();
    // }
    // mpu.dmp_initialize().unwrap();
    // mpu.setZAccelOffset(EEPROMReadInt(MPUCALIB + 4));
    // mpu.setXGyroOffset(EEPROMReadInt(MPUCALIB + 6));
    // mpu.setYGyroOffset(EEPROMReadInt(MPUCALIB + 8));
    // mpu.setZGyroOffset(EEPROMReadInt(MPUCALIB + 10));
    let mut rx = aio::serial::AsyncUsartReader::new(rx);
    executor::block_on(async move {
        aio::timer::Timeout::new(1000).await;
        aio::timer::Timeout::new(1000).await;
        aio::timer::Timeout::new(1000).await;
        use futures::stream::StreamExt;
        for b in rx.next().await {
            uprintln!("Got {}!\r", b);
            aio::timer::delay(1000).await;
        }
    });
    // loop {
    //     uprintln!("waiting input");
    //     // let b = nb::block!(rx.read()).void_unwrap();
    //     // uprintln!("Got {}!\r", b);

    //     // let acc = mpu.get_acc_angles().unwrap(); // get roll and pitch estimate
    //     // let temp = mpu.get_temp().unwrap(); // get sensor temp
    //     // let gyro = mpu.get_gyro().unwrap(); // get gyro data, scaled with sensitivity
    //     // // let acc = mpu.get_acc().unwrap(); // get accelerometer data, scaled with sensitivity
    //     // for f in acc.as_slice(){
    //     //     ufmt::uwriteln!(&mut serial, "r/p: {}", (*f * 255_f32) as u16).void_unwrap();
    //     // }
    //     // ufmt::uwriteln!(&mut serial, "temp: {}c", (temp* 255_f32) as u16).void_unwrap();
    //     // for f in gyro.as_slice(){
    //     //     ufmt::uwriteln!(&mut serial, "gyro: {}", (*f* 255_f32) as u16).void_unwrap();
    //     // }

    //     // ufmt::uwriteln!(&mut serial, "temp: {}c", temp).void_unwrap();
    //     // ufmt::uwriteln!(&mut serial, "gyro: {}", gyro).void_unwrap();
    //     // ufmt::uwriteln!(&mut serial, "acc: {}", acc).void_unwrap();
    // }
    panic!("ended");
}
