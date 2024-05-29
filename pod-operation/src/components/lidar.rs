use i2cdev::linux::LinuxI2CDevice;
use lidar_lite_v3::{LidarLiteV3, Status};
use std::error::Error;

pub struct Lidar {
	lidar_lite: LidarLiteV3<LinuxI2CDevice>,
}

impl Lidar {
	pub fn new() -> Lidar {
		// Initialize the I2C device
		let i2cdev =
			LinuxI2CDevice::new("/dev/i2c-1", 0x62).expect("Failed to initialize I2C device");
		// Create LidarLiteV3 instance
		let mut lidar_lite = LidarLiteV3::new(i2cdev).expect("Failed to initialize LidarLiteV3");

		Lidar { lidar_lite }
	}

	pub fn read_distance(&mut self) -> f32 {
		let distance = self.lidar_lite.read_distance(false).unwrap();
		let distance_meters = distance as f32 / 100.0;

		distance_meters
	}
}
