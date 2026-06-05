//! Package Metadata and Local Distribution Descriptor v1 contract (#730).

use ouroforge_core::export_package_metadata::{
    PackageMetadata, EXPORT_PACKAGE_METADATA_SCHEMA_VERSION,
    LOCAL_DISTRIBUTION_DESCRIPTOR_SCHEMA_VERSION,
};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn fixture(rel: &str) -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/export-package-metadata-v1")
            .join(rel),
    )
    .unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn valid_metadata_generates_local_descriptor() {
    let metadata = PackageMetadata::from_json_str(&fixture("package-metadata.valid.fixture.json"))
        .expect("valid metadata parses");
    assert_eq!(
        metadata.schema_version,
        EXPORT_PACKAGE_METADATA_SCHEMA_VERSION
    );
    let descriptor = metadata.to_local_descriptor();
    assert_eq!(
        descriptor.schema_version,
        LOCAL_DISTRIBUTION_DESCRIPTOR_SCHEMA_VERSION
    );
    assert_eq!(descriptor.name, "Collect and Exit");
    assert_eq!(descriptor.version, "0.1.0");
    assert_eq!(descriptor.project_id, "proj_export_bundle_fixture");
    assert_eq!(descriptor.entry, "scene/main.json");
    // The descriptor never authorizes publishing.
    assert_eq!(descriptor.distribution, "local");
    let json = descriptor.to_json().unwrap();
    assert!(json.contains("\"distribution\": \"local\""));
}

#[test]
fn missing_required_metadata_is_rejected() {
    let err = PackageMetadata::from_json_str(&fixture("invalid/missing-title.json"))
        .expect_err("missing title rejected");
    assert!(err.to_string().contains("title"));
}

#[test]
fn store_release_signing_credential_fields_are_blocked() {
    let err = PackageMetadata::from_json_str(&fixture("invalid/signing-field.json"))
        .expect_err("signing field rejected");
    assert!(err
        .to_string()
        .contains("failed to parse Package Metadata JSON"));
}

#[test]
fn forbidden_publish_wording_is_rejected() {
    let mut value: serde_json::Value =
        serde_json::from_str(&fixture("package-metadata.valid.fixture.json")).unwrap();
    value["description"] = serde_json::json!("Production-ready app store release.");
    let err =
        PackageMetadata::from_json_str(&value.to_string()).expect_err("publish wording rejected");
    assert!(err.to_string().contains("local package only"));
}

#[test]
fn metadata_round_trips() {
    let metadata =
        PackageMetadata::from_json_str(&fixture("package-metadata.valid.fixture.json")).unwrap();
    let json = metadata.to_json().unwrap();
    assert_eq!(PackageMetadata::from_json_str(&json).unwrap(), metadata);
}
