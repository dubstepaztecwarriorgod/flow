// https://en.wikipedia.org/wiki/VGA_text_mode
use crate::flowlib::spinlock::SpinLock;
use core::fmt::{self, Write};

pub struct VgaCharacter(u16);

pub struct VgaBuffer {
    addr: *mut VgaCharacter,
    buffer: [[VgaCharacter; 80]; 25]
}

impl VgaCharacter {
    pub const fn new(byte: u8) -> Self {
        // White forground with no background
        VgaCharacter(0xf << 8 | byte as u16)
    }
}

impl VgaBuffer {
    pub const fn new() -> Self {
        VgaBuffer { addr: 0xb8000 as *mut VgaCharacter }
    }

    pub fn print_str(&mut self, bytes: &[u8]) {
        for i in 0..bytes.len() {
            unsafe {
                *self.addr.add(i) = VgaCharacter::new(bytes[i])
            }
        }
    }
}

impl fmt::Write for VgaBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_str(s.as_bytes());
        Ok(())
    }
}

pub static WRITER: SpinLock<VgaBuffer> = SpinLock::new(VgaBuffer::new());

pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap()
}

