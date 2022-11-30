use core::fmt;

pub struct Uart;

impl Uart {
    // TODO: if on nightly, use `ptr::with_addr` for provenance.
    const BASE_ADDR: *mut u8 = 0x10000000 as *mut u8;

    // Register offsets
    const THR: isize = 0x00;
    const LSR: isize = 0x05;

    // Masks for the LSR register
    const RT: u8 = 0x60; // Ready to Transmit

    pub fn new() -> Self {
        Self {}
    }

    fn write_byte(&self, b: u8) {
        // Busy loop until ready
        while unsafe { Self::BASE_ADDR.offset(Self::LSR).read_volatile() } & Self::RT == 0 {}
        // Write single byte
        unsafe {
            Self::BASE_ADDR.offset(Self::THR).write_volatile(b);
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.as_bytes() {
            self.write_byte(*b);
        }

        Ok(())
    }
}
