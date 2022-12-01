# Advent of Code 2022

Planning to do as much as possible in bare-metal Rust running in QEMU.

## Setup

This repo is set up for running Rust inside QEMU's RISC-V `virt` machine. It's
going to be pretty heavily hardcoded for that specific target.

Project structure:

- `src/bin/`: binaries for each day's problem, plus some for testing `virt`.
- `src/virt/`: library for hosting binaries within `virt`.
- `qemu_rv32_virt.ld`: linker script for the `virt` machine, expecting binaries
  to define a `_start` symbol. See copyright notice in the file.
- `data/`: Input files for each day's problem.
- `.cargo/config.toml`: config for using the linker script and setting up QEMU
  to run on `cargo run`.

## How it works

The `virt` machine has a UART which the support library in `src/virt/` uses
directly for input and output.

The support library defines the `_start` symbol in assembly for setting up the
stack etc, then jumping the the `main` symbol. Each binary in `src/bin/` defines
a main function with the raw symbol like this:

```rust
extern crate virt; // Ensures the _start asm is included

#[no_mangle] // Disables Rust's symbol mangling
fn main() -> ! { ... }
```

Run binaries using `cargo run -r --bin <name>`. This runs QEMU using the
terminal as the console.

The console can be quite fiddly. Quit with `<alt-a> x`. I don't think you can
pipe input into `cargo run -r` easily, I've found that pasting the data into the
terminal and hitting <ctrl-d> will send the input.

At some point I might set up something nicer for getting data in and out.
