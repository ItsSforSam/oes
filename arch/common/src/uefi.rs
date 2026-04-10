use uefi::prelude::*;

#[entry]
#[doc(hidden)] // Not supposed to be called
pub unsafe fn main() -> Status {
    uefi::helpers::init().unwrap(); // This doesn't 
    // SAFETY: We call this once (in uefi entrypoint)
    unsafe { crate::start_kernel() };
    // Status::SUCCESS
}
