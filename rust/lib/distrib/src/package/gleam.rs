// This is free and unencumbered software released into the public domain.

impl TryFrom<distrib_gleam::PackageConfig> for Package {
    type Error = distrib_gleam::LoadPackageError;

    fn try_from(input: distrib_gleam::PackageConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: PackageKind::Gleam,
            language: Language::Gleam,
            languages: vec![Language::Gleam],
            name: input.name,
            version: input.version,
            authors: vec![], // N/A
            description: input.description,
            homepage: input
                .links
                .unwrap_or_default()
                .into_iter()
                .filter_map(|link| {
                    if link.title == "Website" {
                        Some(link.href.clone())
                    } else {
                        None
                    }
                })
                .next(),
            keywords: vec![],   // N/A
            categories: vec![], // N/A
            licenses: input.licenses.unwrap_or_default(),
            repository: None, // TODO: input.repository
                              //metadata: None,   // TODO
        })
    }
}
