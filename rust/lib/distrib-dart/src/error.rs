// This is free and unencumbered software released into the public domain.

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use distrib_common::BoxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadPubspecError {
    #[error(transparent)]
    Other(#[from] BoxError),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for LoadPubspecError {
    fn from(error: std::io::Error) -> Self {
        Self::Other(error.into())
    }
}
