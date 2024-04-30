use ina219::INA219;
use rppal::i2c::I2c;
use tracing::debug;

pub struct PressureTransducer {
	ina: INA219<I2c>,
}

// The calibration value is used to adjust the maximum current measurement
// and precision of measurements.
const INA219_CALIBRATION_VALUE: u16 = 0xffff;

// Even with the calibration values, the readings from the INA219 are not
// in mA. A scaling factor is needed in order to convert the raw reading to mA
// and this is not provided in the INA219 library that we are using. Note that
// this value changes according to the calibration value. The exact formula can
// be found in the INA219 datasheet.
const INA219_SCALING_VALUE: f32 = 160.0;

impl PressureTransducer {
	pub fn new(ina219_addr: u8) -> Self {
		let device = I2c::new().unwrap();

		let mut ina219 = INA219::new(device, ina219_addr);
		debug!("Initialized I2C and INA219");

		ina219.calibrate(INA219_CALIBRATION_VALUE).unwrap();
		debug!("Calibrating INA219");

		PressureTransducer { ina: ina219 }
	}

	// Read current from the INA219 and apply a scaling factor to translate
	// the current reading to PSI.
	pub fn read(&mut self) -> f32 {
		self.read_current()
	}

	// Read from the INA219 and divide the reading by a scalar factor to
	// convert the reading to mA.
	pub fn read_current(&mut self) -> f32 {
		self.ina.current().unwrap() as f32 / INA219_SCALING_VALUE
	}
}
