#[cfg(not(feature = "gpio"))]
use crate::utils::mock::gpio::OutputPin;
#[cfg(feature = "gpio")]
use rppal::gpio::{Gpio, OutputPin};

use tracing::debug;

use crate::utils::GpioPins;

pub struct SignalLight {
	pin: OutputPin,
}

impl SignalLight {
	pub fn new() -> Self {
		SignalLight {
			#[cfg(not(feature = "gpio"))]
			pin: OutputPin {
				pin: GpioPins::CONTACTOR_RELAY.into(),
			},
			#[cfg(feature = "gpio")]
			pin: Gpio::new()
				.unwrap()
				.get(GpioPins::SIGNAL_LIGHT_RELAY.into())
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
