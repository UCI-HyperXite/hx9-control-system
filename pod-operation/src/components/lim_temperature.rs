use ads1x1x::ic::{Ads1015, Resolution12Bit};
use ads1x1x::interface::I2cInterface;
use ads1x1x::mode::Continuous;
use ads1x1x::{Ads1x1x, SlaveAddr};
use rppal::i2c::I2c;

pub struct LimTemperature {
	ads1015: Ads1x1x<I2cInterface<I2c>, Ads1015, Resolution12Bit, Continuous>,
}

impl LimTemperature {
	pub fn new() -> Self {
		let i2cdev = I2c::new().unwrap();
		let adc = Ads1x1x::new_ads1015(i2cdev, SlaveAddr::default());
		match adc.into_continuous() {
			Err(ads1x1x::ModeChangeError::I2C(e, _adc)) => {
				panic!("Error converting ADS1015 to continuous mode: {:?}", e)
			}
			Ok(mut adc) => {
                let _channel_converksion = adc.select_channel(&mut ads1x1x::channel::SingleA0);
                return LimTemperature { ads1015: adc }
            }
		}
	}

	pub fn cleanup(self) {
		self.ads1015.destroy_ads1015();
	}

	pub fn read_pin_a0(&mut self) -> i16 {
		let value = self.ads1015.read().unwrap();
		return value;
	}
}
