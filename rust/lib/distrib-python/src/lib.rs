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

#[cfg(feature = "pyo3")]
#[doc(hidden)]
pub use pyo3;

mod error;
pub use error::*;

pub mod pyproject;
pub use pyproject::*;

#[cfg(feature = "std")]
mod load;
#[cfg(feature = "std")]
pub use load::*;
