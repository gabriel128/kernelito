#![allow(dead_code)]
use core::arch::asm;

#[derive(Debug)]
pub struct Port {
    port: u16,
}

impl Port {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub unsafe fn write_byte(&self, val: u8) {
        asm!(
            "out dx, al",
            in("dx") self.port,
            in("al") val,
            options(nostack),
        );
    }

    pub unsafe fn read_byte(&self) -> u8 {
        let val: u8;

        asm!(
            "in al, dx",
            in("dx") self.port,
            out("al") val,
            options(nostack),
        );

        val
    }

    pub unsafe fn write_word(&self, val: u16) {
        asm!(
            "out dx, al",
            in("dx") self.port,
            in("ax") val,
            options(nostack),
        );
    }

    pub unsafe fn read_word(&self) -> u16 {
        let val: u16;

        asm!(
            "in al, dx",
            in("dx") self.port,
            out("ax") val,
            options(nostack),
        );

        val
    }
}
