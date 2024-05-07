// ads1015.rs

use ads1x1x::{channel, Ads1x1x, SlaveAddr};
use linux_embedded_hal::{I2cdev, Delay};

pub fn read_voltage() -> f32 {
    // Open the I2C device
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

 new ADS1015 instance
    let mut ads1015 = Ads1x1x::new_ads1015(i2c, SlaveAddr::default());

    // Start a single-ended conversion on pin A0
    let result = ads1015.read(&mut Delay, channel::SingleA0).unwrap();

    // Convert the ADC reading to voltage
    ads1015.compute_voltage(result)
}

