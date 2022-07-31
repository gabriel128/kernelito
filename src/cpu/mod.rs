/// Since sse is disabled we can't use core spin_loop hint, so we add a pause asm instruction manually
#[inline(always)]
pub fn pause() {
    unsafe {
        core::arch::asm!("pause", options(nomem, nostack));
    }
}
