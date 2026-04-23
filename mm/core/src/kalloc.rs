use core::alloc::{Allocator, GlobalAlloc};

/// Allocator for most Kernel structures
///
/// This is set as the [Global Allocator], although all of this
/// structs implement the [`GlobalAlloc`] trait
///
/// [Global Allocator]:GlobalAlloc
#[non_exhaustive]
pub struct KernelAlloc;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}
