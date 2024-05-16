use ads1x1x::ic::{Ads1015, Resolution12Bit};
use ads1x1x::interface::I2cInterface;
use ads1x1x::mode::OneShot;
use ads1x1x::ChannelSelection::{SingleA0, SingleA1, SingleA2, SingleA3};
use ads1x1x::{Ads1x1x, DynamicOneShot, SlaveAddr};
use nb::block;
use rppal::i2c::I2c;

const DIVIDER_RESISTANCE: f32 = 22000.0;
const VCC: f32 = 5.0;
const BETA: f32 = 3950.0;
const R_0: f32 = 10000.0;
const ROOM_TEMP: f32 = 298.15;

fn voltage_to_temp(voltage: i16) -> f32 {
	let voltage = f32::from(voltage) / 1000.0;
	let thermistor_resistance: f32 = (voltage * DIVIDER_RESISTANCE) / (VCC - voltage);
	let r_inf = R_0 * std::f32::consts::E.powf(-BETA / ROOM_TEMP);
	(BETA / (thermistor_resistance / r_inf).ln()) - 273.15
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

	pub fn read_pins(&mut self) -> (f32, f32, f32, f32) {
		[SingleA0, SingleA1, SingleA2, SingleA3]
			.map(|channel| block!(self.ads1015.read(channel)).unwrap())
			.map(voltage_to_temp)
			.into()
	}
}
