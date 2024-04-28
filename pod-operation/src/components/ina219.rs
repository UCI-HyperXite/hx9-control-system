use ina219::INA219;
use rppal::i2c::I2c;
use tracing::debug;

pub fn read_current(ina: &mut INA219<I2c>) -> f32 {
	ina.calibrate(0xffff).unwrap();
	debug!("Calibrating INA219");

	return ina.current().unwrap() as f32 / 160.0;
}
