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

impl !UAbiBoundary for f32 {}
impl !UAbiBoundary for f64 {}
