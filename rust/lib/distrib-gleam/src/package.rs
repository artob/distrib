// This is free and unencumbered software released into the public domain.

use alloc::{string::String, vec::Vec};

pub type Map<K, V> = indexmap::IndexMap<K, V>;

/// See: <https://gleam.run/documentation/gleam-toml-reference/>
#[cfg_attr(feature = "serde", serde_with::serde_as)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct PackageConfig {
    /// See: <https://gleam.run/documentation/gleam-toml-reference/#name>
    pub name: PackageName,

    /// See: <https://gleam.run/documentation/gleam-toml-reference/#version>
    pub version: PackageVersion,

    /// See: <https://gleam.run/documentation/gleam-toml-reference/#licences>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub licenses: Option<Vec<LicenseId>>,

    /// See: <https://gleam.run/documentation/gleam-toml-reference/#description>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// See: <https://gleam.run/documentation/gleam-toml-reference/#repository>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub repository: Option<PackageRepository>,

    /// See: <https://gleam.run/documentation/gleam-toml-reference/#links>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub links: Option<Vec<PackageLink>>,

    /// See: <https://gleam.run/documentation/gleam-toml-reference/#dependencies>    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dependencies: Option<Map<String, PackageVersionConstraint>>,

    /// See: <https://gleam.run/documentation/gleam-toml-reference/#dev_dependencies>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dev_dependencies: Option<Map<String, PackageVersionConstraint>>,
}

#[cfg(all(feature = "serde"))]
impl PackageConfig {
    pub fn try_to_string(&self) -> Result<String, toml1::ser::Error> {
        toml1::to_string(self)
    }
}

/// See: <https://gleam.run/documentation/gleam-toml-reference/#name>
pub type PackageName = String;

/// See: <https://gleam.run/documentation/gleam-toml-reference/#version>
pub type PackageVersion = String;

/// See: <https://gleam.run/documentation/gleam-toml-reference/#dependencies>
pub type PackageVersionConstraint = String;

/// See: <https://gleam.run/documentation/gleam-toml-reference/#repository>
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct PackageRepository {
    pub r#type: String,
    pub host: Option<String>,
    pub user: Option<String>,
    pub repo: Option<String>,
    pub url: Option<String>,
    pub path: Option<String>,
    pub tag_prefix: Option<String>,
}

/// See: <https://gleam.run/documentation/gleam-toml-reference/#links>
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct PackageLink {
    pub title: String,
    pub href: Url,
}

/// See: <https://gleam.run/documentation/gleam-toml-reference/#links>
pub type Url = String;

/// See: <https://gleam.run/documentation/gleam-toml-reference/#licences>
pub type LicenseId = String;
