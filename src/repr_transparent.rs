/// Helper functions for `repr(transparent)` types.
///
/// ## Safety
///
/// This is only safely implemented by `#[repr(transparent)]` wrapper types, and only if
/// [`ReprTransparent::Wrapped`] actually matches the wrapped type. It is recommended to use the
/// derive macro to implement this trait, as it prevents any accidental misuse.
///
/// ## Derive macro
///
/// If the `derive` feature is enabled, use the derive macro `derive(ReprTransparent)` to implement
/// this trait. The macro checks for the existence of the `repr(transparent)` attribute, sets
/// [`ReprTransparent::Wrapped`] to the inner type and implements [`ReprTransparent::into_wrapped`]
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
/// let mut value = UsizeWrapper(42);
///
/// let reference: &usize = value.as_wrapped_ref();
/// let const_ptr: *const usize = value.as_wrapped_ptr();
/// let mut_ptr: *mut usize = value.as_wrapped_mut_ptr();
/// assert_eq!(value.into_wrapped(), 42usize);
/// ```
pub unsafe trait ReprTransparent: Sized {
    type Wrapped;

    /// Converts the `Self` into the type it is wrapping.
    fn into_wrapped(self) -> Self::Wrapped;

    /// Returns a reference to the type it is wrapping.
    fn as_wrapped_ref(&self) -> &Self::Wrapped {
        // SAFETY: Safe for the conditions described in the trait documentation.
        unsafe { core::mem::transmute(self) }
    }

    /// Returns a raw pointer to the type it is wrapping.
    fn as_wrapped_ptr(&self) -> *const Self::Wrapped {
        self as *const Self as *const _
    }

    /// Returns a raw mutable pointer to the type it is wrapping.
    fn as_wrapped_mut_ptr(&mut self) -> *mut Self::Wrapped {
        self as *mut Self as *mut _
    }
}
