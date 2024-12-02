use rppal::gpio::Gpio;

fn main() -> Result<(), rppal::gpio::Error> {
    let pins = Gpio::new()?;
    let pin2 = pins.get(2)?;
    let led = pin2.into_output_low();

    for _ in 0..10 {
        //
    }

    Ok(())
}
