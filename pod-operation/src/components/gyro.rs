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
	pub yaw: f32,
}

#[derive(Serialize)]
pub struct Acceleration {
	pub x: f32,
	pub y: f32,
	pub z: f32,
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
			yaw: 0.0,
		}
	}
	
	#[cfg(feature = "mpu6050")]
	pub fn read_acceleration(&mut self) -> Acceleration {
		let acceleration = self.mpu6050.get_acc().unwrap();
		Acceleration {
			x: (acceleration[0] * 9.80665),
			y: (acceleration[1] * 9.80665),
			z: ((acceleration[2] * 9.80665) - 9.80665),
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
			yaw: 0.0,
		}
	}

	#[cfg(not(feature = "mpu6050"))]
	pub fn read_acceleration(&mut self) -> Acceleration {
		Acceleration {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}
}
