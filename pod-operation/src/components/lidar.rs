use i2cdev::linux::LinuxI2CDevice;
use lidar_lite_v3::LidarLiteV3;

const LIDAR_ADDRESS: u16 = 0x62;

pub struct Lidar {
	lidar_lite: LidarLiteV3<LinuxI2CDevice>,
}

impl Lidar {
	pub fn new() -> Lidar {
		let i2cdev = LinuxI2CDevice::new("/dev/i2c-1", LIDAR_ADDRESS)
			.expect("Failed to initialize I2C device");
		let lidar_lite = LidarLiteV3::new(i2cdev).expect("Failed to initialize LidarLiteV3");

		Lidar { lidar_lite }
	}

	/// Convert the distance from centimeters to meters
	pub fn read_distance(&mut self) -> f32 {
		let distance = self.lidar_lite.read_distance(false).unwrap();
		f32::from(distance) / 100.0
	}
}
