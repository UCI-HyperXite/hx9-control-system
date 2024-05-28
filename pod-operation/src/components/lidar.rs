use rppal::i2c::{Error, I2c};
use std::{io, thread, time};

const LIDARLITE_ADDR_DEFAULT: u16 = 0x62;

// LIDAR-Lite internal register addresses
const LLv3_ACQ_CMD: u8 = 0x00;
const LLv3_STATUS: u8 = 0x01;
const LLv3_SIG_CNT_VAL: u8 = 0x02;
const LLv3_ACQ_CONFIG: u8 = 0x04;
const LLv3_DISTANCE: u8 = 0x0f;
const LLv3_REF_CNT_VAL: u8 = 0x12;
const LLv3_UNIT_ID_HIGH: u8 = 0x16;
const LLv3_UNIT_ID_LOW: u8 = 0x17;
const LLv3_I2C_ID_HIGH: u8 = 0x18;
const LLv3_I2C_ID_LOW: u8 = 0x19;
const LLv3_I2C_SEC_ADR: u8 = 0x1a;
const LLv3_THRESH_BYPASS: u8 = 0x1c;
const LLv3_I2C_CONFIG: u8 = 0x1e;
const LLv3_COMMAND: u8 = 0x40;
const LLv3_CORR_DATA: u8 = 0x52;
const LLv3_ACQ_SETTINGS: u8 = 0x5d;

pub struct LidarliteV3 {
	i2c: I2c,
}

impl LidarliteV3 {
	pub fn new() -> io::Result<Self> {
		let i2c = I2c::new()?;
		Ok(Self { i2c })
	}

	fn i2c_write(
		&mut self,
		reg_addr: u8,
		data_bytes: &[u8],
		lidarlite_address: u16,
	) -> Result<(), Error> {
		let mut buffer = vec![reg_addr];
		buffer.extend_from_slice(data_bytes);
		self.i2c.transaction(lidarlite_address, buffer)?;
		Ok(())
	}

	fn i2c_read(
		&mut self,
		reg_addr: u8,
		data_bytes: &mut [u8],
		lidarlite_address: u16,
	) -> Result<(), Error> {
		self.i2c
			.write_read(&[reg_addr], data_bytes)?;
		Ok(())
	}

	pub fn take_range(&mut self, lidarlite_address: u16) -> Result<(), Error> {
		let command_byte = 0x04;
		self.i2c_write(LLv3_ACQ_CMD, &[command_byte], lidarlite_address)
	}

	pub fn read_distance(&mut self, lidarlite_address: u16) -> Result<u16, Error> {
		let mut dist_bytes = [0u8; 2];
		self.i2c_read(LLv3_DISTANCE | 0x80, &mut dist_bytes, lidarlite_address)?;
		Ok(u16::from_be_bytes(dist_bytes))
	}

	pub fn configure(&mut self, configuration: u8, lidarlite_address: u16) -> Result<(), Error> {
		let (sig_count_max, acq_config_reg, ref_count_max, threshold_bypass) = match configuration {
			1 => (0x1d, 0x08, 0x03, 0x00),
			2 => (0x80, 0x00, 0x03, 0x00),
			3 => (0xff, 0x08, 0x05, 0x00),
			4 => (0x80, 0x08, 0x05, 0x80),
			5 => (0x80, 0x08, 0x05, 0xb0),
			6 => (0x04, 0x01, 0x03, 0x00),
			_ => (0x80, 0x08, 0x05, 0x00), // Default
		};

		self.i2c_write(LLv3_SIG_CNT_VAL, &[sig_count_max], lidarlite_address)?;
		self.i2c_write(LLv3_ACQ_CONFIG, &[acq_config_reg], lidarlite_address)?;
		self.i2c_write(LLv3_REF_CNT_VAL, &[ref_count_max], lidarlite_address)?;
		self.i2c_write(LLv3_THRESH_BYPASS, &[threshold_bypass], lidarlite_address)?;

		Ok(())
	}

	pub fn get_busy_flag(&mut self, lidarlite_address: u16) -> Result<u8, Error> {
		let mut status_byte = [0u8; 1];
		self.i2c_read(LLv3_STATUS, &mut status_byte, lidarlite_address)?;
		Ok(status_byte[0] & 0x01)
	}
}

fn main() -> io::Result<()> {
	let mut lidar_lite = LidarliteV3::new()?;
	let lidarlite_address = LIDARLITE_ADDR_DEFAULT;

	// Optionally configure LIDAR-Lite
	lidar_lite.configure(0, lidarlite_address)?;

	loop {
		// Each time through the loop, check BUSY
		if lidar_lite.get_busy_flag(lidarlite_address)? == 0x00 {
			// When no longer busy, immediately initialize another measurement
			// and then read the distance data from the last measurement.
			// This method will result in faster I2C rep rates.
			lidar_lite.take_range(lidarlite_address)?;
			let distance = lidar_lite.read_distance(lidarlite_address)?;
			println!("{:4}", distance);
		}
		// Sleep for a while to avoid busy-waiting
		thread::sleep(time::Duration::from_millis(10));
	}
}
