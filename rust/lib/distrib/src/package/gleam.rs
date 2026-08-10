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
            config: None,
        })
    }
}

impl From<Package> for distrib_gleam::PackageConfig {
    fn from(input: Package) -> Self {
        use distrib_gleam::{Map, PackageRepository};
        let config = input.config.unwrap_or_default();
        let gleam = config.lang.gleam.unwrap_or_default();
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
            links: match input.homepage {
                Some(url) => Some(vec![distrib_gleam::PackageLink {
                    title: "Website".to_string(),
                    href: url.into(),
                }]),
                None => None,
            },
            dependencies: Some(Map::from_iter([(
                "gleam_stdlib".into(),
                ">= 1.0.0 and < 2.0.0".into(),
            )])),
            dev_dependencies: Some(Map::from_iter([
                (
                    "distrib".into(),
                    format!(">= {} and < 0.1.0", env!("CARGO_PKG_VERSION")),
                ),
                ("gleeunit".into(), ">= 1.0.0 and < 2.0.0".into()),
            ])),
        }
    }
}
