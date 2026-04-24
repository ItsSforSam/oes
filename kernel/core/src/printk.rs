//! Provides the [`print!`] and [`println!`] macros
use core::{ffi::c_char, fmt, usize};

use oes_drivers_core::io::{self, Write};
use spin::mutex::{TicketMutex,TicketMutexGuard};

static WRITER: TicketMutex<&mut (dyn Write + Sync)> = TicketMutex::new(&mut io::no_op());

/// internal
#[doc(hidden)]
pub fn _print(msg: fmt::Arguments<'_>) {}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {$crate::printk::_print(::core::format_args!($($arg)*))};
}
#[macro_export]
macro_rules! println {
    () => {$crate::print!("\n")};
    ($($arg:tt)*) => {$crate::print!("{}\n", format_args!($($arg)*))};
}
/// A more raw printk function
///
/// This is perfect if you don't want any special formatting
/// 
/// Returns
/// How many bytes written
/// 
/// It returns [`None`] if couldn't get internal lock. Which could mean a deadlock occurred
pub fn printk<T:AsRef<[u8]>>(msg:T) -> Option<usize> {
    for _ in 0..10000 {
        match WRITER.try_lock(){
            None => continue,
            Some(guard) =>{
                guard.write_all(msg.as_ref())?;
            }
        }
    };
    // If we break out of the for loop. Potential deadlock occurred
    // Unsure on how we can report it. Considering our printing is borked
    // Panic?
    None
}

/// This shouldn't be called in rust but this instead is supposed to document
/// safety requirements when outside of Rust (like C or assembly)
/// 
/// *`string` should point to a valid string up to `size`. `size` is bytes
/// 
/// # Returns
/// * How many bytes written.
/// * Unless [`usize::MAX`] (or -1 is signed), An error occurred (Equivalent to [`printk`] return [`None`])
#[deprecated = "Use normal `printk` function when interfacing in Rust"]
#[unsafe(export_name = "printk")]
pub unsafe extern "C" fn _printk(level: u8, string: *const u8, size: usize) -> usize {
    assert!(!string.is_null());
    
    // SAFETY: caller guarantees that the pointer points to a valid string
    // from string..size
    let buf = unsafe {
        str::from_utf8_unchecked(core::slice::from_raw_parts(string, size))
    }
    printk(buf).unwrap_or(usize::MAX) as usize
    
}

/// Change the internal writer of where messages go
/// 
/// This gets the lock switches the writers around and returns the old writer
/// 
/// If possible you should [`forget`] the passed value, as it can be mutated
/// 
/// # mut-ness and ownership
/// 
/// 
/// [`forget`]: core::mem::forget
pub fn change_writer(new:&'static mut (dyn Write + Sync))->&'static mut (dyn Write + Sync){
    // This just holds the lock till the end of the scope
    let _lock = WRITER.lock();
    
    // SAFETY: we hold the lock so no race conditions
    // WRITER.as_mut_ptr  should return a valid, properly aligned pointer
    // We cannot use mem::replace due to us not having 
    unsafe {core::ptr::replace(
        WRITER.as_mut_ptr(),
        new
    )}
}
/// Same as [`change_writer`] but takes in a [`Box`]
/// 
/// The return is not Boxed, sense we cannot know which allocator was used, if any
/// 
/// [`Box`]:alloc::boxed::Box
#[cfg(feature = "alloc")]
pub fn change_writer_boxed(new:alloc::boxed::Box<dyn Write + Sync>)->&'static mut (dyn Write + Sync){
    todo!()
}
