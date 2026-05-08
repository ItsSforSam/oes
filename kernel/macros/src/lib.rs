mod impls;

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
/// * This is called once
///
///
/// # Example
/// ```Rust,no_test
/// use oes_kernel_core::prelude::*;
/// #[syscall]
/// extern "C" unsafe fn fork()->Result<pid_t,Errno>{
/// // The most amazing implementation, you won't believe it!
/// }
/// ```
///
/// [`unsafe`]: <https://doc.rust-lang.org/std/keyword.unsafe.html>
#[proc_macro_attribute]
pub fn syscall(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemFn);
    match impls::check_syscall_sig(&item) {
        Ok(_) => { /* Nothing */ }
        Err(e) => {
            e.into_compile_error();
        }
    };
    impls::syscall_impl(item).into()
}
