use ads1x1x::ic::{Ads1015, Resolution12Bit};
use ads1x1x::interface::I2cInterface;
use ads1x1x::mode::OneShot;
use ads1x1x::ChannelSelection::{SingleA0, SingleA1, SingleA2, SingleA3};
use ads1x1x::{Ads1x1x, DynamicOneShot, SlaveAddr};
use nb::block;
use rppal::i2c::I2c;

fn voltage_to_current(voltage: i16) -> f32 {
	let current = 20 * voltage - 50;
}

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

	pub fn read_lim_temps(&mut self) -> (f32, f32, f32, f32) {
		[SingleA0, SingleA1, SingleA2, SingleA3]
			.map(|channel| block!(self.ads1015.read(channel)).unwrap())
			.map(voltage_to_temp)
			.into()
	}
}
