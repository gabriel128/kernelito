// Full reference https://wiki.osdev.org/Exceptions

const DIVIDE_BY_ZERO_VNO: u16 = 0;
const DOUBLE_FAULT_VNO: u16 = 8;
const GENERAL_PROTECTION_FAULT_VNO: u16 = 13;
const PAGE_FAULT_VNO: u16 = 14;
const TIMER_IRQ_NO: u16 = 32;
const KYBD_IRQ_NO: u16 = 33;

#[derive(Clone)]
enum HandlerFn {
    Exception(extern "x86-interrupt" fn(frame: IntStackFrame)),
    ExceptionWithCode(extern "x86-interrupt" fn(frame: IntStackFrame, code: u32)),
    Irq(extern "x86-interrupt" fn()),
}

impl From<HandlerFn> for u32 {
    fn from(the_fn: HandlerFn) -> Self {
        match the_fn {
            HandlerFn::Exception(inner_fn) => inner_fn as u32,
            HandlerFn::ExceptionWithCode(inner_fn) => inner_fn as u32,
            HandlerFn::Irq(inner_fn) => inner_fn as u32,
        }
    }
}

pub struct Handler {
    pub interrupt_num: u16,
    handler_fn: HandlerFn,
}

impl Handler {
    fn new(interrupt_num: u16, handler_fn: HandlerFn) -> Self {
        Self {
            interrupt_num,
            handler_fn,
        }
    }

    pub fn handler_fn_ptr(&self) -> u32 {
        self.handler_fn.clone().into()
    }
}

pub fn all() -> [Handler; 6] {
    [
        Handler::new(
            DIVIDE_BY_ZERO_VNO,
            HandlerFn::Exception(exceptions::divide_by_zero),
        ),
        Handler::new(
            DOUBLE_FAULT_VNO,
            HandlerFn::Exception(exceptions::double_fault),
        ),
        Handler::new(
            GENERAL_PROTECTION_FAULT_VNO,
            HandlerFn::ExceptionWithCode(exceptions::general_protection),
        ),
        Handler::new(
            PAGE_FAULT_VNO,
            HandlerFn::ExceptionWithCode(exceptions::page_fault),
        ),
        Handler::new(TIMER_IRQ_NO, HandlerFn::Irq(irq::timer)),
        Handler::new(KYBD_IRQ_NO, HandlerFn::Irq(irq::keyboard_press)),
    ]
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

mod exceptions {
    use crate::idt::handlers::IntStackFrame;
    use crate::mem;

    pub extern "x86-interrupt" fn divide_by_zero(frame: IntStackFrame) {
        kprinterror!("Error frame: {:?}\n", frame);

        panic!("Exeception! Division by zero macho",);
    }

    pub extern "x86-interrupt" fn general_protection(frame: IntStackFrame, code: u32) {
        kprinterror!("Error frame: {:?}, code: {}\n", frame, code);

        panic!("Exeception! General Protection");
    }

    pub extern "x86-interrupt" fn double_fault(frame: IntStackFrame) {
        kprinterror!("Error frame: {:?}\n", frame);

        panic!("Exeception! Double fault");
    }

    pub extern "x86-interrupt" fn page_fault(frame: IntStackFrame, code: u32) {
        kprinterror!(
            "PAGE FAULT: address attempted to be used: 0x{:x}\n, frame: {:?}, code: {:?}\n",
            mem::page_faulted_addr(),
            frame,
            code
        );
        panic!("Exeception! Page fault");
    }
}

mod irq {
    use crate::pic;

    use crate::io_ports::Port8;

    pub extern "x86-interrupt" fn timer() {
        #[cfg(feature = "checks-mode")]
        kprint!(".");

        pic::end_of_interrupt();
    }

    pub extern "x86-interrupt" fn keyboard_press() {
        let scan_code = Port8::KeybData.read_byte();
        kprint!("Keyboard pressed, Scan code {}. ", scan_code);

        pic::end_of_interrupt();
    }
}
