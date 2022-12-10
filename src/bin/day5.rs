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

    let mut print_tops = |stacks: &[Option<Stack>; MAX_STACKS]| {
        stacks
            .iter()
            .flatten()
            .map(|stack| stack.buf[stack.height - 1].unwrap_or('*'))
            .for_each(|top| write!(uart, "{top}").unwrap());
        writeln!(uart).unwrap();
    };

    let stacks = parse_stacks::<MAX_STACKS>(&mut lines);
    let mut stacks_9000 = stacks.clone();
    let mut stacks_9001 = stacks.clone();

    for line in lines.clone().filter(|line| line.len() > 0) {
        let command = parse_command(line);

        // Move one crate at a time `command.count` times
        for _ in 0..command.count {
            move_crate(
                &mut stacks_9000,
                Command {
                    count: 1,
                    ..command
                },
            );
        }
        move_crate(&mut stacks_9001, command);
    }

    print_tops(&stacks_9000);
    print_tops(&stacks_9001);

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
        count.parse::<usize>().unwrap(),
        from.parse::<usize>().unwrap() - 1,
        to.parse::<usize>().unwrap() - 1,
    );

    Command { count, from, to }
}

fn move_crate(stacks: &mut [Option<Stack>; MAX_STACKS], command: Command) {
    let Command { count, from, to } = command;

    for i in 0..command.count {
        let stack_from = stacks[from].as_mut().unwrap();
        let c = stack_from.buf[stack_from.height - count + i].take();

        let stack_to = stacks[to].as_mut().unwrap();
        stack_to.buf[stack_to.height + i] = c;
    }

    stacks[from].as_mut().unwrap().height -= count;
    stacks[to].as_mut().unwrap().height += count;
}
