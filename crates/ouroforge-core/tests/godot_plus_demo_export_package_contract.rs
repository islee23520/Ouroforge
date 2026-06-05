//! Godot-Plus demo exported playable package contract (#791).
//!
//! Walks the existing Build / Export / Packaging pipeline over the canonical
//! collect-and-exit demo export profile and asserts a local web package is
//! produced and verified with a pass verdict, content-hashed asset manifest,
//! build fingerprint/checksums, runtime probe compatibility, export verification,
//! and an evidence bundle. The package is written under an ignored temp staging
//! dir and removed; the flow is local-only and adds no publish/deploy/sign/upload
//! authority.

use ouroforge_core::export_asset_manifest::build_asset_manifest;
use ouroforge_core::export_bundle::assemble_web_bundle;
use ouroforge_core::export_evidence::{build_export_evidence, ExportEvidenceBundle, ExportVerdict};
use ouroforge_core::export_fingerprint::build_fingerprint;
use ouroforge_core::export_plan::ExportPlan;
use ouroforge_core::export_probe_check::{check_bundle_probe, ExportProbeMode};
use ouroforge_core::export_profile::ExportProfile;
use ouroforge_core::export_verification::verify_export_bundle;
use serde_json::Value;
use std::path::{Path, PathBuf};

const DEMO_PROFILE: &str =
    include_str!("../../../examples/playable-demo-v2/collect-and-exit/export/export-profile.json");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn staging() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ouroforge-godot-plus-demo-export-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn demo_export_profile_is_a_local_web_target() {
    let profile = ExportProfile::from_json_str(DEMO_PROFILE).expect("demo profile parses");
    assert_eq!(profile.export_target, "web-local");
    assert!(
        profile.target_is_allowed(),
        "demo export target must be allowed"
    );
    assert!(
        profile.output_dir.starts_with("dist/"),
        "export output must land in an ignored staging root"
    );
}

#[test]
fn demo_exports_to_local_web_package_with_pass_verdict() {
    let dir = staging();
    let profile = ExportProfile::from_json_str(DEMO_PROFILE).expect("demo profile parses");
    let plan = ExportPlan::from_profile(&profile).expect("profile plans");

    // Local web bundle under ignored staging.
    let report = assemble_web_bundle(&plan, &repo_root(), &dir).expect("bundle assembles");
    for expected in ["index.html", "runtime/bootstrap.js"] {
        assert!(
            report.bundle_root.join(expected).is_file(),
            "missing packaged file {expected}"
        );
    }

    // Asset manifest with sha256 content hashes.
    let manifest =
        build_asset_manifest(&plan, &repo_root(), &profile.project_id).expect("manifest builds");
    assert!(
        !manifest.entries.is_empty(),
        "package asset manifest is non-empty"
    );
    for entry in &manifest.entries {
        assert!(
            entry.content_hash.starts_with("sha256:"),
            "asset content hash must be sha256"
        );
    }

    // Build fingerprint / checksums.
    let fingerprint = build_fingerprint(
        &profile,
        &plan,
        &manifest,
        &report.bundle_root,
        "ouroforge-core-0.1.0",
    )
    .expect("fingerprint builds");
    assert!(
        !fingerprint.artifact_checksums.is_empty(),
        "fingerprint records artifact checksums"
    );

    // Runtime probe compatibility survives packaging in both modes.
    for mode in [
        ExportProbeMode::DevProbeEnabled,
        ExportProbeMode::PackagedProbeLimited,
    ] {
        let probe = check_bundle_probe(&report.bundle_root, mode).expect("probe checks");
        assert!(probe.global_present, "probe global missing in {mode:?}");
        assert!(
            probe.passed,
            "probe failed in {mode:?}: {:?}",
            probe.missing_methods
        );
    }

    // Export verification (allowlisted local report).
    let verification =
        verify_export_bundle(&plan, &report.bundle_root, ExportProbeMode::DevProbeEnabled);
    assert!(verification.passed(), "export verification did not pass");

    // Evidence bundle aggregates the chain with a pass verdict.
    let bundle: ExportEvidenceBundle = build_export_evidence(
        "run_godot_plus_demo_791",
        "",
        profile,
        plan,
        manifest,
        fingerprint,
        verification,
        vec![],
        vec![],
    )
    .expect("evidence bundle builds");
    assert_eq!(bundle.verdict, ExportVerdict::Pass);

    // Read-only evidence: no apply/publish/deploy command surface.
    let read_model_json = bundle.read_model_json().expect("read model serializes");
    assert!(read_model_json.contains("\"verdict\": \"pass\""));
    let read_model: Value = serde_json::from_str(&read_model_json).expect("read model parses");
    assert!(
        read_model.get("publishCommand").is_none(),
        "no publish command"
    );
    assert!(
        read_model.get("deployCommand").is_none(),
        "no deploy command"
    );
    assert!(read_model.get("applyCommand").is_none(), "no apply command");

    std::fs::remove_dir_all(&dir).ok();
}
