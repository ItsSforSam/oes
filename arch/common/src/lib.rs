//! Architecture independent variables
//!
#![no_std]
#![no_main]
unsafe extern "C-unwind" {
    // Read it's docs to ensure
    pub(crate) unsafe fn start_kernel() -> !;
}
#[cfg(feature = "uefi")]
pub mod uefi;
