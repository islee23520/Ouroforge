//! Export Evidence Bundle v1 contract (#729).

use ouroforge_core::export_asset_manifest::build_asset_manifest;
use ouroforge_core::export_bundle::assemble_web_bundle;
use ouroforge_core::export_evidence::{
    build_export_evidence, write_export_evidence, ExportEvidenceBundle, ExportVerdict,
    EXPORT_EVIDENCE_FILE, EXPORT_EVIDENCE_SCHEMA_VERSION,
};
use ouroforge_core::export_fingerprint::build_fingerprint;
use ouroforge_core::export_plan::ExportPlan;
use ouroforge_core::export_probe_check::ExportProbeMode;
use ouroforge_core::export_profile::ExportProfile;
use ouroforge_core::export_verification::verify_export_bundle;
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
    let dir =
        std::env::temp_dir().join(format!("ouroforge-evidence-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn evidence(run_id: &str, bundle_dir: &Path) -> ExportEvidenceBundle {
    let profile = profile();
    let plan = ExportPlan::from_profile(&profile).unwrap();
    let manifest = build_asset_manifest(&plan, &repo_root(), &profile.project_id).unwrap();
    assemble_web_bundle(&plan, &repo_root(), bundle_dir).unwrap();
    let fingerprint = build_fingerprint(
        &profile,
        &plan,
        &manifest,
        bundle_dir,
        "ouroforge-core-0.1.0",
    )
    .unwrap();
    let verification = verify_export_bundle(&plan, bundle_dir, ExportProbeMode::DevProbeEnabled);
    build_export_evidence(
        run_id,
        "",
        profile,
        plan,
        manifest,
        fingerprint,
        verification,
        vec![],
        vec![],
    )
    .unwrap()
}

#[test]
fn aggregates_all_export_artifacts_with_pass_verdict() {
    let dir = staging("agg");
    let bundle = evidence("run_demo", &dir);
    assert_eq!(bundle.schema_version, EXPORT_EVIDENCE_SCHEMA_VERSION);
    assert_eq!(bundle.links.run_id, "run_demo");
    assert_eq!(bundle.links.project_id, "proj_export_bundle_fixture");
    assert_eq!(bundle.verdict, ExportVerdict::Pass);
    assert!(!bundle.fingerprint.artifact_checksums.is_empty());
    assert_eq!(bundle.asset_manifest.entries.len(), 1);
    assert!(bundle.verification.passed());
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn round_trips_and_exposes_read_model() {
    let dir = staging("rt");
    let bundle = evidence("run_rt", &dir);
    let json = bundle.to_json().unwrap();
    assert_eq!(ExportEvidenceBundle::from_json_str(&json).unwrap(), bundle);

    let rm = bundle.read_model();
    assert_eq!(rm.run_id, "run_rt");
    assert_eq!(rm.export_target, "web-static-bundle");
    assert_eq!(rm.verdict, ExportVerdict::Pass);
    assert_eq!(rm.asset_count, 1);
    assert!(rm.artifact_count > 0);
    // Read model serializes for dashboard/Studio consumption.
    let rm_json = bundle.read_model_json().unwrap();
    assert!(rm_json.contains("\"verdict\": \"pass\""));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn writes_evidence_under_ignored_staging_root() {
    let dir = staging("write-bundle");
    let bundle = evidence("run_write", &dir);
    // Use a temp repo root so the staging path is created under it and cleaned.
    let fake_repo = staging("write-repo");
    std::fs::create_dir_all(&fake_repo).unwrap();
    let path = write_export_evidence(&bundle, &fake_repo).unwrap();
    assert!(path.ends_with(EXPORT_EVIDENCE_FILE));
    assert!(path.is_file());
    // Lives under target/ouroforge/exports/<run-id>/ (ignored by default).
    let rel = path.strip_prefix(&fake_repo).unwrap().to_string_lossy();
    assert!(
        rel.starts_with("target/ouroforge/exports/run_write/"),
        "rel={rel}"
    );
    std::fs::remove_dir_all(&dir).ok();
    std::fs::remove_dir_all(&fake_repo).ok();
}

#[test]
fn rejects_project_id_mismatch() {
    let dir = staging("mismatch");
    let mut bundle = evidence("run_mm", &dir);
    bundle.links.project_id = "proj_other".to_string();
    let err = bundle.validate().expect_err("project mismatch rejected");
    assert!(err.to_string().contains("projectId"));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn rejects_verdict_inconsistent_with_verification() {
    let dir = staging("verdict");
    let mut bundle = evidence("run_vd", &dir);
    bundle.verdict = ExportVerdict::Fail; // verification passed -> inconsistent
    let err = bundle
        .validate()
        .expect_err("inconsistent verdict rejected");
    assert!(err.to_string().contains("verdict"));
    std::fs::remove_dir_all(&dir).ok();
}
