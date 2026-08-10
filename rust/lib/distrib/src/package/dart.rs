// This is free and unencumbered software released into the public domain.

impl TryFrom<distrib_dart::Pubspec> for Package {
    type Error = distrib_dart::LoadPubspecError;

    fn try_from(input: distrib_dart::Pubspec) -> Result<Self, Self::Error> {
        use itertools::Itertools;
        let dart_version = input
            .environment
            .map(|e| e.sdk.replace('^', "").split('.').take(2).join(".")); // FIXME
        Ok(Self {
            kind: PackageKind::Pub,
            language: Language::Dart,
            languages: vec![Language::Dart],
            name: input.name,
            version: input.version.unwrap_or_default(),
            authors: vec![], // N/A: deprecated since Dart 2.7
            description: input.description,
            homepage: input.homepage,
            keywords: input.topics.unwrap_or_default(),
            categories: vec![], // N/A
            licenses: vec![],   // TODO: detect from `LICENSE` file
            repository: input.repository,
            //metadata: None, // TODO
        })
    }
}

impl From<Package> for distrib_dart::Pubspec {
    fn from(input: Package) -> Self {
        use distrib_dart::{Map, pubspec::Environment};
        let homepage = input.homepage().cloned();
        let repository = input.repository().cloned();
        let issue_tracker = input.issue_tracker().map(|s| s.into_owned());
        Self {
            name: input.name,
            version: Some(input.version),
            description: input.description, // TODO: 60-180 characters
            homepage,
            repository,
            issue_tracker,
            documentation: None, // TODO
            dependencies: None,
            dev_dependencies: Some(Map::from_iter([
                ("distrib".into(), format!("^{}", env!("CARGO_PKG_VERSION"))),
                ("lints".into(), "^6.0.0".into()),
                ("test".into(), "^1.25.0".into()),
            ])),
            dependency_overrides: None,
            environment: Some(Environment {
                sdk: "^3.11.0".into(),
                ..Default::default()
            }),
            executables: None,
            platforms: None,
            publish_to: None, // "none" for private packages
            funding: None,
            false_secrets: None,
            screenshots: None,
            topics: None, // TODO: limit to 5 topics
            ignored_advisories: None,
            hooks: None,
        }
    }
}
