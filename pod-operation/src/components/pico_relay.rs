#[cfg(not(feature = "gpio"))]
use crate::utils::mock::gpio::OutputPin;
#[cfg(feature = "gpio")]
use rppal::gpio::{Gpio, OutputPin};

use crate::utils::GpioPins;
use tracing::debug;

pub struct PicoRelay {
	pin: OutputPin,
}

impl PicoRelay {
	pub fn new() -> Self {
		PicoRelay {
			#[cfg(not(feature = "gpio"))]
			pin: OutputPin {
				pin: GpioPins::PICO_RELAY.into(),
			},
			#[cfg(feature = "gpio")]
			pin: Gpio::new()
				.unwrap()
				.get(GpioPins::PICO_RELAY.into())
				.unwrap()
				.into_output(),
		}
	}
	pub fn disable(&mut self) {
		debug!("Disabling Picos.");
		self.pin.set_low();
	}

	pub fn enable(&mut self) {
		debug!("Enabling Picos.");
		self.pin.set_high();
	}
}
