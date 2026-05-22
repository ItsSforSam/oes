#[panic_handler]
#[cfg(not(any(test, feature = "libtest")))]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {
        //@TODO: implement proper panic handler
        core::hint::spin_loop();
    }
}
// https://github.com/rust-lang/rust/issues/98133
// https://github.com/rust-lang/rust/blob/a4cec9742b7e05c33c84cd75002cd56762f7e33b/library/panic_abort/src/lib.rs#L90-L156
// https://learn.microsoft.com/en-us/windows/win32/memory/cxxframehandler3
#[unsafe(no_mangle)]
#[cfg(target_os = "uefi")]
pub extern "C" fn __CxxFrameHandler3(
    _record: usize,
    _frame: usize,
    _context: usize,
    _dispatcher: usize,
) -> u32 {
    1 // `ExceptionContinueSearch`
}
