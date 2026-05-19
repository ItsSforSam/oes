#[repr(C)]
#[derive(Clone, Default)]
pub struct StackFrame {
    regs: super::Registers,
}
/// returns `false` on normal execution, but if it's from [`restore_context`], it will return `true`
#[doc(alias = "setjmp")]
pub fn save_context(out: &mut StackFrame) -> bool {
    false
}

pub unsafe fn restore_context(env: StackFrame) -> ! {
    todo!()
}

#[repr(C)]
pub struct StackTrace {
    /// The base of the stack frame
    ///
    /// First thing pushed to the stack
    ebp: *mut StackTrace,
    /// the program counter
    ///
    #[cfg(target_arch = "x86")]
    eip: u32,
    #[cfg(target_arch = "x86_64")]
    /// The program counter
    // We call it eip, even tho it's RIP in x86_64 to make more portable
    eip: u64,
}

extern "C" fn stacktrace() {}
