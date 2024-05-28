use lidar_lite_v3::{LidarLiteV3, Status};
use i2cdev::linux::LinuxI2CDevice;
use std::error::Error;

pub struct Lidar {
    lidar_lite: LidarLiteV3<LinuxI2CDevice>,
}

impl Lidar {
    pub fn new() -> Lidar {
        // Initialize the I2C device
        let i2cdev = LinuxI2CDevice::new("/dev/i2c-1", lidar_lite_v3::LIDAR_LITE_DEFAULT_I2C_ADDRESS)
            .expect("Failed to initialize I2C device");
        // Create LidarLiteV3 instance
        let lidar_lite = LidarLiteV3::new(i2cdev)
            .expect("Failed to initialize LidarLiteV3");

        Lidar { lidar_lite }
    }

    pub fn read_distance(&mut self) -> Result<f32, Box<dyn Error>> {
        // Read distance from the LIDAR
        let distance_raw = self.lidar_lite.read_distance(false)?;
        // Convert raw distance to meters
        let distance_meters = distance_raw as f32 / 100.0;

        Ok(distance_meters)
    }
}
