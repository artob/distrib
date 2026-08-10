// This is free and unencumbered software released into the public domain.

use super::LoadError;
use crate::{Config, Utf8Path};
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

    /// The package configuration, if any.
    pub config: Option<Config>,
    // The package metadata, if any.
    //pub metadata: Option<Value>,
}

impl Package {
    pub fn load(
        file_path: impl AsRef<Utf8Path>,
        kind: Option<PackageKind>,
        config: Option<Config>,
    ) -> Result<Self, LoadError> {
        use PackageKind::*;
        let file_path = file_path.as_ref();
        let kind = match kind {
            Some(kind) => kind,
            None => {
                PackageKind::try_from(file_path).map_err(|err| LoadError::Other(Box::new(err)))?
            },
        };
        let package = match kind {
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
        };
        Ok(Package { config, ..package })
    }

    pub fn summary(&self) -> Option<&String> {
        self.description.as_ref() // TODO: first sentence only
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn homepage(&self) -> Option<&String> {
        self.homepage.as_ref()
    }

    pub fn repository(&self) -> Option<&String> {
        self.repository.as_ref()
    }

    pub fn issue_tracker(&self) -> Option<Cow<'_, str>> {
        match self.repository.as_deref() {
            Some(repo) if repo.starts_with("https://github.com/") => {
                Some(format!("{}/issues", repo).into())
            },
            Some(_) | None => None,
        }
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
