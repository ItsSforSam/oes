//! Architecture independent variables
//!
#![no_std]
#![feature(optimize_attribute)]
#![feature(doc_cfg)] // only needed during docs anyways
// #![cfg_attr(not(doc), expect(unused_features))]
use core::{fmt, marker::PhantomData};
unsafe extern "C-unwind" {
    // Read it's docs to ensure
    #[allow(unused)]
    pub(crate) unsafe fn start_kernel() -> !;
}
#[doc(cfg(miri))]
#[cfg(any(miri, doc))]
pub mod miri;
#[cfg(feature = "uefi")]
pub mod uefi;

/// This is based on Bindgen's [`__IncompleteArrayField`] from bindgen
///
/// This allows a C compliant flexible array member,
///
/// [`__IncompleteArrayField`]:https://rust-lang.github.io/rust-bindgen/using-fam.html#__incompletearrayfield
#[repr(transparent)]
#[derive(Default)]
// mark as dead code as it's not really needed
// and slate for it's removal if it has no use
struct IncompleteArrayField<T> {
    inner: [T; 0],
    _marker: PhantomData<T>,
}
#[expect(dead_code)]
impl<T> IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        IncompleteArrayField {
            inner: [],
            _marker: PhantomData,
        }
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), len) }
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &[T] {
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), len) }
    }
}

impl<T> fmt::Debug for IncompleteArrayField<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IncompleteArrayField")
    }
}
