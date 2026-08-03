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

pub mod prelude {
    #[cfg(feature = "macros")]
    pub use distrib_macros::export;
}
pub use prelude::*;

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

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
