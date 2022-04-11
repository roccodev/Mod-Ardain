use std::ptr::NonNull;

use crate::PlatformData;

pub struct DestroyLater<'p, T> {
    inner: T,
    destructor: unsafe fn(&mut T, &'p PlatformData),
    platform: &'p PlatformData,
}

/// A container for a resource that is meant to cross language boundaries.
///
/// There are two types of FFI resources:
///   * **Owned**: the resource is created from Rust and it is destroyed from Rust.
///   * **Foreign**: the resource is created by foreign code, thus it shouldn't
///   be destroyed from the Rust side.
///
/// An **owned** FFI resource will always run its destructor when it is dropped.
pub enum FfiPointer<'p, O, F> {
    Owned(DestroyLater<'p, O>),
    Foreign(NonNull<F>),
}

impl<'p, O, F> FfiPointer<'p, O, F> {
    pub fn new_owned(
        platform: &'p PlatformData,
        obj: O,
        destructor: unsafe fn(&mut O, &'p PlatformData),
    ) -> FfiPointer<'p, O, F> {
        Self::Owned(DestroyLater {
            inner: obj,
            destructor,
            platform,
        })
    }

    /// Creates a `Foreign` pointer from a raw pointer.
    ///
    /// Returns `None` if the pointer is null.
    pub fn from_ptr(ptr: *const F) -> Option<FfiPointer<'p, O, F>> {
        NonNull::new(ptr as *mut F).map(Self::Foreign)
    }

    pub fn from_mut_ptr(ptr: *mut F) -> Option<FfiPointer<'p, O, F>> {
        FfiPointer::from_ptr(ptr)
    }

    pub fn as_ptr(&self) -> *const F {
        match self {
            FfiPointer::Owned(o) => (&o.inner as *const O).cast::<F>(),
            FfiPointer::Foreign(f) => f.as_ptr(),
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut F {
        match self {
            FfiPointer::Owned(o) => (&mut o.inner as *mut O).cast::<F>(),
            FfiPointer::Foreign(f) => f.as_ptr(),
        }
    }
}

impl<'t, T> Drop for DestroyLater<'t, T> {
    fn drop(&mut self) {
        unsafe { (self.destructor)(&mut self.inner, &self.platform) }
    }
}
