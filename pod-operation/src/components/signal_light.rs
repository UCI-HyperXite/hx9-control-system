use rppal::gpio::{Gpio, OutputPin};
use tracing::debug;

use crate::utils::GpioPins;

pub struct SignalLight {
	pin: OutputPin,
}

impl SignalLight {
	pub fn new() -> Self {
		SignalLight {
			pin: Gpio::new()
				.unwrap()
				.get(GpioPins::SIGNAL_LIGHT_RELAY)
				.unwrap()
				.into_output(),
		}
	}

	pub fn disable(&mut self) {
		debug!("Disabling signal light.");
		self.pin.set_low();
	}

	pub fn enable(&mut self) {
		debug!("Enabling signal light.");
		self.pin.set_high();
	}
}
