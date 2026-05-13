//! Handles the scheduler logic

/// A Task can be anything
///
/// A user process, a kernel thread
pub struct Task(());

/// Yields the current thread
///
/// This is great if you are waiting on data from a another
/// thread, although take a look at the [`sync`] module
/// for synchronization primitives, which may better suit your
/// needs
///
/// [`sync`]: crate::sync
pub fn yield_now() {
    cfg_if::cfg_if! {
        if #[cfg(miri)]{
        oes_arch::common::miri::miri_yield();

        }else {
    // @TODO: make scheduler
    // Not using todo!(), as to avoid panicking and due to
    // us not having a scheduler, and this functioning as a no-op
    // Something that may be helpful that if no other tasks are needed,
    // either use core::hint::spin_loop() or use hlt (or architecture
    // equivalent)
    //
    // Side note, which should have some discussion, is that if
    // there should be some sort of fast pass to this function
    //
    // Thought should be if there should be of sort a custom
    // interrupt, similar to unix-like's int 0x80 for syscalls.
    // This is due to the nature of yielding
    // BUT this would have to be a commitment and considered if it's even worth
    // the cost, at-least on x86_64, the sysenter/sysexit is designed
    // for transering to and from userspace.
    core::hint::spin_loop();
        }
    }
}
