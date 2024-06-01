#[cfg(feature = "inverter")]
use rppal::uart::{Parity, Uart};

use tracing::debug;

#[cfg(feature = "inverter")]
mod serial_constants {
	use super::Parity;
	pub const SERIAL_PATH: &str = "/dev/ttyACM0";
	pub const BAUD_RATE: u32 = 9600;
	pub const PARITY: Parity = Parity::None;
	pub const DATA_BITS: u8 = 8;
	pub const STOP_BITS: u8 = 1;
}

pub struct InverterBoard {
	#[cfg(feature = "inverter")]
	uart: Uart,
}

impl InverterBoard {
	#[cfg(feature = "inverter")]
	pub fn new() -> Self {
		use serial_constants::*;
		let uart = Uart::with_path(SERIAL_PATH, BAUD_RATE, PARITY, DATA_BITS, STOP_BITS).unwrap();
		Self { uart }
	}

	/// Combine velocity and throttle into a space-separated string message and then send it over to
	/// the Pico as bytes.
	#[cfg(feature = "inverter")]
	pub fn send_control(&mut self, velocity: f32, throttle: f32) {
		let message = format!("{velocity} {throttle}\n");
		debug!(
			"Sending inverter control message: {} {}",
			velocity, throttle
		);
		self.uart.write(message.as_bytes()).unwrap();
	}

	#[cfg(not(feature = "inverter"))]
	pub fn new() -> Self {
		InverterBoard {}
	}

	/// Combine velocity and throttle into a space-separated string message
	#[cfg(not(feature = "inverter"))]
	pub fn send_control(&mut self, velocity: f32, throttle: f32) {
		debug!(
			"Mocking inverter sending message: {} {}",
			velocity, throttle
		);
	}
}
