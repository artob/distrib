// This is free and unencumbered software released into the public domain.

use crate::{BoxError, PackageRegistry};

pub trait Publish {
    fn publish(&self, _registry: Option<PackageRegistry>) -> Result<(), BoxError>;
}
