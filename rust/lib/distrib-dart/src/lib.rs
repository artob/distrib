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

mod error;
pub use error::*;

pub mod pubspec;
pub use pubspec::Pubspec;

#[cfg(feature = "std")]
mod load;
#[cfg(feature = "std")]
pub use load::*;
