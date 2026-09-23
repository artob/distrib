// This is free and unencumbered software released into the public domain.

//! Distrib helps you distribute your software.
//!
//! Manifest types and loading require `std`, as does the upstream JSON schema
//! library. `alloc` alone exposes the boxed load error without enabling it.

#![no_std]
#![allow(unused)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod error;
#[cfg(feature = "alloc")]
pub use error::*;

#[cfg(feature = "std")]
mod load;
#[cfg(feature = "std")]
pub use load::*;

#[cfg(feature = "std")]
pub mod package;
#[cfg(feature = "std")]
pub use package::*;

#[cfg(feature = "std")]
mod program;
#[cfg(feature = "std")]
pub use program::*;
