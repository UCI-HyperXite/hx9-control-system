use ads1x1x::SlaveAddr;
use tracing::info;
#[cfg(feature = "ads1015")]
use {
	ads1x1x::ic::{Ads1015, Resolution12Bit},
	ads1x1x::interface::I2cInterface,
	ads1x1x::mode::OneShot,
	ads1x1x::ChannelSelection::{SingleA0, SingleA1, SingleA2},
	ads1x1x::{Ads1x1x, DynamicOneShot, FullScaleRange},
	nb::block,
	rppal::i2c::I2c,
};

const QUIESCENT_VOLTAGE: f32 = 2.5; //Units: volts (v)
const SENSITIVITY: f32 = 0.066; //Unit: vots/amp (v/a)

fn voltage_to_current(voltage: i16) -> f32 {
	let voltage = f32::from(voltage) / 1000.0;
	(voltage - QUIESCENT_VOLTAGE) / SENSITIVITY
}

pub struct LimCurrent {
	#[cfg(feature = "ads1015")]
	ads1015: Ads1x1x<I2cInterface<I2c>, Ads1015, Resolution12Bit, OneShot>,
}

impl LimCurrent {
	#[cfg(feature = "ads1015")]
	pub fn new(device_address: SlaveAddr) -> Self {
		let i2cdev = I2c::new().unwrap();
		let mut adc = Ads1x1x::new_ads1015(i2cdev, device_address);
		adc.set_full_scale_range(FullScaleRange::Within4_096V)
			.unwrap();
		info!("Configured ADS1015 for for LimCurrent");
		LimCurrent { ads1015: adc }
	}

	#[cfg(not(feature = "ads1015"))]
	pub fn new(device_address: SlaveAddr) -> Self {
		info!("Mocking ADS at {:?} for LimCurrent", device_address);
		LimCurrent {}
	}

	pub fn cleanup(self) {
		#[cfg(feature = "ads1015")]
		self.ads1015.destroy_ads1015();
	}

	#[cfg(feature = "ads1015")]
	pub fn read_currents(&mut self) -> (f32, f32, f32) {
		[SingleA0, SingleA1, SingleA2]
			.map(|channel| block!(self.ads1015.read(channel)).unwrap() * 2)
			.map(voltage_to_current)
			.into()
	}

	#[cfg(not(feature = "ads1015"))]
	pub fn read_currents(&mut self) -> (f32, f32, f32) {
		[2500, 2500, 2500].map(voltage_to_current).into()
	}
}
