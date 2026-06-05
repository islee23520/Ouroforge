//! Release / Publish Blocker v1 contract (#733).

use ouroforge_core::export_release_blocker::{
    audit_local_export_wording, ensure_no_publish_config, scan_for_publish_fields,
    BLOCKED_PUBLISH_KEY_TERMS, EXPORT_RELEASE_BLOCKER_BOUNDARY,
};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn valid_export_profile_has_no_publish_fields() {
    let profile = read("examples/export-bundle-v1/export-profile.fixture.json");
    ensure_no_publish_config(&profile).expect("clean profile passes the blocker");
}

#[test]
fn publish_config_fails_closed() {
    let cfg = read("examples/export-release-blocker-v1/invalid/publish-config.json");
    let err = ensure_no_publish_config(&cfg).expect_err("publish config blocked");
    assert!(err.to_string().contains("publish"));
}

#[test]
fn signing_deploy_upload_credentials_fail_closed() {
    let cfg = read("examples/export-release-blocker-v1/invalid/signing-deploy-config.json");
    let value: serde_json::Value = serde_json::from_str(&cfg).unwrap();
    let hits = scan_for_publish_fields(&value);
    for expected in [
        "release",
        "release.deployTarget",
        "release.signingKey",
        "release.uploadUrl",
    ] {
        assert!(
            hits.iter().any(|h| h == expected),
            "missing {expected} in {hits:?}"
        );
    }
    assert!(ensure_no_publish_config(&cfg).is_err());
}

#[test]
fn wording_audit_keeps_claims_local() {
    audit_local_export_wording("A local web export bundle for inspection and QA.")
        .expect("local wording passes");
    for bad in [
        "This is production-ready.",
        "Ready for public release on the app store.",
        "A secure distribution and Godot replacement.",
    ] {
        assert!(
            audit_local_export_wording(bad).is_err(),
            "should block: {bad}"
        );
    }
}

#[test]
fn blocker_covers_release_publish_field_families() {
    // Upload, deploy, signing, credentials, store, hosted, and release fields
    // are all covered by the blocked-key term list.
    for term in [
        "publish",
        "deploy",
        "signing",
        "upload",
        "credential",
        "store",
        "hosted",
        "release",
    ] {
        assert!(
            BLOCKED_PUBLISH_KEY_TERMS.contains(&term),
            "blocker must cover `{term}`"
        );
    }
}

#[test]
fn boundary_preserves_command_write_safety() {
    assert!(EXPORT_RELEASE_BLOCKER_BOUNDARY.contains("does not execute commands"));
    assert!(EXPORT_RELEASE_BLOCKER_BOUNDARY.contains("blocked"));
}
