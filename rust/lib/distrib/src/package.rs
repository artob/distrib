// This is free and unencumbered software released into the public domain.

use super::LoadError;
use crate::Utf8Path;
use alloc::{
    borrow::Cow,
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use distrib_common::{Language, PackageKind, PackageManager, PackageRegistry};
//use serde_json::{Map, Value, json};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Package {
    /// The package kind.
    pub kind: PackageKind,

    /// The primary language.
    pub language: Language,

    /// The implementation languages.
    pub languages: Vec<Language>,

    /// The package name.
    pub name: String,

    /// The package version.
    pub version: String,

    /// The package authors.
    pub authors: Vec<String>,

    /// The package title.
    pub description: Option<String>,

    /// The package summary.
    pub homepage: Option<String>,

    /// The package keywords.
    pub keywords: Vec<String>,

    /// The package categories.
    pub categories: Vec<String>,

    /// The package license.
    pub licenses: Vec<String>,

    /// The package repository.
    pub repository: Option<String>,
    ///// The package metadata, if any.
    //pub metadata: Option<Value>,
}

impl Package {
    pub fn locate(dir_path: impl AsRef<Utf8Path>) -> Result<Self, LoadError> {
        let dir_path = dir_path.as_ref();
        for file_name in [
            #[cfg(feature = "gleam")]
            "gleam.toml",
            #[cfg(feature = "js")]
            "package.json",
            #[cfg(feature = "dart")]
            "pubspec.yaml",
            #[cfg(feature = "python")]
            "pyproject.toml",
            #[cfg(feature = "ruby")]
            ".gemspec.yaml", // TODO
            // This should be last, to support polyglot projects:
            #[cfg(feature = "rust")]
            "Cargo.toml",
        ] {
            let file_path = dir_path.join(file_name);
            if file_path.exists() {
                return Self::load(file_path, None);
            }
        }
        Err(LoadError::NoPackageFound(dir_path.into()))
    }

    pub fn load(
        file_path: impl AsRef<Utf8Path>,
        package_kind: Option<PackageKind>,
    ) -> Result<Self, LoadError> {
        use PackageKind::*;
        let file_path = file_path.as_ref();
        let package_kind = match package_kind {
            Some(kind) => kind,
            None => {
                PackageKind::try_from(file_path).map_err(|err| LoadError::Other(Box::new(err)))?
            },
        };
        Ok(match package_kind {
            #[cfg(feature = "rust")]
            Cargo => distrib_rust::load_cargo_toml(file_path)?.try_into()?,
            #[cfg(feature = "gleam")]
            Gleam => distrib_gleam::load_package_config(file_path)?.try_into()?,
            // #[cfg(feature = "jsr")]
            // Jsr => distrib_jsr::load_package_json(file_path)?.try_into()?,
            #[cfg(feature = "js")]
            Npm => distrib_js::load_package_json(file_path)?.try_into()?,
            #[cfg(feature = "dart")]
            Pub => distrib_dart::load_pubspec(file_path)?.try_into()?,
            #[cfg(feature = "python")]
            Python => distrib_python::load_pyproject_toml(file_path)?.try_into()?,
            #[cfg(feature = "ruby")]
            Ruby => distrib_ruby::load_gemspec(file_path)?.try_into()?,
            _ => {
                return Err(LoadError::UnknownPackageFormat(file_path.into()));
            },
        })
    }

    pub fn registry(&self) -> Option<PackageRegistry> {
        Some(PackageRegistry::Crates)
    }

    pub fn tool(&self) -> Option<PackageManager> {
        use PackageKind::*;
        Some(match self.kind {
            #[cfg(feature = "rust")]
            Cargo => PackageManager::Cargo,
            #[cfg(feature = "gleam")]
            Gleam => PackageManager::Gleam,
            #[cfg(feature = "js")]
            Npm => PackageManager::Npm,
            #[cfg(feature = "dart")]
            Pub => PackageManager::Pub,
            #[cfg(feature = "python")]
            Python => PackageManager::PyPi, // TODO
            #[cfg(feature = "ruby")]
            Ruby => PackageManager::RubyGems,
            _ => return None,
        })
    }
}

#[cfg(feature = "dart")]
include!("package/dart.rs");

#[cfg(feature = "gleam")]
include!("package/gleam.rs");

#[cfg(feature = "js")]
include!("package/js.rs");

#[cfg(feature = "python")]
include!("package/python.rs");

#[cfg(feature = "ruby")]
include!("package/ruby.rs");

#[cfg(feature = "rust")]
include!("package/rust.rs");
