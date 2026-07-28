// This is free and unencumbered software released into the public domain.

use super::{LoadPubspecError, Pubspec};
use std::path::Path;

pub fn load_pubspec(path: impl AsRef<Path>) -> Result<Pubspec, LoadPubspecError> {
    let input = std::fs::read_to_string(path.as_ref())?;
    let output = serde_norway::from_str(&input).unwrap(); // FIXME
    Ok(output)
}
