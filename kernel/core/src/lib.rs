#![feature(negative_impls)]
// #![feature(allocator_api)]
#![feature(never_type)]
#![feature(ptr_metadata)]
#![feature(deprecated_suggestion)]
#![feature(negative_bounds)]
#![feature(allocator_api)]
#![no_std]
pub mod device;
pub mod error;
pub mod marker;
pub mod panicking;
pub mod printk;
pub mod sched;
pub mod sync;
pub mod traits;
// #[cfg(feature = "alloc")] // We use it regardless
extern crate alloc as liballoc;
#[cfg(feature = "alloc")]
pub mod alloc;
#[cfg(not(feature = "alloc"))]
mod alloc;
pub mod prelude {
    pub use crate::error::{Errno, ToErrno};
    pub use crate::traits::IntoUserSpace as _;
    #[cfg(feature = "alloc")]
    pub use liballoc::boxed::Box;
}
#[doc(inline)]
pub use oes_macros::*;
pub(crate) mod private {
    /// Internal Sealed crate
    pub trait Sealed {}
}
