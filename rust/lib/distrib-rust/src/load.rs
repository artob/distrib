// This is free and unencumbered software released into the public domain.

use super::{LoadManifestError, Manifest, Value};
use std::path::{Path, absolute};

pub fn load_cargo_toml(path: impl AsRef<Path>) -> Result<Manifest, LoadManifestError> {
    let path = path.as_ref();
    let input = std::fs::read_to_string(path)?;
    let mut manifest = Manifest::from_str(&input)?;
    if manifest.needs_workspace_inheritance() {
        let path = absolute(path)?; // needs an absolute path here
        manifest.complete_from_path(&path)?;
    }
    Ok(manifest)
}
