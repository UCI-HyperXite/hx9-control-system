use rppal::uart::{Parity, Uart};
use vesc_comm::{VescConnection, VescErrorWithBacktrace};

const BAUD_RATE: u32 = 115200;
const PARITY: Parity = Parity::None;
const DATA_BITS: u8 = 8;
const STOP_BITS: u8 = 1;

const WHEEL_DIAMETER: f32 = 1.5; // inches
const MPH_TO_IN_PER_MIN: f32 = 1056.0;
const MPH_TO_RPM: f32 = MPH_TO_IN_PER_MIN / WHEEL_DIAMETER;

pub struct Motors {
	pub vesc: VescConnection<Uart>,
}

impl Motors {
	pub fn new(serial_path: &str) -> Self {
		let uart = Uart::with_path(serial_path, BAUD_RATE, PARITY, DATA_BITS, STOP_BITS).unwrap();
		let conn = VescConnection::new(uart);
		Self { vesc: conn }
	}

	pub fn set_speed_mph(&mut self, new_speed_mph: f32) -> Result<(), VescErrorWithBacktrace> {
		self.vesc
			.set_rpm((new_speed_mph * MPH_TO_RPM).round() as u32)
	}
}
