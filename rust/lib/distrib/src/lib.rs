// This is free and unencumbered software released into the public domain.

//! Distrib helps you distribute your software.
//!
//! # Feature requirements
//!
//! With no default features, only the allocation-free common types are exposed.
//! `alloc` adds `Package` and `Config`; `serde` adds their serialization.
//! `std` enables filesystem loading and process adapters for selected ecosystems.
//! The CLI requires `cli`; select ecosystems separately when disabling defaults,
//! for example `--no-default-features --features cli,rust`.
//!
//! Dart, Ruby, and Gleam models support allocation-only builds. Their maps retain
//! insertion order with `std` and use key order otherwise. The upstream Cargo,
//! npm, and Python manifest models require `std`. YAML text encoding also
//! requires `std`.
//!
//! `export` activates the attribute macro backends for the enabled languages:
//! `python,export` generates PyO3 functions and `wasm,export` generates
//! wasm-bindgen functions. Other runtime features currently expose FFI libraries
//! but do not implement attribute-macro function exports.

#![no_std]
#![allow(unused)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod prelude {
    #[cfg(feature = "macros")]
    pub use distrib_macros::export;
}
pub use prelude::*;

pub use distrib_common::*;

/// Support for the BEAM runtime (Elixir, Erlang, Gleam, etc).
#[cfg(feature = "beam")]
pub use distrib_beam as beam;

/// Support for Dart language projects.
#[cfg(feature = "dart")]
pub use distrib_dart as dart;

/// Support for Gleam language projects.
#[cfg(feature = "gleam")]
pub use distrib_gleam as gleam;

/// Support for JavaScript/TypeScript language projects.
#[cfg(feature = "js")]
pub use distrib_js as js;

/// Support for the JSR (JavaScript Registry) package manager.
#[cfg(feature = "jsr")]
pub use distrib_jsr as jsr;

/// Support for the Node.js runtime.
#[cfg(feature = "node")]
pub use distrib_node as node;

/// Support for the NPM (Node Package Manager) package manager.
#[cfg(feature = "npm")]
pub use distrib_npm as npm;

/// Support for Python language projects.
#[cfg(feature = "python")]
pub use distrib_python as python;

/// Support for Ruby language projects.
#[cfg(feature = "ruby")]
pub use distrib_ruby as ruby;

/// Support for Rust language projects.
#[cfg(feature = "rust")]
pub use distrib_rust as rust;

/// Support for the WebAssembly (aka Wasm) runtime.
#[cfg(feature = "wasm")]
pub use distrib_wasm as wasm;

#[cfg(feature = "alloc")]
mod config;
#[cfg(feature = "alloc")]
pub use config::*;

#[cfg(feature = "std")]
mod load_error;
#[cfg(feature = "std")]
pub use load_error::*;

#[cfg(feature = "alloc")]
mod package;
#[cfg(feature = "alloc")]
pub use package::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[cfg(all(doctest, feature = "wasm", feature = "export"))]
#[doc = include_str!("../tests/fixtures/wasm-export.md")]
mod wasm_export_contract {}
