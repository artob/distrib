// This is free and unencumbered software released into the public domain.

use alloc::{string::String, vec::Vec};

#[cfg(all(feature = "alloc", feature = "serde"))]
pub type Map<K, V = serde_json::Value> = indexmap::IndexMap<K, V>;

/// See: <https://dart.dev/tools/pub/pubspec>
#[cfg_attr(feature = "serde", serde_with::serde_as)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default, tag = "@type"))]
pub struct Pubspec {
    /// See: <https://dart.dev/tools/pub/pubspec#name>
    pub name: PackageName,

    /// See: <https://dart.dev/tools/pub/pubspec#version>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub version: Option<Version>,

    /// See: <https://dart.dev/tools/pub/pubspec#description>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// See: <https://dart.dev/tools/pub/pubspec#homepage>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub homepage: Option<Url>,

    /// See: <https://dart.dev/tools/pub/pubspec#repository>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub repository: Option<Url>,

    /// See: <https://dart.dev/tools/pub/pubspec#issue-tracker>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub issue_tracker: Option<Url>,

    /// See: <https://dart.dev/tools/pub/pubspec#documentation>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub documentation: Option<Url>,

    /// See: <https://dart.dev/tools/pub/pubspec#dependencies>
    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dependencies: Option<Map<String, Dependency>>,

    /// See: <https://dart.dev/tools/pub/pubspec#dependencies>
    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dev_dependencies: Option<Map<String, Dependency>>,

    /// See: <https://dart.dev/tools/pub/pubspec#dependencies>
    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dependency_overrides: Option<Map<String, Dependency>>,

    /// See: <https://dart.dev/tools/pub/pubspec#sdk-constraints>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub environment: Option<Environment>,

    /// See: <https://dart.dev/tools/pub/pubspec#executables>
    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub executables: Option<Map<String, Option<String>>>,

    /// See: <https://dart.dev/tools/pub/pubspec#platforms>
    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub platforms: Option<Map<String, Option<()>>>,

    /// See: <https://dart.dev/tools/pub/pubspec#publish_to>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub publish_to: Option<String>,

    /// See: <https://dart.dev/tools/pub/pubspec#funding>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub funding: Option<Vec<Url>>,

    /// See: <https://dart.dev/tools/pub/pubspec#false_secrets>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub false_secrets: Option<Vec<String>>,

    /// See: <https://dart.dev/tools/pub/pubspec#screenshots>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub screenshots: Option<Vec<Screenshot>>,

    /// See: <https://dart.dev/tools/pub/pubspec#topics>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub topics: Option<Vec<String>>,

    /// See: <https://dart.dev/tools/pub/pubspec#ignored_advisories>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ignored_advisories: Option<Vec<String>>,

    /// See: <https://dart.dev/tools/pub/pubspec#hooks>
    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hooks: Option<Map<String, Hook>>,
}

/// See: <https://dart.dev/tools/pub/pubspec#dependencies>
pub type Dependency = VersionConstraint;

/// See: <https://dart.dev/tools/pub/pubspec#sdk-constraints>
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Environment {
    pub sdk: VersionConstraint,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub flutter: Option<VersionConstraint>,
}

/// See: <https://dart.dev/tools/hooks#hook-configuration>
#[cfg(all(feature = "alloc", feature = "serde"))]
pub type Hook = Map<String, String>; // TODO

/// See: <https://dart.dev/tools/pub/pubspec#name>
pub type PackageName = String;

/// See: <https://dart.dev/tools/pub/pubspec#screenshots>
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Screenshot {
    pub path: String,
    pub description: String,
}

/// See: <https://dart.dev/tools/pub/pubspec#version>
pub type Version = String;

/// See: <https://dart.dev/tools/pub/dependencies#version-constraints>
pub type VersionConstraint = String;

/// See: <https://dart.dev/tools/pub/pubspec#repository>
pub type Url = String;
