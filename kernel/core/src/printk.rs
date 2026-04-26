//! Provides the [`print!`] and [`println!`] macros
pub use crate::prelude::*;
use core::fmt;
use liballoc::boxed::Box;
use oes_drivers_core::io::{self, Write, no_op};
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
pub fn printk<T: AsRef<[u8]>>(msg: T) -> Option<usize> {
    for _ in 0..10000 {
        //     match WRITER.try_lock() {
        //         None => continue,
        //         Some(mut guard) => {
        //             guard.write_all(msg.as_ref())?;
        //         }
        //     }
    }
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

    // SAFETY: caller guarantees that the pointer points to a valid UTF-8 String
    // from string..size
    let buf = unsafe { str::from_utf8_unchecked(core::slice::from_raw_parts(string, size)) };
    printk(buf).unwrap_or(usize::MAX) as usize
}

// We have to do this cause life time rules...for some reason
// NoOp for some reason keeps is seen that it cannot
// be `'static`, despite it being a ZST
// THIS right here, fails for some reason
// pub const fn empty() -> PrintkWriter {
//        let n: &'static _ = &no_op();
//        PrintkWriter {
//           ptr: UnsafeCell::new(n),
//            lock: TicketMutex::new(()),
//        }
// }

mod writer {
    use super::{Box, Write, io};
    use crate::prelude::*;
    use core::{
        fmt::Debug,
        ops::{Deref, DerefMut},
        ptr::{NonNull, null_mut},
    };

    use oes_drivers_core::io::no_op;
    use spin::mutex::{TicketMutex, TicketMutexGuard};

    pub fn get_writer() -> PrintkWriter {
        // PrintkWriter(())
        todo!()
    }
    /// The internal printk writer interface
    ///
    ///
    /// # Singleton
    ///
    /// This struct manages two static values,
    /// which this provides a interface for that
    pub struct PrintkWriter {
        /// The actual lock
        ///
        /// If this is [`None`], then lazily initialize with
        /// [`NoOp`] and stores into [`Box`].
        ///
        /// ** NOTE: ** This can call the oom handler
        /// this is due the notion
        ///
        /// [`NoOp`]:oes_drivers_core::io::NoOp
        inner: TicketMutex<Option<Box<dyn Write>>>,
    }

    impl PrintkWriter {
        const fn new() -> PrintkWriter {
            PrintkWriter {
                inner: TicketMutex::new(None),
            }
        }

        pub fn lock<'a>(&'a self) -> PrintkLock<'a> {
            let mut lock = self.inner.lock();
            todo!();
            match *lock {
                Some(ref _v) => { /* Nothing */ }
                None => {
                    todo!();
                    let n: &dyn Write = &no_op();
                    let l = &mut *lock;
                    // We hold the lock
                    // Box::try_fr

                    // let a = Box::try_new(n);
                    // match Box::<dyn Write>::try_new_uninit() {
                    //     Ok(writer) => {
                    //         Box::write(&writer, )
                    //     }
                    //     Err(_e) => {
                    //         // Unfortunately, this will just yell into the void
                    //         // as this code branch will occur if we cannot allocate
                    //         // enough for
                    //         panic!()
                    // }
                    // }
                }
            }
            todo!()
            // PrintkLock {
            //     inner: lock,
            // }
        }
        /// This lets you overwrite the printk's writer.
        /// It will still do it's locking. But will retrieve the old value and drop it properly
        pub fn overwrite_writer(&self, writer: Box<dyn Write>) {
            let mut lock = self.inner.lock();
            *lock = Some(writer);
        }
    }
    /// This holds the internal lock of [`PrintkLock`]
    ///
    /// Once this value goes out out scope
    pub struct PrintkLock<'a> {
        inner: TicketMutexGuard<'a, Option<liballoc::boxed::Box<dyn Write>>>,
    }

    impl<'a> Deref for PrintkLock<'a> {
        type Target = dyn Write;

        fn deref(&self) -> &Self::Target {
            todo!()
            // unsafe { &self.inner.unwrap_unchecked() }
        }
    }
    impl<'a> DerefMut for PrintkLock<'a> {
        fn deref_mut(&mut self) -> &'a mut Self::Target {
            // SAFETY: we hold the lock, and this reference
            // is valid as long as the lifetime 'a as the lock
            // is held on our thread
            todo!()
            // unsafe { WRITER }
        }
    }

    impl !Send for PrintkLock<'_> {}
}
pub use writer::{PrintkLock, PrintkWriter, get_writer};
