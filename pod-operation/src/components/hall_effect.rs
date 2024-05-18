use ads1x1x::ic::{Ads1015, Resolution12Bit};
use ads1x1x::interface::I2cInterface;
use ads1x1x::mode::OneShot;
use ads1x1x::ChannelSelection::{SingleA0, SingleA1, SingleA2, SingleA3};
use ads1x1x::{Ads1x1x, DynamicOneShot, SlaveAddr, FullScaleRange};
use nb::block;
use rppal::i2c::I2c;

fn voltage_to_current(voltage: i16) -> f32 {
    // Convert voltage to f32 and scale it
    let voltage = f32::from(voltage) / 1000.0;
    
    // Multiply voltage by 15.873 and subtract 39.683
    let current = (voltage * 15.873 - 39.683) as f32;

    // Print the voltage value
    println!("Voltage: {}", voltage);
    
    // Return the current value
    current
}



pub struct HallEffect {
    ads1015: Ads1x1x<I2cInterface<I2c>, Ads1015, Resolution12Bit, OneShot>,
}

impl HallEffect {
    pub fn new(device_address: SlaveAddr) -> Self {
        let i2cdev = I2c::new().unwrap();
        let mut adc = Ads1x1x::new_ads1015(i2cdev, device_address);
        adc.set_full_scale_range(FullScaleRange::Within4_096V).unwrap();
        HallEffect { ads1015: adc }
    }

    pub fn cleanup(self) {
        self.ads1015.destroy_ads1015();
    }

    pub fn read_currents(&mut self) -> (f32, f32, f32, f32) {
        let currents: [f32; 4] = [SingleA0, SingleA1, SingleA2, SingleA3]
            .map(|channel| {
                let voltage = block!(self.ads1015.read(channel)).unwrap() * 2;
                let current:f32 = voltage_to_current(voltage);
                current as f32
            });

        (currents[0], currents[1], currents[2], currents[3])
    }
}
