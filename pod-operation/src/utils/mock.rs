#[cfg(not(feature = "gpio"))]
pub mod gpio {
	use tracing::{debug, info};

	/// Pin logic levels, copied from rppal::gpio
	#[derive(Debug, PartialEq, Eq, Copy, Clone)]
	#[repr(u8)]
	#[allow(dead_code)]
	pub enum Level {
		Low = 0,
		High = 1,
	}

	/// Mock for GPIO InputPin
	pub struct InputPin {
		pub(crate) pin: u8,
	}

	#[allow(dead_code)]
	impl InputPin {
		pub fn read(&self) -> Level {
			debug!("Mocking reading pin {} as low", self.pin);
			Level::Low
		}

		pub fn is_low(&self) -> bool {
			debug!("Mocking reading pin {} as low", self.pin);
			true
		}

		pub fn is_high(&self) -> bool {
			debug!("Mocking reading pin {} as low", self.pin);
			false
		}
	}

	/// Mock for GPIO OutputPin
	pub struct OutputPin {
		pub(crate) pin: u8,
	}

	impl OutputPin {
		pub fn set_low(&self) {
			info!("Mocking pin {} to low", self.pin);
		}

		pub fn set_high(&self) {
			info!("Mocking pin {} to high", self.pin);
		}
	}
}
