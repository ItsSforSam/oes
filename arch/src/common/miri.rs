//! Provides all of Miri's intrinsics.
//!
//! These are handy if certain things aren't available in testing with Miri.
//!
//! Please note that
//!
//! Modified from [miri_extern.rs]
//!
//! [miri_extern.rs]:https://github.com/rust-lang/miri/blob/master/tests/utils/miri_extern.rs>

use core::ptr::NonNull;

/// A frame, returned from [`miri_resolve_frame()`]
///
/// If you want
#[repr(C)]
pub struct MiriFrame {
    // The size of the name of the function being executed, encoded in UTF-8
    pub name_len: usize,
    // The size of filename of the function being executed, encoded in UTF-8
    pub filename_len: usize,
    // The line number currently being executed in `filename`, starting from '1'.
    pub lineno: core::num::NonZero<u32>,
    // The column number currently being executed in `filename`, starting from '1'.
    pub colno: core::num::NonZero<u32>,
    // The function pointer to the function currently being executed.
    // This can be compared against function pointers obtained by
    // casting a function (e.g. `my_fn as *mut ()`)
    pub fn_ptr: NonNull<()>,
}
impl MiriFrame {
    pub fn compare_fn_ptr(&self, other: fn()) -> bool {
        self.fn_ptr.as_ptr() == other as *mut ()
    }
}
/// Miri-provided extern function to get the amount of frames in the current backtrace.
///
/// This is modified from the original signature with the `flags` parameter.
/// This is simply a wrapper function
pub fn miri_backtrace_size() -> usize {
    // SAFETY: flags is `0`
    unsafe { bt_size(0) }
}

unsafe extern "Rust" {
    /// Miri-provided extern function to mark the block `ptr` points to as a "root"
    /// for some static memory. This memory and everything reachable by it is not
    /// considered leaking even if it still exists when the program terminates.
    ///
    /// # Safety
    /// `ptr` has to point to the beginning of an allocated block.
    ///
    pub unsafe fn miri_static_root(ptr: *const u8);

    /// Miri-provided extern function to get the amount of frames in the current backtrace.
    /// The `flags` argument must be `0`.
    #[link_name = "miri_backtrace_size"]
    unsafe fn bt_size(flags: u64) -> usize;

    /// Miri-provided extern function to obtain a backtrace of the current call stack.
    /// This writes a slice of pointers into `buf` - each pointer is an opaque value
    /// that is only useful when passed to `miri_resolve_frame`.
    /// `buf` must have `miri_backtrace_size(0) * pointer_size` bytes of space.
    /// The `flags` argument must be `1`.
    #[link_name = "miri_get_backtrace"]
    pub unsafe fn get_backtrace_raw(flags: u64, buf: *mut *mut ());

    /// Miri-provided extern function to resolve a frame pointer obtained
    /// from [`miri_get_backtrace`]. The `flags` argument must be `1`.
    ///
    /// This function can be called on any thread (not just the one which obtained `frame`).
    pub unsafe fn miri_resolve_frame(frame: *mut (), flags: u64) -> MiriFrame;

    /// Miri-provided extern function to get the name and filename of the frame provided by `miri_resolve_frame`.
    /// `name_buf` and `filename_buf` should be allocated with the `name_len` and `filename_len` fields of `MiriFrame`.
    /// The flags argument must be `0`.
    pub unsafe fn miri_resolve_frame_names(
        ptr: *mut (),
        flags: u64,
        name_buf: *mut u8,
        filename_buf: *mut u8,
    );

    /// Miri-provided extern function to begin unwinding with the given payload.
    ///
    /// This is internal and unstable and should not be used; we give it here
    /// just to be complete.
    ///
    /// If you REALLY wanted you can pass a `Box<dyn Any + Send>`, which is
    /// what payloads actual type is[^payload]. That being said, this is uns
    ///
    /// [^payload]: This is only visible via looking in [rust's source code][https://github.com/rust-lang/rust/blob/main/library/panic_unwind/src/miri.rs]
    /// but it is what it is in our pinned version. Should you? No ofc not
    #[doc(hidden)]
    pub unsafe fn miri_start_unwind(payload: *mut u8) -> !;

    /// Miri-provided extern function to get the internal unique identifier for the allocation that a pointer
    /// points to. If this pointer is invalid (not pointing to an allocation), interpretation will abort.
    ///
    /// This is only useful as an input to `miri_print_borrow_stacks`, and it is a separate call because
    /// getting a pointer to an allocation at runtime can change the borrow stacks in the allocation.
    /// This function should be considered unstable. It exists only to support `miri_print_borrow_state` and so
    /// inherits all of its instability.
    pub unsafe fn miri_get_alloc_id(ptr: *const ()) -> u64;

    /// Miri-provided extern function to print (from the interpreter, not the program) the contents of all
    /// borrows in an allocation.
    ///
    /// If Stacked Borrows is running, this prints all the stacks. The leftmost tag is the bottom of the stack.
    ///
    /// If Tree borrows is running, this prints on the left the permissions of each tag on each range,
    /// an on the right the tree structure of the tags. If some tags were named via `miri_pointer_name`,
    /// their names appear here.
    ///
    /// If additionally `show_unnamed` is `false` then tags that did *not* receive a name will be hidden.
    /// Ensure that either the important tags have been named, or `show_unnamed = true`.
    /// Note: as Stacked Borrows does not have tag names at all, `show_unnamed` is ignored and all tags are shown.
    /// In general, unless you strongly want some tags to be hidden (as is the case in `tree-borrows` tests),
    /// `show_unnamed = true` should be the default.
    ///
    /// The format of what this emits is unstable and may change at any time. In particular, users should be
    /// aware that Miri will periodically attempt to garbage collect the contents of all stacks. Callers of
    /// this function may wish to pass `-Zmiri-provenance-gc=0` to disable the GC.
    ///
    /// This function is extremely unstable. At any time the format of its output may change, its signature may
    /// change, or it may be removed entirely.
    pub safe fn miri_print_borrow_state(alloc_id: u64, show_unnamed: bool);

    /// Miri-provided extern function to associate a name to the nth parent of a tag.
    /// Typically the name given would be the name of the program variable that holds the pointer.
    /// Unreachable tags can still be named by using nonzero `nth_parent` and a child tag.
    ///
    /// This function does nothing under Stacked Borrows, since Stacked Borrows's implementation
    /// of `miri_print_borrow_state` does not show the names.
    ///
    /// Under Tree Borrows, the names also appear in error messages.
    pub fn miri_pointer_name(ptr: *const (), nth_parent: u8, name: &[u8]);

    /// Miri-provided extern function to print (from the interpreter, not the
    /// program) the contents of a section of program memory, as bytes. Bytes
    /// written using this function will emerge from the interpreter's stdout.
    pub safe fn miri_write_to_stdout(bytes: &[u8]);

    /// Miri-provided extern function to print (from the interpreter, not the
    /// program) the contents of a section of program memory, as bytes. Bytes
    /// written using this function will emerge from the interpreter's stderr.
    pub safe fn miri_write_to_stderr(bytes: &[u8]);

    /// Miri-provided extern function to allocate memory from the interpreter.
    ///
    /// This is useful when no fundamental way of allocating memory is
    /// available, e.g. when using `no_std` + `alloc`.
    pub safe fn miri_alloc(size: usize, align: usize) -> *mut u8;

    /// Miri-provided extern function to deallocate memory.
    ///
    /// # Safety
    /// * `ptr` must point to the block of memory currently allocated
    /// * `size` and `align` must be the same values passed to [`miri_alloc`]
    pub unsafe fn miri_dealloc(ptr: NonNull<u8>, size: usize, align: usize);

    /// Add the allocation that this pointer points to to the "tracked" allocations.
    /// This is equivalent to `-Zmiri-track-allic-id=<id>`, but also works if the ID is
    /// only known at runtime.
    ///
    /// # Safety
    /// No safety requirements were listed by this function. BUT it's safe to say
    /// that this function should only be passed a pointer either via [`miri_alloc`]
    pub unsafe fn miri_track_alloc(ptr: NonNull<u8>);

    // /// Convert a path from the host Miri runs on to the target Miri interprets.
    // /// Performs conversion of path separators as needed.
    // ///
    // /// Usually Miri performs this kind of conversion automatically. However, manual conversion
    // /// might be necessary when reading an environment variable that was set on the host
    // /// (such as TMPDIR) and using it as a target path.
    // ///
    // /// Only works with isolation disabled.
    // ///
    // /// `in` must point to a null-terminated string, and will be read as the input host path.
    // /// `out` must point to at least `out_size` many bytes, and the result will be stored there
    // /// with a null terminator.
    // /// Returns 0 if the `out` buffer was large enough, and the required size otherwise.
    // pub fn miri_host_to_target_path(
    //     path: *const core::ffi::c_char,
    //     out: *mut core::ffi::c_char,
    //     out_size: usize,
    // ) -> usize;

    /// Run the provenance . The GC will run automatically at some cadence,
    /// but in tests we want to for sure run it at certain points to check
    /// that it doesn't break anything.
    pub safe fn miri_run_provenance_gc();

    // /// Miri-provided extern function to promise that a given pointer is properly aligned for
    // /// "symbolic" alignment checks.
    // /// Will fail if the pointer is not actually aligned or `align` is
    // /// not a power of two. Has no effect when alignment checks are concrete (which is the default).
    // // # Safety
    // // Pointer is simply checked for alignment
    // pub safe fn miri_promise_symbolic_alignment(ptr: *const (), align: usize);

    /// Blocks the current execution if the argument is false
    // What's the sure case here?
    safe fn miri_genmc_assume(condition: bool);

    /// Miri-provided extern function to spawn a new thread in the interpreter.
    ///
    /// Returns the thread id.
    ///
    /// This is useful when no fundamental way of spawning threads is available, e.g. when using
    /// `no_std`.
    ///
    /// `data` is passed into `t`
    #[link_name = "miri_thread_spawn"]
    #[doc(alias = "miri_thread_spawn")]
    pub unsafe fn spawn_thread_raw(t: extern "Rust" fn(*mut ()), data: *mut ()) -> usize;

    /// Miri-provided extern function to join a thread that was spawned by Miri.
    ///
    /// # Safety
    /// Passing in the wrong thread ID may cause unintentional behaver
    /// Not undefined, but easy to do wrong
    #[doc(alias = "miri_thread_join")]
    #[link_name = "miri_thread_join"]
    pub unsafe fn join_thread(thread_id: usize) -> bool;

    /// Indicate to Miri that this thread is busy-waiting in a spin loop.
    ///
    /// As far as Miri is concerned, this is equivalent to `yield_now`.
    #[doc(alias = "miri_spin_loop")]
    // #[doc(alias = "yield_now")]
    #[link_name = "miri_spin_loop"]
    pub safe fn miri_yield();
}
/// A safe wrapper over miri's raw extern, which is still available just under
/// [`spawn_thread_raw`] if this function cannot satisfy you
///
/// This returns the tread ID, which can be passed into []
#[doc(alias = "miri_thread_spawn")]
#[rustfmt::skip]
pub fn spawn_thread<D: Sized>(func:fn(Option<&'static mut D>), d: Option<&'static mut D>) -> usize 
where
// F:Send + 'static,
// F:FnOnce(Option<&mut D>) + core::marker::FnPtr,
D: 'static + Send
{   
    // Safety: Due to null pointer optimization this is valid.
    let f = unsafe {core::mem::transmute(func)};
    // Consume life reference, since it is still assumed
    let d = d.map_or(core::ptr::null_mut(), |f| &raw mut *f);
    
    // Safety: Type safe (atleast as much as we can)
    unsafe { spawn_thread_raw(f, d.cast()) }
}
#[inline(always)]
pub fn get_backtrace_size() -> usize {
    // SAFETY: flags is zero
    unsafe { bt_size(0) }
}

// const fn get_fn<R: Sized + Send>(func: &dyn core::any::Any) -> fn(&'static mut R) {
//     func.typ
// }
///
/// Traverses the current call stack, writing slice pointers into `buffer`.
/// The pointers are opaque
///
/// # Panics
/// if `buffer` isn't the size of the result of [`get_backtrace_size()`].
///
/// We attempt to have this function be inlined, so this wouldn't be a cause of a
/// [TOCTOU]<https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use>.
///
/// **note:** That's a implementation detail
#[inline(always)]
#[optimize(size)] // Just because we have to inline the 
pub fn get_backtrace(buffer: &mut [*mut ()]) {
    let size = get_backtrace_size();
    assert!(size == buffer.len(), "get_backtrace needed buffer of size {size}.");
    // SAFETY: flags is 1, and checked if buffer is the correct length
    unsafe { get_backtrace_raw(1, buffer.as_mut_ptr()) }
}

#[cfg(all(test, miri))]
mod test {
    use super::*;
    #[test]
    fn thread_test() {
        let tid = spawn_thread(|_| {}, Option::<&mut u8>::None);
        // SAFETY: we join the right thread
        assert!(unsafe { join_thread(tid) })
    }

    #[test]
    fn thread() {
        // thread_id = miri_thread_spawn(|_| {}, None);
    }
}
