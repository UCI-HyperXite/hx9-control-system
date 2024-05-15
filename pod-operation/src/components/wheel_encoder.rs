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
	last_distance: f32,
	last_velocity_time: Instant,
	velocity: f32,
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
			last_distance: 0.0,
			last_velocity_time: Instant::now(),
			velocity: 0.0,
		}
	}

	pub fn read(&mut self) -> f32 {
		let a_state = self.pin_a.read();
		let b_state = self.pin_b.read();
		if (a_state != self.a_last_read || b_state != self.b_last_read) && b_state != a_state {
			self.counter += 1.0;
		}
		self.a_last_read = a_state;
		self.b_last_read = b_state;

		let current_time = Instant::now();
		self.last_time = current_time;

		let distance = (self.counter * 5.0) / 1000.0;
		let velocity_elapsed = current_time
			.duration_since(self.last_velocity_time)
			.as_secs_f32();
		if velocity_elapsed >= 0.1 {
			let distance_delta = distance - self.last_distance;
			self.velocity = distance_delta / velocity_elapsed;
			self.last_velocity_time = current_time;
			self.last_distance = distance;
		}

		distance
	}

	pub fn _reset(&mut self) {
		self.counter = 0.0;
		self.last_time = Instant::now();
		self.last_distance = 0.0;
		self.velocity = 0.0;
		self.last_velocity_time = Instant::now();
	}

	pub fn get_velocity(&self) -> f32 {
		self.velocity
	}
}
