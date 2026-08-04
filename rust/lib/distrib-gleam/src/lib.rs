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

pub use distrib_beam::*;

mod error;
pub use error::*;

#[cfg(all(feature = "std", feature = "serde"))]
mod load;
#[cfg(all(feature = "std", feature = "serde"))]
pub use load::*;

#[cfg(all(feature = "parse"))]
pub mod package;
#[cfg(all(feature = "parse"))]
pub use package::*;

#[cfg(feature = "std")]
mod program;
#[cfg(feature = "std")]
pub use program::*;
