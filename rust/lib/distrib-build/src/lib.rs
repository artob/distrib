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

use alloc::boxed::Box;
use core::error::Error;

pub fn all() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "dart")]
    dart()?;
    #[cfg(feature = "js")]
    js()?;
    #[cfg(feature = "python")]
    python()?;
    #[cfg(feature = "ruby")]
    ruby()?;
    #[cfg(feature = "rust")]
    rust()?;
    #[cfg(feature = "wasm")]
    wasm()?;
    Ok(())
}

#[cfg(feature = "dart")]
pub fn dart() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(feature = "js")]
pub fn js() -> Result<(), Box<dyn Error>> {
    // See: <https://crates.io/crates/napi-build>
    // See: <https://docs.rs/napi-build/latest/napi_build/fn.setup.html>
    napi_build::setup();
    Ok(())
}

#[cfg(feature = "python")]
pub fn python() -> Result<(), Box<dyn Error>> {
    pyo3_build_config::add_python_framework_link_args();
    //pyo3_build_config::add_libpython_rpath_link_args();
    //pyo3_build_config::add_extension_module_link_args();
    Ok(())
}

#[cfg(feature = "ruby")]
pub fn ruby() -> Result<rb_sys_env::RbEnv, Box<dyn Error>> {
    rb_sys_env::activate()
}

#[cfg(feature = "rust")]
pub fn rust() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(feature = "wasm")]
pub fn wasm() -> Result<(), Box<dyn Error>> {
    Ok(())
}
