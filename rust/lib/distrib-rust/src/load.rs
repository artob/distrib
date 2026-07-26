// This is free and unencumbered software released into the public domain.

use super::{LoadManifestError, Manifest, Value};
use std::path::Path;

pub fn load_cargo_toml(path: impl AsRef<Path>) -> Result<Manifest, LoadManifestError> {
    Ok(Manifest::from_path(path.as_ref())?)
}
