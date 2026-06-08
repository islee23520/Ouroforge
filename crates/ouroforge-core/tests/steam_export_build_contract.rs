//! Contract tests for Steam Web-to-Desktop Wrapper and Build Pipeline v1 (#1838).

use ouroforge_core::steam_export_build::{
    SteamDepotConfig, SteamExportBuildManifest, SteamPackageDescriptor,
    STEAM_DEPOT_CONFIG_SCHEMA_VERSION, STEAM_EXPORT_BUILD_MANIFEST_SCHEMA_VERSION,
    STEAM_PACKAGE_DESCRIPTOR_SCHEMA_VERSION,
};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn fixture(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join("examples/steam-export-build-v1").join(rel))
        .unwrap_or_else(|error| panic!("read {rel}: {error}"))
}

#[test]
fn valid_manifest_and_depot_config_define_electron_steam_pipeline() {
    let manifest =
        SteamExportBuildManifest::from_json_str(&fixture("build-manifest.valid.fixture.json"))
            .expect("valid build manifest parses");
    let depot = SteamDepotConfig::from_json_str(&fixture("depot-config.valid.fixture.json"))
        .expect("valid depot config parses");

    assert_eq!(
        manifest.schema_version,
        STEAM_EXPORT_BUILD_MANIFEST_SCHEMA_VERSION
    );
    assert_eq!(depot.schema_version, STEAM_DEPOT_CONFIG_SCHEMA_VERSION);
    assert_eq!(manifest.issue, 1838);
    assert_eq!(manifest.source_web_build.runtime, "existing-web-runtime");
    assert_eq!(manifest.electron_wrapper.framework, "electron");
    assert_eq!(manifest.electron_wrapper.steam_bridge, "steamworks.js");
    assert_eq!(manifest.windows_artifact.platform, "windows-x64");
    assert!(manifest
        .windows_artifact
        .executable_path
        .ends_with("OuroforgeFixture.exe"));
    assert_eq!(depot.upload_mode, "local-dry-run");
    assert_eq!(depot.file_mappings.len(), 3);
}

#[test]
fn deterministic_package_descriptor_is_reproducible() {
    let manifest =
        SteamExportBuildManifest::from_json_str(&fixture("build-manifest.valid.fixture.json"))
            .unwrap();
    let depot =
        SteamDepotConfig::from_json_str(&fixture("depot-config.valid.fixture.json")).unwrap();

    let first = SteamPackageDescriptor::from_manifest_and_depot(&manifest, &depot)
        .expect("descriptor builds");
    let second = SteamPackageDescriptor::from_manifest_and_depot(&manifest, &depot)
        .expect("descriptor rebuilds");
    assert_eq!(first, second);
    assert_eq!(
        first.schema_version,
        STEAM_PACKAGE_DESCRIPTOR_SCHEMA_VERSION
    );
    assert_eq!(first.wrapper, "electron");
    assert_eq!(first.steam_bridge, "steamworks.js");
    assert_eq!(first.release_authority, "human-ring3-required");
    assert!(first.depot_config_hash.starts_with("sha256:"));
    assert!(first.build_manifest_hash.starts_with("sha256:"));
    assert!(first.artifact_hash.starts_with("sha256:"));
    assert_eq!(first.to_json().unwrap(), second.to_json().unwrap());
}

#[test]
fn descriptor_fixture_shape_validates_without_release_authority() {
    let descriptor =
        SteamPackageDescriptor::from_json_str(&fixture("package-descriptor.valid.fixture.json"))
            .expect("descriptor fixture validates");
    assert_eq!(
        descriptor.schema_version,
        STEAM_PACKAGE_DESCRIPTOR_SCHEMA_VERSION
    );
    assert_eq!(descriptor.release_authority, "human-ring3-required");
}

#[test]
fn invalid_fixtures_fail_closed_for_new_runtime_credentials_and_source_outputs() {
    let new_runtime = SteamExportBuildManifest::from_json_str(&fixture("invalid/new-runtime.json"))
        .expect_err("new runtime rejected");
    assert!(new_runtime.to_string().contains("existing-web-runtime"));

    let credential_upload =
        SteamDepotConfig::from_json_str(&fixture("invalid/credential-upload.json"))
            .expect_err("credentialed upload rejected");
    assert!(credential_upload.to_string().contains("local-dry-run"));

    let source_artifact =
        SteamExportBuildManifest::from_json_str(&fixture("invalid/source-artifact.json"))
            .expect_err("source artifact output rejected");
    assert!(source_artifact
        .to_string()
        .contains("ignored generated root"));
}

#[test]
fn contracts_preserve_generated_state_wording_and_governance_boundaries() {
    let combined = format!(
        "{}\n{}",
        fixture("build-manifest.valid.fixture.json"),
        fixture("depot-config.valid.fixture.json")
    );
    for required in [
        "reuse existing web runtime; no new runtime and no new engine",
        "read-only wrapper bridge; Rust/local owns trusted validation; no direct trusted writes",
        "Generated Windows artifacts and depot builds stay untracked unless fixture-scoped.",
        "Steam account/signing/content survey/Release button are human/Ring-3",
        "Steam desktop export is not Layer-3 cloud/mobile",
    ] {
        assert!(
            combined.contains(required),
            "missing boundary text: {required}"
        );
    }

    for forbidden in [
        "production-ready",
        "Godot replacement",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "Release button is automated",
        "market demand is automated",
        "Layer-3 cloud/mobile is GO",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording leaked: {forbidden}"
        );
    }
}
