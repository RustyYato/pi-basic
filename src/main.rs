use rppal::gpio::Gpio;

use std::{thread, time};

fn main() -> Result<(), rppal::gpio::Error> {
    let pins = Gpio::new()?;
    let pin2 = pins.get(2)?;

    let mut led = pin2.into_output_high();

    led.set_low();
    thread::sleep(time::Duration::from_secs(1));
    led.set_hi();
    thread::sleep(time::Duration::from_secs(1));
    led.set_lo();
    thread::sleep(time::Duration::from_secs(1));
    led.set_hi();
    thread::sleep(time::Duration::from_secs(1));

    Ok(())
}
