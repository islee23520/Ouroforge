//! Export Verification Runner v1 contract (#728).

use ouroforge_core::export_bundle::assemble_web_bundle;
use ouroforge_core::export_plan::ExportPlan;
use ouroforge_core::export_probe_check::ExportProbeMode;
use ouroforge_core::export_verification::{
    command_is_allowlisted, ensure_export_verified, verify_export_bundle, CheckStatus,
    EXPORT_VERIFICATION_BOUNDARY, EXPORT_VERIFICATION_SCHEMA_VERSION,
};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn plan() -> ExportPlan {
    let json = std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .unwrap();
    ExportPlan::from_profile_json(&json).unwrap()
}

fn staging(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ouroforge-verify-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn assembled(name: &str) -> PathBuf {
    let dir = staging(name);
    assemble_web_bundle(&plan(), &repo_root(), &dir).unwrap();
    dir
}

#[test]
fn assembled_bundle_passes_verification() {
    let bundle = assembled("ok");
    let report = verify_export_bundle(&plan(), &bundle, ExportProbeMode::DevProbeEnabled);
    assert_eq!(report.schema_version, EXPORT_VERIFICATION_SCHEMA_VERSION);
    assert!(report.passed(), "checks: {:?}", report.checks);
    assert!(report.checks.iter().all(|c| c.status != CheckStatus::Fail));
    // Key checks present.
    for id in [
        "target-allowed",
        "bundle-present",
        "scene-loads",
        "runtime-probe-compatibility",
        "no-unsafe-runtime-surface",
        "scenario-smoke",
    ] {
        assert!(
            report.checks.iter().any(|c| c.id == id),
            "missing check {id}"
        );
    }
    ensure_export_verified(&plan(), &bundle, ExportProbeMode::DevProbeEnabled).expect("verified");
    std::fs::remove_dir_all(&bundle).ok();
}

#[test]
fn scenario_smoke_is_skipped_not_passed_by_static_runner() {
    // The static runner cannot execute declared scenarios, so it must not
    // report scenario-smoke as a Pass derived only from bundle presence (#728).
    let bundle = assembled("scenario-skip");
    let report = verify_export_bundle(&plan(), &bundle, ExportProbeMode::DevProbeEnabled);
    let scenario = report
        .checks
        .iter()
        .find(|c| c.id == "scenario-smoke")
        .expect("scenario-smoke check present");
    assert_eq!(
        scenario.status,
        CheckStatus::Skipped,
        "scenario-smoke must not assert Pass without a validated scenario result: {scenario:?}"
    );
    // The overall report is still not a Fail (Skipped does not fail closed).
    assert!(report.checks.iter().all(|c| c.status != CheckStatus::Fail));
    std::fs::remove_dir_all(&bundle).ok();
}

#[test]
fn missing_bundle_fails_closed() {
    let missing = staging("missing");
    let report = verify_export_bundle(&plan(), &missing, ExportProbeMode::DevProbeEnabled);
    assert!(!report.passed());
    let err = ensure_export_verified(&plan(), &missing, ExportProbeMode::DevProbeEnabled)
        .expect_err("missing bundle fails");
    assert!(err.to_string().contains("bundle-present"));
}

#[test]
fn blocked_target_fails_closed() {
    let bundle = assembled("blocked");
    let mut p = plan();
    p.export_target = "steam".to_string();
    let report = verify_export_bundle(&p, &bundle, ExportProbeMode::DevProbeEnabled);
    assert!(!report.passed());
    assert!(report
        .checks
        .iter()
        .any(|c| c.id == "target-allowed" && c.status == CheckStatus::Fail));
    std::fs::remove_dir_all(&bundle).ok();
}

#[test]
fn missing_probe_fails_closed() {
    let bundle = assembled("noprobe");
    // Corrupt the runtime so the probe surface disappears.
    std::fs::write(bundle.join("runtime/bootstrap.js"), "// no probe here\n").unwrap();
    let report = verify_export_bundle(&plan(), &bundle, ExportProbeMode::DevProbeEnabled);
    assert!(!report.passed());
    assert!(report
        .checks
        .iter()
        .any(|c| c.id == "runtime-probe-compatibility" && c.status == CheckStatus::Fail));
    std::fs::remove_dir_all(&bundle).ok();
}

#[test]
fn unloadable_scene_fails_closed() {
    let bundle = assembled("badscene");
    // Find the scene file and corrupt its JSON.
    let scene_dir = bundle.join("scene");
    let scene = std::fs::read_dir(&scene_dir)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .find(|p| p.extension().and_then(|x| x.to_str()) == Some("json"))
        .unwrap();
    std::fs::write(&scene, "{ not valid json").unwrap();
    let report = verify_export_bundle(&plan(), &bundle, ExportProbeMode::DevProbeEnabled);
    assert!(!report.passed());
    assert!(report
        .checks
        .iter()
        .any(|c| c.id == "scene-loads" && c.status == CheckStatus::Fail));
    std::fs::remove_dir_all(&bundle).ok();
}

#[test]
fn command_allowlist_is_inert_policy() {
    assert!(command_is_allowlisted(&[
        "node",
        "--check",
        "runtime/bootstrap.js"
    ]));
    assert!(!command_is_allowlisted(&["curl", "https://example.com"]));
    assert!(!command_is_allowlisted(&["rm", "-rf", "/"]));
    assert!(EXPORT_VERIFICATION_BOUNDARY.contains("does not execute"));
}

#[test]
fn report_serializes_as_evidence() {
    let bundle = assembled("evidence");
    let report = verify_export_bundle(&plan(), &bundle, ExportProbeMode::PackagedProbeLimited);
    let json = report.to_json().unwrap();
    assert!(json.contains("\"verdict\": \"pass\""));
    assert!(json.contains("runtime-probe-compatibility"));
    std::fs::remove_dir_all(&bundle).ok();
}
