#[cfg(feature = "mpu6050")]
use {
	mpu6050::Mpu6050,
	rppal::{hal::Delay, i2c::I2c},
	std::f32::consts::PI,
};

use serde::Serialize;

pub struct Gyroscope {
	#[cfg(feature = "mpu6050")]
	mpu6050: Mpu6050<I2c>,
}

#[derive(Serialize)]
pub struct Orientation {
	pub pitch: f32,
	pub roll: f32,
}
impl Gyroscope {
	#[cfg(feature = "mpu6050")]
	pub fn new() -> Self {
		let i2c = I2c::new().unwrap();
		let mut mpu6050 = Mpu6050::new(i2c);
		mpu6050.init(&mut Delay::new()).unwrap();
		Gyroscope { mpu6050 }
	}

	#[cfg(feature = "mpu6050")]

	pub fn read_orientation(&mut self) -> Orientation {
		let angles = self.mpu6050.get_acc_angles().unwrap();
		Orientation {
			pitch: (angles[1] * 180.0 / PI),
			roll: (angles[0] * 180.0 / PI),
		}
	}

	#[cfg(not(feature = "mpu6050"))]
	pub fn new() -> Self {
		Gyroscope {}
	}

	#[cfg(not(feature = "mpu6050"))]
	pub fn read_orientation(&mut self) -> Orientation {
		Orientation {
			pitch: 0.0,
			roll: 0.0,
		}
	}
}
