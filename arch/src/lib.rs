// This re-exports all the architecture specific
// This is a meta package. This allows each architecture to have their own build scripts and
// use the common the crate if needed
#![no_std]
#![feature(optimize_attribute)]
#![feature(doc_cfg)] // only needed during docs anyways
#![feature(abi_custom)]
#![feature(abi_x86_interrupt)]
#![feature(arbitrary_self_types_pointers)]
#![allow(unused_features, reason = "Not all architectures use all features")]
// NOTE: due to how optimization passes work, builtins can still be called
// These simply get hindered...slightly. These are used for the mem module which contain alternatives to
// Rust's compiler
// There is no way to narrow this attribute to function level or even module level
#![no_builtins]

unsafe extern "C-unwind" {
    /// Read it's docs to ensure safety is upheld
    /// at it's [actual docs]
    ///
    /// [actual docs]: ::oes_init_start_impl::start_kernel
    #[allow(unused)]
    pub unsafe fn start_kernel() -> !;
}
cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")]{
        #[path = "../x86/mod.rs"]
        pub mod x86;
        pub use x86 as current;
    }else {
        compile_error!("Architecture ",env!("CARGO_CFG_TARGET_ARCH"), "not supported.");
    }
}
#[path = "common/mod.rs"]
mod co;
pub mod common {
    use super::current;
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use crate::co::*;

    // When adding a architecture. These docs should be seen as the rules
    // The normal comments mixed with the docs is included in such
    // It's not yet decide if details implementers should account for
    // in the actual doc, or left in comments
    // The mem functions (which is [`bcmp`],[`memcmp`],[`memmove`],[`memset`],[`strlen`])
    // Which need to attributed with `#[unsafe(no_mangle)]`, to compiler inserts to the functions
    // use our functions (since they are required by compiler)
    // `#[cfg_attr(not(test),unsafe(no_mange))]` instead to avoid overloading libc's
    // functions from the host, AND tests are free to use libc's functions to compare
    //

    /// Use [`memcmp`] whenever possible. This definition is simply here for completeness
    #[deprecated = "bcmp() is identical to memcmp(); use the latter instead."]
    #[expect(clippy::missing_panics_doc)]
    #[expect(deprecated)]
    pub unsafe fn bcmp(s1: *const u8, s2: *const u8, size: usize) -> i32 {
        // SAFETY: caller guarantees safety requirements
        unsafe { current::mem::bcmp(s1, s2, size) }
    }
    /// Compares the contents of first `size` bytes. Each of which interpreted as a
    /// [`u8`] (or [`c_uchar`]) of memory regions `s1` and `s2`.
    ///
    /// # Return
    ///
    // Copied from the man page
    // https://man.archlinux.org/man/memcmp.3p.en
    // https://man.archlinux.org/man/memcmp.3.en
    //
    /// The `memcmp()` function returns an integer less than,
    /// equal to, or greater than zero if the first `n` bytes of `s1` is found,
    /// respectively, to be less than, to match, or be greater than the first `n` bytes of `s2`.
    /// For a nonzero return value, the sign is determined by the sign of the
    /// difference between the first pair of bytes (interpreted as unsigned char) that differ in `s1` and `s2`.
    /// If `n` is zero, the return value is zero.
    ///
    /// # SAFETY
    ///
    /// The pointers `s1` and `s2` are properly initialized and can properly represented as [`u8`] of all of `size` bytes.
    /// This includes padding of structs which can be uninitialized!!
    ///
    /// An additional note (taken from the [man page])
    /// Don't rely on this function for confidential data, as this function is subject to side channeling
    /// attacks
    ///
    // If size is zero. This function is assumed to be safe and not cause UB
    // according to the libcore's docs. BUT it's said it can be changed and should follow
    // the standard. (Which is why this isn't included in the docs)
    // https://doc.rust-lang.org/core/#how-to-use-the-core-library
    /// [man page]: https://man.archlinux.org/man/memcmp.3.en
    /// [`c_uchar`]:core::ffi::c_uchar
    pub unsafe fn memcmp(s1: *const u8, s2: *const u8, size: usize) -> i32 {
        unsafe { current::mem::memcmp(s1, s2, size) }
    }
    /// Copies `n` bytes from `src` to `dest`.
    ///
    /// Memory regions may overlap
    ///
    // Implementers should refrain from using the heap with Box, or equivalents to store
    // the overlapping bytes as for
    // 1. Allocator can not be available. These builtin functions can inject themselves anywhere
    // 2. The allocator can implicitly call this function, causing infinite recursion (cause builtin)
    // 3. If you aren't convinced, look at yourself in the mirror
    ///
    /// # Returns
    /// This function returns `dest`
    /// # SAFETY
    /// * `src` and `dest` are non-null and properly aligned.
    /// * Requires `src` to be valid for reads.
    /// * Requires `dest` to be valid for writes.
    /// * Reads and writes for both `src` and `dest` for `n` length in bytes
    pub unsafe fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        unsafe { current::mem::memmove(dest, src, n) }
    }
    /// Sets fills the region of memory pointed at `dest` for `n` bytes with the constant byte `filler`
    ///
    /// # Returns
    /// Returns `dest`
    ///
    /// # SAFETY
    /// * `dest` must be valid for writes for `n` bytes
    ///
    ///
    pub unsafe fn memset(dest: *mut u8, filler: i32, n: usize) -> *mut u8 {
        unsafe { current::mem::memset(dest, filler, n) }
    }
    /// Calculate the size of a [c-string] pointed to by `s`.
    ///
    ///
    /// # Returns
    ///
    /// The number of bytes in the [c-string], **excluding** the null terminating
    /// byte (`\0`)
    ///
    /// # SAFETY
    ///
    /// * `s` must be valid for reads
    /// * `s` must point to a null terminated [c-string].
    /// * If using with [strndup(3p)] or equivalent functions, ensure to add `+1`
    /// to the return to guarantee that the null terminator is captured. As well
    /// as any other [off by one errors] do not occur
    ///
    // No, you cannot just add +1. Unfortunately, this can allow errors as devs
    // expect it not to include null terminator in the output
    // (as well as rust's CStr relay on strlen explicitly)
    // AND strlen is one of the special functions the compiler expects to exist
    //
    // strndup is not implemented, but...better cover our bases
    //
    /// [c-string]: core::ffi::CStr
    /// [off by one errors]:https://en.wikipedia.org/wiki/Off-by-one_error
    /// [strndup(3p)]: https://man.archlinux.org/man/strdup.3p.en
    pub unsafe fn strlen(s: *const i8) -> usize {
        unsafe { current::mem::strlen(s) }
    }

    unsafe extern "Rust" {
        /// See [`oes-kernel-core::abort()`] for details, as this is an alias to that
        ///
        /// [`oes-kernel-core::abort()`]
        pub unsafe fn abort() -> !;
    }
}
#[cfg(test)]
#[expect(clippy::undocumented_unsafe_blocks, reason = "We are testing here.")]
mod tests {
    use super::common::*;
    use super::*;
    #[test]
    fn test_empty_strlen() {
        // assert_eq!(common::strlen())
    }
}
