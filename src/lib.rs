#![no_std]
#![no_main]

#[panic_handler]
fn panic_fmt(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
