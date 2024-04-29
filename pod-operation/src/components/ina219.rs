use ina219::INA219;
use rppal::i2c::I2c;

pub fn read_current(ina: &mut INA219<I2c>) -> f32 {
	ina.current().unwrap() as f32 / 160.0
}
