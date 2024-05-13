use ads1x1x::ic::{Ads1015, Resolution12Bit};
use ads1x1x::interface::I2cInterface;
use ads1x1x::mode::OneShot;
use ads1x1x::ChannelSelection::{SingleA0, SingleA1, SingleA2, SingleA3};
use ads1x1x::{Ads1x1x, DynamicOneShot, SlaveAddr};
use nb::block;
use rppal::i2c::I2c;

pub struct LimTemperature {
	ads1015: Ads1x1x<I2cInterface<I2c>, Ads1015, Resolution12Bit, OneShot>,
}

impl LimTemperature {
	pub fn new(device_address: SlaveAddr) -> Self {
		let i2cdev = I2c::new().unwrap();
		let adc = Ads1x1x::new_ads1015(i2cdev, device_address);
		LimTemperature { ads1015: adc }
	}

	pub fn cleanup(self) {
		self.ads1015.destroy_ads1015();
	}

	pub fn read_pins(&mut self) -> Vec<i16> {
		let mut read_values: Vec<i16> = Vec::new();
		let channels = vec![SingleA0, SingleA1, SingleA2, SingleA3];

		for channel in channels {
			let read = block!(self.ads1015.read(channel)).unwrap();
			read_values.push(read);
		}

		read_values
	}
}
