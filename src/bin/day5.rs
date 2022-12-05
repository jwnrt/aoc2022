#![no_std]
#![no_main]

use core::fmt::Write;
use core::str::Lines;

extern crate virt;
use virt::uart::Uart;

const MAX_STACKS: usize = 9;
const MAX_HEIGHT: usize = 64;

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

struct Command {
    count: usize,
    from: usize,
    to: usize,
}

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let input_str = include_str!("data/day5.txt");
    let mut lines = input_str.lines();

    let mut stacks = parse_stacks::<MAX_STACKS>(&mut lines);

    for line in lines.filter(|line| line.len() > 0) {
        let Command { count, from, to } = parse_command(line);

        for _ in 0..=count {
            let stack_from = stacks.get_mut(from).unwrap().as_mut().unwrap();
            let c = stack_from.buf[stack_from.height - 1].take();
            stack_from.height -= 1;

            let stack_to = stacks.get_mut(to).unwrap().as_mut().unwrap();
            stack_to.buf[stack_to.height] = c;
            stack_to.height += 1;
        }
    }

    writeln!(uart, "Printing stacks:").unwrap();

    for stack in stacks {
        let Some(stack) = stack else {
            break;
        };

        for elem in stack.buf.iter() {
            let c = elem.unwrap_or(' ');
            write!(uart, "{c}").unwrap();
        }
        writeln!(uart).unwrap();
    }

    loop {}
}

fn parse_stacks<'a, const N: usize>(lines: &mut Lines) -> [Option<Stack>; N] {
    let mut line_count = 0;
    let mut stack_count = 0;
    for (i, line) in lines.clone().enumerate() {
        if line.chars().nth(1).unwrap().is_numeric() {
            line_count = i;
            stack_count = (line.len() + 1) / 4;
            break;
        }
    }

    let mut stack_buf = [None; N];
    for stack in stack_buf.iter_mut().take(stack_count) {
        *stack = Some(Stack::default());
    }

    for (line_idx, line) in lines.enumerate().take(line_count) {
        let mut chars = line.chars();
        // Skip the first `[` char
        chars.next();

        // Only care about every fourth char
        let chars = chars.step_by(4);

        for (stack_idx, c) in chars.enumerate() {
            let stack = stack_buf.get_mut(stack_idx).unwrap();
            let stack = stack.as_mut().unwrap();

            if c != ' ' {
                stack.buf[line_count - line_idx - 1] = Some(c);
                stack.height += 1;
            }
        }
    }

    // Skip the line of indices
    lines.next();

    stack_buf
}

fn parse_command(line: &str) -> Command {
    let mut words = line.split(' ');
    // Would be so nice to `.collect`, but arrays aren't `FromIterator`
    let words = [
        words.next().unwrap(),
        words.next().unwrap(),
        words.next().unwrap(),
        words.next().unwrap(),
        words.next().unwrap(),
        words.next().unwrap(),
    ];

    let [ "move", count, "from", from, "to", to ] = words else {
        panic!("unexpected command format: {words:?}");
    };

    // Parse all into usize indices
    let (count, from, to) = (
        count.parse::<usize>().unwrap() - 1,
        from.parse::<usize>().unwrap() - 1,
        to.parse::<usize>().unwrap() - 1,
    );

    Command { count, from, to }
}
