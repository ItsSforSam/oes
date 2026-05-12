/// A marker trait that says that a given trait can be passed to and from
///
/// user land and Kernel land and vice versa
///
///
/// # Safety
/// When this trait is implemented, you are saying a value can be passed by value
/// to functions.
///
///
pub unsafe trait UAbiBoundary: Sized {}
/// A trait which simply marks itself as a number
pub trait Number:
    Sized
    + Copy
    + PartialOrd
    + Ord
    + Eq
    + core::ops::Not<Output = Self>
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOr<Output = Self>
    + core::ops::BitXor<Output = Self>
    + core::ops::Shl<usize, Output = Self>
    + core::ops::Shr<usize, Output = Self>
    + crate::private::Sealed
{
}
/// Use of floating point numbers should be limited,
/// and should never be passed to User Space on the times it is
impl !UAbiBoundary for f32 {}
/// Use of floating point numbers should be limited,
/// and should never be passed to User Space on the times it is
impl !UAbiBoundary for f64 {}
// We would negative impl f16 and f128 in general, but they are nightly, and we
// shouldn't enable the nightly feature with the sole-purpose of doing a negative
// impl.

/// You shouldn't send a Rust's VTable to Userspace (or any of our vtables), but y-know
impl<T: core::ptr::Pointee<Metadata = Self>> !UAbiBoundary for &T {}
/// We do this due to it having gaps and being a [Unicode scalar value]
/// which we cannot just pass to user space willy nilly.
///
/// You may want [`char`][core::ffi::c_char], which has the same name, or
/// use of [`u32`]/[`i32`] which is the same size as [`char`] without the gaps
/// in valid values
///
/// [Unicode scalar value]: <https://www.unicode.org/glossary/#unicode_scalar_value>
impl !UAbiBoundary for char {}
/// This expects UTF-8 encoding and is [unsized].
///
/// And is the equivalent to [`&[char]`]
///
/// [unsized]: core::marker::Sized
/// [`[`&[char]`]`]: primitive@slice
impl !UAbiBoundary for str {}
/// Shouldn't be sending a slice to user space
///
/// This is primarily, due to it a unsized type, with ffi
///
/// If you need to send or retrieve a array of items, use slice's [`.as_array()`]
///
/// [`.as_array()`]: slice::as_array()
impl<T: ?Sized> !UAbiBoundary for [T] {}
#[doc(hidden)]
// We hide this due to the normal one
impl<T: ?Sized> !UAbiBoundary for &[T] {}
unsafe impl UAbiBoundary for u8 {}
unsafe impl UAbiBoundary for i8 {}
unsafe impl UAbiBoundary for u16 {}
unsafe impl UAbiBoundary for i16 {}
unsafe impl UAbiBoundary for u32 {}
unsafe impl UAbiBoundary for i32 {}
unsafe impl UAbiBoundary for u64 {}
unsafe impl UAbiBoundary for i64 {}
unsafe impl UAbiBoundary for usize {}
unsafe impl UAbiBoundary for isize {}

// unsafe impl UAbiBoundary for *const core::ffi::c_void {}
// unsafe impl UAbiBoundary for *mut core::ffi::c_void {}
unsafe impl<T: Sized> UAbiBoundary for *const T {}
unsafe impl<T: Sized> UAbiBoundary for *mut T {}
/// Due to Rust's type system, this isn't really a value, and isn't truly
/// sent to user space, as there isn't truly a value contained here
unsafe impl UAbiBoundary for ! {}

unsafe impl<T: UAbiBoundary, const N: usize> UAbiBoundary for [T; N] {}

// unsafe impl<T: UAbiBoundary> UAbiBoundary for Option<NonZero<T>> {}

impl crate::private::Sealed for u8 {}
impl crate::private::Sealed for i8 {}
impl crate::private::Sealed for u16 {}
impl crate::private::Sealed for i16 {}
impl crate::private::Sealed for u32 {}
impl crate::private::Sealed for i32 {}
impl crate::private::Sealed for u64 {}
impl crate::private::Sealed for i64 {}
impl crate::private::Sealed for usize {}
impl crate::private::Sealed for isize {}

impl Number for u8 {}
impl Number for i8 {}
impl Number for u16 {}
impl Number for i16 {}
impl Number for u32 {}
impl Number for i32 {}
impl Number for u64 {}
impl Number for i64 {}
impl Number for usize {}
impl Number for isize {}
