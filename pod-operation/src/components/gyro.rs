use mpu6050::Mpu6050;
use rppal::i2c::I2c;

pub struct Mpu6050Sensor {
	mpu6050: Mpu6050<I2c>,
}

impl Mpu6050Sensor {
	pub fn new() -> Self {
		let i2c = I2c::new().unwrap();
		let mpu6050 = Mpu6050::new(i2c);
		Mpu6050Sensor { mpu6050 }
	}

	pub fn read_accel_gyro(&mut self) -> (f32, f32) {
		let acc = self.mpu6050.get_acc_angles().unwrap();
		(acc[0], acc[1])
	}
}
