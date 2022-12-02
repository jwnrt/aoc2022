#![no_std]
#![no_main]

use core::fmt::Write;
use core::str;

extern crate virt;
use virt::uart::Uart;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    // Allow 32Kb for input
    let mut input_buf = [0u8; 32 * 1024];
    let input_len = uart.read_buf(&mut input_buf);
    let input_str = str::from_utf8(&input_buf[0..input_len]).unwrap();

    let lines = input_str.split('\n');

    let mut score = 0;

    for line in lines {
        let points = match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => 0,
        };

        score += points;
    }

    writeln!(uart, "Points: {score}").unwrap();

    loop {}
}
