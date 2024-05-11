use rppal::gpio::{Gpio, InputPin, Level};
use std::thread::sleep;
use std::time::Duration;
use tracing::debug;

const PIN_ENCODER_A: u8 = 1;
const PIN_ENCODER_B: u8 = 2;

pub struct WheelEncoder {
    counter: i32,
    pin_a: InputPin,
    pin_b: InputPin,
}

impl WheelEncoder {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        let pin_a = gpio.get(PIN_ENCODER_A).unwrap().into_input();
        let pin_b = gpio.get(PIN_ENCODER_B).unwrap().into_input();

        WheelEncoder {
            counter: 0,
            pin_a,
            pin_b,
        }
    }

    pub fn read(&mut self) {
        let a_state = self.pin_a.read().unwrap();
        let b_state = self.pin_b.read().unwrap();

        if a_state != self.pin_a.last_read.unwrap_or(a_state)
            || b_state != self.pin_b.last_read.unwrap_or(b_state)
        {
            if b_state != a_state {
                self.counter += 1;
            } else {
                self.counter -= 1;
            }

            println!("Position: {}", self.counter);
        }

        self.pin_a.last_read = Some(a_state);
        self.pin_b.last_read = Some(b_state);
    }
}

fn main() {
    let mut wheel_encoder = WheelEncoder::new();

    loop {
        wheel_encoder.read();
        sleep(Duration::from_millis(10));
    }
}

