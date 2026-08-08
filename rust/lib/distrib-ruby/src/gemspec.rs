// This is free and unencumbered software released into the public domain.

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use indexmap::IndexMap;

pub type Gemspec = Specification;

#[cfg(all(feature = "alloc", feature = "serde"))]
pub type Map<K, V = serde_json::Value> = IndexMap<K, V>;

/// The package information for a gem, typically defined in a `.gemspec` file.
///
/// This is the tag `!ruby/object:Gem::Specification` in YAML.
///
/// Note that we (pedantically) distinguish between `Option<Vec<T>` and
/// `Vec<T>` so as to indicate whether the property was unset or set to empty
/// the input specification file.
///
/// See: <https://guides.rubygems.org/specification-reference/>
#[cfg_attr(feature = "serde", serde_with::serde_as)]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Specification<T = Metadata> {
    /// See: <https://guides.rubygems.org/specification-reference/#authors=>
    #[cfg_attr(feature = "serde", serde(alias = "author"))]
    #[cfg_attr(feature = "serde", serde_as(as = "OneOrMany<_, PreferMany>"))]
    pub authors: Vec<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#bindir>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub bindir: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#cert_chain>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cert_chain: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#add_dependency>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dependencies: Option<Vec<Dependency>>,

    /// See: <https://guides.rubygems.org/specification-reference/#description>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#email>
    #[cfg_attr(
        feature = "serde",
        serde(alias = "emails", skip_serializing_if = "Vec::is_empty")
    )]
    #[cfg_attr(feature = "serde", serde_as(as = "OneOrMany<_, PreferOne>"))]
    pub email: Vec<String>, // TODO: Option<Vec<String>>

    /// See: <https://guides.rubygems.org/specification-reference/#executables>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub executables: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#extensions>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub extensions: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#extensions_dir>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub extensions_dir: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#extra_rdoc_files>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub extra_rdoc_files: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#files>
    pub files: Vec<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#homepage>
    pub homepage: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#licenses=>
    #[cfg_attr(
        feature = "serde",
        serde(alias = "license", skip_serializing_if = "Vec::is_empty")
    )]
    #[cfg_attr(feature = "serde", serde_as(as = "OneOrMany<_, PreferMany>"))]
    pub licenses: Vec<String>, // TODO: Option<Vec<String>>

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub metadata: Option<T>,

    /// See: <https://guides.rubygems.org/specification-reference/#name>
    pub name: String,

    /// See: <https://guides.rubygems.org/specification-reference/#platform=>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub platform: Option<Platform>,

    /// See: <https://guides.rubygems.org/specification-reference/#post_install_message>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub post_install_message: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#rdoc_options>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub rdoc_options: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#require_paths=>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub require_paths: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#required_ruby_version>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub required_ruby_version: Option<Requirement>,

    /// See: <https://guides.rubygems.org/specification-reference/#required_rubygems_version>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub required_rubygems_version: Option<Requirement>,

    /// See: <https://guides.rubygems.org/specification-reference/#requirements>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub requirements: Option<Vec<String>>,

    /// See: <https://guides.rubygems.org/specification-reference/#rubygems_version>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub rubygems_version: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#signing_key>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub signing_key: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#summary>
    pub summary: String,

    /// See: <https://guides.rubygems.org/specification-reference/#version>
    pub version: Version,
}

#[cfg(all(feature = "serde"))]
impl<T: serde::Serialize> Specification<T> {
    pub fn try_to_string(&self) -> Result<String, serde_norway::Error> {
        serde_norway::to_string(self)
    }
}

/// !ruby/object:Gem::Dependency
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Dependency {
    pub name: String,
    pub requirement: Requirement,
    pub r#type: String,
    pub prerelease: bool,
    pub version_requirements: Requirement,
}

/// `!ruby/object:Gem::Requirement`
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Requirement {
    pub requirements: Vec<(String, Version)>,
}

/// `!ruby/object:Gem::Version`
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Version {
    pub version: String,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))
)]
pub enum Platform {
    #[default]
    Ruby,
    Current,
    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// See: <https://guides.rubygems.org/specification-reference/#metadata>
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Metadata {
    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub bug_tracker_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub changelog_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub documentation_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub funding_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub homepage_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mailing_list_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub source_code_uri: Option<String>,

    /// See: <https://guides.rubygems.org/specification-reference/#metadata>
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub wiki_uri: Option<String>,

    #[cfg(all(feature = "alloc", feature = "serde"))]
    #[cfg_attr(
        feature = "serde",
        serde(flatten, skip_serializing_if = "Map::is_empty")
    )]
    pub other: Map<String>,
}
