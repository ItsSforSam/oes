//! Provides traits for dealing with IO
//!
//! This allows an easy Hardware abstraction layer
use core::fmt;
pub trait Write {
    /// Write a buffer into the writer
    ///
    /// # Returns
    /// [`Some(n)`] is how many bytes where written
    ///
    /// [`None`] is an error occured
    ///
    /// **Note, this is pained to return a [`Result<T,E>`] in the future**
    ///
    /// [`Some(n)`]: Some
    // TODO: make a generic error type
    fn write(&mut self, buf: &[u8]) -> Option<usize>;
    /// Flush from internal buffer into IO object
    ///
    /// # Returns
    /// [`Some(())`] on success
    ///
    /// [`None`] when a error occurred.
    ///
    /// **Note, this is pained to return a [`Result<T,E>`] in the future**
    ///
    /// [`Some(())`]: Some
    fn flush(&mut self) -> Option<()>;
    /// Attempts to write the whole buffer into the writer
    ///
    /// # Returns
    /// [`Some(())`] on success
    ///
    /// [`None`] when a error occurred.
    ///
    /// **Note, this is pained to return a [`Result<T,E>`] in the future**
    /// [`Some(())`]: Some
    fn write_all(&mut self, mut buf: &[u8]) -> Option<()> {
        // TODO: once proper error type is made, check if it's
        // because it was interrupted
        while !buf.is_empty() {
            match self.write(buf) {
                Some(0) => return None,
                Some(n) => buf = &buf[n..],
                None => return None,
            }
        }
        Some(())
    }
    /// # Returns
    /// [`Some(())`] on success
    ///
    /// [`None`] when a error occurred.
    ///
    /// **Note, this is pained to return a [`Result<T,E>`] in the future**
    /// [`Some(())`]: Some
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Option<()> {
        if let Some(s) = args.as_str() {
            self.write_all(s.as_bytes())
        } else {
            todo!();
        }
    }
}

// fn default_write_fmt<W:Write+?Sized+fmt::Write>(this:&mut W,args: fmt::Arguments)->Option<()>{

//     match fmt::write(&*this,args){
//         Ok(()) => Some(()),
//         Err(_) => None
//     }
// }

impl fmt::Write for dyn Write + '_ {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match Write::write_all(self, s.as_bytes()) {
            None => Err(fmt::Error),
            Some(_) => Ok(()),
        }
    }
}
/// Creates a value that
#[doc(alias = "empty")]
pub const fn no_op() -> NoOp {
    NoOp
}

/// `NoOp` ignores any data written to via [`Write`] and data will always
/// be empty if read.
/// This struct is constructed by [`no_op()`] function.
/// Read the documentation of [`no_op()`] for more details
#[doc(alias = "Empty")]
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct NoOp;

impl Write for NoOp {
    fn write(&mut self, buf: &[u8]) -> Option<usize> {
        Some(buf.len())
    }

    fn flush(&mut self) -> Option<()> {
        Some(())
    }
}
