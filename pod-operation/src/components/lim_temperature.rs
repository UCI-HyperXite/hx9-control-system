use ads1x1x::ic::{Ads1015, Resolution12Bit};
use ads1x1x::interface::I2cInterface;
use ads1x1x::mode::OneShot;
use ads1x1x::ChannelSelection::{SingleA0, SingleA1, SingleA2, SingleA3};
use ads1x1x::FullScaleRange;
use ads1x1x::{Ads1x1x, DynamicOneShot, SlaveAddr};
use nb::block;
use rppal::i2c::I2c;
use std::time::Duration;
use tokio::time::sleep;

const C_TO_K_CONVERSION: f32 = 273.15;

// These constants assume that 5 volts is provided to a 22k Ohm resistor
// connected to a thermistor, with the ADS1015 is measuring the node connecting
// the two (a voltage divider circuit).

const DIVIDER_RESISTANCE: f32 = 22000.0; // Ohms
const V_IN: f32 = 5.0; // Volts
const BETA: f32 = 3950.0; // Kelvins
const R_0: f32 = 10000.0; // Ohms
const ROOM_TEMP: f32 = 25.0 + C_TO_K_CONVERSION; // Kelvins

fn voltage_to_temp(voltage: i16) -> f32 {
	let voltage = f32::from(voltage) / 500.0;
	println!("Voltage: {}", voltage);

	let thermistor_resistance = ((V_IN - voltage) * DIVIDER_RESISTANCE) / (voltage);
	let r_inf = R_0 * std::f32::consts::E.powf(-BETA / ROOM_TEMP);
	let temp_kelvins = BETA / (thermistor_resistance / r_inf).ln();
	temp_kelvins - C_TO_K_CONVERSION
}

pub struct LimTemperature {
	ads1015: Ads1x1x<I2cInterface<I2c>, Ads1015, Resolution12Bit, OneShot>,
}

impl LimTemperature {
	pub fn new(device_address: SlaveAddr) -> Self {
		sleep(Duration::from_secs(1)).await;
		let i2cdev = I2c::new().unwrap();
		let mut adc = Ads1x1x::new_ads1015(i2cdev, device_address);
		adc.set_full_scale_range(FullScaleRange::Within4_096V)
			.unwrap();
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
