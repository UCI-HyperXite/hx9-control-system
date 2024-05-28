use i2cdev::linux::LinuxI2CDevice;
use lidar_lite_v3::{LidarLiteV3, Status};
use std::error::Error;

pub const LIDAR_LITE_DEFAULT_I2C_ADDRESS: u16 = 0x62;

struct Lidar {
	lidar_lite: LidarLiteV3<LinuxI2CDevice>,
}

impl Lidar {
	fn new() -> Result<Self, Box<dyn Error>> {
		let i2c_namee = "/dev/i2c-1";
		let i2cdev = LinuxI2CDevice::new(&i2c_name, LIDAR_LITE_DEFAULT_I2C_ADDRESS).unwrap();
		let mut lidar_lite = LidarLiteV3::new(i2cdev).unwrap();
		println!("LidarLiteV3");
		Ok(Lidar { lidar_lite })
	}

	fn read_distance(&mut self) -> Result<f32, Box<dyn Error>> {
		// Read distance from the LIDAR
		let distance_raw = self.lidar_lite.read_distance(false)?;
		// Convert raw distance to meters
		let distance_meters = distance_raw as f32 / 100.0;

		Ok(distance_meters)
	}
}
