use std::{ops::Sub, time::Instant};

use rppal::gpio::{Gpio, InputPin, Level};

use crate::utils::GpioPins;

// TODO: wheel diameter
const DELTA_D: f32 = 1.0 / 16.0;

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

// type EncoderDiff = i8;
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
	let state = ((a as u8) << 1) + (a as u8 ^ b as u8);
	match state {
		0b00 => EncoderState::A,
		0b01 => EncoderState::B,
		0b11 => EncoderState::C,
		0b10 => EncoderState::D,
		_ => unreachable!(),
	}
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
		let gpio = Gpio::new().unwrap();
		let pin_a = gpio
			.get(GpioPins::WHEEL_ENCODER_A.into())
			.unwrap()
			.into_input();
		let pin_b = gpio
			.get(GpioPins::WHEEL_ENCODER_B.into())
			.unwrap()
			.into_input();

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
			self.velocity = DELTA_D * f32::from(i8::from(inc)) / dt;
			self.last_time = time;
		}

		// TODO: fix resting velocity

		self.counter += i16::from(inc);
		self.last_state = state;

		Ok(f32::from(self.counter) * DELTA_D)
	}

	pub fn get_velocity(&self) -> f32 {
		self.velocity
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
