#![no_std]
#![no_main]

use core::cmp;
use core::fmt::Write;

extern crate virt;
use virt::uart::Uart;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let input_str = include_str!("data/day1.txt");
    let lines = input_str.split('\n');

    let mut accumulator = 0;
    let mut largest = [0; 3];

    for line in lines {
        if line.len() == 0 {
            largest[0] = cmp::max(largest[0], accumulator);
            sort(&mut largest);
            accumulator = 0;
        } else {
            accumulator += u32::from_str_radix(line, 10).unwrap();
        }
    }

    writeln!(uart, "Most calorific elves:").unwrap();
    for (i, calories) in largest.iter().rev().enumerate() {
        writeln!(uart, "{}: {calories}", i + 1).unwrap();
    }
    writeln!(uart, "total: {}", largest[0] + largest[1] + largest[2]).unwrap();

    loop {}
}

/// Hard coded sort for a three element set.
fn sort(set: &mut [u32; 3]) {
    if set[0] > set[1] {
        set.swap(0, 1);
    }
    if set[1] > set[2] {
        set.swap(1, 2);
    }
    if set[0] > set[1] {
        set.swap(0, 1);
    }
}
