#![allow(dead_code)]

mod handlers;

use core::arch::asm;
use lazy_static::lazy_static;

use self::handlers::{Handler, HandlerFn};

const TOTAL_INTERRUPTS: usize = 64;

lazy_static! {
    static ref IDT: Idt = Idt::new();
}

#[inline(always)]
pub fn init() {
    IDT.load_idt();
}

pub struct Idt([IdtDescriptor; TOTAL_INTERRUPTS]);

impl Idt {
    #[inline(always)]
    fn new() -> Self {
        let mut idt = Self([IdtDescriptor::null(); TOTAL_INTERRUPTS]);
        idt.config_handlers();
        idt
    }

    #[inline(always)]
    pub fn load_idt(&'static self) {
        let size = core::mem::size_of::<Self>() - 1;
        let idtr = IdtRegister::new(size as u16, (self as *const _) as u32);

        // Taken from https://docs.rs/x86_64/latest/src/x86_64/instructions/tables.rs.html#39
        unsafe {
            asm!("lidt [{}]", in(reg) &idtr, options(readonly, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    fn config_handlers(&mut self) {
        for handler in handlers::all() {
            self.set_interrupt_handler(handler);
        }
    }

    #[inline(always)]
    fn set_interrupt_handler(&mut self, handler: Handler) {
        self.0[handler.interrupt_num as usize] = IdtDescriptor::new(handler.handler_fn);
    }
}

/// Instructr Descriptor Register
#[repr(C, packed)]
#[derive(Default)]
struct IdtRegister {
    /// One less than the size of the IDT in bytes
    size: u16,
    /// The address of the idt
    base: u32,
}

impl IdtRegister {
    #[inline(always)]
    fn new(size: u16, base: u32) -> Self {
        Self { size, base }
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IdtDescriptor {
    // offset bits 0..15
    offset_1: u16,
    // a code segment selector in GDT or LDT
    selector: u16,
    // unused, set to 0
    zero: u8,
    // gate type, dpl, and p fields
    type_attributes: u8,
    // offset bits 16..31
    offset_2: u16,
}

impl IdtDescriptor {
    #[inline(always)]
    pub fn null() -> Self {
        Self {
            offset_1: 0,
            selector: 0,
            zero: 0,
            type_attributes: 0,
            offset_2: 0,
        }
    }

    #[inline(always)]
    pub fn new(handler: HandlerFn) -> Self {
        let handler_addr = handler as u32;
        let type_attributes: u8 = TypeAttrs::new(true, Dpl::Ring0, GateType::InterruptGate).into();

        Self {
            offset_1: handler_addr as u16,
            // TODO: Extract to a constant CODE_SEG, create DATA_SEG 0x10 as well
            selector: 0x08,
            zero: 0,
            offset_2: (handler_addr >> 16) as u16,
            type_attributes,
        }
    }
}

#[derive(Debug)]
struct TypeAttrs {
    // Present bit. Must be set true for the descriptor to be valid.
    present: bool,
    // CPU Privilege Levels which are allowed to access this interrupt via the INT instruction. Hardware interrupts ignore this mechanism
    dpl: Dpl,
    //  4-bit value which defines the type of gate this Interrupt Descriptor represents
    gate_type: GateType,
}

impl TypeAttrs {
    #[inline(always)]
    fn new(present: bool, dpl: Dpl, gate_type: GateType) -> Self {
        Self {
            present,
            dpl,
            gate_type,
        }
    }
}

// bit representation
// [7 present | 6,5 dpl(ring) | 4 unused | 3-0 Gate type]
impl From<TypeAttrs> for u8 {
    #[inline(always)]
    fn from(type_attrs: TypeAttrs) -> Self {
        let present_bits: u8 = if type_attrs.present { 0b1000_0000 } else { 0 };
        let gt_type: u8 = type_attrs.gate_type.into();
        let dpl: u8 = type_attrs.dpl.into();

        present_bits | dpl | gt_type
    }
}

#[derive(Debug)]
enum GateType {
    InterruptGate,
    TrapGate,
}

impl From<GateType> for u8 {
    fn from(gt: GateType) -> u8 {
        match gt {
            GateType::InterruptGate => 0xE,
            GateType::TrapGate => 0xF,
        }
    }
}

#[derive(Debug)]
enum Dpl {
    Ring0,
    Ring3,
}
impl From<Dpl> for u8 {
    #[inline(always)]
    fn from(ring: Dpl) -> Self {
        match ring {
            Dpl::Ring0 => 0b0000_0000,
            Dpl::Ring3 => 0b0110_0000,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::idt::*;

    #[test]
    fn test_typeattrs() {
        let type_attributes: u8 = TypeAttrs::new(true, Dpl::Ring0, GateType::InterruptGate).into();

        assert_eq!(type_attributes, 0x8E);
    }
}
