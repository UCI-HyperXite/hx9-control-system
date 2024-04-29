use crate::components::ina219::read_current;
use ina219::INA219;
use rppal::i2c::I2c;
use tracing::debug;

pub struct PressureTransducer {
	ina: INA219<I2c>,
}

impl PressureTransducer {
	pub fn new(ina219_addr: u8) -> Self {
		let device = I2c::new().unwrap();

		let mut ina219 = INA219::new(device, ina219_addr);
		debug!("Initialized I2C and INA219");

		ina219.calibrate(0xffff).unwrap();
		debug!("Calibrating INA219");

		PressureTransducer { ina: ina219 }
	}

	pub fn read(&mut self) -> f32 {
		read_current(&mut self.ina) / 10.0
	}
}
