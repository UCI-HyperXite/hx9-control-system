use rppal::i2c::{Error, I2c};
use std::{thread, time};

const LIDARLITE_ADDR_DEFAULT: u16 = 0x62;

// LIDAR-Lite internal register addresses
const LLv3_ACQ_CMD: u8 = 0x00;
const LLv3_STATUS: u8 = 0x01;
const LLv3_SIG_CNT_VAL: u8 = 0x02;
const LLv3_ACQ_CONFIG: u8 = 0x04;
const LLv3_DISTANCE: u8 = 0x0f;
const LLv3_REF_CNT_VAL: u8 = 0x12;
const LLv3_THRESH_BYPASS: u8 = 0x1c;

pub struct LidarliteV3 {
    i2c: I2c,
    address: u16,
}

impl LidarliteV3 {
    pub fn new() -> Self {
        let i2c = I2c::new().expect("Failed to initialize I2C");
        let address = LIDARLITE_ADDR_DEFAULT;
        let mut lidar = Self { i2c, address };
        lidar.configure(0).expect("Failed to configure LIDAR"); // Default configuration
        lidar
    }

    fn i2c_write(&mut self, reg_addr: u8, data_bytes: &[u8]) -> Result<(), Error> {
        let mut buffer = vec![reg_addr];
        buffer.extend_from_slice(data_bytes);
        self.i2c.block_write(self.address.try_into().unwrap(), &buffer)?;
        Ok(())
    }

    fn i2c_read(&mut self, reg_addr: u8, data_bytes: &mut [u8]) -> Result<(), Error> {
        self.i2c.block_write(self.address.try_into().unwrap(), &[reg_addr])?;
        self.i2c.block_read(self.address.try_into().unwrap(), data_bytes)?;
        Ok(())
    }

    fn take_range(&mut self) -> Result<(), Error> {
        let command_byte = 0x04;
        self.i2c_write(LLv3_ACQ_CMD, &[command_byte])
    }

    fn read_distance(&mut self) -> Result<u16, Error> {
        let mut dist_bytes = [0u8; 2];
        self.i2c_read(LLv3_DISTANCE | 0x80, &mut dist_bytes)?;
        Ok(u16::from_be_bytes(dist_bytes))
    }

    fn configure(&mut self, configuration: u8) -> Result<(), Error> {
        let (sig_count_max, acq_config_reg, ref_count_max, threshold_bypass) = match configuration {
            1 => (0x1d, 0x08, 0x03, 0x00),
            2 => (0x80, 0x00, 0x03, 0x00),
            3 => (0xff, 0x08, 0x05, 0x00),
            4 => (0x80, 0x08, 0x05, 0x80),
            5 => (0x80, 0x08, 0x05, 0xb0),
            6 => (0x04, 0x01, 0x03, 0x00),
            _ => (0x80, 0x08, 0x05, 0x00), // Default
        };

        self.i2c_write(LLv3_SIG_CNT_VAL, &[sig_count_max])?;
        self.i2c_write(LLv3_ACQ_CONFIG, &[acq_config_reg])?;
        self.i2c_write(LLv3_REF_CNT_VAL, &[ref_count_max])?;
        self.i2c_write(LLv3_THRESH_BYPASS, &[threshold_bypass])?;

        Ok(())
    }

    fn get_busy_flag(&mut self) -> Result<u8, Error> {
        let mut status_byte = [0u8; 1];
        self.i2c_read(LLv3_STATUS, &mut status_byte)?;
        Ok(status_byte[0] & 0x01)
    }

    pub fn read(&mut self) -> Result<u16, Error> {
        loop {
            if self.get_busy_flag()? == 0x00 {
                self.take_range()?;
                let distance = self.read_distance()?;
                return Ok(distance);
            }
            // Sleep for a while to avoid busy-waiting
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
