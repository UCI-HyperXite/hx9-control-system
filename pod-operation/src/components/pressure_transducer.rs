use ina219::INA219;
use rppal::i2c::I2c;
use tracing::debug;

pub struct PressureTransducer {
	ina: INA219<I2c>,
}

// The calibration value is used to adjust the maximum current measurement
// and precision of measurements.
const INA219_CALIBRATION_VALUE: u16 = 0xffff;

// Even with the calibration values, the readings from the INA219 are not in
// mA. A scaling factor is needed in order to convert the raw reading to mA and
// this is not provided in the INA219 library that we are using. Note that this
// value changes according to the calibration value. The exact formula can be
// found in the INA219 datasheet.
const INA219_SCALING_VALUE: f32 = 160.0;

// The pressure transducer outputs a current between 4 mA and 20 mA with 0 PSI
// and 300 PSI respectively. Assuming a linear interpolation, a 1 mA increase
// results in a 18.75 PSI increase.
const REF_CURRENT_LOW: f32 = 4.0;
const REF_CURRENT_HIGH: f32 = 20.0;
const REF_PRESSURE_LOW: f32 = 0.0;
const REF_PRESSURE_HIGH: f32 = 300.0;

const REF_CURRENT_SPAN: f32 = REF_CURRENT_HIGH - REF_CURRENT_LOW;
const REF_PRESSURE_SPAN: f32 = REF_PRESSURE_HIGH - REF_PRESSURE_LOW;

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
		let current = self.read_current();

		REF_PRESSURE_LOW + REF_PRESSURE_SPAN * (current - REF_CURRENT_LOW) / REF_CURRENT_SPAN
	}

	// Read from the INA219 and divide the reading by a scalar factor to
	// convert the reading to mA.
	pub fn read_current(&mut self) -> f32 {
		self.ina.current().unwrap() as f32 / INA219_SCALING_VALUE
	}
}
