mod syscall;

extern crate proc_macro;
extern crate self as oes_kernel_core;
// use syn::Expr
/// Mark function as a syscall
///
/// # Requirements
/// * Marked [`unsafe`]- This is used as a sort of for an unsafe block [See Safety](syscall#safety)
///
///
/// # Safety
/// * This is called once per-syscall
/// * The proper parameters of proper type, and return type is given
///
/// # Example
///
/// ```Rust,no_test
/// #use oes_kernel_core::prelude::*;
///
/// #[syscall]
/// async extern "C" unsafe fn fork(caller:Task)->Result<pid_t,Errno>{
/// // The most amazing implementation, you won't believe it!
/// }
/// ```
///
/// Even allows you to neglect a return if
/// ```Rust,no_test
///
/// #use oes_kernel_core::prelude::*;
///
/// #[syscall]
/// async extern "C" unsafe fn exit(caller:Task, status:u32){
/// // We the quickest exit function in the west
/// }
///
/// ```
///
/// [`unsafe`]: <https://doc.rust-lang.org/std/keyword.unsafe.html>
#[proc_macro_attribute]
pub fn syscall(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as syscall::SyscallItem);

    match syscall::syscall_impl(item) {
        Ok(tt) => tt.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
