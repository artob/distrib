// This is free and unencumbered software released into the public domain.

impl TryFrom<distrib_dart::Pubspec> for Package {
    type Error = distrib_dart::LoadPubspecError;

    fn try_from(input: distrib_dart::Pubspec) -> Result<Self, Self::Error> {
        use itertools::Itertools;
        let dart_version = input
            .environment
            .map(|e| e.sdk.replace('^', "").split('.').take(2).join(".")); // FIXME
        Ok(Self {
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
