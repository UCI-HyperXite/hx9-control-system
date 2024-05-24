use ads1x1x::ic::{Ads1015, Resolution12Bit};
use ads1x1x::interface::I2cInterface;
use ads1x1x::mode::OneShot;
use ads1x1x::ChannelSelection::{SingleA0, SingleA1, SingleA2};
use ads1x1x::{Ads1x1x, DynamicOneShot, FullScaleRange, SlaveAddr};
use nb::block;
use rppal::i2c::I2c;

const QUIESCENT_VOLTAGE: f32 = 2.5; //Units: volts (v)
const SENSITIVITY: f32 = 0.066; //Unit: vots/amp (v/a)

fn voltage_to_current(voltage: i16) -> f32 {
	let voltage = f32::from(voltage) / 1000.0;
	let current = (voltage - QUIESCENT_VOLTAGE) / SENSITIVITY as f32;
	println!("Voltage: {}", voltage);
	current
}

pub struct LimCurrent {
	ads1015: Ads1x1x<I2cInterface<I2c>, Ads1015, Resolution12Bit, OneShot>,
}

impl LimCurrent {
	pub fn new(device_address: SlaveAddr) -> Self {
		let i2cdev = I2c::new().unwrap();
		let mut adc = Ads1x1x::new_ads1015(i2cdev, device_address);
		adc.set_full_scale_range(FullScaleRange::Within4_096V)
			.unwrap();
		LimCurrent { ads1015: adc }
	}

	pub fn cleanup(self) {
		self.ads1015.destroy_ads1015();
	}

	pub fn read_currents(&mut self) -> (f32, f32, f32) {
		[SingleA0, SingleA1, SingleA2]
			.map(|channel| block!(self.ads1015.read(channel)).unwrap()*2)
			.map(voltage_to_current)
			.into()
	}
}
