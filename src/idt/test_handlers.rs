///! Test version of handlers to be able run test in stable
//

#[derive(Clone)]
enum HandlerFn {}

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
        0
    }
}

pub fn all() -> [Handler; 0] {
    []
}
