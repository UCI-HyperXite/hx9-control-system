use rppal::gpio::{Gpio, InputPin, Level};
use std::time::Instant;

const PIN_ENCODER_A: u8 = 1;
const PIN_ENCODER_B: u8 = 2;

pub struct WheelEncoder {
	counter: f32,
	pin_a: InputPin,
	pin_b: InputPin,
	a_last_read: Level,
	b_last_read: Level,
	last_time: Instant,
}

impl WheelEncoder {
	pub fn new() -> Self {
		let gpio = Gpio::new().unwrap();
		WheelEncoder {
			counter: 0.0,
			pin_a: gpio.get(PIN_ENCODER_A).unwrap().into_input(),
			pin_b: gpio.get(PIN_ENCODER_B).unwrap().into_input(),
			a_last_read: Level::High,
			b_last_read: Level::Low,
			last_time: Instant::now(),
		}
	}

	pub fn read(&mut self) -> f32 {
		let a_state = self.pin_a.read();
		let b_state = self.pin_b.read();
		if a_state != self.a_last_read || b_state != self.b_last_read {
			if b_state != a_state {
				self.counter += 1.0;
			}
		}
		self.a_last_read = a_state;
		self.b_last_read = b_state;

		let current_time = Instant::now();
		let elapsed = current_time.duration_since(self.last_time).as_secs_f32();
		self.last_time = current_time;

		(self.counter * 5.0 / 1000.0)
	}

	pub fn reset(&mut self) {
		self.counter = 0.0;
		self.last_time = Instant::now();
	}
}
