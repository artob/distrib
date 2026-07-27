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

/// Support for Dart projects.
#[cfg(feature = "dart")]
pub use distrib_dart as dart;

/// Support for JavaScript/TypeScript projects.
#[cfg(feature = "js")]
pub use distrib_js as js;

/// Support for Python projects.
#[cfg(feature = "python")]
pub use distrib_python as python;

/// Support for Ruby projects.
#[cfg(feature = "ruby")]
pub use distrib_ruby as ruby;

/// Support for Rust projects.
#[cfg(feature = "rust")]
pub use distrib_rust as rust;

/// Support for WebAssembly targets.
#[cfg(feature = "wasm")]
pub use distrib_wasm as wasm;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
