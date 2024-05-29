use rppal::uart::{Parity, Uart};

const SERIAL_PATH: &str = "/dev/tty/ACM0";
const BAUD_RATE: u32 = 9600;
const PARITY: Parity = Parity::None;
const DATA_BITS: u8 = 8;
const STOP_BITS: u8 = 1;

pub struct InverterBoard {
	uart: Uart,
}

impl InverterBoard {
	pub fn new() -> Self {
		let uart = Uart::with_path(SERIAL_PATH, BAUD_RATE, PARITY, DATA_BITS, STOP_BITS).unwrap();
		Self { uart }
	}

	/// Combine velocity and throttle into a space-separated string message and then send it over to
	/// the Pico as bytes.
	pub fn send_control(&mut self, velocity: f32, throttle: f32) {
		let message = format!("{velocity} {throttle}\n");
		self.uart.write(message.as_bytes()).unwrap();
	}
}
