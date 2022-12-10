#![no_std]
#![no_main]

use core::fmt::Write;

extern crate virt;
use virt::uart::Uart;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let input_str = include_str!("data/day6.txt");

    let start_of_packet = unique_run(input_str, 4).unwrap();
    let start_of_message = unique_run(input_str, 14).unwrap();

    writeln!(uart, "Start of packet: {start_of_packet}").unwrap();
    writeln!(uart, "Start of message: {start_of_message}").unwrap();

    loop {}
}

// Only works for [a-z] currently
fn unique_run(input: &str, len: usize) -> Option<usize> {
    let mut mask: u32 = 0;
    let mut leaving_byte: u8 = 0;

    for (i, window) in input.as_bytes().windows(len).enumerate() {
        if i == 0 {
            window.iter().for_each(|b| mask ^= 1 << value(*b));
        } else {
            mask ^= 1 << value(leaving_byte);
            mask ^= 1 << value(*window.last().unwrap());
        }

        leaving_byte = *window.first().unwrap();

        if mask.count_ones() as usize == len {
            return Some(i + len);
        }
    }

    None
}

fn value(b: u8) -> u32 {
    match b as char {
        'a'..='z' => b as u32 - 96,
        _ => 0,
    }
}
