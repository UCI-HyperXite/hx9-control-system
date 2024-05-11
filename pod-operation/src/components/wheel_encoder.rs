use rppal::gpio::{Gpio, InputPin, Level};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tracing::debug;

const PIN_ENCODER_A: u8 = 1;
const PIN_ENCODER_B: u8 = 2;
const WHEEL_RADIUS: f32 = 0.1; // Example wheel radius in meters

pub struct WheelEncoder {
    counter: i32,
    pin_a: InputPin,
    pin_b: InputPin,
    a_last_read: Level,
    b_last_read: Level,
    last_time: Instant,
}

impl WheelEncoder {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        WheelEncoder {
            counter: 0,
            pin_a: gpio.get(PIN_ENCODER_A).unwrap().into_input(),
            pin_b: gpio.get(PIN_ENCODER_B).unwrap().into_input(),
            a_last_read: Level::High,
            b_last_read: Level::Low,
            last_time: Instant::now(),
        }
    }

    pub fn read(&mut self) -> (i32, f32) {
        let a_state = self.pin_a.read();
        let b_state = self.pin_b.read();
        if a_state != self.a_last_read || b_state != self.b_last_read {
            if b_state != a_state {
                self.counter += 1;
            }
        }
        self.a_last_read = a_state;
        self.b_last_read = b_state;

        let current_time = Instant::now();
        let elapsed = current_time.duration_since(self.last_time).as_secs_f32();
        self.last_time = current_time;

        let velocity = (self.counter as f32) * 2.0 * std::f32::consts::PI * 0.05 / elapsed;

        (self.counter, velocity)
    }

    pub fn reset(&mut self) {
        self.counter = 0;
        self.last_time = Instant::now();
    }
}


