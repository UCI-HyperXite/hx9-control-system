use tracing::debug;

use rppal::gpio::{Gpio, OutputPin};

pub struct Brakes {
	pin: OutputPin,
}

const PIN_BRAKES: u8 = 26;

impl Brakes {
	pub fn new() -> Self {
		Brakes {
			pin: Gpio::new().unwrap().get(PIN_BRAKES).unwrap().into_output(),
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
