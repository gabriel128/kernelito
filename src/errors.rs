#![allow(dead_code)]

#[derive(Debug)]
pub struct KernelError {
    message: &'static str,
    kind: KernelErrorKind,
}

impl KernelError {
    pub fn new(message: &'static str, kind: KernelErrorKind) -> Self {
        Self { message, kind }
    }
}

#[derive(Debug)]
pub enum KernelErrorKind {
    MemError,
}
