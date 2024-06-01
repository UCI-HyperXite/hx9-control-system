#[cfg(feature = "vesc")]
use {rppal::uart::Uart, vesc_comm::VescConnection};

use tracing::debug;
use tracing::info;
use vesc_comm::VescErrorWithBacktrace;

#[cfg(feature = "vesc")]
mod serial_constants {
	use rppal::uart::Parity;

	pub const BAUD_RATE: u32 = 115200;
	pub const PARITY: Parity = Parity::None;
	pub const DATA_BITS: u8 = 8;
	pub const STOP_BITS: u8 = 1;
}

const WHEEL_DIAMETER: f32 = 1.5; // inches
const MPH_TO_IN_PER_MIN: f32 = 1056.0;
const MPH_TO_RPM: f32 = MPH_TO_IN_PER_MIN / WHEEL_DIAMETER;

pub struct Motors {
	#[cfg(feature = "vesc")]
	pub vesc: VescConnection<Uart>,
	#[cfg(feature = "vesc")]
	pub vesc2: VescConnection<Uart>,
}

impl Motors {
	#[cfg(feature = "vesc")]
	pub fn new(serial_path_one: &str, serial_path_two: &str) -> Self {
		use serial_constants::*;
		use vesc_comm::VescConnection;
		let uart =
			Uart::with_path(serial_path_one, BAUD_RATE, PARITY, DATA_BITS, STOP_BITS).unwrap();
		let uart2 =
			Uart::with_path(serial_path_two, BAUD_RATE, PARITY, DATA_BITS, STOP_BITS).unwrap();
		let conn = VescConnection::new(uart);
		let conn2 = VescConnection::new(uart2);
		info!(
			"Initialized VESC on paths {} and {}",
			serial_path_one, serial_path_two
		);
		Self {
			vesc: conn,
			vesc2: conn2,
		}
	}

	#[cfg(not(feature = "vesc"))]
	pub fn new(serial_path_one: &str, serial_path_two: &str) -> Self {
		info!(
			"Mocking VESC on {} and {}",
			serial_path_one, serial_path_two
		);
		Self {}
	}

	#[cfg(feature = "vesc")]
	pub fn set_speed_mph(&mut self, new_speed_mph: f32) -> Result<(), VescErrorWithBacktrace> {
		debug!("Driving motors at {}", new_speed_mph);
		let new_speed: u32 = (new_speed_mph * MPH_TO_RPM).round() as u32;
		self.vesc.set_rpm(new_speed)?;
		self.vesc2.set_rpm(new_speed)?;
		Ok(())
	}

	#[cfg(not(feature = "vesc"))]
	pub fn set_speed_mph(&mut self, new_speed_mph: f32) -> Result<(), VescErrorWithBacktrace> {
		debug!("Mocking motors at {} RPM", new_speed_mph * MPH_TO_RPM);
		Ok(())
	}
}
