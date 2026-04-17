#![no_std]
#![no_main]
#![feature(abi_custom)]
#![feature(abi_x86_interrupt)]

pub(crate) mod interrupts;
pub mod paging;
/// The current Interrupt Descriptor Table
///
/// # Sync-ness
/// There is none currently.
///
/// The default value has an empty table
static mut IDT: interrupts::Idt = interrupts::Idt::empty();

unsafe extern "custom" {
    unsafe fn _start() -> !;
}
unsafe extern "C" {
    #[expect(unused, reason = "Used by assembly")]
    unsafe fn start_kernel() -> !;
}

/// Initialize
///
/// # SAFETY
/// This should only be called once. Multiple calls can
/// cause reinitializing hardware and have internal states be inconstant
/// or corrupted
#[rustfmt::skip]
pub unsafe fn init_hardware() {
    // SAFETY: The caller that this function is only called once
    // which that the 
    // says this should only be called once, and we aren't
    // referencing or mutating the global mut anywhere else  
    #[expect(static_mut_refs)] // ^^^^^^^^^^^^^^^^^^^^^^^
    unsafe {
        IDT = interrupts::Idt::new();
        IDT.load();
    }


}
