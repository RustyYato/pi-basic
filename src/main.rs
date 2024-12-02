use rppal::gpio::Gpio;

use std::{thread, time};

fn main() -> Result<(), rppal::gpio::Error> {
    let pins = Gpio::new()?;
    let pin2 = pins.get(2)?;

    let mut led = pin2.into_output_high();

    for _ in 0..10 {
        led.set_low();
        thread::sleep(time::Duration::from_millis(50));
        led.set_high();
        thread::sleep(time::Duration::from_millis(200));
    }

    led.set_low();

    Ok(())
}
