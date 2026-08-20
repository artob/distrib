// This is free and unencumbered software released into the public domain.

use super::{LoadPackageError, PackageConfig};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use distrib_common::BoxError;
use std::path::Path;
use thiserror::Error;

pub fn write(path: impl AsRef<Path>, config: &PackageConfig) -> Result<(), WriteError> {
    let path = path.as_ref();
    let output = config.try_to_string()?;
    let _ = std::fs::write(path, output)?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum WriteError {
    #[error(transparent)]
    Toml(#[from] toml1::ser::Error),

    #[cfg(feature = "std")]
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] BoxError),
}
