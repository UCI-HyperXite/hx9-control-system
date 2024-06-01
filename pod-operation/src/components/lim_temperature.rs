#[cfg(feature = "ads1015")]
use {
	ads1x1x::ic::{Ads1015, Resolution12Bit},
	ads1x1x::interface::I2cInterface,
	ads1x1x::mode::OneShot,
	ads1x1x::ChannelSelection::{SingleA0, SingleA1, SingleA2, SingleA3},
	ads1x1x::{Ads1x1x, DynamicOneShot},
	nb::block,
	rppal::i2c::I2c,
};

use ads1x1x::SlaveAddr;
use tracing::info;

const C_TO_K_CONVERSION: f32 = 273.15;

// These constants assume that 5 volts is provided to a 22k Ohm resistor
// connected to a thermistor, with the ADS1015 is measuring the node connecting
// the two (a voltage divider circuit).

const DIVIDER_RESISTANCE: f32 = 1000.0; // Ohms
const V_IN: f32 = 5.0; // Volts
const BETA: f32 = 3950.0; // Kelvins
const R_0: f32 = 10000.0; // Ohms
const ROOM_TEMP: f32 = 25.0 + C_TO_K_CONVERSION; // Kelvins

fn voltage_to_temp(voltage: f32) -> f32 {
	let thermistor_resistance = ((V_IN - voltage) * DIVIDER_RESISTANCE) / voltage;
	let r_inf = R_0 * std::f32::consts::E.powf(-BETA / ROOM_TEMP);
	let temp_kelvins = BETA / (thermistor_resistance / r_inf).ln();
	temp_kelvins - C_TO_K_CONVERSION
}

pub struct LimTemperature {
	#[cfg(feature = "ads1015")]
	ads1015: Ads1x1x<I2cInterface<I2c>, Ads1015, Resolution12Bit, OneShot>,
}

impl LimTemperature {
	#[cfg(feature = "ads1015")]
	pub fn new(device_address: SlaveAddr) -> Self {
		let i2cdev = I2c::new().unwrap();
		let adc = Ads1x1x::new_ads1015(i2cdev, device_address);
		info!("Configured ADS1015 for for LimTemperature");
		LimTemperature { ads1015: adc }
	}

	#[cfg(not(feature = "ads1015"))]
	pub fn new(device_address: SlaveAddr) -> Self {
		info!("Mocking ADS at {:?} for LimTemperature", device_address);
		LimTemperature {}
	}

	pub fn cleanup(self) {
		#[cfg(feature = "ads1015")]
		self.ads1015.destroy_ads1015();
	}

	#[cfg(feature = "ads1015")]
	pub fn read_lim_temps(&mut self) -> [f32; 4] {
		[SingleA0, SingleA1, SingleA2, SingleA3]
			.map(|channel| f32::from(block!(self.ads1015.read(channel)).unwrap()) / 1000.0)
			.map(voltage_to_temp)
	}

	#[cfg(not(feature = "ads1015"))]
	pub fn read_lim_temps(&mut self) -> [f32; 4] {
		[0.45, 0.45, 0.45, 0.45].map(voltage_to_temp)
	}
}
