// This is free and unencumbered software released into the public domain.

use crate::BoxError;

pub trait Build {
    fn build(&self) -> Result<(), BoxError>;
}
