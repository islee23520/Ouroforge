//! Scenario Coverage v15: Build / Export / Packaging regression suite (#735).
//!
//! This file composes the existing export-profile, plan, bundle, asset-manifest,
//! probe, staging, and target-matrix contracts into a single fixture-scoped
//! scenario matrix. It adds no publish/deploy/sign/upload authority.

use ouroforge_core::export_asset_manifest::{build_asset_manifest, AssetManifest};
use ouroforge_core::export_bundle::assemble_web_bundle;
use ouroforge_core::export_hash::sha256_prefixed;
use ouroforge_core::export_plan::ExportPlan;
use ouroforge_core::export_probe_check::{check_bundle_probe, check_probe_source, ExportProbeMode};
use ouroforge_core::export_profile::ExportProfile;
use ouroforge_core::export_staging::{
    is_ignored_by_default, is_within_staging_root, staging_dir_for_run, GENERATED_ARTIFACT_KINDS,
};
use serde_json::Value;
use std::path::{Path, PathBuf};

const COVERAGE_DOC: &str =
    include_str!("../../../docs/scenario-coverage-v15-build-export-packaging.md");
const COVERAGE_FIXTURE: &str = include_str!(
    "../../../examples/build-export-packaging-regression-v15/coverage-matrix.fixture.json"
);
const BUNDLE_PROFILE: &str =
    include_str!("../../../examples/export-bundle-v1/export-profile.fixture.json");
const VALID_ASSET_MANIFEST: &str =
    include_str!("../../../examples/export-asset-manifest-v1/asset-manifest.valid.fixture.json");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn unique_staging(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ouroforge-scenario-coverage-v15-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn coverage_fixture() -> Value {
    serde_json::from_str(COVERAGE_FIXTURE).expect("coverage fixture parses")
}

fn profile_with_target(target: &str) -> String {
    let mut value: Value = serde_json::from_str(BUNDLE_PROFILE).expect("bundle profile parses");
    value["exportTarget"] = Value::String(target.to_string());
    value.to_string()
}

#[test]
fn scenario_coverage_v15_matrix_declares_success_and_blocked_cases() {
    let fixture = coverage_fixture();
    assert_eq!(
        fixture["schemaVersion"],
        "scenario-coverage-v15-build-export-packaging-v1"
    );
    assert_eq!(fixture["issue"], 735);
    let scenarios = fixture["scenarios"]
        .as_array()
        .expect("scenarios are an array");
    let ids: Vec<&str> = scenarios
        .iter()
        .map(|scenario| scenario["id"].as_str().expect("scenario id"))
        .collect();
    for required in [
        "BEP15.success-local-web-export",
        "BEP15.success-asset-manifest",
        "BEP15.success-probe-preserved",
        "BEP15.success-verification-pass",
        "BEP15.success-evidence-bundle",
        "BEP15.success-read-only-inspection",
        "BEP15.block-missing-asset",
        "BEP15.block-unsafe-path",
        "BEP15.block-publish-target",
        "BEP15.block-missing-probe",
        "BEP15.block-dirty-generated-output",
        "BEP15.block-checksum-mismatch",
        "BEP15.block-desktop-mobile-store-target",
    ] {
        assert!(ids.contains(&required), "missing scenario {required}");
        assert!(COVERAGE_DOC.contains(required), "doc missing {required}");
    }

    let success = scenarios
        .iter()
        .filter(|scenario| scenario["kind"] == "success")
        .count();
    let blocked = scenarios
        .iter()
        .filter(|scenario| scenario["kind"] == "blocked")
        .count();
    assert!(success >= 6, "expected success scenarios, got {success}");
    assert!(blocked >= 7, "expected blocked scenarios, got {blocked}");
}

#[test]
fn scenario_coverage_v15_documents_boundaries_and_governance() {
    let lower_doc = COVERAGE_DOC.to_ascii_lowercase();
    let lower_fixture = COVERAGE_FIXTURE.to_ascii_lowercase();
    for term in [
        "read-only",
        "fixture-scoped",
        "generated",
        "no publish",
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
            lower_fixture.contains(term) || term == "credential",
            "fixture missing guardrail {term}"
        );
    }
    assert!(COVERAGE_DOC.contains("#1 remains open"));
    assert!(COVERAGE_DOC.contains("#23 remains open"));
}

#[test]
fn scenario_coverage_v15_success_path_builds_local_bundle_manifest_probe_and_staging_evidence() {
    let plan = ExportPlan::from_profile_json(BUNDLE_PROFILE).expect("bundle profile plans");
    let staging = unique_staging("success");
    let report = assemble_web_bundle(&plan, &repo_root(), &staging).expect("bundle assembles");

    for expected in [
        "index.html",
        "styles.css",
        "runtime/bootstrap.js",
        "scene/main.json",
    ] {
        assert!(report.package_files.iter().any(|path| path == expected));
        assert!(report.bundle_root.join(expected).is_file());
    }
    assert_eq!(report.probe_mode, "preserve");
    assert_eq!(report.asset_files, vec!["assets/assets/sprites/hero.json"]);

    let manifest = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture")
        .expect("asset manifest builds");
    assert_eq!(manifest.entries.len(), 1);
    let entry = &manifest.entries[0];
    assert_eq!(entry.output_path, "assets/assets/sprites/hero.json");
    assert!(entry.content_hash.starts_with("sha256:"));
    assert_eq!(entry.content_hash.len(), "sha256:".len() + 64);
    assert_eq!(
        manifest.rewrite_reference(&entry.source_path),
        Some(entry.output_path.as_str())
    );

    for mode in [
        ExportProbeMode::DevProbeEnabled,
        ExportProbeMode::PackagedProbeLimited,
    ] {
        let probe_report = check_bundle_probe(&report.bundle_root, mode).expect("probe checks");
        assert!(probe_report.global_present);
        assert!(
            probe_report.passed,
            "missing methods: {:?}",
            probe_report.missing_methods
        );
    }

    let staging_policy_path = staging_dir_for_run("scenario_coverage_v15_success").unwrap();
    assert!(is_within_staging_root(&staging_policy_path));
    assert!(is_ignored_by_default(&staging_policy_path));
    assert!(GENERATED_ARTIFACT_KINDS.contains(&"bundle-output"));
    assert!(GENERATED_ARTIFACT_KINDS.contains(&"checksums"));
    assert!(GENERATED_ARTIFACT_KINDS.contains(&"verification-log"));

    let verification_ids: Vec<&str> = plan
        .verification_steps
        .iter()
        .map(|step| step.id.as_str())
        .collect();
    assert!(verification_ids.contains(&"load-without-console-errors"));
    assert!(verification_ids.contains(&"runtime-probe-compatibility"));
    assert!(verification_ids
        .iter()
        .any(|id| id.starts_with("scenario-smoke:")));

    std::fs::remove_dir_all(&staging).ok();
}

#[test]
fn scenario_coverage_v15_failure_cases_fail_closed_with_actionable_diagnostics() {
    let missing_asset_profile = std::fs::read_to_string(
        repo_root().join("examples/export-asset-manifest-v1/missing-asset-profile.fixture.json"),
    )
    .expect("missing asset fixture reads");
    let missing_plan =
        ExportPlan::from_profile_json(&missing_asset_profile).expect("profile plans");
    let missing_asset =
        build_asset_manifest(&missing_plan, &repo_root(), "proj_export_bundle_fixture")
            .expect_err("missing asset root blocks manifest");
    assert!(missing_asset.to_string().contains("missing asset root"));

    let mut forged_plan = ExportPlan::from_profile_json(BUNDLE_PROFILE).expect("profile plans");
    forged_plan
        .source_inputs
        .push(ouroforge_core::export_plan::PlannedInput {
            kind: ouroforge_core::export_plan::PlannedInputKind::AssetRoot,
            path: "target/debug/leak".to_string(),
        });
    let blocked_staging = unique_staging("blocked-source");
    let blocked_source = assemble_web_bundle(&forged_plan, &repo_root(), &blocked_staging)
        .expect_err("blocked generated source path rejected");
    assert!(blocked_source.to_string().contains("blocked source path"));
    std::fs::remove_dir_all(&blocked_staging).ok();

    for target in [
        "steam",
        "itch",
        "hosted-deploy",
        "signed-release",
        "ci-release",
        "desktop-wrapper",
        "mobile",
        "console",
        "app-store",
    ] {
        let err = ExportProfile::from_json_str(&profile_with_target(target))
            .expect_err("blocked/future target rejects");
        let msg = err.to_string();
        assert!(
            msg.contains("blocked") || msg.contains("future/design-gated"),
            "target {target} diagnostic was not actionable: {msg}"
        );
    }

    let missing_probe = std::fs::read_to_string(
        repo_root().join("examples/export-probe-v1/invalid/no-probe-global-bootstrap.js"),
    )
    .expect("probe fixture reads");
    let missing_probe_report =
        check_probe_source(&missing_probe, ExportProbeMode::PackagedProbeLimited);
    assert!(!missing_probe_report.global_present);
    assert!(!missing_probe_report.passed);

    let missing_method = std::fs::read_to_string(
        repo_root().join("examples/export-probe-v1/invalid/missing-getevents-bootstrap.js"),
    )
    .expect("probe fixture reads");
    let missing_method_report =
        check_probe_source(&missing_method, ExportProbeMode::DevProbeEnabled);
    assert!(!missing_method_report.passed);
    assert!(missing_method_report
        .missing_methods
        .contains(&"getEvents".to_string()));

    let valid_manifest = AssetManifest::from_json_str(VALID_ASSET_MANIFEST).unwrap();
    let declared_fixture_hash = &valid_manifest.entries[0].content_hash;
    let observed_fixture_hash = sha256_prefixed(
        &std::fs::read(
            repo_root().join("examples/export-bundle-v1/fixture-game/assets/sprites/hero.json"),
        )
        .expect("fixture asset reads"),
    );
    assert_ne!(
        declared_fixture_hash, &observed_fixture_hash,
        "fixture intentionally models checksum drift for the blocked scenario"
    );
    let malformed =
        VALID_ASSET_MANIFEST.replace(declared_fixture_hash, "sha256:not-a-64-hex-digest");
    let checksum_err =
        AssetManifest::from_json_str(&malformed).expect_err("malformed checksum fails closed");
    assert!(checksum_err.to_string().contains("sha256:<64 hex>"));
}
