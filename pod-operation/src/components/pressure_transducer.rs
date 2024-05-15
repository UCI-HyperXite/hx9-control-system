use ina219::INA219;
use rppal::i2c::I2c;
use tracing::debug;

// The calibration value is used to adjust the maximum current measurement
// and precision of measurements.
const INA219_CALIBRATION_VALUE: u16 = 0xffff;

// The pod will be using two INA219s, so we'll need to differentiate them
// via their device address.
const INA219_UPSTREAM_ADDRESS: u8 = 0x40;
const INA219_DOWNSTREAM_ADDRESS: u8 = 0x41;

// Even with the calibration values, the readings from the INA219 are not in
// mA. A scaling factor is needed in order to convert the raw reading to mA and
// this is not provided in the INA219 library that we are using. Note that this
// value changes according to the calibration value. The exact formula can be
// found in the INA219 datasheet.
const INA219_SCALING_VALUE: f32 = 160.0;

struct Reference {
	pressure_lo: f32,
	pressure_span: f32,
	current_lo: f32,
	current_span: f32,
}

impl Reference {
	fn new(pressure_lo: f32, pressure_hi: f32, current_lo: f32, current_hi: f32) -> Self {
		Self {
			pressure_lo,
			current_lo,
			pressure_span: pressure_hi - pressure_lo,
			current_span: current_hi - current_lo,
		}
	}
}

struct CalibratedINA;

impl CalibratedINA {
	fn init_ina(device_address: u8) -> INA219<I2c> {
		let device = I2c::new().unwrap();

		let mut ina219 = INA219::new(device, device_address);
		debug!("Initialized I2C and INA219");

		ina219.calibrate(INA219_CALIBRATION_VALUE).unwrap();
		debug!("Calibrating INA219");

		ina219
	}
}

pub struct PressureTransducer {
	ina: INA219<I2c>,
	ref_values: Reference,
}

impl PressureTransducer {
	// This constructor should be used for INA219s where the address pins are
	// grounded. That is, the device address is 0x40.
	pub fn upstream() -> Self {
		// The upstream pressure transducer outputs a current between 4 mA and 20 mA
		// with 0 PSI and 300 PSI respectively.
		let upstream_ref = Reference::new(0.0, 300.0, 4.0, 20.0);

		Self {
			ina: CalibratedINA::init_ina(INA219_UPSTREAM_ADDRESS),
			ref_values: upstream_ref,
		}
	}

	// This constructor should be used for INA219s where the address pin A0 is
	// jumped. That is, the device address is 0x41.
	pub fn downstream() -> Self {
		// The downtream pressure transducer outputs a current between 4 mA and 20 mA
		// with 0 PSI and 300 PSI respectively.
		let downstream_ref = Reference::new(0.0, 300.0, 4.0, 20.0);
		Self {
			ina: CalibratedINA::init_ina(INA219_DOWNSTREAM_ADDRESS),
			ref_values: downstream_ref,
		}
	}

	// Read current from the INA219 and apply a scaling factor to translate
	// the current reading to PSI.
	pub fn read(&mut self) -> f32 {
		let current = self.read_current();

		let Reference {
			pressure_lo,
			current_lo,
			pressure_span,
			current_span,
			..
		} = self.ref_values;

		pressure_lo + pressure_span * (current - current_lo) / current_span
	}

	// Read from the INA219 and divide the reading by a scalar factor to
	// convert the reading to mA.
	fn read_current(&mut self) -> f32 {
		f32::from(self.ina.current().unwrap()) / INA219_SCALING_VALUE
	}
}
