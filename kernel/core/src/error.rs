//! Defines the [`Errno`] enum.
//!
//! This is a more generic error type where having complex error types is
//! simply not needed.
//!
//! These are aligned with our `errno.h` file.

// MAKE SURE THEY ARE SYNCED UP WITH errno.h
macro_rules! define_err {
    (
        // () => {
        //     ::core::compile_error!("Errno cannot be empty");
        // };
        $(
            $(#[$attr:meta])*
            $name:ident => $posix_name:expr,  $No:literal $gen_msg:literal;
        )*
    ) => {
        #[derive(Debug,Clone,Copy)]
        #[repr(u32)] // c_int
    pub enum Errno {
        $(
            $(#[$attr])*
            // #[doc(alias = stringify!($posix_name))]
            $name = $No,
        )*
    }
    // Errno can never be zero
    const _:() = {
        $(::core::assert!($No != 0, );)*
    };
    impl ::core::fmt::Display for Errno{
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            use Errno as e;

            match self{
                $(
                    e::$name => core::write!(f,::core::concat!(stringify!($name),":",$gen_msg)),
                )*
            }
        }
    }
    impl ::core::error::Error for Errno{}

}}

define_err! {
    NoSys => ENOSYS,   1          "Function/syscall not implemented";

Invalid => EINVAL, 2          "Invalid Argument";
#[doc(alias="ENOTSUP")]
NotSupported => EOPNOTSUPP,  3          "Operation not supported";

// => ENOTSUP    EOPNOTSUPP

NoMem => ENOMEM,     4          "Cannot Allocate Memory";
#[doc(alias = "ENOBUFS")]
TooSmall => EOVERFLOW,  5          "Buffer is too small for requested data";

Blocked => EAGAIN,     6          "Resource temporarily unavailable, try again";

// => EINTR      7          "Syscall was interupted, try again";

// => ERESTART   EINTR

BadExchange => EBADE,      8          "Invalid exchange";

TooBig => E2BIG,      9          "Argument list is too big";

// FD Errors

InvalidFD => EBADF,      10         "Bad File Descriptor";

BadFD => EBADFD,     11         "File Descriptor in bad state";

SaleFD => ESTALE,     12         "Stale file handle";

// IO/File System errors

IO => EIO,        13         "Input/Output error";
PermissionDenied => EPERM,      14         "Permission denied";
FileExists=> EEXIST,     15         "File Exists";
MaxFD=> EMFILE,     16         "Reached max open files";
MaxFDSystems=> ENFILE,     17         "Reached max open files for system";
FileBusy=> ETXTBSY,    18         "Text file busy (depends on Filesystem)";
ReadOnly => EROFS  ,    19         "Read only filesystem";
NotADir => ENOTDIR  ,  20         "Not a directory, expected directory";
IsDir => EISDIR     ,21         "A directory, didn't expect a directory ";
DirNotEmpty => ENOTEMPTY,  22         "Directory not empty";
QuotaReached=> EDQUOT    , 23         "Quota Reached";
NoSpace => ENOSPC    , 24         "Out of space";
SymLoop => ELOOP   ,   25         "Too many levels of symbolic links";
TooMuchLinks => EMLINK ,    26         "Too many (hard) links";
CouldNotExec => ENOEXEC ,   27         "File Found, but could not be executed";
// => ELIBEXEC   28         " Cannot exec a shared library directly";
// => ELIBACC    29         "Can not access a needed shared library";
#[doc(alias="ENXIO")]
NonExistentIo => ENXIO   ,   30         "No such device or address";
NoTTY=> ENOTTY     ,31         "Inappropriate ioctl for device";
NameTooLong => ENAMETOOLONG,    141 "File name is too long";

// => ENOANO          142 "No Anode";

NotRecoverable => ENOTRECOVERABLE, 143 "State not recoverable";

}

pub trait ToErrno {
    /// Converts [`Self`] into [`Errno`].
    ///
    /// This is useful if you want self to be consumed for whatever
    fn into_errno(self) -> Errno
    where
        Self: Sized;
    /// Converts [`Self`] into [`Errno`]
    fn as_errno(&self) -> Errno;
}
impl ToErrno for Errno {
    /// Simple no-op so use of a [`ToErrno`]
    #[inline(always)]
    fn into_errno(self) -> Errno {
        self
    }
    #[inline(always)]
    fn as_errno(&self) -> Errno {
        *self
    }
}
