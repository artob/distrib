// This is free and unencumbered software released into the public domain.

//! Distrib helps you distribute your software.
//!
//! Python manifest and version types require `std`, as do their upstream parser
//! libraries. `export` enables PyO3 macros and the Python `distrib::export`
//! backend; using PyO3 alone does not change attribute-macro expansion.

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

#[cfg(feature = "std")]
pub use pep440_rs::{Operator, Version, VersionParseError, VersionSpecifier, VersionSpecifiers};

#[cfg(feature = "alloc")]
mod error;
#[cfg(feature = "alloc")]
pub use error::*;

#[cfg(feature = "std")]
mod load;
#[cfg(feature = "std")]
pub use load::*;

#[cfg(feature = "std")]
mod program;
#[cfg(feature = "std")]
pub use program::*;

#[cfg(feature = "std")]
pub mod pyproject;
#[cfg(feature = "std")]
pub use pyproject::*;
