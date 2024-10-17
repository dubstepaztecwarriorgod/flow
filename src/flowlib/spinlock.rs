use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T> {
    used: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            used: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
    pub fn lock(&self) -> SpinGuard<T> {
        while self.used.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }

        SpinGuard { lock: self }
    }
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.used.store(false, Ordering::Release)
    }
}
