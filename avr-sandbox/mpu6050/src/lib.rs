#![no_std]

use i2cdev::I2cDev;
use embedded_hal::blocking::i2c::{WriteRead, Write};
use embedded_hal::blocking::delay::DelayMs;

pub struct Mpu6050<I2c, Delay> {
    i2c: I2cDev<I2c, Delay>,
    delay: Delay,
    dev_addr: u8,
}

impl<I2c, Delay> Mpu6050<I2c, Delay>
where
    I2c: Write + WriteRead,
    // <I2c as WriteRead>::Error: core::fmt::Debug,
    // <I2c as Write>::Error: core::fmt::Debug,
    Delay: DelayMs<u16>
{
    pub fn new(i2c: I2cDev<I2c, Delay>, delay: Delay, dev_addr: Option<u8>) -> Self {
        Self {
            i2c,
            delay,
            dev_addr: dev_addr.unwrap_or(DEFAULT_ADDRESS),
        }
    }

    // /** Power on and prepare for general usage.
    // * This will activate the device and take it out of sleep mode (which must be done
    // * after start-up). This function also sets both the accelerometer and the gyroscope
    // * to their most sensitive settings, namely +/- 2g and +/- 250 degrees/sec, and sets
    // * the clock source to use the X Gyro for reference, which is slightly better than
    // * the default internal clock source.
    // */
    pub fn initialize(&mut self) -> Result<(), ()> {
        self.set_clock_source(CLOCK_PLL_XGYRO)?;
        self.set_full_scale_gyro_range(GYRO_FS_250)?;
        self.set_full_scale_accel_range(ACCEL_FS_2)?;
        self.set_sleep_enabled(false)?; // thanks to Jack Elston for pointing this one out!
        Ok(())
    }
    // /** Verify the I2C connection.
    // * Make sure the device is connected and responds as expected.
    // * @return True if connection is valid, false otherwise
    // */
    // bool MPU6050::testConnection() {
    pub fn test_connection(&mut self) -> Result<bool, ()> {
        Ok(self.get_device_id()? == 0x34)
    }

    // /** Set full-scale gyroscope range.
    // * @param range New full-scale gyroscope range value
    // * @see getFullScaleRange()
    // * @see MPU6050_GYRO_FS_250
    // * @see MPU6050_RA_GYRO_CONFIG
    // * @see MPU6050_GCONFIG_FS_SEL_BIT
    // * @see MPU6050_GCONFIG_FS_SEL_LENGTH
    // */
    // void MPU6050::setFullScaleGyroRange(uint8_t range) {
    pub fn set_full_scale_gyro_range(&mut self, range: u8) -> Result<(), ()> {
        self.i2c.write_bits(
            self.dev_addr,
            RA_GYRO_CONFIG,
            GCONFIG_FS_SEL_BIT,
            GCONFIG_FS_SEL_LENGTH,
            range,
        )
    }

    // /** Set full-scale accelerometer range.
    // * @param range New full-scale accelerometer range setting
    // * @see getFullScaleAccelRange()
    // */
    // void MPU6050::setFullScaleAccelRange(uint8_t range) {
    pub fn set_full_scale_accel_range(&mut self, range: u8) -> Result<(), ()> {
        self.i2c.write_bits(
            self.dev_addr,
            RA_ACCEL_CONFIG,
            ACONFIG_AFS_SEL_BIT,
            ACONFIG_AFS_SEL_LENGTH,
            range,
        )
    }

    // /** Set sleep mode status.
    // * @param enabled New sleep mode enabled status
    // * @see getSleepEnabled()
    // * @see MPU6050_RA_PWR_MGMT_1
    // * @see MPU6050_PWR1_SLEEP_BIT
    // */
    // void MPU6050::setSleepEnabled(bool enabled) {
    pub fn set_sleep_enabled(&mut self, enabled: bool) -> Result<(), ()> {
        self.i2c.write_bit(
            self.dev_addr,
            RA_PWR_MGMT_1,
            PWR1_SLEEP_BIT,
            enabled,
        )
    }

    // /** Set clock source setting.
    // * An internal 8MHz oscillator, gyroscope based clock, or external sources can
    // * be selected as the MPU-60X0 clock source. When the internal 8 MHz oscillator
    // * or an external source is chosen as the clock source, the MPU-60X0 can operate
    // * in low power modes with the gyroscopes disabled.
    // *
    // * Upon power up, the MPU-60X0 clock source defaults to the internal oscillator.
    // * However, it is highly recommended that the device be configured to use one of
    // * the gyroscopes (or an external clock source) as the clock reference for
    // * improved stability. The clock source can be selected according to the following table:
    // *
    // * <pre>
    // * CLK_SEL | Clock Source
    // * --------+--------------------------------------
    // * 0       | Internal oscillator
    // * 1       | PLL with X Gyro reference
    // * 2       | PLL with Y Gyro reference
    // * 3       | PLL with Z Gyro reference
    // * 4       | PLL with external 32.768kHz reference
    // * 5       | PLL with external 19.2MHz reference
    // * 6       | Reserved
    // * 7       | Stops the clock and keeps the timing generator in reset
    // * </pre>
    // *
    // * @param source New clock source setting
    // * @see getClockSource()
    // * @see MPU6050_RA_PWR_MGMT_1
    // * @see MPU6050_PWR1_CLKSEL_BIT
    // * @see MPU6050_PWR1_CLKSEL_LENGTH
    // */
    // void MPU6050::setClockSource(uint8_t source) {
    pub fn set_clock_source(&mut self, source: u8) -> Result<(), ()> {
        self.i2c.write_bits(
            self.dev_addr,
            RA_PWR_MGMT_1,
            PWR1_CLKSEL_BIT,
            PWR1_CLKSEL_LENGTH,
            source,
        )
    }
    

    // WHO_AM_I register
    // /** Get Device ID.
    // * This register is used to verify the identity of the device (0b110100, 0x34).
    // * @return Device ID (6 bits only! should be 0x34)
    // * @see MPU6050_RA_WHO_AM_I
    // * @see MPU6050_WHO_AM_I_BIT
    // * @see MPU6050_WHO_AM_I_LENGTH
    // */
    // uint8_t MPU6050::getDeviceID() {
    pub fn get_device_id(&mut self) -> Result<u8, ()> {
        self.i2c.read_bits(
            self.dev_addr,
            RA_WHO_AM_I,
            WHO_AM_I_BIT,
            WHO_AM_I_LENGTH,
            None,
        )
    }

    // BANK_SEL register
    // void setMemoryBank(uint8_t bank, bool prefetchEnabled=false, bool userBank=false);
    fn set_memory_bank(&mut self, mut bank: u8, prefetch_enabled: bool, user_bank: bool) -> Result<(), ()> {
        bank &= 0x1F;
        if user_bank {bank |= 0x20; }
        if prefetch_enabled { bank |= 0x40; }
        self.i2c.write_byte(self.dev_addr, RA_BANK_SEL, bank)
    }
    // MEM_START_ADDR register
    // void MPU6050::setMemoryStartAddress(uint8_t address) {
    fn set_memory_start_address(&mut self, address: u8) -> Result<(), ()> {
        self.i2c.write_byte(self.dev_addr, RA_MEM_START_ADDR, address)
    }

    // /** Set the I2C address of the specified slave (0-3).
    // * @param num Slave number (0-3)
    // * @param address New address for specified slave
    // * @see getSlaveAddress()
    // * @see MPU6050_RA_I2C_SLV0_ADDR
    // */
    // void MPU6050::setSlaveAddress(uint8_t num, uint8_t address) {
    fn set_slave_address(&mut self, num: u8, address: u8) -> Result<(), ()> {
        if num > 3 { return Ok(()); }
        self.i2c.write_byte(self.dev_addr,RA_I2C_SLV0_ADDR + num*3, address)
    }

    // PWR_MGMT_1 register
    // /** Trigger a full device reset.
    // * A small delay of ~50ms may be desirable after triggering a reset.
    // * @see MPU6050_RA_PWR_MGMT_1
    // * @see MPU6050_PWR1_DEVICE_RESET_BIT
    // */
    // void MPU6050::reset() {
    pub fn reset(&mut self) -> Result<(), ()> {
        self.i2c.write_bit(
            self.dev_addr,
            RA_PWR_MGMT_1,
            PWR1_DEVICE_RESET_BIT,
            true,
        )
    }


    // uint8_t MPU6050::dmpInitialize() {
    pub fn dmp_initialize(&mut self) -> Result<(), ()> {
        self.reset()?;
        self.set_sleep_enabled(true)?;
        self.delay.delay_ms(30);
        // disable sleep mode
        self.set_sleep_enabled(false)?;
        // get MPU hardware revision
        self.set_memory_bank(0x10, true, true)?;
        self.set_memory_start_address(0x06)?;
        self.set_memory_bank(0x00, false, false)?;
        // // check OTP bank valid
        // // setup weird slave stuff (?)
        self.set_slave_address(0, 0x7F)?;
        // self.setI2CMasterModeEnabled(false)?;
        self.set_slave_address(0, 0x68)?;
        // self.resetI2CMaster()?;
        self.delay.delay_ms(20);
        // self.setClockSource(MPU6050_CLOCK_PLL_ZGYRO)?;
        // self.setIntEnabled(1<<MPU6050_INTERRUPT_FIFO_OFLOW_BIT|1<<MPU6050_INTERRUPT_DMP_INT_BIT)?;
        // self.setRate(4)?; // 1khz / (1 + 4) = 200 Hz
        // self.setExternalFrameSync(MPU6050_EXT_SYNC_TEMP_OUT_L)?;
        // self.setDLPFMode(MPU6050_DLPF_BW_42)?;
        // self.setFullScaleGyroRange(MPU6050_GYRO_FS_2000)?;
        // // load DMP code into memory banks
        // self.writeProgMemoryBlock(dmpMemory, MPU6050_DMP_CODE_SIZE)?;
        // // Set the FIFO Rate Divisor int the DMP Firmware Memory
        // let dmpUpdate: [u8; 2] = [0x00, MPU6050_DMP_FIFO_RATE_DIVISOR];
        // // Lets write the dmpUpdate data to the Firmware image, we have 2 bytes to write in bank 0x02 with the Offset 0x16
        // self.writeMemoryBlock(&dmpUpdate, 0x02, 0x02, 0x16)?;
        // //write start address MSB into register
        // self.setDMPConfig1(0x03)?;
        // //write start address LSB into register
        // self.setDMPConfig2(0x00)?;
        // self.setOTPBankValid(false);
        // self.setMotionDetectionThreshold(2);
        // self.setZeroMotionDetectionThreshold(156);
        // self.setMotionDetectionDuration(80);
        // self.setZeroMotionDetectionDuration(0);
        // self.setFIFOEnabled(true);
        // self.resetDMP();
        // self.setDMPEnabled(false);
        // self.dmpPacketSize = 42;
        // self.resetFIFO();
        // self.getIntStatus();
        Ok(())
    }
}
    pub const ADDRESS_AD0_LOW: u8 = 0x68; // address pin low (GND), default for InvenSense evaluation board
    pub const ADDRESS_AD0_HIGH: u8 = 0x69; // address pin high (VCC)
    pub const DEFAULT_ADDRESS: u8 = ADDRESS_AD0_LOW;
    pub const RA_XG_OFFS_TC: u8 = 0x00; //[7] PWR_MODE, [6:1] XG_OFFS_TC, [0] OTP_BNK_VLD
    pub const RA_YG_OFFS_TC: u8 = 0x01; //[7] PWR_MODE, [6:1] YG_OFFS_TC, [0] OTP_BNK_VLD
    pub const RA_ZG_OFFS_TC: u8 = 0x02; //[7] PWR_MODE, [6:1] ZG_OFFS_TC, [0] OTP_BNK_VLD
    pub const RA_X_FINE_GAIN: u8 = 0x03; //[7:0] X_FINE_GAIN
    pub const RA_Y_FINE_GAIN: u8 = 0x04; //[7:0] Y_FINE_GAIN
    pub const RA_Z_FINE_GAIN: u8 = 0x05; //[7:0] Z_FINE_GAIN
    pub const RA_XA_OFFS_H: u8 = 0x06; //[15:0] XA_OFFS
    pub const RA_XA_OFFS_L_TC: u8 = 0x07;
    pub const RA_YA_OFFS_H: u8 = 0x08; //[15:0] YA_OFFS
    pub const RA_YA_OFFS_L_TC: u8 = 0x09;
    pub const RA_ZA_OFFS_H: u8 = 0x0A; //[15:0] ZA_OFFS
    pub const RA_ZA_OFFS_L_TC: u8 = 0x0B;
    pub const RA_SELF_TEST_X: u8 = 0x0D; //[7:5] XA_TEST[4-2], [4:0] XG_TEST[4-0]
    pub const RA_SELF_TEST_Y: u8 = 0x0E; //[7:5] YA_TEST[4-2], [4:0] YG_TEST[4-0]
    pub const RA_SELF_TEST_Z: u8 = 0x0F; //[7:5] ZA_TEST[4-2], [4:0] ZG_TEST[4-0]
    pub const RA_SELF_TEST_A: u8 = 0x10; //[5:4] XA_TEST[1-0], [3:2] YA_TEST[1-0], [1:0] ZA_TEST[1-0]
    pub const RA_XG_OFFS_USRH: u8 = 0x13; //[15:0] XG_OFFS_USR
    pub const RA_XG_OFFS_USRL: u8 = 0x14;
    pub const RA_YG_OFFS_USRH: u8 = 0x15; //[15:0] YG_OFFS_USR
    pub const RA_YG_OFFS_USRL: u8 = 0x16;
    pub const RA_ZG_OFFS_USRH: u8 = 0x17; //[15:0] ZG_OFFS_USR
    pub const RA_ZG_OFFS_USRL: u8 = 0x18;
    pub const RA_SMPLRT_DIV: u8 = 0x19;
    pub const RA_CONFIG: u8 = 0x1A;
    pub const RA_GYRO_CONFIG: u8 = 0x1B;
    pub const RA_ACCEL_CONFIG: u8 = 0x1C;
    pub const RA_FF_THR: u8 = 0x1D;
    pub const RA_FF_DUR: u8 = 0x1E;
    pub const RA_MOT_THR: u8 = 0x1F;
    pub const RA_MOT_DUR: u8 = 0x20;
    pub const RA_ZRMOT_THR: u8 = 0x21;
    pub const RA_ZRMOT_DUR: u8 = 0x22;
    pub const RA_FIFO_EN: u8 = 0x23;
    pub const RA_I2C_MST_CTRL: u8 = 0x24;
    pub const RA_I2C_SLV0_ADDR: u8 = 0x25;
    pub const RA_I2C_SLV0_REG: u8 = 0x26;
    pub const RA_I2C_SLV0_CTRL: u8 = 0x27;
    pub const RA_I2C_SLV1_ADDR: u8 = 0x28;
    pub const RA_I2C_SLV1_REG: u8 = 0x29;
    pub const RA_I2C_SLV1_CTRL: u8 = 0x2A;
    pub const RA_I2C_SLV2_ADDR: u8 = 0x2B;
    pub const RA_I2C_SLV2_REG: u8 = 0x2C;
    pub const RA_I2C_SLV2_CTRL: u8 = 0x2D;
    pub const RA_I2C_SLV3_ADDR: u8 = 0x2E;
    pub const RA_I2C_SLV3_REG: u8 = 0x2F;
    pub const RA_I2C_SLV3_CTRL: u8 = 0x30;
    pub const RA_I2C_SLV4_ADDR: u8 = 0x31;
    pub const RA_I2C_SLV4_REG: u8 = 0x32;
    pub const RA_I2C_SLV4_DO: u8 = 0x33;
    pub const RA_I2C_SLV4_CTRL: u8 = 0x34;
    pub const RA_I2C_SLV4_DI: u8 = 0x35;
    pub const RA_I2C_MST_STATUS: u8 = 0x36;
    pub const RA_INT_PIN_CFG: u8 = 0x37;
    pub const RA_INT_ENABLE: u8 = 0x38;
    pub const RA_DMP_INT_STATUS: u8 = 0x39;
    pub const RA_INT_STATUS: u8 = 0x3A;
    pub const RA_ACCEL_XOUT_H: u8 = 0x3B;
    pub const RA_ACCEL_XOUT_L: u8 = 0x3C;
    pub const RA_ACCEL_YOUT_H: u8 = 0x3D;
    pub const RA_ACCEL_YOUT_L: u8 = 0x3E;
    pub const RA_ACCEL_ZOUT_H: u8 = 0x3F;
    pub const RA_ACCEL_ZOUT_L: u8 = 0x40;
    pub const RA_TEMP_OUT_H: u8 = 0x41;
    pub const RA_TEMP_OUT_L: u8 = 0x42;
    pub const RA_GYRO_XOUT_H: u8 = 0x43;
    pub const RA_GYRO_XOUT_L: u8 = 0x44;
    pub const RA_GYRO_YOUT_H: u8 = 0x45;
    pub const RA_GYRO_YOUT_L: u8 = 0x46;
    pub const RA_GYRO_ZOUT_H: u8 = 0x47;
    pub const RA_GYRO_ZOUT_L: u8 = 0x48;
    pub const RA_EXT_SENS_DATA_00: u8 = 0x49;
    pub const RA_EXT_SENS_DATA_01: u8 = 0x4A;
    pub const RA_EXT_SENS_DATA_02: u8 = 0x4B;
    pub const RA_EXT_SENS_DATA_03: u8 = 0x4C;
    pub const RA_EXT_SENS_DATA_04: u8 = 0x4D;
    pub const RA_EXT_SENS_DATA_05: u8 = 0x4E;
    pub const RA_EXT_SENS_DATA_06: u8 = 0x4F;
    pub const RA_EXT_SENS_DATA_07: u8 = 0x50;
    pub const RA_EXT_SENS_DATA_08: u8 = 0x51;
    pub const RA_EXT_SENS_DATA_09: u8 = 0x52;
    pub const RA_EXT_SENS_DATA_10: u8 = 0x53;
    pub const RA_EXT_SENS_DATA_11: u8 = 0x54;
    pub const RA_EXT_SENS_DATA_12: u8 = 0x55;
    pub const RA_EXT_SENS_DATA_13: u8 = 0x56;
    pub const RA_EXT_SENS_DATA_14: u8 = 0x57;
    pub const RA_EXT_SENS_DATA_15: u8 = 0x58;
    pub const RA_EXT_SENS_DATA_16: u8 = 0x59;
    pub const RA_EXT_SENS_DATA_17: u8 = 0x5A;
    pub const RA_EXT_SENS_DATA_18: u8 = 0x5B;
    pub const RA_EXT_SENS_DATA_19: u8 = 0x5C;
    pub const RA_EXT_SENS_DATA_20: u8 = 0x5D;
    pub const RA_EXT_SENS_DATA_21: u8 = 0x5E;
    pub const RA_EXT_SENS_DATA_22: u8 = 0x5F;
    pub const RA_EXT_SENS_DATA_23: u8 = 0x60;
    pub const RA_MOT_DETECT_STATUS: u8 = 0x61;
    pub const RA_I2C_SLV0_DO: u8 = 0x63;
    pub const RA_I2C_SLV1_DO: u8 = 0x64;
    pub const RA_I2C_SLV2_DO: u8 = 0x65;
    pub const RA_I2C_SLV3_DO: u8 = 0x66;
    pub const RA_I2C_MST_DELAY_CTRL: u8 = 0x67;
    pub const RA_SIGNAL_PATH_RESET: u8 = 0x68;
    pub const RA_MOT_DETECT_CTRL: u8 = 0x69;
    pub const RA_USER_CTRL: u8 = 0x6A;
    pub const RA_PWR_MGMT_1: u8 = 0x6B;
    pub const RA_PWR_MGMT_2: u8 = 0x6C;
    pub const RA_BANK_SEL: u8 = 0x6D;
    pub const RA_MEM_START_ADDR: u8 = 0x6E;
    pub const RA_MEM_R_W: u8 = 0x6F;
    pub const RA_DMP_CFG_1: u8 = 0x70;
    pub const RA_DMP_CFG_2: u8 = 0x71;
    pub const RA_FIFO_COUNTH: u8 = 0x72;
    pub const RA_FIFO_COUNTL: u8 = 0x73;
    pub const RA_FIFO_R_W: u8 = 0x74;
    pub const RA_WHO_AM_I: u8 = 0x75;
    pub const SELF_TEST_XA_1_BIT: u8 = 0x07;
    pub const SELF_TEST_XA_1_LENGTH: u8 = 0x03;
    pub const SELF_TEST_XA_2_BIT: u8 = 0x05;
    pub const SELF_TEST_XA_2_LENGTH: u8 = 0x02;
    pub const SELF_TEST_YA_1_BIT: u8 = 0x07;
    pub const SELF_TEST_YA_1_LENGTH: u8 = 0x03;
    pub const SELF_TEST_YA_2_BIT: u8 = 0x03;
    pub const SELF_TEST_YA_2_LENGTH: u8 = 0x02;
    pub const SELF_TEST_ZA_1_BIT: u8 = 0x07;
    pub const SELF_TEST_ZA_1_LENGTH: u8 = 0x03;
    pub const SELF_TEST_ZA_2_BIT: u8 = 0x01;
    pub const SELF_TEST_ZA_2_LENGTH: u8 = 0x02;
    pub const SELF_TEST_XG_1_BIT: u8 = 0x04;
    pub const SELF_TEST_XG_1_LENGTH: u8 = 0x05;
    pub const SELF_TEST_YG_1_BIT: u8 = 0x04;
    pub const SELF_TEST_YG_1_LENGTH: u8 = 0x05;
    pub const SELF_TEST_ZG_1_BIT: u8 = 0x04;
    pub const SELF_TEST_ZG_1_LENGTH: u8 = 0x05;
    pub const TC_PWR_MODE_BIT: u8 = 7;
    pub const TC_OFFSET_BIT: u8 = 6;
    pub const TC_OFFSET_LENGTH: u8 = 6;
    pub const TC_OTP_BNK_VLD_BIT: u8 = 0;
    pub const VDDIO_LEVEL_VLOGIC: u8 = 0;
    pub const VDDIO_LEVEL_VDD: u8 = 1;
    pub const CFG_EXT_SYNC_SET_BIT: u8 = 5;
    pub const CFG_EXT_SYNC_SET_LENGTH: u8 = 3;
    pub const CFG_DLPF_CFG_BIT: u8 = 2;
    pub const CFG_DLPF_CFG_LENGTH: u8 = 3;
    pub const EXT_SYNC_DISABLED: u8 = 0x0;
    pub const EXT_SYNC_TEMP_OUT_L: u8 = 0x1;
    pub const EXT_SYNC_GYRO_XOUT_L: u8 = 0x2;
    pub const EXT_SYNC_GYRO_YOUT_L: u8 = 0x3;
    pub const EXT_SYNC_GYRO_ZOUT_L: u8 = 0x4;
    pub const EXT_SYNC_ACCEL_XOUT_L: u8 = 0x5;
    pub const EXT_SYNC_ACCEL_YOUT_L: u8 = 0x6;
    pub const EXT_SYNC_ACCEL_ZOUT_L: u8 = 0x7;
    pub const DLPF_BW_256: u8 = 0x00;
    pub const DLPF_BW_188: u8 = 0x01;
    pub const DLPF_BW_98: u8 = 0x02;
    pub const DLPF_BW_42: u8 = 0x03;
    pub const DLPF_BW_20: u8 = 0x04;
    pub const DLPF_BW_10: u8 = 0x05;
    pub const DLPF_BW_5: u8 = 0x06;
    pub const GCONFIG_FS_SEL_BIT: u8 = 4;
    pub const GCONFIG_FS_SEL_LENGTH: u8 = 2;
    pub const GYRO_FS_250: u8 = 0x00;
    pub const GYRO_FS_500: u8 = 0x01;
    pub const GYRO_FS_1000: u8 = 0x02;
    pub const GYRO_FS_2000: u8 = 0x03;
    pub const ACONFIG_XA_ST_BIT: u8 = 7;
    pub const ACONFIG_YA_ST_BIT: u8 = 6;
    pub const ACONFIG_ZA_ST_BIT: u8 = 5;
    pub const ACONFIG_AFS_SEL_BIT: u8 = 4;
    pub const ACONFIG_AFS_SEL_LENGTH: u8 = 2;
    pub const ACONFIG_ACCEL_HPF_BIT: u8 = 2;
    pub const ACONFIG_ACCEL_HPF_LENGTH: u8 = 3;
    pub const ACCEL_FS_2: u8 = 0x00;
    pub const ACCEL_FS_4: u8 = 0x01;
    pub const ACCEL_FS_8: u8 = 0x02;
    pub const ACCEL_FS_16: u8 = 0x03;
    pub const DHPF_RESET: u8 = 0x00;
    pub const DHPF_5: u8 = 0x01;
    pub const DHPF_2P5: u8 = 0x02;
    pub const DHPF_1P25: u8 = 0x03;
    pub const DHPF_0P63: u8 = 0x04;
    pub const DHPF_HOLD: u8 = 0x07;
    pub const TEMP_FIFO_EN_BIT: u8 = 7;
    pub const XG_FIFO_EN_BIT: u8 = 6;
    pub const YG_FIFO_EN_BIT: u8 = 5;
    pub const ZG_FIFO_EN_BIT: u8 = 4;
    pub const ACCEL_FIFO_EN_BIT: u8 = 3;
    pub const SLV2_FIFO_EN_BIT: u8 = 2;
    pub const SLV1_FIFO_EN_BIT: u8 = 1;
    pub const SLV0_FIFO_EN_BIT: u8 = 0;
    pub const MULT_MST_EN_BIT: u8 = 7;
    pub const WAIT_FOR_ES_BIT: u8 = 6;
    pub const SLV_3_FIFO_EN_BIT: u8 = 5;
    pub const I2C_MST_P_NSR_BIT: u8 = 4;
    pub const I2C_MST_CLK_BIT: u8 = 3;
    pub const I2C_MST_CLK_LENGTH: u8 = 4;
    pub const CLOCK_DIV_348: u8 = 0x0;
    pub const CLOCK_DIV_333: u8 = 0x1;
    pub const CLOCK_DIV_320: u8 = 0x2;
    pub const CLOCK_DIV_308: u8 = 0x3;
    pub const CLOCK_DIV_296: u8 = 0x4;
    pub const CLOCK_DIV_286: u8 = 0x5;
    pub const CLOCK_DIV_276: u8 = 0x6;
    pub const CLOCK_DIV_267: u8 = 0x7;
    pub const CLOCK_DIV_258: u8 = 0x8;
    pub const CLOCK_DIV_500: u8 = 0x9;
    pub const CLOCK_DIV_471: u8 = 0xA;
    pub const CLOCK_DIV_444: u8 = 0xB;
    pub const CLOCK_DIV_421: u8 = 0xC;
    pub const CLOCK_DIV_400: u8 = 0xD;
    pub const CLOCK_DIV_381: u8 = 0xE;
    pub const CLOCK_DIV_364: u8 = 0xF;
    pub const I2C_SLV_RW_BIT: u8 = 7;
    pub const I2C_SLV_ADDR_BIT: u8 = 6;
    pub const I2C_SLV_ADDR_LENGTH: u8 = 7;
    pub const I2C_SLV_EN_BIT: u8 = 7;
    pub const I2C_SLV_BYTE_SW_BIT: u8 = 6;
    pub const I2C_SLV_REG_DIS_BIT: u8 = 5;
    pub const I2C_SLV_GRP_BIT: u8 = 4;
    pub const I2C_SLV_LEN_BIT: u8 = 3;
    pub const I2C_SLV_LEN_LENGTH: u8 = 4;
    pub const I2C_SLV4_RW_BIT: u8 = 7;
    pub const I2C_SLV4_ADDR_BIT: u8 = 6;
    pub const I2C_SLV4_ADDR_LENGTH: u8 = 7;
    pub const I2C_SLV4_EN_BIT: u8 = 7;
    pub const I2C_SLV4_INT_EN_BIT: u8 = 6;
    pub const I2C_SLV4_REG_DIS_BIT: u8 = 5;
    pub const I2C_SLV4_MST_DLY_BIT: u8 = 4;
    pub const I2C_SLV4_MST_DLY_LENGTH: u8 = 5;
    pub const MST_PASS_THROUGH_BIT: u8 = 7;
    pub const MST_I2C_SLV4_DONE_BIT: u8 = 6;
    pub const MST_I2C_LOST_ARB_BIT: u8 = 5;
    pub const MST_I2C_SLV4_NACK_BIT: u8 = 4;
    pub const MST_I2C_SLV3_NACK_BIT: u8 = 3;
    pub const MST_I2C_SLV2_NACK_BIT: u8 = 2;
    pub const MST_I2C_SLV1_NACK_BIT: u8 = 1;
    pub const MST_I2C_SLV0_NACK_BIT: u8 = 0;
    pub const INTCFG_INT_LEVEL_BIT: u8 = 7;
    pub const INTCFG_INT_OPEN_BIT: u8 = 6;
    pub const INTCFG_LATCH_INT_EN_BIT: u8 = 5;
    pub const INTCFG_INT_RD_CLEAR_BIT: u8 = 4;
    pub const INTCFG_FSYNC_INT_LEVEL_BIT: u8 = 3;
    pub const INTCFG_FSYNC_INT_EN_BIT: u8 = 2;
    pub const INTCFG_I2C_BYPASS_EN_BIT: u8 = 1;
    pub const INTCFG_CLKOUT_EN_BIT: u8 = 0;
    pub const INTMODE_ACTIVEHIGH: u8 = 0x00;
    pub const INTMODE_ACTIVELOW: u8 = 0x01;
    pub const INTDRV_PUSHPULL: u8 = 0x00;
    pub const INTDRV_OPENDRAIN: u8 = 0x01;
    pub const INTLATCH_50USPULSE: u8 = 0x00;
    pub const INTLATCH_WAITCLEAR: u8 = 0x01;
    pub const INTCLEAR_STATUSREAD: u8 = 0x00;
    pub const INTCLEAR_ANYREAD: u8 = 0x01;
    pub const INTERRUPT_FF_BIT: u8 = 7;
    pub const INTERRUPT_MOT_BIT: u8 = 6;
    pub const INTERRUPT_ZMOT_BIT: u8 = 5;
    pub const INTERRUPT_FIFO_OFLOW_BIT: u8 = 4;
    pub const INTERRUPT_I2C_MST_INT_BIT: u8 = 3;
    pub const INTERRUPT_PLL_RDY_INT_BIT: u8 = 2;
    pub const INTERRUPT_DMP_INT_BIT: u8 = 1;
    pub const INTERRUPT_DATA_RDY_BIT: u8 = 0;
    // TODO: figure out what these actually do
    // UMPL source code is not very obivous
    pub const DMPINT_5_BIT: u8 = 5;
    pub const DMPINT_4_BIT: u8 = 4;
    pub const DMPINT_3_BIT: u8 = 3;
    pub const DMPINT_2_BIT: u8 = 2;
    pub const DMPINT_1_BIT: u8 = 1;
    pub const DMPINT_0_BIT: u8 = 0;
    pub const MOTION_MOT_XNEG_BIT: u8 = 7;
    pub const MOTION_MOT_XPOS_BIT: u8 = 6;
    pub const MOTION_MOT_YNEG_BIT: u8 = 5;
    pub const MOTION_MOT_YPOS_BIT: u8 = 4;
    pub const MOTION_MOT_ZNEG_BIT: u8 = 3;
    pub const MOTION_MOT_ZPOS_BIT: u8 = 2;
    pub const MOTION_MOT_ZRMOT_BIT: u8 = 0;
    pub const DELAYCTRL_DELAY_ES_SHADOW_BIT: u8 = 7;
    pub const DELAYCTRL_I2C_SLV4_DLY_EN_BIT: u8 = 4;
    pub const DELAYCTRL_I2C_SLV3_DLY_EN_BIT: u8 = 3;
    pub const DELAYCTRL_I2C_SLV2_DLY_EN_BIT: u8 = 2;
    pub const DELAYCTRL_I2C_SLV1_DLY_EN_BIT: u8 = 1;
    pub const DELAYCTRL_I2C_SLV0_DLY_EN_BIT: u8 = 0;
    pub const PATHRESET_GYRO_RESET_BIT: u8 = 2;
    pub const PATHRESET_ACCEL_RESET_BIT: u8 = 1;
    pub const PATHRESET_TEMP_RESET_BIT: u8 = 0;
    pub const DETECT_ACCEL_ON_DELAY_BIT: u8 = 5;
    pub const DETECT_ACCEL_ON_DELAY_LENGTH: u8 = 2;
    pub const DETECT_FF_COUNT_BIT: u8 = 3;
    pub const DETECT_FF_COUNT_LENGTH: u8 = 2;
    pub const DETECT_MOT_COUNT_BIT: u8 = 1;
    pub const DETECT_MOT_COUNT_LENGTH: u8 = 2;
    pub const DETECT_DECREMENT_RESET: u8 = 0x0;
    pub const DETECT_DECREMENT_1: u8 = 0x1;
    pub const DETECT_DECREMENT_2: u8 = 0x2;
    pub const DETECT_DECREMENT_4: u8 = 0x3;
    pub const USERCTRL_DMP_EN_BIT: u8 = 7;
    pub const USERCTRL_FIFO_EN_BIT: u8 = 6;
    pub const USERCTRL_I2C_MST_EN_BIT: u8 = 5;
    pub const USERCTRL_I2C_IF_DIS_BIT: u8 = 4;
    pub const USERCTRL_DMP_RESET_BIT: u8 = 3;
    pub const USERCTRL_FIFO_RESET_BIT: u8 = 2;
    pub const USERCTRL_I2C_MST_RESET_BIT: u8 = 1;
    pub const USERCTRL_SIG_COND_RESET_BIT: u8 = 0;
    pub const PWR1_DEVICE_RESET_BIT: u8 = 7;
    pub const PWR1_SLEEP_BIT: u8 = 6;
    pub const PWR1_CYCLE_BIT: u8 = 5;
    pub const PWR1_TEMP_DIS_BIT: u8 = 3;
    pub const PWR1_CLKSEL_BIT: u8 = 2;
    pub const PWR1_CLKSEL_LENGTH: u8 = 3;
    pub const CLOCK_INTERNAL: u8 = 0x00;
    pub const CLOCK_PLL_XGYRO: u8 = 0x01;
    pub const CLOCK_PLL_YGYRO: u8 = 0x02;
    pub const CLOCK_PLL_ZGYRO: u8 = 0x03;
    pub const CLOCK_PLL_EXT32K: u8 = 0x04;
    pub const CLOCK_PLL_EXT19M: u8 = 0x05;
    pub const CLOCK_KEEP_RESET: u8 = 0x07;
    pub const PWR2_LP_WAKE_CTRL_BIT: u8 = 7;
    pub const PWR2_LP_WAKE_CTRL_LENGTH: u8 = 2;
    pub const PWR2_STBY_XA_BIT: u8 = 5;
    pub const PWR2_STBY_YA_BIT: u8 = 4;
    pub const PWR2_STBY_ZA_BIT: u8 = 3;
    pub const PWR2_STBY_XG_BIT: u8 = 2;
    pub const PWR2_STBY_YG_BIT: u8 = 1;
    pub const PWR2_STBY_ZG_BIT: u8 = 0;
    pub const WAKE_FREQ_1P25: u8 = 0x0;
    pub const WAKE_FREQ_2P5: u8 = 0x1;
    pub const WAKE_FREQ_5: u8 = 0x2;
    pub const WAKE_FREQ_10: u8 = 0x3;
    pub const BANKSEL_PRFTCH_EN_BIT: u8 = 6;
    pub const BANKSEL_CFG_USER_BANK_BIT: u8 = 5;
    pub const BANKSEL_MEM_SEL_BIT: u8 = 4;
    pub const BANKSEL_MEM_SEL_LENGTH: u8 = 5;
    pub const WHO_AM_I_BIT: u8 = 6;
    pub const WHO_AM_I_LENGTH: u8 = 6;
    pub const DMP_MEMORY_BANKS: u8 = 8;
    pub const DMP_MEMORY_BANK_SIZE: u16 = 256;
    pub const DMP_MEMORY_CHUNK_SIZE: u8 = 16;

