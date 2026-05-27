//! Ascii strings
use core::ascii::Char as AsciiChar;
use core::intrinsics::unreachable;
use core::{fmt::write, ops::Deref};

pub struct NonAsciiPattern(());
impl core::fmt::Debug for NonAsciiPattern {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NonAsciiPattern").finish_non_exhaustive()
    }
}
impl core::fmt::Display for NonAsciiPattern {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Non ascii data")
    }
}
impl core::error::Error for NonAsciiPattern {}
/// An ascii encoded string
///
/// This has stricter encoding then the normal [`str`]
#[repr(transparent)]
#[derive(PartialEq, Eq)]
// INVARIANT: NON VALID ASCII
pub struct AStr([u8]);

impl AStr {
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    pub fn as_astr(&self) -> &AStr {
        self
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<&AStr, NonAsciiPattern> {
        if !bytes.is_ascii() {
            return Err(NonAsciiPattern(()));
        }
        // SAFETY: Checked if
        Ok(unsafe { core::mem::transmute(bytes) })
    }
    /// Unsafe version of [`from_bytes`]
    ///
    /// # SAFETY
    ///
    /// `value` is valid ascii
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &AStr {
        debug_assert!(bytes.is_ascii());
        // SAFETY: Caller guarantees we
        unsafe { core::mem::transmute(bytes) }
    }
    pub fn as_str(&self) -> &str {
        // SAFETY: Ascii is valid UTF-8
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }
}
impl Default for &AStr {
    fn default() -> Self {
        // Valid due to #[repr(transparent)]
        unsafe { core::mem::transmute::<&[u8], &AStr>(&[]) }
    }
}
impl core::fmt::Debug for AStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "a\"")?;

        for chunck in self.0.iter() {
            match *chunck {
                b'\0' => write!(f, "\\0")?,
                b'\t' => write!(f, "\\t")?,
                b'\r' => write!(f, "\\r")?,
                b'\n' => write!(f, "\\n")?,
                b'\"' => write!(f, "\\\"")?,
                b'\'' => write!(f, "\\'")?,
                0x20..=0x7e => write!(f, "{}", chunck.escape_ascii())?,
                0x7F => write!(f, "")?,
                v => {
                    core::hint::cold_path();
                    if v.is_ascii() {
                        // We panic here as we have a logic bug here and not a invariant
                        // on the AStr not being proper ascii
                        panic!("Unhandled ascii byte display {v:X} ({v})");
                    }
                    // Once we are 1,000% sure, we can use the unchecked variant

                    unreachable!("AStr invariant! Found non-ascii character: {v:X} ({v})")
                }
            }
        }
        write!(f, "\"")
    }
}
impl core::fmt::Display for AStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let fill = self.as_str();
        write!(f, "{fill}")
    }
    //
}
impl From<&AStr> for &[AsciiChar] {
    fn from(value: &AStr) -> Self {
        // SAFETY: AsciiChar has the representation os u8
        // so &[core::ascii::Char] is equal to &[u8]
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&[core::ascii::Char]> for &AStr {
    fn from(value: &[core::ascii::Char]) -> Self {
        // SAFETY: AsciiChar has the representation os u8
        // so &[core::ascii::Char] is equal to &[u8]
        unsafe { core::mem::transmute(value) }
    }
}
impl<'a> core::convert::TryFrom<&'a [u8]> for &'a AStr {
    type Error = NonAsciiPattern;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        AStr::from_bytes(value)
    }
}
impl core::convert::TryFrom<&str> for &AStr {
    type Error = NonAsciiPattern;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.as_ascii().ok_or(NonAsciiPattern(())).map(|v| v.into())
    }
}
#[cfg(feature = "alloc")]
mod ascii_string {
    use super::AStr;

    pub struct AsciiString<A: core::alloc::Allocator = alloc::alloc::Global>(
        alloc::vec::Vec<u8, A>,
    );
    impl AsciiString {
        pub const fn as_astr(&self) -> &AStr {
            unsafe { core::mem::transmute(self.0.as_slice()) }
        }
        pub const fn as_mut_astr(&mut self) -> &mut AStr {
            unsafe { core::mem::transmute(self.0.as_mut_slice()) }
        }
    }
    impl core::ops::Deref for AsciiString {
        type Target = AStr;

        fn deref(&self) -> &Self::Target {
            self.as_astr()
        }
    }
    impl core::ops::DerefMut for AsciiString {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.as_mut_astr()
        }
    }
    impl core::fmt::Display for AsciiString {
        #[inline]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Display::fmt(&**self, f)
        }
    }
    impl core::fmt::Debug for AsciiString {
        #[inline]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(&**self, f)
        }
    }
}
#[cfg(feature = "alloc")]
pub use ascii_string::*;
