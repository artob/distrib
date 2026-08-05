// This is free and unencumbered software released into the public domain.

impl TryFrom<distrib_js::PackageJson> for Package {
    type Error = distrib_js::LoadPackageError;

    fn try_from(input: distrib_js::PackageJson) -> Result<Self, Self::Error> {
        use distrib_js::{Person, PersonObject, Repository};
        Ok(Self {
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
