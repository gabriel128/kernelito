// Full reference https://wiki.osdev.org/Exceptions

pub type HandlerFn = extern "x86-interrupt" fn();

const DIVIDE_BY_ZERO_VNO: u16 = 0;
const DOUBLE_FAULT_VNO: u16 = 8;
const GENERAL_PROTECTION_FAULT_VNO: u16 = 13;
const PAGE_FAULT_VNO: u16 = 14;
const TIMER_IRQ_NO: u16 = 32;
const KYBD_IRQ_NO: u16 = 33;

// #[derive(Debug)]
pub struct Handler {
    pub interrupt_num: u16,
    pub handler_fn: HandlerFn,
}

impl Handler {
    pub fn new(interrupt_num: u16, handler_fn: HandlerFn) -> Self {
        Self {
            interrupt_num,
            handler_fn,
        }
    }
}

pub fn all() -> [Handler; 6] {
    [
        Handler::new(DIVIDE_BY_ZERO_VNO, exceptions::divide_by_zero),
        Handler::new(DOUBLE_FAULT_VNO, exceptions::double_fault),
        Handler::new(GENERAL_PROTECTION_FAULT_VNO, exceptions::general_protection),
        Handler::new(PAGE_FAULT_VNO, exceptions::page_fault),
        Handler::new(TIMER_IRQ_NO, irq::timer),
        Handler::new(KYBD_IRQ_NO, irq::keyboard_press),
    ]
}

mod exceptions {
    pub extern "x86-interrupt" fn divide_by_zero() {
        panic!("Exeception! Division by zero macho");
    }

    pub extern "x86-interrupt" fn general_protection() {
        panic!("Exeception! General Protection");
    }

    pub extern "x86-interrupt" fn double_fault() {
        panic!("Exeception! Double fault");
    }

    pub extern "x86-interrupt" fn page_fault() {
        panic!("Exeception! Page fault");
    }
}

mod irq {
    use crate::pic;

    #[cfg(feature = "checks-mode")]
    use crate::io::Port8;

    pub extern "x86-interrupt" fn timer() {
        #[cfg(feature = "checks-mode")]
        kprint!(".");

        pic::end_of_interrupt();
    }

    pub extern "x86-interrupt" fn keyboard_press() {
        #[cfg(feature = "checks-mode")]
        {
            let scan_code = Port8::KeybData.read_byte();
            kprint!("Keyboard pressed, Scan code {}", scan_code);
        }

        pic::end_of_interrupt();
    }
}
