use tracing::debug;

use rppal::gpio::{Gpio, OutputPin};

pub struct HighVoltageSystem {
	pin: OutputPin,
}

const PIN_HIGH_VOLTAGE_SYSTEM: u8 = 20;

impl HighVoltageSystem {
	pub fn new() -> Self {
		HighVoltageSystem {
			pin: Gpio::new()
				.unwrap()
				.get(PIN_HIGH_VOLTAGE_SYSTEM)
				.unwrap()
				.into_output(),
		}
	}

	pub fn disable(&mut self) {
		debug!("Disabling high voltage system.");
		self.pin.set_low();
	}

	pub fn enable(&mut self) {
		debug!("Enabling high voltage system.");
		self.pin.set_high();
	}
}
