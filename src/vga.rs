// https://en.wikipedia.org/wiki/VGA_text_mode
use crate::flowlib::spinlock::SpinLock;


const VGA_ADDRESS: usize = 0xb8000;
// White foreground
const CHAR_ATTRIBUTE: u8 = 0xf;

#[repr(C)]
pub struct VgaCharacter {
    attribute: u8,
    ascii_byte: u8
}

impl VgaCharacter {
    pub const fn new(byte: u8) -> Self {
        VgaCharacter { attribute: CHAR_ATTRIBUTE, ascii_byte: byte }
    }

    pub const fn as_u16(&self) -> u16 {
        (self.attribute as u16) << 8 | (self.ascii_byte as u16)
    }
}

pub struct VgaBuffer {
    addr: *mut u16
}

impl VgaBuffer {
    pub const fn new() -> Self {
        VgaBuffer { addr: VGA_ADDRESS as *mut u16 }
    }

    pub fn print_str(&mut self, bytes: &[u8]) {
        for i in 0..bytes.len() {
            unsafe {
                *self.addr.add(i) = VgaCharacter::new(bytes[i]).as_u16()
            }
        }
    }
}

pub static Writer: SpinLock<VgaBuffer> = VgaBuffer {
    addr: VGA_ADDRESS as *mut u16
};