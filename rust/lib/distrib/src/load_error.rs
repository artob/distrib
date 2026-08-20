// This is free and unencumbered software released into the public domain.

use crate::Utf8PathBuf;
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use distrib_common::BoxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("no package found: {0}")]
    NoPackageFound(Utf8PathBuf),

    #[error("unknown package format: {0}")]
    UnknownPackageFormat(Utf8PathBuf),

    #[error(transparent)]
    Other(#[from] BoxError),
}

#[cfg(feature = "dart")]
impl From<distrib_dart::LoadPubspecError> for LoadError {
    fn from(error: distrib_dart::LoadPubspecError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "gleam")]
impl From<distrib_gleam::LoadPackageError> for LoadError {
    fn from(error: distrib_gleam::LoadPackageError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "js")]
impl From<distrib_js::LoadPackageError> for LoadError {
    fn from(error: distrib_js::LoadPackageError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "python")]
impl From<distrib_python::LoadPyprojectError> for LoadError {
    fn from(error: distrib_python::LoadPyprojectError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "ruby")]
impl From<distrib_ruby::LoadGemspecError> for LoadError {
    fn from(error: distrib_ruby::LoadGemspecError) -> Self {
        LoadError::Other(error.into())
    }
}

#[cfg(feature = "rust")]
impl From<distrib_rust::LoadManifestError> for LoadError {
    fn from(error: distrib_rust::LoadManifestError) -> Self {
        LoadError::Other(error.into())
    }
}
