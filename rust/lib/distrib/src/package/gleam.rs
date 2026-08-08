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

impl From<Package> for distrib_gleam::PackageConfig {
    fn from(input: Package) -> Self {
        use distrib_gleam::{Map, PackageRepository};
        Self {
            name: input.name,
            version: input.version,
            licenses: Some(input.licenses),
            description: input.description,
            repository: Some(PackageRepository {
                r#type: "custom".to_string(), // TODO
                url: input.repository,
                ..Default::default()
            }),
            links: None, // TODO
            dependencies: Some(Map::default()),
            dev_dependencies: Some(Map::default()), // TODO: distrib
        }
    }
}
