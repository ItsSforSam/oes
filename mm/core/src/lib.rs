//! The core components required for memory management
#![no_std]
#![feature(allocator_api)]
mod kalloc;
mod ualloc;
mod valloc;
pub use kalloc::KernelAlloc;
pub use ualloc::UserAlloc;
pub use valloc::VirtAlloc;

/// Kernel Allocator
///
/// See [`KernelAlloc`] for more info on how to use this type
#[global_allocator]
pub static KALLOC: KernelAlloc = KernelAlloc;
