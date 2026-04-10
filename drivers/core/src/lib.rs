//! Core interfaces for drivers
#![no_std]
use core::{
    any::{Any, TypeId},
    cell::UnsafeCell,
    marker::PhantomPinned,
    mem::MaybeUninit,
};
pub mod error;
pub mod generic;
use core::marker::{Send, Sync};

// use downcast_rs::{Downcast, impl_downcast};
/// Used as an opaque reference to a type
pub unsafe trait Device: Sync + Send {
    fn init(&mut self) -> Result<(), error::DeviceError>;
}
impl dyn Device {
    // We implement this to have a dyn Device to be moved around opaque
    pub fn is<T: Device + Any>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    pub fn downcast_unchecked_ref<T: Device + Any>(&self) -> &T {
        debug_assert!(self.is::<T>());
        // SAFETY: caller guarantees that T is the correct type
        unsafe { &*(self as *const dyn Device as *const T) }
    }
}
// impl_downcast!(Device);
#[repr(transparent)]
pub struct Opaque<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    /// We cannot have this type being moved around
    _pin: PhantomPinned,
}

impl<T> Opaque<T> {
    pub const fn new(value: T) -> Self {
        Opaque {
            value: UnsafeCell::new(MaybeUninit::new(value)),
            _pin: PhantomPinned,
        }
    }

    pub const fn unit() -> Self {
        Opaque {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            _pin: PhantomPinned,
        }
    }
    pub const fn zeroed() -> Self {
        Opaque {
            value: UnsafeCell::new(MaybeUninit::zeroed()),
            _pin: PhantomPinned,
        }
    }
}

pub trait Display: Device {}
