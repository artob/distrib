// This is free and unencumbered software released into the public domain.

use alloc::string::String;

pub use pyproject_toml::{Contact, License, Project};

pub type PyprojectToml = pyproject_toml::PyProjectToml;

pub trait PyprojectTomlExt {
    fn try_to_string(&self) -> Result<String, toml0::ser::Error>;
}

impl PyprojectTomlExt for PyprojectToml {
    fn try_to_string(&self) -> Result<String, toml0::ser::Error> {
        toml0::to_string_pretty(self)
    }
}
