// This is free and unencumbered software released into the public domain.

pub use package_json_schema::{
    AdditionalFields, Binary, Bug, BundledDependencies, Directories, Dist, EsNext, Exports, Man,
    PackageJson, Person, PersonObject, Private, PublishConfig, Repository, Type, Workspaces,
};

#[cfg(feature = "serde")]
pub use serde_json::Value;
