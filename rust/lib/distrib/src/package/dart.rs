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
        Self {
            name: input.name,
            version: Some(input.version),
            description: input.description,
            homepage: input.homepage,
            repository: input.repository,
            issue_tracker: None,
            documentation: None,
            dependencies: None,
            dev_dependencies: None, // TODO: distrib
            dependency_overrides: None,
            environment: None,
            executables: None,
            platforms: None,
            publish_to: None,
            funding: None,
            false_secrets: None,
            screenshots: None,
            topics: None,
            ignored_advisories: None,
            hooks: None,
        }
    }
}
