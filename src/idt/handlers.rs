// Full reference https://wiki.osdev.org/Exceptions

#[cfg(test)]
pub type HandlerFn = extern "C" fn();
#[cfg(test)]
pub type HandlerFnEx = extern "C" fn();

#[cfg(not(test))]
pub type HandlerFn = extern "x86-interrupt" fn(frame: IntStackFrame);

#[cfg(not(test))]
pub type HandlerFnEx = extern "x86-interrupt" fn(frame: IntStackFrame, code: u32);

const DIVIDE_BY_ZERO_VNO: u16 = 0;
const DOUBLE_FAULT_VNO: u16 = 8;
const GENERAL_PROTECTION_FAULT_VNO: u16 = 13;
const PAGE_FAULT_VNO: u16 = 14;
const TIMER_IRQ_NO: u16 = 32;
const KYBD_IRQ_NO: u16 = 33;

pub enum HandlerFunction {
    Fn(HandlerFn),
    FnWithErr(HandlerFnEx),
}

impl From<HandlerFunction> for u32 {
    fn from(the_fn: HandlerFunction) -> Self {
        match the_fn {
            HandlerFunction::Fn(inner_fn) => inner_fn as u32,
            HandlerFunction::FnWithErr(inner_fn) => inner_fn as u32,
        }
    }
}

pub struct Handler {
    pub interrupt_num: u16,
    pub handler_fn: HandlerFunction,
}

impl Handler {
    pub fn new(interrupt_num: u16, handler_fn: HandlerFunction) -> Self {
        Self {
            interrupt_num,
            handler_fn,
        }
    }
}

#[cfg(not(test))]
pub fn all() -> [Handler; 6] {
    [
        Handler::new(
            DIVIDE_BY_ZERO_VNO,
            HandlerFunction::Fn(exceptions::divide_by_zero),
        ),
        Handler::new(
            DOUBLE_FAULT_VNO,
            HandlerFunction::Fn(exceptions::double_fault),
        ),
        Handler::new(
            GENERAL_PROTECTION_FAULT_VNO,
            HandlerFunction::Fn(exceptions::general_protection),
        ),
        Handler::new(
            PAGE_FAULT_VNO,
            HandlerFunction::FnWithErr(exceptions::page_fault),
        ),
        Handler::new(TIMER_IRQ_NO, HandlerFunction::Fn(irq::timer)),
        Handler::new(KYBD_IRQ_NO, HandlerFunction::Fn(irq::keyboard_press)),
    ]
}

#[cfg(test)]
pub fn all() -> [Handler; 0] {
    []
}

/// Representation of 32bit interrupt stack frame
///
///  ------
///  eflags (32 bits)
///  ------
///  CS     (16 bits plus 16 bits of padding)
///  ------
///  EIP    (32 bits)
///  ------ (lower memory)
///
#[derive(Debug)]
#[repr(C)]
pub struct IntStackFrame {
    eip: u32,
    cs: u32,
    eflags: u32,
}

#[cfg(not(test))]
mod exceptions {
    use crate::idt::handlers::IntStackFrame;
    use crate::mem;

    pub extern "x86-interrupt" fn divide_by_zero(frame: IntStackFrame) {
        kprintln!("Frame {:?}", frame);
        panic!("Exeception! Division by zero macho");
    }

    pub extern "x86-interrupt" fn general_protection(frame: IntStackFrame) {
        panic!("Exeception! General Protection");
    }

    pub extern "x86-interrupt" fn double_fault(frame: IntStackFrame) {
        panic!("Exeception! Double fault");
    }

    pub extern "x86-interrupt" fn page_fault(frame: IntStackFrame, code: u32) {
        kprinterror!(
            "PAGE FAULT: address attempted to be used: 0x{:x}\n, frame: {:?}, code: {:?}",
            mem::page_faulted_addr(),
            frame,
            code
        );
        panic!("Exeception! Page fault");
    }
}

#[cfg(not(test))]
mod irq {
    use crate::idt::handlers::IntStackFrame;
    use crate::pic;

    #[cfg(feature = "checks-mode")]
    use crate::io::Port8;

    pub extern "x86-interrupt" fn timer(frame: IntStackFrame) {
        #[cfg(feature = "checks-mode")]
        kprint!(".");

        pic::end_of_interrupt();
    }

    pub extern "x86-interrupt" fn keyboard_press(frame: IntStackFrame) {
        #[cfg(feature = "checks-mode")]
        {
            let scan_code = Port8::KeybData.read_byte();
            kprint!("Keyboard pressed, Scan code {}", scan_code);
        }

        pic::end_of_interrupt();
    }
}
