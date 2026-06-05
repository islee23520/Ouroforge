//! Build / Export / Packaging Demo v1 (#734).
//!
//! Walks the full v1 build/export/packaging flow over one small fixture game:
//! export profile, export plan, local web bundle, asset manifest, build
//! fingerprint/checksums, runtime probe check, export verification, and evidence
//! bundle. The demo is local-only and evidence-backed; it composes the existing
//! export pipeline read-only and adds no publish/deploy/sign/upload authority.

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

const DEMO_DOC: &str = include_str!("../../../docs/build-export-packaging-demo-v1.md");
const DEMO_FIXTURE: &str =
    include_str!("../../../examples/build-export-packaging-demo-v1/demo-manifest.fixture.json");

const STAGE_IDS: &[&str] = &[
    "BEP734.profile",
    "BEP734.plan",
    "BEP734.web-bundle",
    "BEP734.asset-manifest",
    "BEP734.fingerprint-checksums",
    "BEP734.probe-check",
    "BEP734.verification",
    "BEP734.evidence-bundle",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn staging(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ouroforge-build-export-demo-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn fixture_profile() -> ExportProfile {
    let json = std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .expect("fixture profile reads");
    ExportProfile::from_json_str(&json).expect("fixture profile parses")
}

fn demo_manifest() -> Value {
    serde_json::from_str(DEMO_FIXTURE).expect("demo manifest parses")
}

#[test]
fn build_export_demo_manifest_declares_every_stage() {
    let manifest = demo_manifest();
    assert_eq!(manifest["schemaVersion"], "build-export-packaging-demo-v1");
    assert_eq!(manifest["issue"], 734);
    let stages = manifest["stages"].as_array().expect("stages are an array");
    let ids: Vec<&str> = stages
        .iter()
        .map(|stage| stage["id"].as_str().expect("stage id"))
        .collect();
    for required in STAGE_IDS {
        assert!(ids.contains(required), "manifest missing stage {required}");
        assert!(DEMO_DOC.contains(required), "doc missing stage {required}");
    }
    assert_eq!(stages.len(), STAGE_IDS.len());
}

#[test]
fn build_export_demo_documents_boundaries_and_governance() {
    let lower_doc = DEMO_DOC.to_ascii_lowercase();
    let lower_fixture = DEMO_FIXTURE.to_ascii_lowercase();
    for term in [
        "read-only",
        "fixture-scoped",
        "generated",
        "deploy",
        "sign",
        "upload",
        "credential",
        "command bridge",
        "production-ready",
        "godot replacement",
    ] {
        assert!(lower_doc.contains(term), "doc missing guardrail {term}");
        assert!(
            lower_fixture.contains(term),
            "fixture missing guardrail {term}"
        );
    }
    assert!(DEMO_DOC.contains("#1 remains open"));
    assert!(DEMO_DOC.contains("#23 remains open"));
}

#[test]
fn build_export_demo_exports_fixture_to_local_web_package_with_pass_verdict() {
    let dir = staging("e2e");
    let profile = fixture_profile();
    let plan = ExportPlan::from_profile(&profile).expect("profile plans");

    // Local web bundle under ignored staging.
    let report = assemble_web_bundle(&plan, &repo_root(), &dir).expect("bundle assembles");
    for expected in ["index.html", "runtime/bootstrap.js"] {
        assert!(
            report.bundle_root.join(expected).is_file(),
            "missing packaged file {expected}"
        );
    }

    // Asset manifest with content hashes.
    let manifest =
        build_asset_manifest(&plan, &repo_root(), &profile.project_id).expect("manifest builds");
    assert!(!manifest.entries.is_empty());
    for entry in &manifest.entries {
        assert!(entry.content_hash.starts_with("sha256:"));
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
    assert!(!fingerprint.artifact_checksums.is_empty());

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
    assert!(verification.passed(), "verification did not pass");

    // Evidence bundle aggregates the chain with a pass verdict.
    let bundle: ExportEvidenceBundle = build_export_evidence(
        "run_demo_734",
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

    let read_model_json = bundle.read_model_json().expect("read model serializes");
    assert!(read_model_json.contains("\"verdict\": \"pass\""));
    // Read-only evidence: no apply/publish/deploy command surface.
    let read_model: Value = serde_json::from_str(&read_model_json).expect("read model parses");
    assert!(read_model.get("publishCommand").is_none());
    assert!(read_model.get("deployCommand").is_none());
    assert!(read_model.get("applyCommand").is_none());

    std::fs::remove_dir_all(&dir).ok();
}
