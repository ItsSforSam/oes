#![feature(negative_impls)]
// #![feature(allocator_api)]
#![feature(linkage)]
#![feature(core_intrinsics)]
#![feature(never_type)]
#![feature(ptr_metadata)]
#![feature(deprecated_suggestion)]
#![expect(internal_features, reason = "for core::intrinsics::abort")]
#![no_std]
pub mod error;
pub mod marker;
pub mod panicking;
pub mod printk;
pub mod sched;
pub mod sync;
// #[cfg(feature = "alloc")] // We use it regardless
extern crate alloc as liballoc;
#[cfg(feature = "alloc")]
pub mod alloc;

pub mod prelude {
    pub use crate::error::IntoUAbi as _;
    pub use crate::error::{Errno, ToErrno};
    #[cfg(feature = "alloc")]
    pub use liballoc::boxed::Box;
}
/// This will abort the kernel. Shut off the system, no questions
///
/// # SAFETY
/// This function doesn't cause any issues of it's own
/// but this can cause corruption with file systems with partially written data.
///
///
/// Each architecture should provide a dedicated
#[linkage = "weak"]
#[unsafe(no_mangle)]
pub unsafe fn abort() -> ! {
    core::intrinsics::abort()
}

pub(crate) mod private {
    /// Internal Sealed crate
    pub trait Sealed {}
}
