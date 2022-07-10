#![allow(dead_code)]

use crate::{idt, io::Port};

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;
const PIC1_COMMAND_PORT: u16 = PIC1;
const PIC2_COMMAND_PORT: u16 = PIC2;
const PIC1_DATA_PORT: u16 = PIC1 + 1;
const PIC2_DATA_PORT: u16 = PIC2 + 1;
const PIC_EOI: u8 = 0x20;

const CMD_INIT: u8 = 0x11;
const MODE_8086: u8 = 0x01;

// NOTE: Mapping PIC1 for now
pub fn init() {
    unsafe {
        // Sending Init Command that is 0x11
        Port::new(PIC1_COMMAND_PORT).write_byte(0x11);
        // Setting offset for idt
        Port::new(PIC1_DATA_PORT).write_byte(idt::IRQ_OFFSET);
        // 0x01 is 8086  mode
        Port::new(PIC1_DATA_PORT).write_byte(MODE_8086);
    }
}

pub fn end_of_interrupt() {
    unsafe {
        // Port::new(PIC1_COMMAND_PORT).write_byte(PIC_EOI);
        Port::new(0x20).write_byte(0x20);
    }
}
