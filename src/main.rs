use std::{thread, process};
use std::time::Duration;

use rppal::gpio::Gpio;

fn main() {
    let gpio = match Gpio::new() {
        Ok(v) => v,
        _ => {
            println!("Error creating GPIO struct.");
            process::exit(1);
        }
    };

    let pin = match gpio.get(18) {
        Ok(v) => v,
        _ => {
            println!("Error creating PIN18.");
            process::exit(1);
        }
    };
    let mut pin = pin.into_output();

    pin.set_high();
    thread::sleep(Duration::from_secs(2));
    pin.set_low();
}
