// SAFETY: we implement this elsewhere

#[doc(inline)]
pub use liballoc::alloc::*;
unsafe extern "Rust" {
    pub safe fn panic_oom() -> !;
}
// #[alloc_error_handler]
// NOTE: this is exported as "__rust_alloc_error_handler(size,align)"
// this is obviously
// fn oom(lay: alloc::Layout) -> ! {

// }
