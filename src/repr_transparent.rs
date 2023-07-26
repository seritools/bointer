/// Helper functions for `repr(transparent)` types.
///
/// ## Safety
///
/// This is only safely implemented by `#[repr(transparent)]` wrapper types, and only if
/// [`ReprTransparent::Inner`] actually matches the inner type. It is recommended to use the
/// derive macro to implement this trait, as it prevents any accidental misuse.
///
/// ## Derive macro
///
/// If the `derive` feature is enabled, use the derive macro `derive(ReprTransparent)` to implement
/// this trait. The macro checks for the existence of the `repr(transparent)` attribute, sets
/// [`ReprTransparent::Inner`] to the inner type and implements [`ReprTransparent::into_inner`]
/// automatically, making the implementation refactoring-proof. Note that the derive macro expects
/// the singular non-zero-sized field to be the first one in the implementing type.
///
/// ## Remarks
///
/// An `as_transparent_mut_ref` method is not provided because the wrapper type might enforce
/// invariants that could be invalidated via a mutable reference to the inner type.
///
/// ## Example
///
/// ```rust
/// use bointer::ReprTransparent;
///
/// #[derive(ReprTransparent)]
/// #[repr(transparent)]
/// struct UsizeWrapper(usize);
///
/// let value = UsizeWrapper(42);
/// assert_eq!(value.into_inner(), 42usize);
///
/// let mut value = UsizeWrapper(42);
/// let reference: &usize = value.as_inner_ref();
/// assert_eq!(*reference, 42usize);
///
/// let mut_ptr: *mut usize = value.as_inner_mut_ptr();
/// unsafe { *mut_ptr = 43; }
/// assert_eq!(value.into_inner(), 43usize);
/// ```
pub unsafe trait ReprTransparent: Sized {
    type Inner;

    /// Converts the `Self` into the type it is wrapping.
    fn into_inner(self) -> Self::Inner;

    /// Returns a reference to the type it is wrapping.
    #[inline(always)]
    fn as_inner_ref(&self) -> &Self::Inner {
        // SAFETY: Safe for the conditions described in the trait documentation.
        unsafe { core::mem::transmute(self) }
    }

    /// Returns a raw pointer to the type it is wrapping.
    #[inline(always)]
    fn as_inner_ptr(&self) -> *const Self::Inner {
        <*const Self>::cast::<Self::Inner>(self)
    }

    /// Returns a raw mutable pointer to the type it is wrapping.
    #[inline(always)]
    fn as_inner_mut_ptr(&mut self) -> *mut Self::Inner {
        <*mut Self>::cast::<Self::Inner>(self)
    }
}
