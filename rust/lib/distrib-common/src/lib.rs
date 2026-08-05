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

#[cfg(feature = "std")]
pub use camino::{Utf8Path, Utf8PathBuf};

mod build;
pub use build::*;

mod clean;
pub use clean::*;

mod language;
pub use language::*;

mod package_manager;
pub use package_manager::*;

mod package_registry;
pub use package_registry::*;

mod runtime;
pub use runtime::*;

mod tool;
pub use tool::*;
