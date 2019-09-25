use core::ops::{AddAssign, SubAssign};
use num::{PrimInt};

use cortex_m::interrupt::CriticalSection;
use core::cell::UnsafeCell;

pub struct CSCounter<T: PrimInt+AddAssign+SubAssign>(pub UnsafeCell<T>);

impl<T: PrimInt+AddAssign+SubAssign> CSCounter<T> {
    pub fn _reset(&self, _cs: &CriticalSection) {
        // By requiring a CriticalSection be passed in, we know we must
        // be operating inside a CriticalSection, and so can confidently
        // use this unsafe block (required to call UnsafeCell::get).
        unsafe { *self.0.get() = T::zero() };
    }

    pub fn increment(&self, _cs: &CriticalSection) {
        unsafe { *self.0.get() += T::one() };
    }

    pub fn decrement(&self, _cs: &CriticalSection) {
        unsafe { *self.0.get() -= T::one() };
    }

    pub fn get(&self) -> T {
        unsafe { *self.0.get() }
    }

}

// Required to allow static CSCounter. See explanation below.
unsafe impl<T: PrimInt+AddAssign+SubAssign> Sync for CSCounter<T> {}