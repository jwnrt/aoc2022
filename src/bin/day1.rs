#![no_std]
#![no_main]

use core::fmt::Write;
use core::{cmp, str};

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

    let mut accumulator = 0;
    let mut largest = 0;

    for line in lines {
        if line.len() == 0 {
            largest = cmp::max(largest, accumulator);
            accumulator = 0;
        } else {
            accumulator += u32::from_str_radix(line, 10).unwrap();
        }
    }

    writeln!(uart, "most calories: {largest}").unwrap();

    loop {}
}
