//! Core interfaces for drivers
#![no_std]
use core::{
    any::{Any, TypeId},
    cell::UnsafeCell,
    ffi::c_void,
    marker::PhantomPinned,
    mem::MaybeUninit,
};
pub mod error;
pub mod generic;
use core::marker::{Send, Sync};

// use downcast_rs::{Downcast, impl_downcast};
/// Generic Driver trait
///
///
/// This is used to initialize a driver.
///
///
/// If there any need for special cleanup if the driver
/// is unloaded, implement [`Drop`]
///
/// # SAFETY
///
pub unsafe trait Driver: Sized {
    fn init() -> Result<Self, error::DeviceError>;
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
#[derive(PartialEq, Eq, Default, Debug)]
pub enum DriverState {
    /// Not done initializing
    #[default]
    NotDone,
    /// The driver is fully live and formed
    Live,
    /// Unloading for one reason or another
    Unloading,
}
impl DriverState {
    /// Turns [`DriverState`] into [`Live`][DriverState::Live]
    pub fn done(&mut self) {
        debug_assert!(*self != DriverState::NotDone);
        *self = DriverState::Live
    }
}
/// The internally stored kernel object
///
pub struct DriverObject<'obj> {
    pub name: &'obj str,
    pub state: DriverState,
    /// This is the drop code for a module
    pub unload: Option<unsafe fn(*mut ())>,
}
impl Default for DriverObject<'_> {
    #[inline]
    fn default() -> Self {
        DriverObject {
            name: "",
            state: DriverState::NotDone,
            unload: None,
        }
    }
}

trait StoreDriver {
    fn store_into(&mut self, obj: &mut DriverObject);
}

impl<T: Driver> StoreDriver for T {
    fn store_into(&mut self, obj: &mut DriverObject) {
        let drop_code: unsafe fn(_) = core::mem::drop::<Self>;
        // SAFETY: raw pointers have same size as references
        obj.unload = Some(unsafe { core::mem::transmute(drop_code) })
    }
}
