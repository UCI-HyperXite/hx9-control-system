use mpu6050::Mpu6050;
#[cfg(feature = "rpi")]
use rppal::{hal::Delay, i2c::I2c};

use serde::Serialize;

pub struct Gyroscope {
	#[cfg(feature = "rpi")]
	mpu6050: Mpu6050<I2c>,
}

#[derive(Serialize)]
pub struct Orientation {
	pub pitch: f32,
	pub roll: f32,
}

impl Gyroscope {
	#[cfg(feature = "rpi")]
	pub fn new() -> Self {
		let i2c = I2c::new().unwrap();
		let mut mpu6050 = Mpu6050::new(i2c);
		mpu6050.init(&mut Delay::new()).unwrap();
		Gyroscope { mpu6050 }
	}

	#[cfg(feature = "rpi")]
	pub fn read_orientation(&mut self) -> Orientation {
		let angles = self.mpu6050.get_acc_angles().unwrap();
		Orientation {
			pitch: angles[1],
			roll: angles[0],
		}
	}

	#[cfg(not(feature = "rpi"))]
	pub fn new() -> Self {
		Gyroscope {}
	}

	#[cfg(not(feature = "rpi"))]
	pub fn read_orientation(&mut self) -> Orientation {
		Orientation {
			pitch: 0.0,
			roll: 0.0,
		}
	}
}
