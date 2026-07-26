// This is free and unencumbered software released into the public domain.

use super::{LoadPackageError, PackageJson};
use std::path::Path;

pub fn load_package_json(path: impl AsRef<Path>) -> Result<PackageJson, LoadPackageError> {
    let input = std::fs::read_to_string(path.as_ref())?;
    let output = PackageJson::try_from(input).unwrap();
    Ok(output)
}
