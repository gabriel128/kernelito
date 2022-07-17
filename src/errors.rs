#![allow(dead_code)]

use core::fmt::Display;

#[derive(Debug)]
pub struct KernelError {
    message: &'static str,
    kind: KernelErrorKind,
}

impl Display for KernelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind {
            KernelErrorKind::MemError(addr) => {
                write!(
                    f,
                    "[Kernel MemError] {}, address 0x{:x}",
                    self.message, addr
                )
            }
        }
    }
}

impl KernelError {
    pub fn new(message: &'static str, kind: KernelErrorKind) -> Self {
        Self { message, kind }
    }

    pub fn lifted<T>(message: &'static str, kind: KernelErrorKind) -> crate::Result<T> {
        Err(Self { message, kind })
    }
}

type Addr = u32;

#[derive(Debug)]
pub enum KernelErrorKind {
    MemError(Addr),
}
