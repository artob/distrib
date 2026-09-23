// This is free and unencumbered software released into the public domain.

//! Distrib helps you distribute your software.
//!
//! Cargo manifest types and loading require `std`, as does the upstream manifest
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
pub mod manifest;
#[cfg(feature = "std")]
pub use manifest::*;

#[cfg(feature = "std")]
mod program;
#[cfg(feature = "std")]
pub use program::*;
