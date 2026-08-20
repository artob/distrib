// This is free and unencumbered software released into the public domain.

use crate::BoxError;

pub trait Clean {
    fn clean(&self) -> Result<(), BoxError>;
}
