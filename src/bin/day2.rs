#![no_std]
#![no_main]

use core::fmt::Write;
use core::str;

extern crate virt;
use virt::uart::Uart;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let input_str = include_str!("data/day2.txt");
    let lines = input_str.split('\n');

    let mut score_rps = 0; // Score using the rock/paper/scissors strategy
    let mut score_ldw = 0; // Score using the lose/draw/win strategy

    for line in lines {
        score_rps += rps_points(line);
        score_ldw += ldw_points(line);
    }

    writeln!(
        uart,
        "Points with rock/paper/scissors strategy: {score_rps}"
    )
    .unwrap();
    writeln!(uart, "Points with lose/draw/win strategy: {score_ldw}").unwrap();

    loop {}
}

/// Scoring system for the rock/paper/scissors interpretation of codes
fn rps_points(code: &str) -> u32 {
    match code {
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
    }
}

/// Scoring system for the lose/draw/win interpretation of codes
fn ldw_points(code: &str) -> u32 {
    match code {
        "A X" => 3 + 0,
        "A Y" => 1 + 3,
        "A Z" => 2 + 6,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 2 + 0,
        "C Y" => 3 + 3,
        "C Z" => 1 + 6,
        _ => 0,
    }
}
