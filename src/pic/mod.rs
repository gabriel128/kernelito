#![allow(dead_code)]

use crate::{idt, io::Port8};

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;
const PIC1_COMMAND_PORT: u16 = PIC1;
const PIC2_COMMAND_PORT: u16 = PIC2;
const PIC1_DATA_PORT: u16 = PIC1 + 1;
const PIC2_DATA_PORT: u16 = PIC2 + 1;

const PIC_EOI: u8 = 0x20;

const CMD_INIT: u8 = 0x11;
const MODE_8086: u8 = 0x01;

// NOTE: Mapping only the primary PIC for now
pub fn init() {
    // Sending Init Command that is 0x11
    Port8::Pic1Cmd.write_byte(CMD_INIT);
    // Setting offset for idt
    Port8::Pic1Data.write_byte(idt::IRQ_OFFSET);
    // 0x01 is 8086  mode
    Port8::Pic1Data.write_byte(MODE_8086);
}

pub fn end_of_interrupt() {
    Port8::Pic1Cmd.write_byte(PIC_EOI);
}
