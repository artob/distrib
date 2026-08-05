// This is free and unencumbered software released into the public domain.

use crate::PackageRegistry;
use alloc::boxed::Box;
use core::error::Error;

pub trait Publish {
    fn publish(&self, _registry: Option<PackageRegistry>) -> Result<(), Box<dyn Error>>;
}
