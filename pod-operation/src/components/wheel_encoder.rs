use std::{ops::Sub, time::Instant};

#[cfg(not(feature = "gpio"))]
use crate::utils::mock::gpio::{InputPin, Level};
#[cfg(feature = "gpio")]
use rppal::gpio::{Gpio, InputPin, Level};

use crate::utils::GpioPins;

const WHEEL_DIAMETER: f32 = 0.0762; // meters
const ENCODER_RESOLUTION: f32 = 16.0; // pulses per revolution
const DISTANCE_PER_COUNT: f32 = WHEEL_DIAMETER * std::f32::consts::PI / ENCODER_RESOLUTION; // feet

#[derive(Clone, Copy, num_enum::FromPrimitive, num_enum::IntoPrimitive)]
#[repr(i8)]
enum EncoderState {
	A = 0b00,
	B = 0b01,
	C = 0b11,
	D = 0b10,
	#[num_enum(catch_all)]
	Unknown(i8) = -1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, num_enum::FromPrimitive, num_enum::IntoPrimitive)]
#[repr(i8)]
enum EncoderDiff {
	Backwards = -1,
	Stationary = 0,
	Forwards = 1,
	Undersampling = 2,
	#[num_enum(catch_all)]
	Unknown(i8),
}

impl From<EncoderDiff> for i16 {
	fn from(value: EncoderDiff) -> Self {
		i16::from(i8::from(value))
	}
}

impl From<EncoderDiff> for f32 {
	fn from(value: EncoderDiff) -> Self {
		f32::from(i8::from(value))
	}
}

impl Sub for EncoderState {
	type Output = EncoderDiff;

	fn sub(self, other: Self) -> Self::Output {
		let diff = (i8::from(self) - i8::from(other) + 5) % 4 - 1;
		EncoderDiff::from(diff)
	}
}

/// Encode wheel encoder state as gray code 00, 01, 11, 10
fn encode_state(a: Level, b: Level) -> EncoderState {
	let state = ((a as i8) << 1) + (a as i8 ^ b as i8);
	EncoderState::from(state)
}

pub struct WheelEncoder {
	counter: i16,
	pin_a: InputPin,
	pin_b: InputPin,
	last_state: EncoderState,
	last_time: Instant,
	velocity: f32,
}

impl WheelEncoder {
	pub fn new() -> Self {
		#[cfg(feature = "gpio")]
		let gpio = Gpio::new().unwrap();
		#[cfg(feature = "gpio")]
		let pin_a = gpio
			.get(GpioPins::WHEEL_ENCODER_A.into())
			.unwrap()
			.into_input();
		#[cfg(feature = "gpio")]
		let pin_b = gpio
			.get(GpioPins::WHEEL_ENCODER_B.into())
			.unwrap()
			.into_input();

		#[cfg(not(feature = "gpio"))]
		let pin_a = InputPin {
			pin: GpioPins::WHEEL_ENCODER_A.into(),
		};
		#[cfg(not(feature = "gpio"))]
		let pin_b = InputPin {
			pin: GpioPins::WHEEL_ENCODER_B.into(),
		};

		let initial_state = encode_state(pin_a.read(), pin_b.read());

		WheelEncoder {
			counter: 0,
			last_time: Instant::now(),
			last_state: initial_state,
			pin_a,
			pin_b,
			velocity: 0.0,
		}
	}

	pub fn measure(&mut self) -> Result<f32, &str> {
		let state = self.read_state();

		let inc = state - self.last_state;

		if inc == EncoderDiff::Undersampling {
			return Err("Wheel encoder faulted");
		}

		let time = Instant::now();
		let dt = time.duration_since(self.last_time).as_secs_f32();

		if inc != EncoderDiff::Stationary {
			self.velocity = DISTANCE_PER_COUNT * f32::from(i8::from(inc)) / dt;
			self.last_time = time;
		}

		// When exceeding expected time to next increment, decrease velocity
		if self.velocity * dt > DISTANCE_PER_COUNT {
			self.velocity = DISTANCE_PER_COUNT * self.velocity.signum() / dt;
		}

		self.counter += i16::from(inc);
		self.last_state = state;

		Ok(f32::from(self.counter) * DISTANCE_PER_COUNT)
	}

	pub fn get_velocity(&self) -> f32 {
		self.velocity
	}

	pub fn get_distance(&self) -> f32 {
		f32::from(self.counter) * DISTANCE_PER_COUNT
	}

	fn read_state(&self) -> EncoderState {
		encode_state(self.pin_a.read(), self.pin_b.read())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn encoder_diff_stationary() {
		let state_1 = encode_state(Level::Low, Level::High);
		let state_2 = encode_state(Level::Low, Level::High);
		assert_eq!(state_2 - state_1, EncoderDiff::Stationary);
	}

	#[test]
	fn encoder_diff_forwards() {
		let state_1 = encode_state(Level::High, Level::Low);
		let state_2 = encode_state(Level::Low, Level::Low);
		let diff = state_2 - state_1;
		assert_eq!(diff, EncoderDiff::Forwards);
		assert_eq!(i8::from(diff), 1);
	}

	#[test]
	fn encoder_diff_backwards() {
		let state_1 = encode_state(Level::High, Level::Low);
		let state_2 = encode_state(Level::High, Level::High);
		let diff = state_2 - state_1;
		assert_eq!(diff, EncoderDiff::Backwards);
		assert_eq!(i8::from(diff), -1);
		assert_eq!(f32::from(diff), -1.0);
	}

	#[test]
	fn encoder_diff_undersampling() {
		let state_1 = encode_state(Level::High, Level::Low);
		let state_2 = encode_state(Level::Low, Level::High);
		assert_eq!(state_2 - state_1, EncoderDiff::Undersampling);
	}
}
