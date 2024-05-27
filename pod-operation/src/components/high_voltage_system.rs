use tracing::debug;

use rppal::gpio::{Gpio, OutputPin};

pub struct HighVoltageSystem {
	pin: OutputPin,
}

const PIN_CONTACTOR_RELAY: u8 = 20;

impl HighVoltageSystem {
	pub fn new() -> Self {
		HighVoltageSystem {
			pin: Gpio::new()
				.unwrap()
				.get(PIN_CONTACTOR_RELAY)
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

	pub fn is_enabled(&self) -> bool {
		self.pin.is_set_high()
	}
}
