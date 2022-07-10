#![allow(dead_code)]
use core::arch::asm;

/// Making the Port interface safe given that we don't
/// allow arbitrary port numbers

/// Port8 represents a port that transmits 8 bits
#[derive(Debug)]
pub enum Port8 {
    Pic1Cmd,
    Pic1Data,
}

impl From<&Port8> for u16 {
    fn from(port: &Port8) -> Self {
        match port {
            Port8::Pic1Cmd => 0x20,
            Port8::Pic1Data => 0x21,
        }
    }
}

#[derive(Debug)]
pub enum Port16 {}

impl From<&Port16> for u16 {
    fn from(_port: &Port16) -> Self {
        unimplemented!()
    }
}

impl Port8 {
    pub fn write_byte(&self, val: u8) {
        let port: u16 = self.into();
        unsafe {
            asm!(
                "out dx, al",
                in("dx") port,
                in("al") val,
                options(nostack),
            );
        }
    }

    pub fn read_byte(&self) -> u8 {
        let val: u8;
        let port: u16 = self.into();

        unsafe {
            asm!(
                "in al, dx",
                in("dx") port,
                out("al") val,
                options(nostack),
            );
        }

        val
    }
}

impl Port16 {
    pub fn write_word(&self, val: u16) {
        let port: u16 = self.into();
        unsafe {
            asm!(
                "out dx, al",
                in("dx") port,
                in("ax") val,
                options(nostack),
            );
        }
    }

    pub fn read_word(&self) -> u16 {
        let val: u16;
        let port: u16 = self.into();

        unsafe {
            asm!(
                "in al, dx",
                in("dx") port,
                out("ax") val,
                options(nostack),
            );
        }

        val
    }
}
