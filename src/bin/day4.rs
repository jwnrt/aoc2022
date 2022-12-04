#![no_std]
#![no_main]

use core::fmt::Write;
use core::ops::RangeInclusive;
use core::str;

extern crate virt;
use virt::uart::Uart;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let input_str = include_str!("data/day4.txt");
    let lines = input_str.split('\n');

    let mut total = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let mut assignments = line.split(',');
        let (first, second) = (assignments.next().unwrap(), assignments.next().unwrap());
        let (first, second) = (to_range(first), to_range(second));

        let first_in_second = first.start() <= second.start() && first.end() >= second.end();
        let second_in_first = second.start() <= first.start() && second.end() >= first.end();

        if first_in_second || second_in_first {
            total += 1;
        }
    }

    writeln!(uart, "overlapped assignments: {total}").unwrap();

    loop {}
}

fn to_range(assignment: &str) -> RangeInclusive<u8> {
    let mut indices = assignment.split('-');
    let (start, end) = (indices.next().unwrap(), indices.next().unwrap());
    let (start, end) = (
        u8::from_str_radix(start, 10).unwrap(),
        u8::from_str_radix(end, 10).unwrap(),
    );
    start..=end
}
