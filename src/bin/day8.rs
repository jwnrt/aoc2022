#![no_std]
#![no_main]

use core::fmt::Write;
use core::iter::Iterator;

extern crate virt;
use virt::uart::Uart;

const MAX_ROWS: usize = 128;
const MAX_COLS: usize = 128;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let input_str = include_str!("data/day8.txt");
    let lines = input_str.lines();

    let mut visible = [[false; MAX_COLS]; MAX_ROWS];

    for (row_idx, line) in lines.clone().enumerate() {
        if line.is_empty() {
            break;
        }

        let heights = line
            .char_indices()
            .map(|(idx, c)| (idx, c.to_digit(10).unwrap()));

        let mut highest = 0;
        for (col_idx, height) in heights.clone() {
            if height > highest {
                visible[row_idx][col_idx] = true;
                highest = height;
            }
        }

        let mut highest = 0;
        for (col_idx, height) in heights.rev() {
            if height > highest {
                visible[row_idx][col_idx] = true;
                highest = height;
            }
        }
    }

    let col_count = lines.peekable().peek().unwrap().len();
    for col_idx in 0..col_count {
        let heights = lines
            .map(|line| {
                line.chars().nth(col_idx).unwrap_or('0')
                    .to_digit(10)
                    .unwrap_or(0)
            })
            .enumerate();

        let mut highest = 0;
        for (row_idx, height) in heights.clone() {
            if height > highest {
                visible[row_idx][col_idx] = true;
                highest = height;
            }
        }

        let mut highest = 0;
        for (row_idx, height) in heights.rev() {
            if height > highest {
                visible[row_idx][col_idx] = true;
                highest = height;
            }
        }
    }

    let visible: u32 = visible
        .iter()
        .flatten()
        .fold(0, |acc, x| if *x { acc + 1 } else { acc });
    writeln!(uart, "visible: {visible}").unwrap();

    loop {}
}
