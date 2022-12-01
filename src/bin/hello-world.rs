#![no_std]
#![no_main]

use core::fmt::Write;

extern crate virt;
use virt::uart::Uart;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();
    write!(uart, "Hello, world!\n").unwrap();

    loop {}
}
