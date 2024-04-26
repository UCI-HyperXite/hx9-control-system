use ina219::INA219;
use rppal::i2c::I2c;
use tracing::debug;

pub async fn read_current() {
	let device = I2c::new().unwrap();

	// Use the default INA219 address, located at 0x40. This changes depending
	// on whether we've set A0 and A1 to high.
	let mut ina = INA219::new(device, 0x40);
	debug!("Initialized I2C and INA219");

	// In the Adafruit INA219 Python Driver, the calibration value is set to
	// 4096 (0x1000) and Current_LSB is set to 0.1 by default.
	ina.calibrate(0x1000).unwrap();
	debug!("Calibrating INA219");

	loop {
		tokio::time::sleep(std::time::Duration::from_millis(500)).await;
		let current = ina.current().unwrap();
		println!("current: {:?}", current);
	}
}
