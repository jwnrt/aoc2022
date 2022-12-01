use core::fmt;

pub struct Uart;

impl Uart {
    // TODO: if on nightly, use `ptr::with_addr` for provenance.
    const BASE_ADDR: *mut u8 = 0x10000000 as *mut u8;

    // Register offsets
    const RBR: isize = 0x00;
    const THR: isize = 0x00;
    const LSR: isize = 0x05;

    // Masks for the LSR register
    const RT: u8 = 0x60; // Ready to Transmit
    const DR: u8 = 0x01; // Data Ready

    pub fn new() -> Self {
        Self {}
    }

    pub fn read_buf(&self, buf: &mut [u8]) -> usize {
        // Byte representing end of input
        const EOT: u8 = 0x04;

        for i in 0..buf.len() {
            let b = self.read_byte();
            if b == EOT {
                return i;
            }
            buf[i] = b;
        }

        buf.len()
    }

    fn read_byte(&self) -> u8 {
        // Busy loop until a byte appears
        while unsafe { Self::BASE_ADDR.offset(Self::LSR).read_volatile() } & Self::DR == 0 {}

        unsafe { Self::BASE_ADDR.offset(Self::RBR).read_volatile() }
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
