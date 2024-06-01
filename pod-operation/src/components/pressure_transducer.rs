use tracing::info;

#[cfg(feature = "ina219")]
use {ina219::INA219, rppal::i2c::I2c};

// The calibration value is used to adjust the maximum current measurement
// and precision of measurements.
#[cfg(feature = "ina219")]
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
#[cfg(feature = "ina219")]
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

	// The upstream pressure transducer outputs a current between 4 mA and 20 mA
	// with 0 PSI and 5000 PSI respectively.
	fn upstream() -> Self {
		Self::new(0.0, 5000.0, 4.0, 20.0)
	}

	// The downtream pressure transducer outputs a current between 4 mA and 20 mA
	// with 0 PSI and 300 PSI respectively.
	fn downstream() -> Self {
		Self::new(0.0, 300.0, 4.0, 20.0)
	}
}

#[cfg(feature = "ina219")]
fn init_ina(device_address: u8) -> INA219<I2c> {
	let device = I2c::new().unwrap();

	let mut ina219 = INA219::new(device, device_address);
	info!("Initialized I2C and INA219");

	ina219.calibrate(INA219_CALIBRATION_VALUE).unwrap();
	info!("Calibrating INA219");

	ina219
}

pub struct PressureTransducer {
	#[cfg(feature = "ina219")]
	ina: INA219<I2c>,
	ref_values: Reference,
}

impl PressureTransducer {
	// This constructor should be used for INA219s where the address pins are
	// grounded. That is, the device address is 0x40.
	pub fn upstream() -> Self {
		#[cfg(not(feature = "ina219"))]
		info!(
			"Mocking upstream pressure transducer at {}",
			INA219_UPSTREAM_ADDRESS
		);
		Self {
			#[cfg(feature = "ina219")]
			ina: init_ina(INA219_UPSTREAM_ADDRESS),
			ref_values: Reference::upstream(),
		}
	}

	// This constructor should be used for INA219s where the address pin A0 is
	// jumped. That is, the device address is 0x41.
	pub fn downstream() -> Self {
		#[cfg(not(feature = "ina219"))]
		info!(
			"Mocking downstream pressure transducer at {}",
			INA219_DOWNSTREAM_ADDRESS
		);
		Self {
			#[cfg(feature = "ina219")]
			ina: init_ina(INA219_DOWNSTREAM_ADDRESS),
			ref_values: Reference::downstream(),
		}
	}

	// Read current from the INA219 and apply a scaling factor to translate
	// the current reading to PSI.
	pub fn read_pressure(&mut self) -> f32 {
		#[cfg(feature = "ina219")]
		let current = self.read_current();
		#[cfg(not(feature = "ina219"))]
		let current = 11.5; // demo value

		let Reference {
			pressure_lo,
			current_lo,
			pressure_span,
			current_span,
		} = self.ref_values;

		pressure_lo + pressure_span * (current - current_lo) / current_span
	}

	// Read from the INA219 and divide the reading by a scalar factor to
	// convert the reading to mA.
	#[cfg(feature = "ina219")]
	fn read_current(&mut self) -> f32 {
		f32::from(self.ina.current().unwrap()) / INA219_SCALING_VALUE
	}
}
