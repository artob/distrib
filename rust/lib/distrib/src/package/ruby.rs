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
        })
    }
}
