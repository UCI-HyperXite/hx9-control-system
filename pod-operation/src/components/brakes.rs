use tracing::debug;

use rppal::gpio::{Gpio, OutputPin};

pub struct Brakes {
	pin: OutputPin,
}

const PIN_BRAKES: u8 = 5; //Replace with real value

impl Brakes {
	pub fn new() -> Self {
		Brakes {
			pin: Gpio::new()
				.unwrap()
				.get(PIN_SIGNAL_LIGHT)
				.unwrap()
				.into_output(),
		}
	}

	pub fn disable(&mut self) {
		debug!("Disabling brakes.");
		self.pin.set_low();
	}

	pub fn enable(&mut self) {
		debug!("Enabling brakes.");
		self.pin.set_high();
	}
}
