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

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// Support for Dart projects.
#[cfg(feature = "dart")]
pub mod dart {
    mod error;
    pub use error::*;

    pub mod pubspec;
    pub use pubspec::Pubspec;

    #[cfg(feature = "std")]
    mod load;
    #[cfg(feature = "std")]
    pub use load::*;
}

/// Support for JavaScript/TypeScript projects.
#[cfg(feature = "js")]
pub mod js {
    mod error;
    pub use error::*;

    pub mod package;
    pub use package::*;

    #[cfg(feature = "std")]
    mod load;
    #[cfg(feature = "std")]
    pub use load::*;
}

/// Support for Python projects.
#[cfg(feature = "python")]
pub mod python {
    mod error;
    pub use error::*;

    pub mod pyproject;
    pub use pyproject::*;

    #[cfg(feature = "std")]
    mod load;
    #[cfg(feature = "std")]
    pub use load::*;
}

/// Support for Ruby projects.
#[cfg(feature = "ruby")]
pub mod ruby {
    mod error;
    pub use error::*;

    pub mod gemspec;
    pub use gemspec::*;

    #[cfg(feature = "std")]
    mod load;
    #[cfg(feature = "std")]
    pub use load::*;
}

/// Support for Rust projects.
#[cfg(feature = "rust")]
pub mod rust {
    mod error;
    pub use error::*;

    pub mod manifest;
    pub use manifest::*;

    #[cfg(feature = "std")]
    mod load;
    #[cfg(feature = "std")]
    pub use load::*;
}
