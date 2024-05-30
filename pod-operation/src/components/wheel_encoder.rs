use rppal::gpio::{Gpio, InputPin, Level};
use tokio::time::Instant;

use crate::utils::GpioPins;

// GPIO pins for the encoder
const PIN_ENCODER_A: u8 = 23; //need to move
const PIN_ENCODER_B: u8 = 24; //need to move

// Constants for calculations
const DELTA_D: f32 = 1.0 / 16.0;

type EncoderState = u8;
type EncoderDiff = i8;

// Function to encode the state
// 00, 01, 10, 11
fn encode_state(a: bool, b: bool) -> EncoderState {
	((a as u8) << 1) + (a as u8 ^ b as u8)
}

// Function to calculate the difference in states
// -1 => moving backwards
// 0 => no movement
// 1 => moving forward
// 2 => Undersampling
fn state_difference(p: EncoderState, q: EncoderState) -> EncoderDiff {
	((p as i8 - q as i8 + 1) % 4 - 1) as EncoderDiff
}

// Struct to represent the wheel encoder
pub struct WheelEncoder {
	counter: i32,
	last_time: Instant,
	last_state: EncoderState,
	pin_a: InputPin,
	pin_b: InputPin,
	faulted: bool,
}

impl WheelEncoder {
	// Constructor to initialize the encoder
	pub fn new() -> Result<Self, rppal::gpio::Error> {
		let gpio = Gpio::new()?;
		let pin_a = gpio.get(GpioPins::WHEEL_ENCODER_A.into())?.into_input();
		let pin_b = gpio.get(GpioPins::WHEEL_ENCODER_B.into())?.into_input();

		let initial_state = encode_state(pin_a.is_high(), pin_b.is_high());

		Ok(WheelEncoder {
			counter: 0,
			last_time: Instant::now(),
			last_state: initial_state,
			pin_a,
			pin_b,
			faulted: false,
		})
	}

	// Method to measure speed and distance
	pub fn measure(&mut self) -> Result<(f32, f32), &'static str> {
		if self.faulted {
			return Err("WheelEncoder is in Faulted state");
		}

		let current_time = Instant::now();
		let state = self.read_state();

		let mut speed = 0.0;
		let inc = state_difference(state, self.last_state);

		if inc == 2 {
			self.faulted = true;
			return Err("Undersampling. Transition to Faulted state please.");
		}

		if inc != 0 {
			let delta_t = current_time.duration_since(self.last_time).as_secs_f32();
			speed = inc as f32 * DELTA_D / delta_t;
			self.last_time = current_time;
		}

		self.last_state = state;
		self.counter += inc as i32;

		let distance = self.counter as f32 * DELTA_D;

		Ok((speed, distance))
	}

	// Private method to read the state of the encoder
	fn read_state(&self) -> EncoderState {
		encode_state(self.pin_a.is_high(), self.pin_b.is_high())
	}
}
