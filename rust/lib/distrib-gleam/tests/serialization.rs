// This is free and unencumbered software released into the public domain.

#![cfg(feature = "serde")]

use distrib_gleam::PackageConfig;

#[test]
fn dependency_maps_round_trip_without_workspace_feature_unification() {
    let package: PackageConfig =
        toml1::from_str(include_str!("fixtures/gleam.toml")).expect("valid fixture");
    assert_eq!(package.name, "feature_fixture");
    assert_eq!(
        package.dependencies.as_ref().unwrap()["gleam_stdlib"],
        ">= 1.0.0 and < 2.0.0"
    );
    let output = package.try_to_string().expect("TOML serialization enabled");
    let restored: PackageConfig = toml1::from_str(&output).expect("valid generated TOML");
    assert_eq!(package, restored);
}

#[cfg(feature = "std")]
#[test]
fn default_features_expose_the_loader() {
    let package = distrib_gleam::load_package_config(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/gleam.toml"
    ))
    .expect("load fixture");
    assert_eq!(package.version, "1.2.3");
}
