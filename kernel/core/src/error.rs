//! Defines the [`Errno`] enum.
//!
//! This is a more generic error type where having complex error types is
//! simply not needed.
//!
//! These are aligned with our `errno.h` file.

// MAKE SURE THEY ARE SYNCED UP WITH errno.h
//@TODO: make build script which simply pulls the values from errno.h
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
    // Errno, if interpreted as signed, cannot be able to be interpreted as a
    // negative
    // As well as any user-api non-error cannot be withen this range as well
    // this is just due that we pass -Errno into the result register
    // also to note, the guaranteed minimum size for Errno is 16 bits due
    // to there being 143 variants of Errno defined by POSIX currently
    // and there can be more added (as we don't define all of them, as kernel space
    // doesn't need all of them, currently)
    //
    // We do this compile time check, even tho it's most likely not ever going to be reached
    // but doing this chec
    const _:() = {
        $(
            ::core::assert!(($No as i16) > 0, ::core::stringify!(Errno::$name value breaks userland guarantees ));
        )*
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


    def_from_into!(
        $(
            $name => $No;
        )*
        @end_of_errno
        u8, u16, i16, u32, i32, u64, i64, usize, isize


    );
}}
macro_rules! def_from_into {

    (
        $(
            $name:ident => $No:literal;
        )*
        @end_of_errno

    ) => {
        /* */
    };
    (
        $($name:ident => $No:literal;)*
        @end_of_errno
        $num:ty
    ) => {

        impl ::core::convert::TryFrom<$num> for Errno {
                type Error = FromIntError;
                fn try_from(__value: $num) -> Result<Self,Self::Error>{
                    match __value{
                    $(
                        $No => ::core::result::Result::Ok(Errno::$name),
                    )*
                    _ => ::core::result::Result::Err(FromIntError(()))

            }
        }
    }
            impl ::core::convert::TryFrom<::core::num::NonZero<$num>> for Errno {
                type Error = FromIntError;
                fn try_from(__value: ::core::num::NonZero<$num>) -> Result<Self,Self::Error>{
                    match __value.get(){
                    $(
                        $No => Ok(Errno::$name),
                    )*
                    _ => Err(FromIntError(()))

            }
        }
        }
        // No need to

    };
    (
        $($name:ident => $No:literal;)*
        @end_of_errno
        $num:ty, $($other:tt)*


    ) => {


        impl ::core::convert::TryFrom<$num> for Errno {
                type Error = FromIntError;
                fn try_from(__value: $num) -> Result<Self,Self::Error>{
                    match __value{
                    $(
                        $No => ::core::result::Result::Ok(Errno::$name),
                    )*
                    _ => ::core::result::Result::Err(FromIntError(()))

                }
            }
        }
            impl ::core::convert::TryFrom<::core::num::NonZero<$num>> for Errno {
                type Error = FromIntError;
                fn try_from(__value: ::core::num::NonZero<$num>) -> Result<Self,Self::Error>{
                    match __value.get(){
                    $(
                        $No => ::core::result::Result::Ok(Errno::$name),
                    )*
                    _ => ::core::result::Result::Err(FromIntError(()))

            }
        }
        }
        // impl ::core::convert::From<Errno> for $num {

        //         fn from(__value: Errno) -> $num {
        //             __value as $num
        //         }

        //     }

        // };
        //     impl ::core::convert::From<Errno> for ::core::num::NonZero<$num> {

        //         fn from(__value: Errno) -> Self {
        //           core::num::NonZero::new_unchecked(__value as _ )
        //         }
        //     }

        def_from_into!{
            $(
                $name => $No;
            )*
            @end_of_errno
            $(
                $other
            )*

        }
    }
    } /* End of scope */

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
impl core::ops::Neg for Errno {
    type Output = i16;
    /// This will coercions [`Errno`] into the smallest
    fn neg(self) -> Self::Output {
        let r: i16 = self as i16;
        // Compile time check says Errno cannot be reach the bound
        unsafe { r.unchecked_neg() }
    }
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

unsafe impl crate::marker::UAbiBoundary for Errno {}

#[derive(Debug)]
pub struct FromIntError(());
impl ::core::error::Error for FromIntError {}
impl core::fmt::Display for FromIntError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Invalid Value passed into TryFrom for Errno")
    }
}

unsafe impl crate::traits::IntoUserSpace for () {
    type Output = usize;

    fn into_userspace(self) -> Self::Output {
        0
    }
}
unsafe impl crate::traits::IntoUserSpace for Result<(), Errno> {
    type Output = isize;
    #[expect(clippy::arithmetic_side_effects, reason = "-Errno won't panic")]
    fn into_userspace(self) -> Self::Output {
        match self {
            Ok(_) => 0,
            Err(e) => (-e) as isize,
        }
    }
}
