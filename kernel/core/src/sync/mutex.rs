//! Locks that give the same behaver as a mutex with proper locking behaver

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU16, Ordering};
/// The classic mutex we know and love
///
/// # Deadlocks
/// If you attempt to acquire a lock if
///
///
/// [Ticket Lock]: <https://en.wikipedia.org/wiki/Ticket_lock>
#[derive(Debug)]
pub struct Mutex<T: ?Sized> {
    // The tickets can be any size
    // I chose u16 as it can be reasonable
    // that we aren't going to have 2^16 tickets active at once
    // without TRYING on a super computer
    // Hell, I'm relucent to even use u16 and not a u8 lol
    // but 255 is way more possible
    /// How many tickets there are
    next_ticket: AtomicU16,
    /// Who's next on this list
    next_serving: AtomicU16,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Mutex<T> {
        Mutex {
            next_ticket: AtomicU16::new(0),
            next_serving: AtomicU16::new(0),
            data: UnsafeCell::new(value),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        let ticket = self.next_ticket.fetch_add(1, Ordering::AcqRel);
        while self.next_serving.load(Ordering::Acquire) != ticket {
            crate::sched::yield_now();
        }
        todo!()
    }
    pub fn try_lock(&self) -> Option<Mutex<T>> {
        let mut prev = self.next_ticket.load(Ordering::SeqCst);
        todo!()
    }
    /// Consumes the [`Mutex`] returning the underlying value
    #[inline]
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
    /// Mutably borrows the underlying data
    ///
    /// Since you are borrowing the [`Mutex`] mutably, there is
    /// a guarantee no locks exist
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        // SAFETY: self is borrowed mutably, and due
        // how Rust's borrowing works (and due to us using &self for .lock and
        // not &mut or whatever). We can guarantee that we have exclusive refences
        // to the underlying data
        unsafe { &mut *self.data.get() }
    }
    /// Returns a pointer to the underlying data stored
    ///
    ///
    /// This is simply here for completeness, you may
    /// want [`.as_mut_ptr()`] instead. Look at [`.as_mut_ptr()`] for
    /// more information
    ///
    /// # Safety
    /// As with all raw pointers, avoid race conditions, and avoid use
    /// after frees. This function doesn't cause undefined behaver,
    /// and doesn't make anything unsound (hense not marked unsafe).
    ///
    /// Only way to cause undefined behaver is using the pointer improperly
    ///
    /// [`.as_mut_ptr()`]:Mutex::as_mut_ptr()
    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.as_mut_ptr()
    }
    /// Returns a mutable pointer to the underlying data stored
    ///
    /// This can allow you to use with FFI, certain [`ptr`] functions,
    /// like [`ptr::swap`].
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// # use oes_kernel_core::sync::mutex::{Mutex,MutexGuard};
    /// # #[derive(PartialEq)]
    /// # struct SomeStruct(u8);
    /// # impl SomeStruct{
    /// #  fn new(value:u8)->Self {SomeStruct(value)}
    /// # fn default()->Self {SomeStruct(0)}
    /// # fn into_inner(self)->u8 {self.0}
    /// # }
    /// # fn main(){
    /// // Some dummy value, like a sentinel
    /// // a NoOp struct, or switching out
    /// // a value with a another, but still
    /// // dropping (like how printk's
    /// // overwrite_writer)
    /// let foo = Mutex::new(SomeStruct::default());
    /// {   
    ///     // Acquire the lock to the data
    ///     let _lock = foo.lock();
    ///     // This can allow ffi, but here we are switching
    ///     // the value and dropping the old
    ///     let new_value = SomeStruct::new(67);
    ///     // SAFETY: we hold the lock
    ///     unsafe {
    ///         // We get the old value, which is dropped
    ///         _ = core::ptr::replace(
    ///             foo.as_mut_ptr(),
    ///             new_value
    ///         
    ///     );
    ///     }
    ///     
    /// } // Lock is dropped here
    /// // We unwrap the mutex
    /// assert!(foo.into_inner() == SomeStruct(67));
    /// # }
    /// ```
    /// # Safety
    /// As with all raw pointers, avoid race conditions, and avoid use
    /// after frees. This function doesn't cause undefined behaver,
    /// and doesn't make anything unsound (hense not marked unsafe).
    ///
    /// Only way to cause undefined behaver is using the pointer improperly
    ///
    /// [`.as_mut_ptr()`]:Mutex::as_mut_ptr()
    /// [`ptr`]:core::ptr
    /// [`ptr::swap`]: core::ptr::swap
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.data.get()
    }
    /// Forces an unlock so the next  
    pub unsafe fn force_unlock(&self) {}
}

pub struct MutexGuard<'a, T: ?Sized> {
    ticket: u16,
    next_serving: &'a AtomicU16,
    data: &'a mut T,
}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Mutex::new(Default::default())
    }
}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
