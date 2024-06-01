use tracing::info;

#[cfg(feature = "lidar")]
use {i2cdev::linux::LinuxI2CDevice, lidar_lite_v3::LidarLiteV3};

#[cfg(feature = "lidar")]
const LIDAR_ADDRESS: u16 = 0x62;

pub struct Lidar {
	#[cfg(feature = "lidar")]
	lidar_lite: LidarLiteV3<LinuxI2CDevice>,
}

impl Lidar {
	#[cfg(feature = "lidar")]
	pub fn new() -> Lidar {
		let i2cdev = LinuxI2CDevice::new("/dev/i2c-1", LIDAR_ADDRESS)
			.expect("Failed to initialize I2C device");
		let lidar_lite = LidarLiteV3::new(i2cdev).expect("Failed to initialize LidarLiteV3");

		info!("Initialized LidarLite");
		Lidar { lidar_lite }
	}

	#[cfg(not(feature = "lidar"))]
	pub fn new() -> Lidar {
		info!("Mocking lidar device");
		Lidar {}
	}

	/// Convert the distance from centimeters to meters
	#[cfg(feature = "lidar")]
	pub fn read_distance(&mut self) -> f32 {
		let distance = self.lidar_lite.read_distance(false).unwrap();
		f32::from(distance) / 100.0
	}

	#[cfg(not(feature = "lidar"))]
	pub fn read_distance(&mut self) -> f32 {
		100.0
	}
}
