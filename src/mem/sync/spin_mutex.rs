use core::{
    cell::UnsafeCell,
    ops::DerefMut,
    sync::atomic::{AtomicBool, Ordering},
};

use core::ops::Deref;

///! SpinMutex
///!
///! Implements a basic spinlock to be able to provide exclusive access and
///! safe interior mutability
///!
///! The interface is similar to std::sync::Mutex

#[derive(Debug)]
pub struct SpinMutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct SpinMutexGuard<'a, T> {
    mutex: &'a SpinMutex<T>,
}

impl<'a, T> Deref for SpinMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> DerefMut for SpinMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for SpinMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

unsafe impl<T> Send for SpinMutex<T> {}
unsafe impl<T> Sync for SpinMutex<T> {}

unsafe impl<T> Send for SpinMutexGuard<'_, T> {}
unsafe impl<T> Sync for SpinMutexGuard<'_, T> {}

impl<T> SpinMutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Obtains Exclusive access to the data.
    ///
    /// E.g.
    ///
    /// {
    ///   let mutex = SpinMutex::new(0);
    ///   *mutex.lock() += 1;
    /// }
    ///
    /// Uses compare_exchange_weak in a loop given that it's faster than the strong version.
    /// Also, it seems that x86 can't have sporious failures so the weak version should be fine
    pub fn lock(&self) -> SpinMutexGuard<T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Tries to make the CPU to not use as many resources given that it's
            // spinning
            core::hint::spin_loop();
        }
        SpinMutexGuard { mutex: &self }
    }

    /// Unlocks
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod test {
    use core::mem::{self, MaybeUninit};
    use core::time::Duration;
    use std::thread::{self, JoinHandle};

    use std::sync::Arc;

    use super::SpinMutex;

    #[test]
    fn creates_new_mutex() {
        let mutex = SpinMutex::new(0);

        unsafe {
            assert_eq!(*mutex.data.get(), 0);
            assert_eq!(
                mutex.locked.load(core::sync::atomic::Ordering::Relaxed),
                false
            );
        }
    }

    #[test]
    fn lock_gets_value_exclusive_reference() {
        let mutex = SpinMutex::new(0);
        let guard = mutex.lock();
        assert_eq!(*guard, 0);
    }

    #[test]
    fn lock_mutates_value_with_exclusive_reference() {
        let mutex = SpinMutex::new(0);
        let mut guard = mutex.lock();
        *guard = 1;

        assert_eq!(*guard, 1);
    }

    #[test]
    #[ignore]
    fn concurrent_locking() {
        let mutex = Arc::new(SpinMutex::new(0));
        const RUNS: usize = 1000;

        for _ in 0..RUNS {
            let inner_mutex = mutex.clone();
            thread::spawn(move || {
                let mut a = inner_mutex.lock();
                for _ in 0..50 {
                    std::thread::yield_now();
                    *a += 1;
                    std::thread::yield_now();
                }
            });
        }

        // Using sleep since there seems to be an issue with vecs including
        // JoinHandles in i686 target on x86_64 host
        std::thread::sleep(Duration::from_secs(30));

        assert_eq!(*mutex.lock(), RUNS * 50);
    }
}
