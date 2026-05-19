//! This provides the memory functions
//!
// We currently use compiler_builtins memory functions (via just ffi sense if import compiler builtins we either need to use
// a nightly feature using a sysroot, as -Zbuild-std causes it to fail to build with crates.io version)
// BUT we don't want to use them long term as they can be using FP registers. Which we don't want to use willy nilly
// especially in
//
use core::ffi::*;

unsafe extern "C" {
    /// Use [`memcmp`] whenever possible. This definition is simply here for completeness
    #[deprecated = "bcmp() is identical to memcmp(); use the latter instead."]
    pub unsafe fn bcmp(s1: *const u8, s2: *const u8, size: usize) -> i32;
    pub unsafe fn memcmp(s1: *const u8, s2: *const u8, size: usize) -> i32;
    pub unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub unsafe fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub unsafe fn memset(s: *mut u8, c: c_int, n: usize) -> *mut u8;

    pub unsafe fn strlen(string: *const c_char) -> usize;
}
