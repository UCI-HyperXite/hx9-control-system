use mpu6050::Mpu6050;
use rppal::i2c::I2c;

pub struct Gyroscope {
    mpu6050: Mpu6050<I2c>,
}

pub struct Orientation {
    pub pitch: f32,
    pub roll: f32,
}

impl Gyroscope {
    pub fn new() -> Self {
        let i2c = I2c::new().unwrap();
        let mpu6050 = Mpu6050::new(i2c);
        Gyroscope { mpu6050 }
    }

    pub fn read_orientation(&mut self) -> Orientation {
        let acc = self.mpu6050.get_acc_angles().unwrap();
        Orientation {
            pitch: acc[0],
            roll: acc[1],
        }
    }
}
