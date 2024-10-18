// https://en.wikipedia.org/wiki/VGA_text_mode
use crate::flowlib::spinlock::SpinLock;
use core::fmt::{self, Write};

const VGA_ADDR: usize = 0xb8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const WHITE: u16 = 0xf;

pub struct VgaCharacter(u16);

pub struct VgaBuffer {
    addr: *mut [[VgaCharacter; VGA_WIDTH]; VGA_HEIGHT],
    cursor: usize
}

impl VgaCharacter {
    pub const fn new(byte: u8) -> Self {
        // White forground with no background
        VgaCharacter(WHITE << 8 | byte as u16)
    }
}

impl VgaBuffer {
    pub const fn new() -> Self {
        VgaBuffer { 
            addr: 0xb8000 as *mut [[VgaCharacter; VGA_WIDTH]; VGA_HEIGHT],
            cursor: 0
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
    
            }
        }
    }

    pub fn new_line(&self) {
        todo!()
    }
}

impl fmt::Write for VgaBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_str(s.as_bytes());
        Ok(())
    }
}

pub static WRITER: SpinLock<VgaBuffer> = SpinLock::new(VgaBuffer::new());

pub fn print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap()
}

