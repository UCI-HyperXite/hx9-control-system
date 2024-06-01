#[cfg(not(feature = "rpi"))]
use crate::utils::mock::OutputPin;
#[cfg(feature = "rpi")]
use rppal::gpio::{Gpio, OutputPin};

use tracing::debug;

use crate::utils::GpioPins;

pub struct Brakes {
	pin: OutputPin,
}

impl Brakes {
	pub fn new() -> Self {
		Brakes {
			#[cfg(not(feature = "rpi"))]
			pin: OutputPin {
				pin: GpioPins::PNEUMATICS_RELAY.into(),
			},
			#[cfg(feature = "rpi")]
			pin: Gpio::new()
				.unwrap()
				.get(GpioPins::PNEUMATICS_RELAY.into())
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
