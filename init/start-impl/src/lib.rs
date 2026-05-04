#![no_main]
#![no_std]

use oes_arch as _;
// use oes_arch::common::uefi::main as _;
/// The entrypoint to the kernel.
///
///
/// # SAFETY
/// This shouldn't be called by rust directly, but safety requirements
/// will still be listed here. (Please note that if you're calling this in Rust
/// undefined behaver will most likely occur)
///
/// These safety requirements are not strictly imposed by the bootloader, atho
/// there will be overlap. But are supposed to be upheld by our linker scripts and assembly
///
/// When this function is called the following must be met:
/// * Called 16 bit aligned (C abi requires this)
/// * Memory is properly segmented
///
/// * Paging is setup
// Makes sure each arch updates this if a signature changes
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn start_kernel() -> ! {
    loop {
        //@TODO: Call initilizers and start up the kernel's runtime and the init binary
        core::hint::spin_loop();
    }
}

#[used]
static GIT_COMMIT: &str = env!("GIT_COMMIT");
