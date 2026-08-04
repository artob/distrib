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

#[cfg(feature = "magnus")]
#[doc(hidden)]
pub use magnus;

mod error;
pub use error::*;

pub mod gemspec;
pub use gemspec::*;

#[cfg(all(feature = "std", feature = "serde"))]
mod load;
#[cfg(all(feature = "std", feature = "serde"))]
pub use load::*;

#[cfg(feature = "std")]
mod program;
#[cfg(feature = "std")]
pub use program::*;
