#![no_std]
#![no_main]

use core::fmt::Write;
use core::str;

extern crate virt;
use virt::uart::Uart;

#[no_mangle]
fn main() -> ! {
    let mut uart = Uart::new();

    let mut buf = [0u8; 16];
    let len = uart.read_buf(&mut buf);
    let buf_str = str::from_utf8(&buf[0..len]).unwrap();

    write!(uart, "{}", buf_str).unwrap();

    loop {}
}
