// This is free and unencumbered software released into the public domain.

#![cfg(feature = "std")]

use distrib::{Package, PackageKind};

const NPM_MANIFEST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/package.json");

#[cfg(feature = "npm")]
#[test]
fn npm_loading_does_not_require_the_js_facade() {
    let package = Package::load(NPM_MANIFEST, None, None).expect("load npm fixture");
    assert_eq!(package.kind, PackageKind::Npm);
    assert_eq!(package.name, "feature-fixture");
    assert_eq!(package.version, "1.2.3");
    assert_eq!(package.tool(), Some(distrib::PackageManager::Npm));
}

#[cfg(not(feature = "npm"))]
#[test]
fn disabled_ecosystems_return_a_load_error() {
    let result = Package::load(NPM_MANIFEST, Some(PackageKind::Npm), None);
    assert!(matches!(
        result,
        Err(distrib::LoadError::UnknownPackageFormat(_))
    ));
}
