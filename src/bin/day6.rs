#![no_std]
#![no_main]

use core::fmt::Write;

extern crate virt;
use virt::uart::Uart;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let input_str = include_str!("data/day6.txt");

    let bytes = input_str.as_bytes();
    for (i, window) in bytes.windows(4).enumerate() {
        let dupe = window[0] == window[1]
            || window[0] == window[2]
            || window[0] == window[3]
            || window[1] == window[2]
            || window[1] == window[3]
            || window[2] == window[3];
        if !dupe {
            writeln!(uart, "{}", i + 4).unwrap();
            break;
        }
    }

    loop {}
}
