#![no_std]

use embedded_hal::blocking::i2c::{WriteRead, Write};
use embedded_hal::blocking::delay::DelayMs;

pub const BUFFER_LENGTH:usize  = 32;
pub const DEFAULT_READ_TIMEOUT: u16 = 1000;

pub struct I2cDev<I2c, Delay>{
    i2c: I2c,
    #[allow(dead_code)]
    delay: Delay
}

impl<I2c, Delay> I2cDev<I2c, Delay>
where
    I2c: Write + WriteRead,
    // <I2c as WriteRead>::Error: core::fmt::Debug,
    // <I2c as Write>::Error: core::fmt::Debug,
    Delay: DelayMs<u16>
{
    pub fn new(i2c: I2c, delay: Delay) -> Self {
        Self{
            i2c,
            delay
        }
    }
    // /** Read a single bit from an 8-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register regAddr to read from
    // * @param bitNum Bit position to read (0-7)
    // * @param data Container for single bit value
    // * @param timeout Optional read timeout in milliseconds (0 to disable, leave off to use default class value in I2Cdev::readTimeout)
    // * @return Status of read operation (true = success)
    // */
    // int8_t I2Cdev::readBit(uint8_t devAddr, uint8_t regAddr, uint8_t bitNum, uint8_t *data, uint16_t timeout) {
    pub fn read_bit(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        bit_num: u8,
        timeout: Option<u16>,
    ) -> Result<bool, ()> {
        let byte = self.read_byte(dev_addr, reg_addr, timeout)?;
        Ok(get_bit(byte, bit_num))
    }
    // /** Read a single bit from a 16-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register regAddr to read from
    // * @param bitNum Bit position to read (0-15)
    // * @param data Container for single bit value
    // * @param timeout Optional read timeout in milliseconds (0 to disable, leave off to use default class value in I2Cdev::readTimeout)
    // * @return Status of read operation (true = success)
    // */
    // int8_t I2Cdev::readBitW(uint8_t devAddr, uint8_t regAddr, uint8_t bitNum, uint16_t *data, uint16_t timeout) {
    pub fn read_bit_word(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        bit_num: u8,
        timeout: Option<u16>,
    ) -> Result<bool, ()> {
        let byte = self.read_word(dev_addr, reg_addr, timeout)?;
        Ok(get_bit_word(byte, bit_num))
    }
    // /** Read multiple bits from an 8-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register regAddr to read from
    // * @param bitStart First bit position to read (0-7)
    // * @param length Number of bits to read (not more than 8)
    // * @param data Container for right-aligned value (i.e. '101' read from any bitStart position will equal 0x05)
    // * @param timeout Optional read timeout in milliseconds (0 to disable, leave off to use default class value in I2Cdev::readTimeout)
    // * @return Status of read operation (true = success)
    // */
    // int8_t I2Cdev::readBits(uint8_t devAddr, uint8_t regAddr, uint8_t bitStart, uint8_t length, uint8_t *data, uint16_t timeout) {
    pub fn read_bits(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        bit_start: u8,
        length: u8,
        timeout: Option<u16>,
    ) -> Result<u8, ()> {
        // assert!(length <= 7);
        let byte = self.read_byte(dev_addr, reg_addr, timeout)?;
        Ok(get_bits(byte, bit_start, length))
    }
    /** Read multiple bits from a 16-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register regAddr to read from
    // * @param bitStart First bit position to read (0-15)
    // * @param length Number of bits to read (not more than 16)
    // * @param data Container for right-aligned value (i.e. '101' read from any bitStart position will equal 0x05)
    // * @param timeout Optional read timeout in milliseconds (0 to disable, leave off to use default class value in I2Cdev::readTimeout)
    // * @return Status of read operation (1 = success, 0 = failure, -1 = timeout)
    // */
    // int8_t I2Cdev::readBitsW(uint8_t devAddr, uint8_t regAddr, uint8_t bitStart, uint8_t length, uint16_t *data, uint16_t timeout) {
    pub fn read_bits_word(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        bit_start: u8,
        length: u8,
        timeout: Option<u16>,
    ) -> Result<u16, ()> {
        // assert!(length <= 7);
        let mut bytes: [u16; 1] = [0; 1];
        self.read_words(dev_addr, reg_addr, &mut bytes, timeout)?;
        Ok(get_bits_word(bytes[0], bit_start, length))
    }
    // /** Read single byte from an 8-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register regAddr to read from
    // * @param data Container for byte value read from device
    // * @param timeout Optional read timeout in milliseconds (0 to disable, leave off to use default class value in I2Cdev::readTimeout)
    // * @return Status of read operation (true = success)
    // */
    // int8_t I2Cdev::readByte(uint8_t devAddr, uint8_t regAddr, uint8_t *data, uint16_t timeout) {
    pub fn read_byte(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        timeout: Option<u16>,
    ) -> Result<u8, ()> {
        // assert!(length <= 7);
        let mut bytes: [u8; 1] = [0; 1];
        self.read_bytes(dev_addr, reg_addr, &mut bytes, timeout)?;
        Ok(bytes[0])
    }
    // /** Read single word from a 16-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register regAddr to read from
    // * @param data Container for word value read from device
    // * @param timeout Optional read timeout in milliseconds (0 to disable, leave off to use default class value in I2Cdev::readTimeout)
    // * @return Status of read operation (true = success)
    // */
    // int8_t I2Cdev::readWord(uint8_t devAddr, uint8_t regAddr, uint16_t *data, uint16_t timeout) {
    pub fn read_word(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        timeout: Option<u16>,
    ) -> Result<u16, ()> {
        // assert!(length <= 7);
        let mut bytes: [u16; 1] = [0; 1];
        self.read_words(dev_addr, reg_addr, &mut bytes, timeout)?;
        Ok(bytes[0])
    }

    // /** Read multiple bytes from an 8-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr First register regAddr to read from
    // * @param length Number of bytes to read
    // * @param data Buffer to store read data in
    // * @param timeout Optional read timeout in milliseconds (0 to disable, leave off to use default class value in I2Cdev::readTimeout)
    // * @return Number of bytes read (-1 indicates failure)
    // */
    // int8_t I2Cdev::readBytes(uint8_t devAddr, uint8_t regAddr, uint8_t length, uint8_t *data, uint16_t timeout) {
    pub fn read_bytes(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        buf: &mut [u8],
        // @todo https://github.com/rust-embedded/embedded-hal/issues/50
        _timeout: Option<u16>,
    ) -> Result<(), ()> {
        self.i2c
            .write_read(dev_addr, &[reg_addr], buf)
            .map_err(|_| ())
    }

    // /** Read multiple words from a 16-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr First register regAddr to read from
    // * @param length Number of words to read
    // * @param data Buffer to store read data in
    // * @param timeout Optional read timeout in milliseconds (0 to disable, leave off to use default class value in I2Cdev::readTimeout)
    // * @return Number of words read (-1 indicates failure)
    // */
    // int8_t I2Cdev::readWords(uint8_t devAddr, uint8_t regAddr, uint8_t length, uint16_t *data, uint16_t timeout) {
    #[allow(unused_variables)]
    pub fn read_words(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        buf: &mut [u16],
        // @todo https://github.com/rust-embedded/embedded-hal/issues/50
        _timeout: Option<u16>,
    ) -> Result<(), ()> {
        unimplemented!()
        // is it works?
        // let mut bytes: [u8; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
        // self.i2c
        //     .write_read(dev_addr, &[reg_addr], &mut bytes)
        //     .map_err(|_| ())?;
        // for (i, b) in buf.iter_mut().enumerate() {
        //     *b = ((bytes[i*2] as u16) << 8) | bytes[i*2+1] as u16;
        // }
        // Ok(())
    }

        
    // /** write a single bit in an 8-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register regAddr to write to
    // * @param bitNum Bit position to write (0-7)
    // * @param value New bit value to write
    // * @return Status of operation (true = success)
    // */
    // bool I2Cdev::writeBit(uint8_t devAddr, uint8_t regAddr, uint8_t bitNum, uint8_t data) {
    pub fn write_bit(&mut self, dev_addr: u8, reg_addr: u8, bit_num: u8, data: bool) -> Result<(), ()> {
        let mut byte: [u8; 1] = [0; 1];
        self.read_bytes(dev_addr, reg_addr, &mut byte, None)?;
        set_bit(&mut byte[0], bit_num, data);
        self.write_byte(dev_addr, reg_addr, byte[0])?;
        Ok(())
    }
    // /** write a single bit in a 16-bit device register.
    //  * @param devAddr I2C slave device address
    //  * @param regAddr Register regAddr to write to
    //  * @param bitNum Bit position to write (0-15)
    //  * @param value New bit value to write
    //  * @return Status of operation (true = success)
    //  */
    // bool I2Cdev::writeBitW(uint8_t devAddr, uint8_t regAddr, uint8_t bitNum, uint16_t data) {
    #[allow(unused_variables)]
    pub fn write_bit_word(&mut self, dev_addr: u8, reg_addr: u8, bit_num: u8, data: bool) -> Result<(), ()> {
        unimplemented!()
    }
    // /** Write multiple bits in an 8-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register regAddr to write to
    // * @param bitStart First bit position to write (0-7)
    // * @param length Number of bits to write (not more than 8)
    // * @param data Right-aligned value to write
    // * @return Status of operation (true = success)
    // */
    // bool I2Cdev::writeBits(uint8_t devAddr, uint8_t regAddr, uint8_t bitStart, uint8_t length, uint8_t data) {
    pub fn write_bits(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        bit_start: u8,
        length: u8,
        data: u8,
    ) -> Result<(), ()> {
        let mut byte: [u8; 1] = [0; 1];
        self.read_bytes(dev_addr, reg_addr, &mut byte, None)?;
        set_bits(&mut byte[0], bit_start, length, data);
        self.write_byte(dev_addr, reg_addr, byte[0])?;
        Ok(())
    }
    // /** Write multiple bits in a 16-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register regAddr to write to
    // * @param bitStart First bit position to write (0-15)
    // * @param length Number of bits to write (not more than 16)
    // * @param data Right-aligned value to write
    // * @return Status of operation (true = success)
    // */
    // bool I2Cdev::writeBitsW(uint8_t devAddr, uint8_t regAddr, uint8_t bitStart, uint8_t length, uint16_t data) {
    #[allow(unused_variables)]
    pub fn write_bits_words(
        &mut self,
        dev_addr: u8,
        reg_addr: u8,
        bit_start: u8,
        length: u8,
        data: u16,
    ) -> Result<(), ()> {
        unimplemented!()
    }
    // /** Write single byte to an 8-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register address to write to
    // * @param data New byte value to write
    // * @return Status of operation (true = success)
    // */
    // bool I2Cdev::writeByte(uint8_t devAddr, uint8_t regAddr, uint8_t data) {
    pub fn write_byte(&mut self, dev_addr: u8, reg_addr: u8, data: u8) -> Result<(), ()> {
        self.i2c
            .write(dev_addr, &[reg_addr, data])
            .map_err(|_| ())?;
        Ok(())
    }
    // /** Write single word to a 16-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr Register address to write to
    // * @param data New word value to write
    // * @return Status of operation (true = success)
    // */
    // bool I2Cdev::writeWord(uint8_t devAddr, uint8_t regAddr, uint16_t data) {
    #[allow(unused_variables)]
    pub fn write_byte_word(&mut self, dev_addr: u8, reg_addr: u8, data: u16) -> Result<(), ()> {
        unimplemented!()
    }
    // /** Write multiple bytes to an 8-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr First register address to write to
    // * @param length Number of bytes to write
    // * @param data Buffer to copy new data from
    // * @return Status of operation (true = success)
    // */
    // bool I2Cdev::writeBytes(uint8_t devAddr, uint8_t regAddr, uint8_t length, uint8_t* data) {
    #[allow(unused_variables)]
    pub fn write_bytes(&mut self, dev_addr: u8, reg_addr: u8, data: &[u8]) -> Result<(), ()> {
        unimplemented!()
    }
    // /** Write multiple words to a 16-bit device register.
    // * @param devAddr I2C slave device address
    // * @param regAddr First register address to write to
    // * @param length Number of words to write
    // * @param data Buffer to copy new data from
    // * @return Status of operation (true = success)
    // */
    // bool I2Cdev::writeWords(uint8_t devAddr, uint8_t regAddr, uint8_t length, uint16_t* data) {
    #[allow(unused_variables)]
    pub fn write_words(&mut self, dev_addr: u8, reg_addr: u8, data: &[u8]) -> Result<(), ()> {
        unimplemented!()
    }
}

/// get bit n of byte
fn get_bit(byte: u8, n: u8) -> bool {
    // assert!(n <= 7);
    ((byte >> n) & 1) == 0b1
}
#[test]
fn get_bit() {
    assert_eq!(get_bit(0b10010010, 0), 0b0);
    assert_eq!(get_bit(0b10010010, 1), 0b1);
    assert_eq!(get_bit(0b10010010, 2), 0b0);
    assert_eq!(get_bit(0b10010010, 3), 0b0);
    assert_eq!(get_bit(0b10010010, 4), 0b1);
    assert_eq!(get_bit(0b10010010, 5), 0b0);
    assert_eq!(get_bit(0b10010010, 6), 0b0);
    assert_eq!(get_bit(0b10010010, 7), 0b1);
}
/// get bits start - start+length from byte
// 01101001 read byte
// 76543210 bit numbers
//    xxx   args: bitStart=4, length=3
//    010   masked
//   -> 010 shifted
fn get_bits(mut byte: u8, bit_start: u8, length: u8) -> u8 {
    // assert!(bit_start <= 7);
    // assert!(length <= 7);
    // assert!(bit_start < length);
    let mask_shift: u8 = if bit_start < length {
        0
    } else {
        bit_start - length + 1
    };
    let mask: u8 = ((1 << length) - 1) << mask_shift;
    byte &= mask;
    byte >>= mask_shift;
    byte
}

#[test]
fn get_bits() {
    assert_eq!(get_bits(0b10010010, 7, 7), 0b1001001);
    assert_eq!(get_bits(0b10010010, 7, 6), 0b100100);
    assert_eq!(get_bits(0b10010010, 7, 5), 0b10010);
    assert_eq!(get_bits(0b10010010, 7, 4), 0b1001);
    assert_eq!(get_bits(0b10010010, 7, 3), 0b100);
    assert_eq!(get_bits(0b10010010, 7, 2), 0b10);
    assert_eq!(get_bits(0b10010010, 7, 1), 0b1);
    assert_eq!(get_bits(0b10010010, 2, 3), 0b010);
    assert_eq!(get_bits(0b10010010, 2, 2), 0b01);
    assert_eq!(get_bits(0b10010010, 2, 1), 0b0);
    assert_eq!(get_bits(0b10010010, 1, 2), 0b10);
    assert_eq!(get_bits(0b10010010, 1, 1), 0b1);
}

fn get_bit_word(byte: u16, n: u8) -> bool {
    // assert!(n <= 7);
    ((byte >> n) & 0b1) == 0b1
}

fn get_bits_word(mut byte: u16, bit_start: u8, length: u8) -> u16 {
    // assert!(bit_start <= 7);
    // assert!(length <= 7);
    // assert!(bit_start < length);
    let mask_shift: u8 = if bit_start < length {
        0
    } else {
        bit_start - length + 1
    };
    let mask: u16 = ((1 << length) - 1) << mask_shift;
    byte &= mask;
    byte >>= mask_shift;
    byte
}

/// set bit n in byte
pub fn set_bit(byte: &mut u8, n: u8, enable: bool) {
    if enable {
        *byte |= 1_u8 << n;
    } else {
        *byte &= !(1_u8 << n);
    }
}
#[test]
fn set_bit(){
    assert_eq!(set_bit(0b00000000, 7, 8), 0b00000011);
    assert_eq!(set_bit(0b00000000, 7, 7), 0b00000010);
}

/// Fill bits bitstart-bitstart+length in byte with data
//      010 value to write
// 76543210 bit numbers
//    xxx   args: bitStart=4, length=3
// 00011100 mask byte
// 10101111 original value (sample)
// 10100011 original & ~mask
// 10101011 masked | value
fn set_bits(byte: &mut u8, bit_start: u8, length: u8, mut data: u8) {
    // without mask_shift, strange behavior occurs, wenn bit_start < length.
    // e.g. bit_start=2, length = 2
    // in SOME cases, you get an 'attempt to subtract with overflow' exception, when
    // bitstart - length + 1 = 0
    // therefore just "cut off" at 0 shift
    let mask_shift: u8 = if bit_start < length {
        0
    } else {
        bit_start - length + 1
    };
    let mask: u8 = ((1 << length) - 1) << mask_shift;
    data <<= mask_shift; // shift data into correct position
    data &= mask; // zero all non-important bits in data
    *byte &= !(mask); // zero all important bits in existing byte
    *byte |= data; // combine data with existing byte
}
