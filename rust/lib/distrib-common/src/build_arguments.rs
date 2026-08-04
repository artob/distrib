// This is free and unencumbered software released into the public domain.

use crate::PackageManager;
use alloc::{string::String, vec, vec::Vec};
use derive_more::{Display, FromStr};

#[derive(Clone, Debug, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[non_exhaustive]
pub struct BuildArguments(PackageManager);

impl From<PackageManager> for BuildArguments {
    fn from(package_manager: PackageManager) -> Self {
        Self(package_manager)
    }
}

impl BuildArguments {
    pub fn program(&self) -> Option<&[&str]> {
        self.0.program()
    }

    pub fn to_vec(&self) -> Option<Vec<String>> {
        use PackageManager::*;
        Some(match self.0 {
            Cargo => vec!["build".into()],
            Jsr | Npm => vec!["run".into(), "build".into()],
            Mix => vec!["compile".into()],
            Pub => vec!["build".into(), "cli".into()],
            PyPi => vec!["build".into()],
            RubyGems => vec!["build".into()],
            _ => return None,
        })
    }
}
