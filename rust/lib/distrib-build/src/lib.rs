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

pub fn all() {
    #[cfg(feature = "dart")]
    dart();
    #[cfg(feature = "js")]
    js();
    #[cfg(feature = "python")]
    python();
    #[cfg(feature = "ruby")]
    ruby();
    #[cfg(feature = "rust")]
    rust();
    #[cfg(feature = "wasm")]
    wasm();
}

#[cfg(feature = "dart")]
pub fn dart() {}

#[cfg(feature = "js")]
pub fn js() {
    // See: <https://crates.io/crates/napi-build>
    // See: <https://docs.rs/napi-build/latest/napi_build/fn.setup.html>
    napi_build::setup();
}

#[cfg(feature = "python")]
pub fn python() {
    pyo3_build_config::add_python_framework_link_args();
    //pyo3_build_config::add_libpython_rpath_link_args();
    //pyo3_build_config::add_extension_module_link_args();
}

#[cfg(feature = "ruby")]
pub fn ruby() {}

#[cfg(feature = "rust")]
pub fn rust() {}

#[cfg(feature = "wasm")]
pub fn wasm() {}
