#![no_std]
#![no_main]

use core::fmt::Write;
use core::str;

extern crate virt;
use virt::uart::Uart;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let input_str = include_str!("data/day3.txt");
    let lines = input_str.split('\n');

    let mut total_priority = 0;

    let mut total_group_priority = 0;
    let mut group_badge_bitset = u64::MAX;

    for (i, line) in lines.enumerate() {
        if line.len() == 0 || line.len() % 2 == 1 {
            continue;
        }

        let comp_size = line.len() / 2;
        let comp1_bitset = count_items(&line[0..comp_size]);
        let comp2_bitset = count_items(&line[comp_size..line.len()]);

        let shared = comp1_bitset & comp2_bitset;
        let priority = shared.ilog2();

        total_priority += priority;

        let full_bitset = count_items(line);
        group_badge_bitset &= full_bitset;

        if i % 3 == 2 {
            let group_priority = group_badge_bitset.ilog2();
            total_group_priority += group_priority;

            group_badge_bitset = u64::MAX;
        }
    }

    writeln!(uart, "Total priority: {total_priority}").unwrap();
    writeln!(uart, "Total group priority: {total_group_priority}").unwrap();

    loop {}
}

fn count_items(data: &str) -> u64 {
    let mut bitset: u64 = 0;

    for b in data.bytes() {
        let priority = match b {
            97..=122 => b - 96,       // Map a..z => 1..26
            65..=90 => (b - 64) + 26, // Map A..Z => 27..52
            _ => 0,
        };
        // Set the bit for this char
        bitset |= 1 << priority;
    }

    bitset
}
