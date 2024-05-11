use rppal::gpio::{Gpio, InputPin, Level};
use std::thread::sleep;
use std::time::Duration;
use tracing::debug;

const PIN_ENCODER_A: u8 = 1;
const PIN_ENCODER_B: u8 = 2;

pub struct WheelEncoder {
	counter: i32,
	pin_a: InputPin,
	pin_b: InputPin,
	a_last_read: Level,
	b_last_read: Level,
}

impl WheelEncoder {
	pub fn new() -> Self {
		WheelEncoder {
			counter: 0,
			pin_a: Gpio::new()
				.unwrap()
				.get(PIN_ENCODER_A)
				.unwrap()
				.into_input(),
			pin_b: Gpio::new()
				.unwrap()
				.get(PIN_ENCODER_B)
				.unwrap()
				.into_input(),
			a_last_read: Level::High,
			b_last_read: Level::Low,
		}
	}

	pub fn read(&mut self) -> i32 {
		let a_state = self.pin_a.read();
		let b_state = self.pin_b.read();
		if a_state != self.a_last_read || b_state != self.b_last_read {
			if b_state != a_state {
				self.counter += 1;
			}
		}

		self.a_last_read = a_state;
		println!("A: {}", a_state);
		self.b_last_read = b_state;
		println!("B: {}", b_state);
		return self.counter;
	}

	pub fn reset(&mut self) {
		self.counter = 0;
	}
}

