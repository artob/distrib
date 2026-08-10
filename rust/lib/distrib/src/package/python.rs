// This is free and unencumbered software released into the public domain.

impl TryFrom<distrib_python::PyprojectToml> for Package {
    type Error = distrib_python::LoadPyprojectError;

    fn try_from(input: distrib_python::PyprojectToml) -> Result<Self, Self::Error> {
        use distrib_python::{Contact, License};
        let project = input.project.unwrap();
        let project_urls = project.urls.unwrap_or_default();
        let python_version = project
            .requires_python
            .and_then(|vs| vs.into_iter().next())
            .map(|v| v.version().only_release().to_string());
        Ok(Self {
            kind: PackageKind::Python,
            language: Language::Python,
            languages: vec![Language::Python],
            name: project.name,
            version: project.version.map(|v| v.to_string()).unwrap_or_default(),
            authors: project
                .authors
                .unwrap_or_default()
                .iter()
                .filter_map(Contact::name)
                .map(ToString::to_string)
                .collect(),
            description: project.description,
            homepage: project_urls.get("Homepage").cloned(),
            keywords: project.keywords.unwrap_or_default(),
            categories: project.classifiers.unwrap_or_default(),
            licenses: match project.license {
                Some(License::Spdx(s)) => vec![s],
                _ => vec![], // TODO
            },
            repository: project_urls.get("Repository").cloned(),
            //metadata: None, // TODO
            config: None,
        })
    }
}

impl From<Package> for distrib_python::PyprojectToml {
    fn from(input: Package) -> Self {
        use core::str::FromStr;
        use distrib_python::{Operator, Project, Version, VersionSpecifier, VersionSpecifiers};
        let config = input.config.unwrap_or_default();
        let python = config.lang.python.unwrap_or_default();
        Self {
            build_system: None,
            project: Some(Project {
                name: input.name,
                version: Version::from_str(&input.version).ok(),
                description: input.description,
                readme: None,
                requires_python: VersionSpecifiers::from_str(&format!(
                    ">={}",
                    python.version.unwrap_or_else(|| "3.9".into())
                ))
                .ok(),
                license: None,
                license_files: None,
                authors: None,
                maintainers: None,
                keywords: None,
                classifiers: None,
                urls: None,
                entry_points: None,
                scripts: None,
                gui_scripts: None,
                dependencies: None,
                optional_dependencies: None, // TODO: distrib
                dynamic: None,
            }),
            dependency_groups: None,
        }
    }
}
