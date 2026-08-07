// This is free and unencumbered software released into the public domain.

use derive_more::{Display, FromStr};

/// A supported package manager.
#[derive(Clone, Debug, Default, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))
)]
#[non_exhaustive]
pub enum PackageKind {
    /// See: <https://crates.io/>
    #[default]
    Cargo,

    /// See: <https://gleam.run/>
    Gleam,

    /// See: <https://jsr.io/>
    Jsr,

    /// See: <https://npmjs.com/>
    Npm,

    /// See: <https://pub.dev/>
    Pub,

    /// See: <https://pypi.org/>
    Python,

    /// See: <https://rubygems.org/>
    Ruby,
}

#[cfg(feature = "std")]
impl TryFrom<&camino::Utf8Path> for PackageKind {
    type Error = std::io::Error;

    fn try_from(input: &camino::Utf8Path) -> Result<Self, Self::Error> {
        use PackageKind::*;
        use std::io::{Error, ErrorKind};
        Ok(match input.file_name().unwrap_or("") {
            "Cargo.toml" => Cargo,
            "gleam.toml" => Gleam,
            "jsr.json" => Jsr,
            "package.json" => Npm,
            "pubspec.yaml" => Pub,
            "pyproject.toml" => Python,
            name if name.ends_with(".gemspec") => Ruby,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidFilename,
                    "unrecognized package filename",
                ));
            },
        })
    }
}

impl PackageKind {
    pub const ALL: [Self; 7] = [
        Self::Cargo,
        Self::Gleam,
        Self::Jsr,
        Self::Npm,
        Self::Pub,
        Self::Python,
        Self::Ruby,
    ];

    pub fn manifest_name(&self) -> &str {
        use PackageKind::*;
        match self {
            Cargo => "Cargo.toml",
            Gleam => "gleam.toml",
            Jsr => "jsr.json",
            Npm => "package.json",
            Pub => "pubspec.yaml",
            Python => "pyproject.toml",
            Ruby => "*.gemspec", // wildcard
        }
    }
}
