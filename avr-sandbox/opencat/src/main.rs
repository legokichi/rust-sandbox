#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(const_in_array_repeat_expressions)]
#![allow(unused_variables)]
#![allow(dead_code)]

use arduino_uno::prelude::*;
use arduino_uno::{Peripherals, Pins, Serial};
use avr_hal_generic::usart::Event::{RxComplete, DataRegisterEmpty};
// #[macro_use]
mod logger;
// mod aio;
mod executor;
// mod millis;
// mod panic;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut serial = Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );
    serial.listen(RxComplete);
    serial.listen(DataRegisterEmpty);
    let (rx, mut tx) = serial.split();
    
    // TWBR = 24
    // 400kHz I2C clock (200kHz if CPU is 8MHz)
    // https://github.com/rust-embedded/embedded-hal/issues/50
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
    // i2c.i2cdetect(&mut tx, atmega328p_hal::i2c::Direction::Write)
    //     .unwrap();
    logger::init(tx);
    executor::init(dp.TC0);
    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

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
    // let mut buffer = ryu::Buffer::new();
    // let printed = buffer.format(1.234);
// assert_eq!(printed, "1.234");

    // let task1 = async {
    //     loop {
    //         logger::_print("periodic");
    //         executor::delay_ms(1000).await;
    //     }
    // };
    // let task2 = async {
    //     for i in 0..10 {
    //         logger::_print("countdowning");
    //         executor::delay_ms(300).await;
    //     }
    //     logger::_print("countdown end");
    // };
    // let task3 = async{
    //     let mut rx = executor::AsyncUsartReader::from(rx);
    //     use futures::StreamExt;
    //     while let Some(b) = rx.next().await {
    //         logger::_print("gotcha");
    //         // ufmt::uwriteln!(&mut tx, "done: {}", b).void_unwrap();
    //         executor::delay_ms(300).await;
    //     }
    // };
    // executor::block_on(futures::future::join3(task1, task2, task3));
    // // executor::block_on(task3);

        // avr_device::interrupt::free(move |cs| {
        //     if let Some(tx) = LOGGER.borrow(cs).borrow_mut().as_mut() {
        //         tx.write_str("hi").unwrap();
        //     }
        // });
        // uprintln!("waiting input");
        // logger::_printfn("waiting input\n");
        // logger::_print(format_args!("waiting input\n"));
        // let b = nb::block!(rx.read()).void_unwrap();
        // ufmt::uwriteln!(&mut tx, "got{}", b).void_unwrap();
        // uprintln!("Got {}!\r", b);

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
        // ufmt::uwriteln!(&mut tx, "acc").void_unwrap();
    loop {
        logger::_print("ended");
        arduino_uno::delay_ms(1000);
    }
}
