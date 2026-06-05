//! Build Fingerprint and Artifact Checksums v1 contract (#727).

use ouroforge_core::export_asset_manifest::build_asset_manifest;
use ouroforge_core::export_bundle::assemble_web_bundle;
use ouroforge_core::export_fingerprint::{
    build_fingerprint, BuildFingerprint, EXPORT_FINGERPRINT_SCHEMA_VERSION,
};
use ouroforge_core::export_plan::ExportPlan;
use ouroforge_core::export_profile::ExportProfile;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn profile() -> ExportProfile {
    let json = std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .unwrap();
    ExportProfile::from_json_str(&json).unwrap()
}

fn staging(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ouroforge-fp-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn fingerprint_in(dir: &Path) -> BuildFingerprint {
    let profile = profile();
    let plan = ExportPlan::from_profile(&profile).unwrap();
    let manifest = build_asset_manifest(&plan, &repo_root(), &profile.project_id).unwrap();
    assemble_web_bundle(&plan, &repo_root(), dir).unwrap();
    build_fingerprint(&profile, &plan, &manifest, dir, "ouroforge-core-0.1.0").unwrap()
}

#[test]
fn fingerprint_covers_all_inputs_and_artifacts() {
    let dir = staging("cover");
    let fp = fingerprint_in(&dir);
    assert_eq!(fp.schema_version, EXPORT_FINGERPRINT_SCHEMA_VERSION);
    for h in [&fp.profile_hash, &fp.plan_hash, &fp.manifest_hash] {
        assert!(h.starts_with("sha256:"));
        assert_eq!(h.len(), "sha256:".len() + 64);
    }
    assert_eq!(fp.runtime_version, "ouroforge-core-0.1.0");
    // Checksums cover every assembled artifact (index.html, styles.css,
    // runtime/bootstrap.js, scene + asset payload).
    assert!(fp
        .artifact_checksums
        .iter()
        .any(|c| c.package_path == "index.html"));
    assert!(fp
        .artifact_checksums
        .iter()
        .any(|c| c.package_path == "runtime/bootstrap.js"));
    assert!(fp.artifact_checksums.iter().all(|c| c.size > 0));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fingerprint_is_deterministic_across_runs() {
    let a = staging("det-a");
    let b = staging("det-b");
    let fa = fingerprint_in(&a);
    let fb = fingerprint_in(&b);
    // Same inputs -> identical fingerprint, including artifact checksums.
    assert_eq!(fa, fb);
    std::fs::remove_dir_all(&a).ok();
    std::fs::remove_dir_all(&b).ok();
}

#[test]
fn fingerprint_round_trips_as_evidence() {
    let dir = staging("rt");
    let fp = fingerprint_in(&dir);
    let json = fp.to_json().unwrap();
    assert_eq!(BuildFingerprint::from_json_str(&json).unwrap(), fp);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn bad_schema_version_is_rejected() {
    let dir = staging("bad");
    let fp = fingerprint_in(&dir);
    let mut value: serde_json::Value = serde_json::from_str(&fp.to_json().unwrap()).unwrap();
    value["schemaVersion"] = serde_json::json!("export-build-fingerprint-v2");
    let err = BuildFingerprint::from_json_str(&value.to_string()).expect_err("bad schema rejected");
    assert!(err.to_string().contains(EXPORT_FINGERPRINT_SCHEMA_VERSION));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn malformed_hash_is_rejected() {
    let dir = staging("hash");
    let fp = fingerprint_in(&dir);
    let mut value: serde_json::Value = serde_json::from_str(&fp.to_json().unwrap()).unwrap();
    value["profileHash"] = serde_json::json!("not-a-hash");
    let err = BuildFingerprint::from_json_str(&value.to_string()).expect_err("bad hash rejected");
    assert!(err.to_string().contains("sha256"));
    std::fs::remove_dir_all(&dir).ok();
}
