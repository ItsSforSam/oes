use uefi::prelude::*;

#[entry]
#[doc(hidden)] // Not supposed to be called
pub unsafe fn main() -> Status {
    uefi::helpers::init().unwrap(); // This doesn't panic in the current version, 0.37 without logging feature

    // SAFETY: We call this once (in uefi entrypoint)
    // The [`uefi::entry`] calls set_virtual_address_map, which sets this to virtual address space
    unsafe { crate::common::start_kernel() };
    // Status::SUCCESS
}
