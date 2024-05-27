use tracing::debug;

use rppal::gpio::{Gpio, OutputPin};

pub struct SignalLight {
	pin: OutputPin,
}

const PIN_SIGNAL_LIGHT: u8 = 21;

impl SignalLight {
	pub fn new() -> Self {
		SignalLight {
			pin: Gpio::new()
				.unwrap()
				.get(PIN_SIGNAL_LIGHT)
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

	pub fn is_enabled(&self) -> bool {
		self.pin.is_set_high()
	}
}
