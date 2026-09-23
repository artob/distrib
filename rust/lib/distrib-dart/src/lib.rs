// This is free and unencumbered software released into the public domain.

//! Distrib helps you distribute your software.

#![no_std]
#![allow(unused)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "dart-sys")]
#[doc(hidden)]
pub use dart_sys;

#[cfg(feature = "alloc")]
mod error;
#[cfg(feature = "alloc")]
pub use error::*;

#[cfg(all(feature = "std", feature = "serde"))]
mod load;
#[cfg(all(feature = "std", feature = "serde"))]
pub use load::*;

#[cfg(feature = "std")]
mod program;
#[cfg(feature = "std")]
pub use program::*;

#[cfg(feature = "alloc")]
pub mod pubspec;
#[cfg(feature = "alloc")]
pub use pubspec::*;
