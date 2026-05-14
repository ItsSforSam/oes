/// A type which keeps itself reference counted
///
/// # Safety
///
/// The implementers should keep itself alive until there
/// increment reaches zero
///
/// If the implementer is [`Send`] and [`Sync`], then
/// the reference count should be atomically counted, similar
/// to [`Arc<T>`]
///
/// [`Arc<T>`]: liballoc::sync::Arc
pub unsafe trait ReferenceCounted {
    /// Increments the reference count on the object.
    fn inc_ref(&self);
    /// Decrements the reference count on the object.
    ///
    /// Frees the object when the count reaches zero.
    ///
    /// # Safety
    /// Callers should not reference there object after
    unsafe fn dec_ref(this: core::ptr::NonNull<Self>);
}
