use rppal::uart::{Parity, Uart};
pub struct InverterBoard {
	uart: Uart,
}

impl InverterBoard {
	pub fn new() -> Self {
		let uart = Uart::with_path("/dev/ttyACM0", 9600, Parity::None, 8, 1).unwrap();
		Self { uart }
	}

	pub fn send_control(&mut self, velocity: f32, throttle: f32) {
		let vel_str = velocity.to_string();
		let throttle_str = throttle.to_string();
		let data = vel_str + " " + &throttle_str + "\n";
		self.uart.write(data.as_bytes()).unwrap();
	}
}
