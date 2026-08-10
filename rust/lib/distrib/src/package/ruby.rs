// This is free and unencumbered software released into the public domain.

impl TryFrom<distrib_ruby::Gemspec> for Package {
    type Error = distrib_ruby::LoadGemspecError;

    fn try_from(input: distrib_ruby::Gemspec) -> Result<Self, Self::Error> {
        let input_metadata = input.metadata.unwrap_or_default();
        let ruby_version = input
            .required_ruby_version
            .and_then(|r| r.requirements.into_iter().next())
            .map(|e| e.1.version.clone());
        Ok(Self {
            kind: PackageKind::Ruby,
            language: Language::Ruby,
            languages: vec![Language::Ruby],
            name: input.name,
            version: input.version.version,
            authors: input.authors,
            description: input.description,
            homepage: input.homepage,
            keywords: vec![],   // N/A
            categories: vec![], // N/A
            licenses: input.licenses,
            repository: input_metadata.source_code_uri,
            //metadata: Some(input_metadata.other.into_iter().collect()),
            config: None,
        })
    }
}

impl From<Package> for distrib_ruby::Gemspec {
    fn from(input: Package) -> Self {
        use distrib_ruby::{Dependency, Metadata, Platform, Requirement, Version};
        let summary = input.summary().cloned().unwrap_or_default();
        let config = input.config.unwrap_or_default();
        let ruby = config.lang.ruby.unwrap_or_default();
        Self {
            name: input.name,
            version: Version {
                version: input.version,
            },
            platform: Some(Platform::Ruby),
            authors: input.authors,
            summary,
            description: input.description,
            dependencies: Some(vec![
                Dependency::development("distrib", ("~>", env!("CARGO_PKG_VERSION"))),
                Dependency::development("rake", ("~>", "13.0")),
            ]),
            files: vec!["AUTHORS".into()], // TODO
            required_ruby_version: Some(Requirement::from((
                ">=".into(),
                ruby.version.unwrap_or_else(|| "2.6".into()),
            ))),
            required_rubygems_version: Some(Requirement::from((">=", "0"))),
            homepage: input.homepage.clone(),
            licenses: input.licenses,
            metadata: Some(Metadata {
                homepage_uri: input.homepage,
                source_code_uri: input.repository,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
