// This is free and unencumbered software released into the public domain.

impl TryFrom<distrib_rust::Manifest> for Package {
    type Error = distrib_rust::LoadManifestError;

    fn try_from(input: distrib_rust::Manifest) -> Result<Self, Self::Error> {
        use distrib_rust::{Edition, Value};
        assert!(!input.needs_workspace_inheritance());
        let package = input.package.ok_or_else(|| {
            distrib_rust::LoadManifestError::Other("package field is missing".into())
        })?;
        let rust_edition = package.edition.unwrap();
        let rust_version = package.rust_version.map(|x| x.unwrap()).unwrap_or_else(|| {
            match rust_edition {
                Edition::E2024 => "2024",
                Edition::E2021 => "2021",
                Edition::E2018 => "2018",
                Edition::E2015 => "2015",
                _ => "2015", // Cargo assumes 2015 if absent
            }
            .into()
        });
        Ok(Self {
            kind: PackageKind::Cargo,
            language: Language::Rust,
            languages: vec![Language::Rust],
            name: package.name,
            version: package.version.unwrap().to_string(),
            authors: package.authors.unwrap(),
            description: package.description.map(|x| x.unwrap()),
            homepage: package.homepage.map(|x| x.unwrap()),
            keywords: package.keywords.unwrap(),
            categories: package.categories.unwrap(),
            licenses: match package.license {
                None => vec![],
                Some(x) => vec![x.unwrap()],
            },
            repository: package.repository.map(|x| x.unwrap()),
            //metadata: package.metadata.map(|x| x.try_into()).transpose()?,
            config: None,
        })
    }
}
