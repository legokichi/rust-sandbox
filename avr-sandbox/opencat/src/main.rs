#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(dead_code)]


use arduino_uno::prelude::*;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );
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
    i2c.i2cdetect(&mut serial, atmega328p_hal::i2c::Direction::Write)
        .unwrap();

    let delay = atmega328p_hal::delay::Delay::<atmega328p_hal::clock::MHz16>::new();
    let i2cdev = i2cdev::I2cDev::new(i2c, delay);
    let delay = atmega328p_hal::delay::Delay::<atmega328p_hal::clock::MHz16>::new();
    let mut mpu = mpu6050::Mpu6050::new(i2cdev, delay, None);
    mpu.initialize().unwrap();
    if mpu.test_connection().unwrap() {
        ufmt::uwriteln!(&mut serial, "MPU successful").void_unwrap();
    } else {
        ufmt::uwriteln!(&mut serial, "MPU failed").void_unwrap();
    }
    mpu.dmp_initialize().unwrap();
    // mpu.setZAccelOffset(EEPROMReadInt(MPUCALIB + 4));
    // mpu.setXGyroOffset(EEPROMReadInt(MPUCALIB + 6));
    // mpu.setYGyroOffset(EEPROMReadInt(MPUCALIB + 8));
    // mpu.setZGyroOffset(EEPROMReadInt(MPUCALIB + 10));

    loop {
        ufmt::uwriteln!(&mut serial, "waiting input").void_unwrap();
        let b = nb::block!(serial.read()).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();

        // let acc = mpu.get_acc_angles().unwrap(); // get roll and pitch estimate
        // let temp = mpu.get_temp().unwrap(); // get sensor temp
        // let gyro = mpu.get_gyro().unwrap(); // get gyro data, scaled with sensitivity
        // // let acc = mpu.get_acc().unwrap(); // get accelerometer data, scaled with sensitivity
        // for f in acc.as_slice(){
        //     ufmt::uwriteln!(&mut serial, "r/p: {}", (*f * 255_f32) as u16).void_unwrap();
        // }
        // ufmt::uwriteln!(&mut serial, "temp: {}c", (temp* 255_f32) as u16).void_unwrap();
        // for f in gyro.as_slice(){
        //     ufmt::uwriteln!(&mut serial, "gyro: {}", (*f* 255_f32) as u16).void_unwrap();
        // }

        // ufmt::uwriteln!(&mut serial, "temp: {}c", temp).void_unwrap();
        // ufmt::uwriteln!(&mut serial, "gyro: {}", gyro).void_unwrap();
        // ufmt::uwriteln!(&mut serial, "acc: {}", acc).void_unwrap();
        arduino_uno::delay_ms(100);
    }
}

use core::fmt::{self, Write};

#[macro_export]
macro_rules! print {
    ($writer:expr, $($arg:tt)*) => ($crate::_print($writer, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($writer:expr, $fmt:expr) => (print!($writer, concat!($fmt, "\n")));
    ($writer:expr, $fmt:expr, $($arg:tt)*) => (print!($writer, concat!($fmt, "\n"), $($arg)*));
}

pub fn _print(
    serial: &mut arduino_uno::Serial<atmega328p_hal::port::mode::Floating>,
    args: fmt::Arguments,
) {
    let mut writer = UartWriter(serial);
    writer.write_fmt(args).unwrap();
}

struct UartWriter<'a>(&'a mut arduino_uno::Serial<atmega328p_hal::port::mode::Floating>);

impl Write for UartWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        ufmt::uwriteln!(&mut self.0, "{}", s).unwrap();
        Ok(())
    }
}
