#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use option::{AsRawMutPtr, AsRawPtr};
pub use repr_transparent::ReprTransparent;

#[cfg(feature = "derive")]
pub use bointer_derive::ReprTransparent;

mod option;
mod repr_transparent;
