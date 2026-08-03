// This is free and unencumbered software released into the public domain.

use super::{LoadPackageError, PackageConfig};
use std::path::Path;

pub fn load_package_config(path: impl AsRef<Path>) -> Result<PackageConfig, LoadPackageError> {
    let input = std::fs::read_to_string(path.as_ref())?;
    let output = toml1::from_str(&input).unwrap(); // FIXME
    Ok(output)
}
