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

mod language;
pub use language::*;

mod package_manager;
pub use package_manager::*;

mod package_registry;
pub use package_registry::*;

mod runtime;
pub use runtime::*;
