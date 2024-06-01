use tracing::info;

/// Pin logic levels, copied from rppal::gpio
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Level {
	Low = 0,
	High = 1,
}

/// Mock for GPIO InputPin
pub struct InputPin {
	pub(crate) pin: u8,
}

impl InputPin {
	pub fn read(&self) -> Level {
		info!("Mocking reading pin {} as low", self.pin);
		Level::Low
	}

	pub fn is_low(&self) -> bool {
		info!("Mocking reading pin {} as low", self.pin);
		true
	}

	pub fn is_high(&self) -> bool {
		info!("Mocking reading pin {} as low", self.pin);
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
