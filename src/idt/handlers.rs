// Full reference https://wiki.osdev.org/Exceptions

pub type HandlerFn = fn();

const DIVIDE_BY_ZERO_VNO: u16 = 0;
const DOUBLE_FAULT_VNO: u16 = 8;
const GENERAL_PROTECTION_FAULT_VNO: u16 = 13;
const PAGE_FAULT_VNO: u16 = 14;

#[derive(Debug)]
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

pub fn all() -> [Handler; 4] {
    [
        Handler::new(DIVIDE_BY_ZERO_VNO, divide_by_zero),
        Handler::new(DOUBLE_FAULT_VNO, double_fault),
        Handler::new(GENERAL_PROTECTION_FAULT_VNO, general_protection),
        Handler::new(PAGE_FAULT_VNO, page_fault),
    ]
}

pub fn divide_by_zero() {
    panic!("Exeception! Division by zero macho");
}

pub fn general_protection() {
    panic!("Exeception! General Protection");
}

pub fn double_fault() {
    panic!("Exeception! Double fault");
}

pub fn page_fault() {
    panic!("Exeception! Page fault");
}
