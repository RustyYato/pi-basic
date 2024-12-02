#![no_std]

use rppal::gpio::Gpio;

fn main() -> Result<(), rppal::gpio::Error> {
    let pins = Gpio::new()?;
    let pin2 = pins.get(2)?;

    Ok(())
}
