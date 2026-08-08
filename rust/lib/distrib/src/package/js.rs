// This is free and unencumbered software released into the public domain.

impl TryFrom<distrib_js::PackageJson> for Package {
    type Error = distrib_js::LoadPackageError;

    fn try_from(input: distrib_js::PackageJson) -> Result<Self, Self::Error> {
        use distrib_js::{Person, PersonObject, Repository};
        Ok(Self {
            kind: PackageKind::Npm,
            language: Language::JavaScript,
            languages: vec![Language::JavaScript],
            name: input.name.unwrap_or_default(),
            version: input.version.unwrap_or_default(),
            authors: input
                .author
                .into_iter()
                .map(|person| match person {
                    Person::String(name) => name,
                    Person::Object(person) => person.name,
                })
                .collect(),
            description: input.description,
            homepage: input.homepage,
            keywords: input.keywords.unwrap_or_default(),
            categories: vec![], // N/A?
            licenses: input.license.into_iter().collect(),
            repository: match input.repository {
                Some(Repository::Object {
                    url: Some(mut url), ..
                }) => Some(if let Some(s) = url.strip_suffix(".git") {
                    url.truncate(s.len());
                    url
                } else {
                    url
                }),
                _ => None,
            },
            //metadata: None, // TODO: Some(serde_json::Value::Object(input_metadata.other)),
        })
    }
}

impl From<Package> for distrib_js::PackageJson {
    fn from(input: Package) -> Self {
        Self {
            name: Some(input.name),
            version: Some(input.version),
            description: input.description,
            keywords: Some(input.keywords),
            homepage: input.homepage,
            bugs: None,
            license: None,
            author: None,
            contributors: None,
            maintainers: None,
            files: None,
            main: None,
            exports: None,
            bin: None,
            type_: None,
            types: None,
            typings: None,
            types_versions: None,
            man: None,
            directories: None,
            repository: None,
            scripts: None,
            config: None,
            dependencies: None,
            dev_dependencies: None, // TODO: distrib
            optional_dependencies: None,
            peer_dependencies: None,
            bundled_dependencies: None,
            resolutions: None,
            package_manager: None,
            engines: None,
            engine_strict: None,
            os: None,
            cpu: None,
            private: None,
            publish_config: None,
            dist: None,
            readme: None,
            module: None,
            browser: None,
            workspaces: None,
            other: None,
        }
    }
}
