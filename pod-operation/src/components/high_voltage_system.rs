use rppal::gpio::{Gpio, OutputPin};
use tracing::debug;

use crate::utils::GpioPins;

pub struct HighVoltageSystem {
	pin: OutputPin,
}

impl HighVoltageSystem {
	pub fn new() -> Self {
		HighVoltageSystem {
			pin: Gpio::new()
				.unwrap()
				.get(GpioPins::CONTACTOR_RELAY)
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
