use ina219::{INA219, INA219_ADDR};
use rppal::i2c::I2c;
use tracing::debug;

pub async fn read_current() {
	let device = I2c::new().unwrap();
	let mut ina = INA219::new(device, INA219_ADDR);
	debug!("Initialized I2C and INA219");

	ina.calibrate(0x0100).unwrap();
	debug!("Calibrating INA219");

	loop {
		tokio::time::sleep(std::time::Duration::from_millis(500)).await;
		let current = ina.current().unwrap();
		println!("current: {:?}", current);
	}
}
