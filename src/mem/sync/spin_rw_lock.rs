use core::sync::atomic::Ordering;
use core::{cell::UnsafeCell, sync::atomic::AtomicBool};

use core::ops::{Deref, DerefMut};

use crate::cpu;

#[derive(Debug)]
pub struct RwLock<T: ?Sized> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

#[derive(Debug)]
pub struct RwLockReadGuard<'a, T: ?Sized> {
    rw_lock: &'a RwLock<T>,
}

#[derive(Debug)]
pub struct RwLockWriteGuard<'a, T: ?Sized> {
    rw_lock: &'a RwLock<T>,
}

unsafe impl<T: ?Sized> Sync for RwLock<T> {}
unsafe impl<T: ?Sized> Send for RwLock<T> {}

unsafe impl<'a, T: ?Sized> Send for RwLockReadGuard<'a, T> {}
unsafe impl<'a, T: ?Sized> Sync for RwLockReadGuard<'a, T> {}
unsafe impl<'a, T: ?Sized> Send for RwLockWriteGuard<'a, T> {}
unsafe impl<'a, T: ?Sized> Sync for RwLockWriteGuard<'a, T> {}

impl<'a, T: ?Sized> Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rw_lock.data.get() }
    }
}

impl<'a, T: ?Sized> Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rw_lock.data.get() }
    }
}

impl<'a, T: ?Sized> DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.rw_lock.data.get() }
    }
}

impl<'a, T: ?Sized> Drop for RwLockWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.rw_lock.unlock();
    }
}

impl<T> RwLock<T> {
    pub fn new(data: T) -> Self {
        RwLock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> RwLock<T> {
    pub fn read(&self) -> RwLockReadGuard<T> {
        while self.locked.load(Ordering::Relaxed) {
            cpu::pause();
        }
        RwLockReadGuard { rw_lock: self }
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            cpu::pause();
        }
        RwLockWriteGuard { rw_lock: self }
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod test {
    use super::RwLock;
    use ntest::timeout;
    use std::sync::Arc;

    #[test]
    fn creates_new_rwlock() {
        let rw_lock = RwLock::new(0);

        unsafe {
            assert_eq!(*rw_lock.data.get(), 0);
            assert_eq!(
                rw_lock.locked.load(core::sync::atomic::Ordering::Relaxed),
                false
            );
        }
    }

    #[test]
    fn write_lock_gets_value_exclusive_reference_and_allow_reads_after_drop() {
        let rw_lock = RwLock::new(0);
        let mut write_guard = rw_lock.write();
        *write_guard += 1;
        assert_eq!(*write_guard, 1);
        drop(write_guard);
        rw_lock.read();
    }

    #[test]
    #[timeout(500)]
    #[should_panic]
    fn multiple_writes_deadlock() {
        let rw_lock = RwLock::new(0);
        let mut write_guard = rw_lock.write();
        *write_guard += 1;
        let mut write_guard2 = rw_lock.write();
        *write_guard2 += 1;
        // Shouldn't reach here
        assert!(false)
    }

    #[test]
    #[timeout(500)]
    #[should_panic]
    fn read_after_writes_deadlock() {
        let rw_lock = RwLock::new(0);
        let mut write_guard = rw_lock.write();
        *write_guard += 1;
        rw_lock.read();
        // Shouldn't reach here
        assert!(false)
    }

    #[test]
    fn allow_multiple_reads_when_no_writes() {
        let rw_lock = RwLock::new(0);
        let read_guard1 = rw_lock.read();
        let read_guard2 = rw_lock.read();
        let read_guard3 = rw_lock.read();
        assert_eq!(*read_guard3, 0);
        assert_eq!(*read_guard2, 0);
        assert_eq!(*read_guard1, 0);
    }

    #[test]
    fn concurrent_locking() {
        let mutex = Arc::new(RwLock::new(0));
        const RUNS: usize = 100000;

        let mutex_b = mutex.clone();
        let t = std::thread::spawn(move || {
            for _ in 0..RUNS {
                let mut a = mutex_b.write();
                std::thread::yield_now();
                *a += 1;
                std::thread::yield_now();
            }
        });

        for _ in 0..RUNS {
            let mut a = mutex.write();
            std::thread::yield_now();
            *a += 1;
            std::thread::yield_now();
        }

        t.join().unwrap();

        assert_eq!(*mutex.read(), RUNS * 2);
    }
}
