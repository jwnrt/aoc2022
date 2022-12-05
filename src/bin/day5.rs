#![no_std]
#![no_main]

use core::fmt::Write;
use core::str::Lines;

extern crate virt;
use virt::uart::Uart;

const MAX_STACKS: usize = 16;
const MAX_HEIGHT: usize = 16;

#[derive(Clone, Copy)]
struct Stack {
    buf: [Option<char>; MAX_HEIGHT],
    height: usize,
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            buf: [None; MAX_HEIGHT],
            height: 0,
        }
    }
}

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let input_str = include_str!("data/day5.txt");
    let mut lines = input_str.lines();

    let mut stack_buf = [Stack::default(); MAX_STACKS];
    parse_stacks(&mut lines, &mut stack_buf[..]);

    writeln!(uart, "").unwrap();

    loop {}
}

fn parse_stacks<'a>(lines: &mut Lines, &mut stack_buf: &mut [Stack]) {
    let mut lines = lines.peekable();
    let stack_count = (lines.peek().unwrap().len() + 1) / 4;

    stack_buf = stack_buf[..stack_count];

    for (row, line) in lines.enumerate() {
        // Finish when line contains no stacks
        if line.find("[").is_none() {
            break;
        }

        // Strip the leading and trailing `[` and `]`
        let line = &mut line[1..line.len() - 1];
        let line = line.split("] [");

        for (stack_idx, c) in line.enumerate() {
            let c = c.chars().next().unwrap();
            stack_buf[stack_idx].buf[row] = Some(c);
            stack_buf[stack_idx].height += 1;
        }
    }

    for stack in stack_buf.iter_mut() {
        stack.buf.reverse();
    }
}
