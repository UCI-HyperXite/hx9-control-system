// wrote in a separate module for ads1015 
#[cfg(feature = "ads1015")]
pub mod ads1015_plswork {
    use ads1x1x::ic::{Ads1015, Resolution12Bit};
    use ads1x1x::interface::I2cInterface;
    use ads1x1x::mode::OneShot;
    use ads1x1x::ChannelSelection::{SingleA0, SingleA1, SingleA2, SingleA3};
    use ads1x1x::{Ads1x1x, DynamicOneShot};
    use nb::block;
    use rppal::i2c::I2c;
    use ads1x1x::SlaveAddr;
    use tracing::info;

    pub const C_TO_K_CONVERSION: f32 = 273.15;
    
    // These constants assume that 5 volts is provided to a 22k Ohm resistor
    // connected to a thermistor, with the ADS1015 measuring the node connecting
    // the two (a voltage divider circuit).
    pub const DIVIDER_RESISTANCE: f32 = 1000.0; // Ohms
    pub const V_IN: f32 = 5.0; // Volts
    pub const BETA: f32 = 3950.0; // Kelvins
    pub const R_0: f32 = 10000.0; // Ohms
    pub const ROOM_TEMP: f32 = 25.0 + C_TO_K_CONVERSION; // Kelvins

    pub fn voltage_to_temp(voltage: f32) -> f32 {
        let thermistor_resistance = ((V_IN - voltage) * DIVIDER_RESISTANCE) / voltage;
        let r_inf = R_0 * std::f32::consts::E.powf(-BETA / ROOM_TEMP);
        let temp_kelvins = BETA / (thermistor_resistance / r_inf).ln();
        temp_kelvins - C_TO_K_CONVERSION
    }

    pub struct LimTemperature {
        ads1015: Ads1x1x<I2cInterface<I2c>, Ads1015, Resolution12Bit, OneShot>,
    }

    impl LimTemperature {
        pub fn new(device_address: SlaveAddr) -> Self {
            let i2cdev = I2c::new().expect("Failed to create I2C instance");
            let adc = Ads1x1x::new_ads1015(i2cdev, device_address);
            info!("Configured ADS1015 for LimTemperature");
            LimTemperature { ads1015: adc }
        }

        pub fn cleanup(self) {
            // no specific cleanup reqs for ads1015
        }

        pub fn read_lim_temps(&mut self) -> [f32; 4] {
            [SingleA0, SingleA1, SingleA2, SingleA3]
                .map(|channel| f32::from(block!(self.ads1015.read(channel)).expect("Failed to read from ADS1015")) / 1000.0)
                .map(voltage_to_temp)
        }
    }
}

// redefined main module :D 
use ads1x1x::SlaveAddr;
use tracing::info;

#[cfg(feature = "ads1015")]
use crate::ads1015_plswork::*;

#[cfg(not(feature = "ads1015"))]
pub struct LimTemperature;

#[cfg(not(feature = "ads1015"))]
impl LimTemperature {
    pub fn new(device_address: SlaveAddr) -> Self {
        info!("Mocking ADS at {:?} for LimTemperature", device_address);
        LimTemperature {}
    }

    pub fn cleanup(self) {
        
    }

    pub fn read_lim_temps(&mut self) -> [f32; 4] {
        [0.45, 0.45, 0.45, 0.45].map(voltage_to_temp)
    }
}

#[cfg(not(feature = "ads1015"))]
fn voltage_to_temp(voltage: f32) -> f32 {
    const C_TO_K_CONVERSION: f32 = 273.15;
    const DIVIDER_RESISTANCE: f32 = 1000.0; // Ohms
    const V_IN: f32 = 5.0; // Volts
    const BETA: f32 = 3950.0; // Kelvins
    const R_0: f32 = 10000.0; // Ohms
    const ROOM_TEMP: f32 = 25.0 + C_TO_K_CONVERSION; // Kelvins

    let thermistor_resistance = ((V_IN - voltage) * DIVIDER_RESISTANCE) / voltage;
    let r_inf = R_0 * std::f32::consts::E.powf(-BETA / ROOM_TEMP);
    let temp_kelvins = BETA / (thermistor_resistance / r_inf).ln();
    temp_kelvins - C_TO_K_CONVERSION
}
