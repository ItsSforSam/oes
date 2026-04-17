//! Core interfaces for drivers
#![no_std]
#![feature(debug_closure_helpers)]
use core::{
    cell::UnsafeCell,
    fmt::{self, Debug},
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
    ptr::NonNull,
};
pub mod error;
pub mod generic;

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
    /// A unique handle for the driver
    const NAME: &str;
    /// A slice of handles that the driver depends on
    ///
    /// This will allow them to be loaded before
    ///
    /// If nothing is needing to be depended on, leave default which is blank
    const DEPENDS: &[&str] = &[];
    fn init(obj: &mut DriverObject<'_>) -> Result<Self, error::DeviceError>;
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
    /// If NULL, this is a no-op
    unload: *const (),
}

impl<'obj> DriverObject<'obj> {
    pub fn change_drop<T: Sized>(&mut self, drop: unsafe fn(&mut T)) {
        self.unload = drop as *const ()
    }
    pub fn get_name(&self) -> &str {
        self.name
    }
}
impl Default for DriverObject<'_> {
    #[inline]
    fn default() -> Self {
        DriverObject {
            name: "",
            state: DriverState::NotDone,
            unload: core::ptr::null(),
        }
    }
}

trait StoreDriver: Sized {
    fn store_into(&mut self, obj: &mut DriverObject);
}

impl<T: Driver + Sized> StoreDriver for T {
    fn store_into(&mut self, obj: &mut DriverObject) {
        let drop_code: unsafe fn(_) = core::ptr::drop_in_place::<Self>;
        // SAFETY: raw pointers have same size as references
        // let drop_code: unsafe fn(*mut ()) = unsafe { core::mem::transmute(drop_code) };
        obj.unload = drop_code as *const ();
        obj.name = Self::NAME;
    }
}

// Based on linux's module_memory:
// https://github.com/torvalds/linux/blob/master/include/linux/module.h#L356-L364
pub struct DriverMemory<'driver> {
    base: NonNull<()>,
    size: usize,
    is_rox: bool,
    /// We hold all the memory from base all the way to size
    _data: PhantomData<&'driver UnsafeCell<[MaybeUninit<u8>]>>,
}

impl Debug for DriverMemory<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Have to do this as theres a mutable reference
        // and borrow rules and what not
        let alt = f.alternate();
        let d = &mut f.debug_struct("DriverMemory");
        if alt {
            // SAFETY: The outer range should be withen the allocation as per safety of constructing
            let outer = unsafe { self.base.byte_add(self.size) };
            d.field_with("layout", |f| write!(f, "[{:?}..{:?}]", self.base, outer));
        } else {
            d.field("base", &self.base).field("size", &self.size);
        }

        d.field("is_rox", &self.is_rox).finish_non_exhaustive()
    }
}

impl<'driver> DriverMemory<'driver> {
    /// Create a [`DriverMemory`] struct
    ///
    /// # SAFETY
    ///
    /// * `base` must be valid for reads or writes up to the `base.byte_add(size)`
    /// * `base` to up to the `size` must be apart of the same allocation, which lives as long as
    ///   the `'driver` lifetime
    /// * size must be smaller then [`isize::MAX`]
    pub unsafe fn new(base: NonNull<()>, size: usize, is_rox: bool) -> DriverMemory<'driver> {
        debug_assert!(size > isize::MAX as usize, "size > isize::MAX. size being `{}`", size);
        DriverMemory {
            base,
            size,
            is_rox,
            _data: PhantomData,
        }
    }
}
impl<'driver> AsRef<UnsafeCell<[MaybeUninit<u8>]>> for DriverMemory<'driver> {
    /// Returns a slice of the held memory
    ///
    /// # UnsafeCell-ness
    /// The underlyning memory can be mutated while you weren't looking
    ///
    /// # MaybeUninit-ness
    /// Even if all of
    fn as_ref(&self) -> &UnsafeCell<[MaybeUninit<u8>]> {
        // SAFETY: constructor requires all the memory from withen this range to be valid
        let d = unsafe {
            core::slice::from_raw_parts_mut(self.base.as_ptr() as *mut MaybeUninit<u8>, self.size)
        };
        UnsafeCell::from_mut(d)
    }
}
impl Default for DriverMemory<'_> {
    /// Implement a blank driver memory layout
    ///
    /// The internal pointer is dangling with size being zero
    fn default() -> Self {
        Self {
            base: NonNull::dangling(),
            size: 0,
            is_rox: true,
            _data: PhantomData,
        }
    }
}

pub trait Write {
    /// Write a buffer into the writer
    ///
    /// # Returns
    /// [`Some(n)`] is how many bytes where written
    ///
    /// [`None`] is an error occured
    ///
    /// **Note, this is pained to return a [`Result<T,E>`] in the future**
    ///
    /// [`Some(n)`]: Some
    // TODO: make a generic error type
    fn write(&mut self, buf: &[u8]) -> Option<usize>;
    /// Flush from internal buffer into IO object
    ///
    /// # Returns
    /// [`Some(())`] on success
    ///
    /// [`None`] when a error occurred.
    ///
    /// **Note, this is pained to return a [`Result<T,E>`] in the future**
    ///
    /// [`Some(())`]: Some
    fn flush(&mut self) -> Option<()>;
    /// Attempts to write the whole buffer into the writer
    ///
    /// # Returns
    /// [`Some(())`] on success
    ///
    /// [`None`] when a error occurred.
    ///
    /// **Note, this is pained to return a [`Result<T,E>`] in the future**
    /// [`Some(())`]: Some
    fn write_all(&mut self, mut buf: &[u8]) -> Option<()> {
        // TODO: once proper error type is made, check if it's
        // because it was interrupted
        while !buf.is_empty() {
            match self.write(buf) {
                Some(0) => return None,
                Some(n) => buf = &buf[n..],
                None => return None,
            }
        }
        Some(())
    }
    /// # Returns
    /// [`Some(())`] on success
    ///
    /// [`None`] when a error occurred.
    ///
    /// **Note, this is pained to return a [`Result<T,E>`] in the future**
    /// [`Some(())`]: Some
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Option<()> {
        if let Some(s) = args.as_str() {
            self.write_all(s.as_bytes())
        } else {
            todo!();
        }
    }
}

// fn default_write_fmt<W:Write+?Sized+fmt::Write>(this:&mut W,args: fmt::Arguments)->Option<()>{

//     match fmt::write(&*this,args){
//         Ok(()) => Some(()),
//         Err(_) => None
//     }
// }

impl fmt::Write for dyn crate::Write + '_ {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match Write::write_all(self, s.as_bytes()) {
            None => Err(fmt::Error),
            Some(_) => Ok(()),
        }
    }
}
