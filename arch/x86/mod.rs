use core::fmt;
pub(crate) mod interrupts;
pub mod mem;
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

/// Initialize
///
/// # SAFETY
/// This should only be called once. Multiple calls can
/// ng hardware and have internal states be inconstant
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
/// Enters [real mode] temporary then enters back into [Protected Mode]
///
///
/// # SAFETY
///
/// [real mode]: https://wiki.osdev.org/Real_Mode#Switching_from_Protected_Mode_to_Real_Mode
/// [Protected Mode]:https://wiki.osdev.org/Protected_Mode
unsafe fn enter_real_mode<F: FnOnce()>(func: F) {
    x86_64::instructions::bochs_breakpoint();
}
unsafe extern "C" fn __x86_store_multiboot2(ptr: *const ()) {}

core::arch::global_asm! {
    "",
// https://www.gnu.org/software/grub/manual/multiboot/multiboot.pdf

".set ALIGN,     1<<0",
".set MEMINFO,   1<<1", /* Memory Map*/
".set FLAGS,     ALIGN|MEMINFO",
".set MAGIC,     0x1BADB002", /* Magic */
".set CHECKSUM, -(MAGIC + FLAGS)",
".section .multiboot",
".align 4",
".long MAGIC",
".long FLAGS",
".long CHECKSUM",
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

        "call {mboot}",
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
    mboot = sym __x86_store_multiboot2,
    kernel_main = sym crate::start_kernel,
    options(att_syntax)
        }
}
/// Is a breakpoint
#[inline(always)]
#[expect(clippy::undocumented_unsafe_blocks)]
pub fn breakpoint() {
    cfg_if::cfg_if! {
        if #[cfg(breakpoint_type = "bochs")] {

            unsafe {core::arch::asm!("xchgw %bx, %bx",options(nomem, nostack, preserves_flags,att_syntax))};
        } else { // a pseudo breakpoint
            unsafe {core::arch::asm!("1: jmp 1b",options(nomem, nostack, preserves_flags,att_syntax))}
        }
    }
}

pub mod unwind;

#[derive(Clone, Default)]
#[repr(transparent)]
pub struct Registers([RegistersInner; 1]);
#[derive(Clone, Default)]
#[repr(C)]
#[cfg(target_arch = "x86_64")]
pub struct RegistersInner {
    pub rbx: usize,
    pub rsp: usize,
    pub rbp: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub rip: usize,
}
impl core::ops::Deref for Registers {
    type Target = RegistersInner;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &(self.0[0])
    }
}

impl RegistersInner {
    fn debug_inner(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
        // f.debug_struct("RegistersInner")
        // .field(name, value)        .finish_non_exhaustive()
        // f.debug_struct("RegistersInner").field("rbx", &self.rbx).field("rsp", &self.rsp).field("rbp", &self.rbp).field("r12", &self.r12).field("r13", &self.r13).field("r14", &self.r14).field("r15", &self.r15).field("rip", &self.rip).finish()
    }
}

impl fmt::Debug for RegistersInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug_inner(f)
    }
}
impl fmt::Octal for RegistersInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(target_arch = "x86")]
pub struct RegistersInner {}
