#![no_std]
#![no_main]
#![feature(abi_custom)]
#![feature(abi_x86_interrupt)]

pub(crate) mod interrupts;
pub mod mem;
pub(crate) mod multiboot;
pub mod paging;
pub mod vga;
/// The current Interrupt Descriptor Table
///
/// # Sync-ness
/// There is none currently.
///
/// The default value has an empty table
static mut IDT: interrupts::Idt = interrupts::Idt::empty();

// unsafe extern "custom" {
//     unsafe fn _start() -> !;
// }
unsafe extern "C" {
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

unsafe extern "C" fn __x86_store_multiboot2(ptr: *const ()) {}

core::arch::global_asm! {
    "",
// https://www.gnu.org/software/grub/manual/multiboot/multiboot.pdf

".set ALIGN,     1<<0",
".set MEMINFO,   1<<1", /* Memory Map*/
".set FLAGS,     ALIGN|MEMINFO",
".set MAGIC,     0x1BADB002", /* Magic */
".section .multiboot",
".align 4",
".long MAGIC",
".long FLAGS",
".long CHECKSUM",
}
core::arch::global_asm! {
    "",
/*
 * multiboot standard does not define a value for esp.
 *
 * This allocates room for a small stack then allocating 16384 bytes
 *
 * This also allows the stack to be 16 bit aligned
*/
".section .bss",
".align 16",
"stack_bottom:",
".skip 16384", // 16 KiB
"stack_top:",
options(att_syntax)
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "custom" fn _start() {
    core::arch::naked_asm! {
        /*
        * We are currently in 32-bit protected mode on x86
        *
        * Interrupts and paging are disabled
        *
        */
        "mov stack_top(%rip), %esp",
        // A Multiboot2-compliant bootloader provides an information structure when the kernel boots
        // A pointer is stored in EBX, while the magic number is stored in
        // EAX
        "cmpl $0xE85250D6, %eax",
        "jne .L3",
        "mov -16(%ebx), %rdi", // %rdi is first param of C-abi

        // "call {mboot}",
    ".L3:",

        // Initialize paging, and segmentation interrupts will be enabled inside
        // kernel_main
        "call {kernel_main}",
    "1:",
        "hlt",
        "jmp 1b",

    /*
     * Set the size of the _start symbol to the current location '.' minus its start.
     * This is useful when debugging or when you implement call tracing.
     * ALSO! There was an error with the
    */
    // For some reason, this isn't seen as correct on x86-64-unknown-uefi target but
    // on the -none target it is?
    #[cfg(not(target_os = "uefi"))]
    ".size _start, . - _start",
    // mboot = sym __x86_store_multiboot2,
    kernel_main = sym start_kernel,
    options(att_syntax)
        }
}
