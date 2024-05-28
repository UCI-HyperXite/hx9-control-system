use rppal::gpio::{Gpio, OutputPin};
use tracing::debug;

use crate::utils::GpioPins;

pub struct Brakes {
	pin: OutputPin,
}

impl Brakes {
	pub fn new() -> Self {
		Brakes {
			pin: Gpio::new()
				.unwrap()
				.get(GpioPins::PNEUMATICS_RELAY)
				.unwrap()
				.into_output(),
		}
	}

	pub fn engage(&mut self) {
		debug!("Engage brakes.");
		self.pin.set_low();
	}

	pub fn disengage(&mut self) {
		debug!("Disengage brakes.");
		self.pin.set_high();
	}
}
