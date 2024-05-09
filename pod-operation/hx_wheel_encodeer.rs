use std::thread::sleep;
use std::time::Duration;
use rppal::gpio::{Gpio, Level};

const OUTPUT_A: u8 = 6;
const OUTPUT_B: u8 = 7;

fn main() {
    let mut counter = 0;
    let mut a_state;
    let mut a_last_state;

    let gpio = Gpio::new().unwrap();
    let mut output_a = gpio.get(OUTPUT_A).unwrap().into_input();
    let mut output_b = gpio.get(OUTPUT_B).unwrap().into_input();

    // Reads the initial state of OUTPUT_A
    a_last_state = output_a.read().unwrap();

    loop {
        a_state = output_a.read().unwrap(); // Reads the "current" state of OUTPUT_A

        // If the previous and the current state of OUTPUT_A are different, that means a Pulse has occurred
        if a_state != a_last_state {
            // If the OUTPUT_B state is different from the OUTPUT_A state, that means the encoder is rotating clockwise
            if output_b.read().unwrap() != a_state {
                counter += 1;
            } else {
                counter -= 1;
            }

            println!("Position: {}", counter);
        }

        a_last_state = a_state; // Updates the previous state of OUTPUT_A with the current state

        sleep(Duration::from_millis(10));
    }
}
