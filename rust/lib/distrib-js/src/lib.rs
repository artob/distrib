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

#[cfg(feature = "napi")]
#[doc(hidden)]
pub use napi;

mod error;
pub use error::*;

pub mod package;
pub use package::*;

#[cfg(feature = "std")]
mod load;
#[cfg(feature = "std")]
pub use load::*;
