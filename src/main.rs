use std::{env, thread, time};

use rppal::gpio::Gpio;

const DOT_DURATION: u64 = 1000 / 20;
const DASH_DURATION: u64 = 3 * DOT_DURATION;
const LETTER_SEP: u64 = 3 * DOT_DURATION;
const WORD_SEP: u64 = 7 * DOT_DURATION;

fn main() -> Result<(), rppal::gpio::Error> {
    let pins = Gpio::new()?;

    let mut message = pins.get(2)?.into_output_low();
    let mut status = pins.get(3)?.into_output_high();

    message.set_pwm_frequency(1.0, 1.0)?;
    let mut first = true;

    for x in env::args().skip(1) {
        if first {
            first = false;
        } else {
            thread::sleep(time::Duration::from_millis(WORD_SEP));
        }
        for x in x.bytes() {
            if x == b' ' {
                thread::sleep(time::Duration::from_millis(WORD_SEP));
                continue;
            }
            for x in SYMBOLS[x as usize].bytes() {
                if x == b'.' {
                    message.set_high();
                    thread::sleep(time::Duration::from_millis(DOT_DURATION));
                    message.set_low();
                    thread::sleep(time::Duration::from_millis(DOT_DURATION));
                } else if x == b'-' {
                    message.set_high();
                    thread::sleep(time::Duration::from_millis(DASH_DURATION));
                    message.set_low();
                    thread::sleep(time::Duration::from_millis(DOT_DURATION));
                }
            }
            thread::sleep(time::Duration::from_millis(LETTER_SEP));
        }
    }

    message.set_low();
    status.set_low();

    Ok(())
}

const LETTERS_DATA: [&str; 26] = [
    ".-",   // a
    "-...", // b
    "-.-.", // c
    "-..",  // d
    ".",    // e
    "..-.", // f
    "--.",  // g
    "....", // h
    "..",   // i
    ".---", // j
    "-.-",  // k
    ".-..", // l
    "--",   // m
    "-.",   // n
    "---",  // o
    ".--.", // p
    "--.-", // q
    ".-.",  // r
    "...",  // s
    "-",    // t
    "..-",  // u
    "...-", // v
    ".--",  // w
    "-..-", // x
    "-.--", // y
    "--..", // z
];
const NUMBERS: [&str; 10] = [
    "-----", // 0
    ".----", // 1
    "..---", // 2
    "...--", // 3
    "....-", // 4
    ".....", // 5
    "-....", // 6
    "--...", // 7
    "---..", // 8
    "----.", // 9
];
static SYMBOLS: [&str; 256] = {
    let mut data = [""; 256];

    let mut i = 0;
    while i < 26 {
        data[i + b'a' as usize] = LETTERS_DATA[i];
        data[i + b'A' as usize] = LETTERS_DATA[i];
        i += 1;
    }
    let mut i = 0;
    while i < 10 {
        data[i + b'0' as usize] = NUMBERS[i];
        i += 1;
    }
    data[b'.' as usize] = ".-.-.-";
    data[b',' as usize] = "--..--";
    data[b'?' as usize] = "..--..";
    data[b'!' as usize] = "-.-.--";
    data[b' ' as usize] = "-.-.--";

    data
};
