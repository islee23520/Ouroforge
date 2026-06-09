//! Consolidated export_*_contract.rs tests.
//!
//! This file merges all 13 export contract test files into one, deduplicating
//! shared helpers and disambiguating duplicate test function names.

// ---------------------------------------------------------------------------
// Imports (deduplicated, union of all files)
// ---------------------------------------------------------------------------

use ouroforge_core::export_asset_manifest::{
    build_asset_manifest, AssetManifest, EXPORT_ASSET_MANIFEST_SCHEMA_VERSION,
};
use ouroforge_core::export_bundle::assemble_web_bundle;
use ouroforge_core::export_capability_report::{
    CapabilityReport, CapabilityStatus, CAPABILITY_REPORT_SCHEMA_VERSION,
};
use ouroforge_core::export_evidence::{
    build_export_evidence, write_export_evidence, ExportEvidenceBundle, ExportVerdict,
    EXPORT_EVIDENCE_FILE, EXPORT_EVIDENCE_SCHEMA_VERSION,
};
use ouroforge_core::export_fingerprint::{
    build_fingerprint, BuildFingerprint, EXPORT_FINGERPRINT_SCHEMA_VERSION,
};
use ouroforge_core::export_package_metadata::{
    PackageMetadata, EXPORT_PACKAGE_METADATA_SCHEMA_VERSION,
    LOCAL_DISTRIBUTION_DESCRIPTOR_SCHEMA_VERSION,
};
use ouroforge_core::export_plan::{
    ExportPlan, PlannedInputKind, PlannedOutputKind, PlannedInput, EXPORT_PLAN_SCHEMA_VERSION,
};
use ouroforge_core::export_probe_check::{
    check_bundle_probe, check_probe_source, ensure_bundle_probe_compatible, ExportProbeMode,
    PROBE_GLOBAL,
};
use ouroforge_core::export_profile::{
    ExportProfile, RuntimeProbeMode, ALLOWED_EXPORT_TARGETS, BLOCKED_EXPORT_TARGETS,
    EXPORT_PROFILE_SCHEMA_VERSION, FUTURE_EXPORT_TARGETS,
};
use ouroforge_core::export_release_blocker::{
    audit_local_export_wording, ensure_no_publish_config, scan_for_publish_fields,
    BLOCKED_PUBLISH_KEY_TERMS, EXPORT_RELEASE_BLOCKER_BOUNDARY,
};
use ouroforge_core::export_staging::{
    is_ignored_by_default, is_within_staging_root, partition_stale_runs, staging_dir_for_run,
    EXPORT_STAGING_ROOT,
};
use ouroforge_core::export_verification::{
    command_is_allowlisted, ensure_export_verified, verify_export_bundle, CheckStatus,
    EXPORT_VERIFICATION_BOUNDARY, EXPORT_VERIFICATION_SCHEMA_VERSION,
};
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Shared helpers (defined once)
// ---------------------------------------------------------------------------

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

// ===========================================================================
// export_asset_manifest_contract (#724)
// ===========================================================================

fn manifest_fixture(rel: &str) -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/export-asset-manifest-v1")
            .join(rel),
    )
    .unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

fn bundle_profile() -> String {
    std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .expect("bundle profile readable")
}

#[test]
fn valid_declared_manifest_parses() {
    let manifest =
        AssetManifest::from_json_str(&manifest_fixture("asset-manifest.valid.fixture.json"))
            .expect("valid manifest parses");
    assert_eq!(
        manifest.schema_version,
        EXPORT_ASSET_MANIFEST_SCHEMA_VERSION
    );
    assert_eq!(manifest.entries.len(), 1);
    let entry = &manifest.entries[0];
    assert!(entry.content_hash.starts_with("sha256:"));
    assert_eq!(entry.url, "assets/assets/sprites/hero.json");
}

#[test]
fn duplicate_id_is_rejected() {
    let err = AssetManifest::from_json_str(&manifest_fixture("invalid/duplicate-id.json"))
        .expect_err("duplicate id rejected");
    assert!(err.to_string().contains("duplicate assetId"));
}

#[test]
fn path_traversal_is_rejected() {
    let err = AssetManifest::from_json_str(&manifest_fixture("invalid/path-traversal.json"))
        .expect_err("traversal rejected");
    assert!(err.to_string().contains(".."));
}

#[test]
fn absolute_path_is_rejected() {
    let err = AssetManifest::from_json_str(&manifest_fixture("invalid/absolute-path.json"))
        .expect_err("absolute path rejected");
    assert!(err.to_string().contains("relative"));
}

#[test]
fn output_collision_is_rejected() {
    let err = AssetManifest::from_json_str(&manifest_fixture("invalid/output-collision.json"))
        .expect_err("output collision rejected");
    assert!(err.to_string().contains("output collision"));
}

#[test]
fn builds_manifest_from_plan_with_real_hash_and_size() {
    let plan = ExportPlan::from_profile_json(&bundle_profile()).expect("plan");
    let manifest = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture")
        .expect("manifest builds from disk");
    assert_eq!(manifest.entries.len(), 1);
    let entry = &manifest.entries[0];
    assert_eq!(
        entry.source_path,
        "examples/export-bundle-v1/fixture-game/assets/sprites/hero.json"
    );
    assert_eq!(entry.output_path, "assets/assets/sprites/hero.json");
    assert!(entry.content_hash.starts_with("sha256:"));
    assert_eq!(entry.content_hash.len(), "sha256:".len() + 64);
    assert!(entry.size > 0);
    // Built manifest re-validates and round-trips through JSON.
    let json = manifest.to_json().unwrap();
    assert_eq!(AssetManifest::from_json_str(&json).unwrap(), manifest);
}

#[test]
fn build_is_deterministic() {
    let plan = ExportPlan::from_profile_json(&bundle_profile()).unwrap();
    let a = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture").unwrap();
    let b = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture").unwrap();
    assert_eq!(a, b);
}

#[test]
fn build_fails_closed_on_missing_asset_root() {
    let profile = std::fs::read_to_string(
        repo_root().join("examples/export-asset-manifest-v1/missing-asset-profile.fixture.json"),
    )
    .unwrap();
    let plan = ExportPlan::from_profile_json(&profile).expect("profile plans");
    let err = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture")
        .expect_err("missing asset root rejected");
    assert!(err.to_string().contains("missing asset root"));
}

#[test]
fn build_fails_closed_on_forged_traversal_asset_root() {
    // A plan can be deserialized/mutated outside ExportPlan::from_profile_json, so the
    // manifest builder must re-validate asset roots instead of trusting the plan. (#724)
    let mut plan = ExportPlan::from_profile_json(&bundle_profile()).expect("plan");
    let root = plan
        .source_inputs
        .iter_mut()
        .find(|input| input.kind == PlannedInputKind::AssetRoot)
        .expect("plan has an asset root");
    root.path = "../../../../../../etc".to_string();
    let err = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture")
        .expect_err("forged traversal asset root rejected");
    assert!(
        err.to_string().contains(".."),
        "expected traversal rejection, got: {err}"
    );
}

#[test]
fn build_fails_closed_on_blocked_prefix_asset_root() {
    let mut plan = ExportPlan::from_profile_json(&bundle_profile()).expect("plan");
    let root = plan
        .source_inputs
        .iter_mut()
        .find(|input| input.kind == PlannedInputKind::AssetRoot)
        .expect("plan has an asset root");
    // `secrets/` is in the plan's blocked-file policy; a forged root under it must fail.
    root.path = "secrets/leak".to_string();
    let err = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture")
        .expect_err("blocked-prefix asset root rejected");
    assert!(
        err.to_string().contains("blocked prefix"),
        "expected blocked-prefix rejection, got: {err}"
    );
}

#[test]
fn rewrites_only_explicitly_mapped_references() {
    let plan = ExportPlan::from_profile_json(&bundle_profile()).unwrap();
    let manifest = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture").unwrap();
    // Source path and output path both rewrite to the package URL.
    assert_eq!(
        manifest
            .rewrite_reference("examples/export-bundle-v1/fixture-game/assets/sprites/hero.json"),
        Some("assets/assets/sprites/hero.json")
    );
    assert_eq!(
        manifest.rewrite_reference("assets/assets/sprites/hero.json"),
        Some("assets/assets/sprites/hero.json")
    );
    // Unknown references are not rewritten.
    assert_eq!(manifest.rewrite_reference("assets/unknown.png"), None);
}

// ===========================================================================
// export_bundle_contract (#723)
// ===========================================================================

fn fixture_profile() -> String {
    std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .expect("fixture profile readable")
}

fn unique_staging(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ouroforge-export-bundle-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn bundle_plan() -> ExportPlan {
    ExportPlan::from_profile_json(&fixture_profile()).expect("fixture profile plans")
}

#[test]
fn assembles_runnable_bundle_from_plan() {
    let staging = unique_staging("runnable");
    let report =
        assemble_web_bundle(&bundle_plan(), &repo_root(), &staging).expect("bundle assembles");

    // Core runnable surface is present.
    for required in ["index.html", "styles.css", "runtime/bootstrap.js"] {
        assert!(
            report.package_files.iter().any(|p| p == required),
            "missing {required} in {:?}",
            report.package_files
        );
        assert!(report.bundle_root.join(required).is_file());
    }

    // Entry scene was copied into the bundle.
    assert_eq!(report.entry_scene_package_path, "scene/main.json");
    assert!(report.bundle_root.join("scene/main.json").is_file());

    // The single declared asset was copied; nothing undeclared appears.
    assert_eq!(
        report.asset_files,
        vec!["assets/assets/sprites/hero.json".to_string()]
    );
    assert!(report
        .bundle_root
        .join("assets/assets/sprites/hero.json")
        .is_file());

    std::fs::remove_dir_all(&staging).ok();
}

#[test]
fn bootstrap_preserves_probe_wiring() {
    let staging = unique_staging("probe");
    let report =
        assemble_web_bundle(&bundle_plan(), &repo_root(), &staging).expect("bundle assembles");
    let bootstrap = std::fs::read_to_string(report.bundle_root.join("runtime/bootstrap.js"))
        .expect("bootstrap readable");

    assert!(bootstrap.contains("__OUROFORGE__"));
    for method in [
        "getWorldState",
        "getFrameStats",
        "getEvents",
        "step",
        "pause",
        "resume",
        "setInput",
        "snapshot",
        "restore",
    ] {
        assert!(bootstrap.contains(method), "probe missing {method}");
    }
    // No publish/network/command surface leaks into the generated runtime.
    let lower = bootstrap.to_lowercase();
    for forbidden in ["xmlhttprequest", "websocket", "eval(", "child_process"] {
        assert!(!lower.contains(forbidden), "bootstrap leaks {forbidden}");
    }
    assert_eq!(report.probe_mode, "preserve");

    std::fs::remove_dir_all(&staging).ok();
}

#[test]
fn assembly_is_deterministic() {
    let a = unique_staging("det-a");
    let b = unique_staging("det-b");
    let ra = assemble_web_bundle(&bundle_plan(), &repo_root(), &a).unwrap();
    let rb = assemble_web_bundle(&bundle_plan(), &repo_root(), &b).unwrap();
    assert_eq!(ra.package_files, rb.package_files);
    assert_eq!(ra.asset_files, rb.asset_files);
    // Byte-identical generated runtime.
    let boot_a = std::fs::read(a.join("runtime/bootstrap.js")).unwrap();
    let boot_b = std::fs::read(b.join("runtime/bootstrap.js")).unwrap();
    assert_eq!(boot_a, boot_b);
    std::fs::remove_dir_all(&a).ok();
    std::fs::remove_dir_all(&b).ok();
}

#[test]
fn outputs_stay_inside_staging_root() {
    let staging = unique_staging("contained");
    let report =
        assemble_web_bundle(&bundle_plan(), &repo_root(), &staging).expect("bundle assembles");
    for rel in &report.package_files {
        let joined = report.bundle_root.join(rel);
        let resolved = joined.canonicalize().expect("written file resolves");
        assert!(
            resolved.starts_with(&report.bundle_root),
            "{rel} escaped staging root"
        );
    }
    std::fs::remove_dir_all(&staging).ok();
}

#[test]
fn refuses_blocked_source_path() {
    // A plan whose asset root points at a blocked generated-state prefix can only
    // be built by hand; the bundle must refuse it even if it reaches assembly.
    let mut p = bundle_plan();
    p.source_inputs
        .push(PlannedInput {
            kind: PlannedInputKind::AssetRoot,
            path: "target/debug/leak".to_string(),
        });
    let staging = unique_staging("blocked");
    let err =
        assemble_web_bundle(&p, &repo_root(), &staging).expect_err("blocked source path refused");
    assert!(err.to_string().contains("blocked source path"));
    std::fs::remove_dir_all(&staging).ok();
}

// ===========================================================================
// export_capability_report_contract (#732)
// ===========================================================================

const CAPABILITY_GATE_DOC: &str =
    include_str!("../../../docs/desktop-packaging-capability-gate-v1.md");

fn report_fixture() -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/desktop-packaging-capability-report-v1/capability-report.fixture.json"),
    )
    .expect("capability report fixture readable")
}

#[test]
fn capability_report_marks_desktop_packaging_not_implemented() {
    let report = CapabilityReport::from_json_str(&report_fixture()).expect("report parses");
    assert_eq!(report.schema_version, CAPABILITY_REPORT_SCHEMA_VERSION);
    assert_eq!(report.capability, "desktop-packaging");
    assert_eq!(report.status, CapabilityStatus::Future);
    assert!(report.is_not_implemented());
    assert!(!report.implemented);
    assert!(report.platforms.contains(&"windows".to_string()));
    assert!(report.platforms.contains(&"macos".to_string()));
    assert!(report.platforms.contains(&"linux".to_string()));
    assert!(!report.requirements.is_empty());
}

#[test]
fn report_cannot_claim_implemented() {
    let mut value: serde_json::Value = serde_json::from_str(&report_fixture()).unwrap();
    value["implemented"] = serde_json::json!(true);
    let err =
        CapabilityReport::from_json_str(&value.to_string()).expect_err("implemented=true rejected");
    assert!(err.to_string().contains("not mark"));
}

#[test]
fn export_validation_still_blocks_desktop_targets() {
    // desktop-wrapper is a declared future/design-gated target...
    assert!(FUTURE_EXPORT_TARGETS.contains(&"desktop-wrapper"));
    // ...and export profile validation still rejects it.
    let valid = std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&valid).unwrap();
    value["exportTarget"] = serde_json::json!("desktop-wrapper");
    let err = ExportProfile::from_json_str(&value.to_string())
        .expect_err("desktop-wrapper target rejected");
    let msg = err.to_string();
    assert!(msg.contains("desktop-wrapper"));
    assert!(msg.contains("future") || msg.contains("design-gated"));
}

#[test]
fn gate_doc_documents_requirements_and_governance() {
    assert!(CAPABILITY_GATE_DOC.contains("NOT implemented in v1"));
    assert!(CAPABILITY_GATE_DOC.to_lowercase().contains("windows"));
    assert!(CAPABILITY_GATE_DOC.to_lowercase().contains("signing"));
    assert!(CAPABILITY_GATE_DOC.to_lowercase().contains("notarization"));
    assert!(CAPABILITY_GATE_DOC.contains("desktop-wrapper"));
    assert!(CAPABILITY_GATE_DOC.contains("#1 remains"));
    assert!(CAPABILITY_GATE_DOC.contains("#23 remains"));
}

// ===========================================================================
// export_evidence_contract (#729)
// ===========================================================================

fn evidence_staging(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("ouroforge-evidence-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn evidence(run_id: &str, bundle_dir: &Path) -> ExportEvidenceBundle {
    let p = profile();
    let plan = ExportPlan::from_profile(&p).unwrap();
    let manifest = build_asset_manifest(&plan, &repo_root(), &p.project_id).unwrap();
    assemble_web_bundle(&plan, &repo_root(), bundle_dir).unwrap();
    let fingerprint = build_fingerprint(
        &p,
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
        p,
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
    let dir = evidence_staging("agg");
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
    let dir = evidence_staging("rt");
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
    let dir = evidence_staging("write-bundle");
    let bundle = evidence("run_write", &dir);
    // Use a temp repo root so the staging path is created under it and cleaned.
    let fake_repo = evidence_staging("write-repo");
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
    let dir = evidence_staging("mismatch");
    let mut bundle = evidence("run_mm", &dir);
    bundle.links.project_id = "proj_other".to_string();
    let err = bundle.validate().expect_err("project mismatch rejected");
    assert!(err.to_string().contains("projectId"));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn rejects_verdict_inconsistent_with_verification() {
    let dir = evidence_staging("verdict");
    let mut bundle = evidence("run_vd", &dir);
    bundle.verdict = ExportVerdict::Fail; // verification passed -> inconsistent
    let err = bundle
        .validate()
        .expect_err("inconsistent verdict rejected");
    assert!(err.to_string().contains("verdict"));
    std::fs::remove_dir_all(&dir).ok();
}

// ===========================================================================
// export_fingerprint_contract (#727)
// ===========================================================================

fn fingerprint_staging(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ouroforge-fp-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn fingerprint_in(dir: &Path) -> BuildFingerprint {
    let p = profile();
    let plan = ExportPlan::from_profile(&p).unwrap();
    let manifest = build_asset_manifest(&plan, &repo_root(), &p.project_id).unwrap();
    assemble_web_bundle(&plan, &repo_root(), dir).unwrap();
    build_fingerprint(&p, &plan, &manifest, dir, "ouroforge-core-0.1.0").unwrap()
}

#[test]
fn fingerprint_covers_all_inputs_and_artifacts() {
    let dir = fingerprint_staging("cover");
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
    let a = fingerprint_staging("det-a");
    let b = fingerprint_staging("det-b");
    let fa = fingerprint_in(&a);
    let fb = fingerprint_in(&b);
    // Same inputs -> identical fingerprint, including artifact checksums.
    assert_eq!(fa, fb);
    std::fs::remove_dir_all(&a).ok();
    std::fs::remove_dir_all(&b).ok();
}

#[test]
fn fingerprint_round_trips_as_evidence() {
    let dir = fingerprint_staging("rt");
    let fp = fingerprint_in(&dir);
    let json = fp.to_json().unwrap();
    assert_eq!(BuildFingerprint::from_json_str(&json).unwrap(), fp);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fingerprint_bad_schema_version_is_rejected() {
    let dir = fingerprint_staging("bad");
    let fp = fingerprint_in(&dir);
    let mut value: serde_json::Value = serde_json::from_str(&fp.to_json().unwrap()).unwrap();
    value["schemaVersion"] = serde_json::json!("export-build-fingerprint-v2");
    let err = BuildFingerprint::from_json_str(&value.to_string()).expect_err("bad schema rejected");
    assert!(err.to_string().contains(EXPORT_FINGERPRINT_SCHEMA_VERSION));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn malformed_hash_is_rejected() {
    let dir = fingerprint_staging("hash");
    let fp = fingerprint_in(&dir);
    let mut value: serde_json::Value = serde_json::from_str(&fp.to_json().unwrap()).unwrap();
    value["profileHash"] = serde_json::json!("not-a-hash");
    let err = BuildFingerprint::from_json_str(&value.to_string()).expect_err("bad hash rejected");
    assert!(err.to_string().contains("sha256"));
    std::fs::remove_dir_all(&dir).ok();
}

// ===========================================================================
// export_package_metadata_contract (#730)
// ===========================================================================

fn package_metadata_fixture(rel: &str) -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/export-package-metadata-v1")
            .join(rel),
    )
    .unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn valid_metadata_generates_local_descriptor() {
    let metadata = PackageMetadata::from_json_str(&package_metadata_fixture("package-metadata.valid.fixture.json"))
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
    let err = PackageMetadata::from_json_str(&package_metadata_fixture("invalid/missing-title.json"))
        .expect_err("missing title rejected");
    assert!(err.to_string().contains("title"));
}

#[test]
fn store_release_signing_credential_fields_are_blocked() {
    let err = PackageMetadata::from_json_str(&package_metadata_fixture("invalid/signing-field.json"))
        .expect_err("signing field rejected");
    assert!(err
        .to_string()
        .contains("failed to parse Package Metadata JSON"));
}

#[test]
fn forbidden_publish_wording_is_rejected() {
    let mut value: serde_json::Value =
        serde_json::from_str(&package_metadata_fixture("package-metadata.valid.fixture.json")).unwrap();
    value["description"] = serde_json::json!("Production-ready app store release.");
    let err =
        PackageMetadata::from_json_str(&value.to_string()).expect_err("publish wording rejected");
    assert!(err.to_string().contains("local package only"));
}

#[test]
fn metadata_round_trips() {
    let metadata =
        PackageMetadata::from_json_str(&package_metadata_fixture("package-metadata.valid.fixture.json")).unwrap();
    let json = metadata.to_json().unwrap();
    assert_eq!(PackageMetadata::from_json_str(&json).unwrap(), metadata);
}

// ===========================================================================
// export_plan_contract (#722)
// ===========================================================================

const VALID_PROFILE: &str =
    include_str!("../../../examples/export-profile-v1/export-profile.valid.fixture.json");

fn invalid_profile(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/export-profile-v1/invalid")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

fn invalid_plan_input(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/export-plan-v1/invalid")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

#[test]
fn plan_from_valid_profile_separates_inputs_outputs_and_steps() {
    let plan = ExportPlan::from_profile_json(VALID_PROFILE).expect("valid profile plans");
    assert_eq!(plan.schema_version, EXPORT_PLAN_SCHEMA_VERSION);
    assert_eq!(plan.plan_id, "plan_profile_collect_and_exit_web_local");
    assert_eq!(plan.profile_id, "profile_collect_and_exit_web_local");
    assert_eq!(plan.export_target, "web-local");
    assert_eq!(plan.output_dir, "dist/export/collect-and-exit");

    // Entry scene + two asset roots = three source inputs in declared order.
    assert_eq!(plan.source_inputs.len(), 3);
    assert_eq!(plan.source_inputs[0].kind, PlannedInputKind::EntryScene);
    assert_eq!(plan.source_inputs[1].kind, PlannedInputKind::AssetRoot);

    // html + bootstrap + 2 asset payloads + manifest + checksums = 6 outputs.
    assert_eq!(plan.generated_outputs.len(), 6);
    assert_eq!(plan.generated_outputs[0].kind, PlannedOutputKind::HtmlEntry);
    assert_eq!(
        plan.generated_outputs[0].staged_path,
        "dist/export/collect-and-exit/index.html"
    );
    assert!(plan
        .generated_outputs
        .iter()
        .any(|o| o.kind == PlannedOutputKind::AssetManifest));
    assert!(plan
        .generated_outputs
        .iter()
        .any(|o| o.kind == PlannedOutputKind::Checksums));

    // Every staged output stays inside the declared output directory.
    for output in &plan.generated_outputs {
        assert!(
            output
                .staged_path
                .starts_with("dist/export/collect-and-exit/"),
            "output {} escaped the staging dir",
            output.staged_path
        );
    }

    assert_eq!(plan.expected_artifacts.len(), plan.generated_outputs.len());

    // Standard steps plus one smoke per scenario id.
    assert!(plan
        .verification_steps
        .iter()
        .any(|s| s.id == "load-without-console-errors"));
    assert!(plan
        .verification_steps
        .iter()
        .any(|s| s.id == "runtime-probe-compatibility"));
    assert!(plan
        .verification_steps
        .iter()
        .any(|s| s.id == "scenario-smoke:scenario_reach_exit"));

    assert!(plan.blocked_files.iter().any(|p| p == "target/"));
    assert!(plan.blocked_files.iter().any(|p| p == ".git/"));
}

#[test]
fn plan_generation_is_deterministic() {
    let a = ExportPlan::from_profile_json(VALID_PROFILE).unwrap();
    let b = ExportPlan::from_profile_json(VALID_PROFILE).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.to_dry_run_json().unwrap(), b.to_dry_run_json().unwrap());
}

#[test]
fn plan_json_round_trips() {
    let plan = ExportPlan::from_profile_json(VALID_PROFILE).unwrap();
    let json = plan.to_dry_run_json().unwrap();
    let parsed: ExportPlan = serde_json::from_str(&json).unwrap();
    assert_eq!(plan, parsed);
}

#[test]
fn plan_fails_closed_on_blocked_target() {
    let err = ExportPlan::from_profile_json(&invalid_profile("blocked-target.json"))
        .expect_err("blocked target rejected before planning");
    assert!(err.to_string().contains("valid export profile"));
}

#[test]
fn plan_fails_closed_on_unsafe_output_path() {
    let err = ExportPlan::from_profile_json(&invalid_profile("path-traversal-output.json"))
        .expect_err("unsafe output rejected before planning");
    assert!(err.to_string().contains("valid export profile"));
}

#[test]
fn plan_fails_closed_on_missing_entry_scene() {
    let err = ExportPlan::from_profile_json(&invalid_profile("missing-entry-scene.json"))
        .expect_err("missing entry scene rejected before planning");
    assert!(err.to_string().contains("valid export profile"));
}

#[test]
fn plan_fails_closed_on_duplicate_asset_root() {
    let err = ExportPlan::from_profile_json(&invalid_plan_input("duplicate-asset-root.json"))
        .expect_err("duplicate asset root rejected");
    assert!(err.to_string().contains("duplicate asset root"));
}

#[test]
fn plan_fails_closed_on_colliding_asset_segment() {
    let err = ExportPlan::from_profile_json(&invalid_plan_input("colliding-asset-segment.json"))
        .expect_err("colliding asset output segment rejected");
    assert!(err.to_string().contains("colliding asset output segment"));
}

// ===========================================================================
// export_probe_check_contract (#725)
// ===========================================================================

fn probe_assembled_bundle(name: &str) -> PathBuf {
    let staging =
        std::env::temp_dir().join(format!("ouroforge-probe-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&staging);
    let prof = std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .unwrap();
    let plan = ExportPlan::from_profile_json(&prof).unwrap();
    assemble_web_bundle(&plan, &repo_root(), &staging).unwrap();
    staging
}

fn probe_fixture(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join("examples/export-probe-v1").join(rel))
        .unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn assembled_bundle_passes_both_probe_modes() {
    let bundle = probe_assembled_bundle("ok");
    for mode in [
        ExportProbeMode::DevProbeEnabled,
        ExportProbeMode::PackagedProbeLimited,
    ] {
        let report = check_bundle_probe(&bundle, mode).expect("probe check runs");
        assert!(report.global_present);
        assert!(report.passed, "missing: {:?}", report.missing_methods);
        assert!(report.missing_methods.is_empty());
    }
    ensure_bundle_probe_compatible(&bundle, ExportProbeMode::DevProbeEnabled)
        .expect("dev probe compatible");
    std::fs::remove_dir_all(&bundle).ok();
}

#[test]
fn limited_mode_requires_fewer_methods_than_dev() {
    let limited = ExportProbeMode::PackagedProbeLimited.required_methods();
    let dev = ExportProbeMode::DevProbeEnabled.required_methods();
    assert!(limited.len() < dev.len());
    for m in &limited {
        assert!(dev.contains(m));
    }
    assert!(dev.contains(&"step"));
    assert!(!limited.contains(&"step"));
}

#[test]
fn missing_probe_hook_fails_closed() {
    let source = probe_fixture("invalid/missing-getevents-bootstrap.js");
    let report = check_probe_source(&source, ExportProbeMode::DevProbeEnabled);
    assert!(report.global_present);
    assert!(!report.passed);
    assert!(report.missing_methods.contains(&"getEvents".to_string()));
}

#[test]
fn absent_probe_global_fails_closed() {
    let source = probe_fixture("invalid/no-probe-global-bootstrap.js");
    let report = check_probe_source(&source, ExportProbeMode::PackagedProbeLimited);
    assert!(!report.global_present);
    assert!(!report.passed);
    assert!(!source.contains(PROBE_GLOBAL));
}

#[test]
fn comment_mention_does_not_satisfy_missing_hook() {
    // `getEvents` appears only in a comment; the real probe object lacks it, so
    // detection must scope to the installed probe object and fail closed (#725).
    let source = "\
'use strict';
(function () {
  // The probe global window.__OUROFORGE__ should expose getEvents() per #725.
  const probe = Object.freeze({
    getWorldState() { return {}; },
    getFrameStats() { return {}; },
    snapshot() { return {}; },
    step() {},
    pause() {},
    resume() {},
    setInput() {},
    restore() {},
  });
  const globalScope = typeof window !== 'undefined' ? window : globalThis;
  globalScope.__OUROFORGE__ = probe;
})();
";
    let report = check_probe_source(source, ExportProbeMode::DevProbeEnabled);
    assert!(report.global_present, "global is genuinely installed");
    assert!(!report.passed, "getEvents only in a comment must not pass");
    assert!(report.missing_methods.contains(&"getEvents".to_string()));
}

#[test]
fn global_mentioned_only_in_comment_is_absent() {
    // A stray `__OUROFORGE__` mention with no real assignment must report the
    // probe global as absent rather than present (#725).
    let source = "\
'use strict';
// Note: window.__OUROFORGE__ is intentionally NOT installed in this bundle.
(function () { let tick = 0; tick += 1; })();
";
    let report = check_probe_source(source, ExportProbeMode::PackagedProbeLimited);
    assert!(
        !report.global_present,
        "comment mention is not an installation"
    );
    assert!(!report.passed);
}

#[test]
fn ensure_returns_actionable_error_for_missing_bundle() {
    let missing =
        std::env::temp_dir().join(format!("ouroforge-probe-missing-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&missing);
    let err = ensure_bundle_probe_compatible(&missing, ExportProbeMode::DevProbeEnabled)
        .expect_err("missing bundle fails");
    assert!(err.to_string().contains("missing runtime bootstrap"));
}

#[test]
fn probe_report_serializes_as_evidence() {
    let bundle = probe_assembled_bundle("evidence");
    let report = check_bundle_probe(&bundle, ExportProbeMode::PackagedProbeLimited).unwrap();
    let json = report.to_json().unwrap();
    assert!(json.contains("packaged-probe-limited"));
    assert!(json.contains("\"passed\": true"));
    std::fs::remove_dir_all(&bundle).ok();
}

// ===========================================================================
// export_profile_contract (#721)
// ===========================================================================

const VALID: &str =
    include_str!("../../../examples/export-profile-v1/export-profile.valid.fixture.json");

fn profile_invalid(name: &str) -> String {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/export-profile-v1/invalid")
        .join(name);
    std::fs::read_to_string(&base).unwrap_or_else(|err| panic!("read {}: {err}", base.display()))
}

#[test]
fn valid_web_local_profile_parses_and_validates() {
    let profile = ExportProfile::from_json_str(VALID).expect("valid profile parses");
    assert_eq!(profile.schema_version, EXPORT_PROFILE_SCHEMA_VERSION);
    assert_eq!(profile.export_target, "web-local");
    assert!(profile.target_is_allowed());
    assert_eq!(profile.runtime_probe_mode, RuntimeProbeMode::Preserve);
    assert_eq!(profile.asset_roots.len(), 2);
    assert_eq!(profile.verification_scenario_ids.len(), 2);
}

#[test]
fn allowed_targets_are_exactly_the_two_v1_web_targets() {
    assert_eq!(ALLOWED_EXPORT_TARGETS, &["web-local", "web-static-bundle"]);
    for target in BLOCKED_EXPORT_TARGETS {
        assert!(!ALLOWED_EXPORT_TARGETS.contains(target));
    }
}

#[test]
fn blocked_target_fails_closed_with_actionable_diagnostic() {
    let err = ExportProfile::from_json_str(&profile_invalid("blocked-target.json"))
        .expect_err("blocked target rejected");
    let msg = err.to_string();
    assert!(msg.contains("steam"));
    assert!(msg.contains("blocked"));
    assert!(msg.contains("export-target-matrix-v1.md"));
}

#[test]
fn future_target_fails_closed_as_design_gated() {
    let err = ExportProfile::from_json_str(&profile_invalid("future-target.json"))
        .expect_err("future target rejected");
    let msg = err.to_string();
    assert!(msg.contains("desktop-wrapper"));
    assert!(msg.contains("future") || msg.contains("design-gated"));
}

#[test]
fn path_traversal_output_is_rejected() {
    let err = ExportProfile::from_json_str(&profile_invalid("path-traversal-output.json"))
        .expect_err("traversal output rejected");
    assert!(err.to_string().contains(".."));
}

#[test]
fn backslash_path_is_rejected() {
    let err = ExportProfile::from_json_str(&profile_invalid("backslash-path.json"))
        .expect_err("backslash path rejected");
    assert!(err.to_string().contains("backslash"));
}

#[test]
fn generated_state_source_input_is_rejected() {
    let err = ExportProfile::from_json_str(&profile_invalid("generated-output-source.json"))
        .expect_err("generated-state source rejected");
    assert!(err.to_string().contains("generated state"));
}

#[test]
fn output_outside_staging_root_is_rejected() {
    let err = ExportProfile::from_json_str(&profile_invalid("output-not-staging.json"))
        .expect_err("non-staging output rejected");
    assert!(err.to_string().contains("staging root"));
}

#[test]
fn missing_entry_scene_is_rejected() {
    let err = ExportProfile::from_json_str(&profile_invalid("missing-entry-scene.json"))
        .expect_err("missing entry scene rejected");
    assert!(err.to_string().contains("entryScene"));
}

#[test]
fn profile_bad_schema_version_is_rejected() {
    let err = ExportProfile::from_json_str(&profile_invalid("bad-schema-version.json"))
        .expect_err("bad schema version rejected");
    assert!(err.to_string().contains(EXPORT_PROFILE_SCHEMA_VERSION));
}

#[test]
fn unknown_fields_are_rejected() {
    let mut value: serde_json::Value = serde_json::from_str(VALID).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("publish".to_string(), serde_json::json!(true));
    let err = ExportProfile::from_json_str(&value.to_string()).expect_err("unknown field rejected");
    assert!(err
        .to_string()
        .contains("failed to parse Export Profile JSON"));
}

// ===========================================================================
// export_release_blocker_contract (#733)
// ===========================================================================

fn release_blocker_read(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn valid_export_profile_has_no_publish_fields() {
    let prof = release_blocker_read("examples/export-bundle-v1/export-profile.fixture.json");
    ensure_no_publish_config(&prof).expect("clean profile passes the blocker");
}

#[test]
fn publish_config_fails_closed() {
    let cfg = release_blocker_read("examples/export-release-blocker-v1/invalid/publish-config.json");
    let err = ensure_no_publish_config(&cfg).expect_err("publish config blocked");
    assert!(err.to_string().contains("publish"));
}

#[test]
fn signing_deploy_upload_credentials_fail_closed() {
    let cfg = release_blocker_read("examples/export-release-blocker-v1/invalid/signing-deploy-config.json");
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

// ===========================================================================
// export_staging_contract (#726)
// ===========================================================================

const STAGING_POLICY: &str = include_str!("../../../docs/export-staging-policy-v1.md");

#[test]
fn staging_dir_is_run_scoped_under_ignored_target() {
    assert_eq!(EXPORT_STAGING_ROOT, "target/ouroforge/exports");
    let dir = staging_dir_for_run("run_demo").unwrap();
    assert_eq!(dir, "target/ouroforge/exports/run_demo");
    assert!(is_within_staging_root(&dir));
    // Under target/, which .gitignore already ignores.
    assert!(is_ignored_by_default(&dir));
    assert!(dir.starts_with("target/"));
}

#[test]
fn outside_staging_paths_are_not_in_root() {
    assert!(!is_within_staging_root(
        "examples/export-bundle-v1/fixture-game"
    ));
    assert!(!is_within_staging_root("src/main.rs"));
}

#[test]
fn stale_runs_partition_for_cleanup() {
    let existing = vec!["old1".to_string(), "keep".to_string(), "old2".to_string()];
    let (kept, stale) = partition_stale_runs(&existing, &["keep".to_string()]);
    assert_eq!(kept, vec!["keep".to_string()]);
    assert_eq!(stale, vec!["old1".to_string(), "old2".to_string()]);
}

#[test]
fn policy_doc_documents_staging_and_generated_state() {
    assert!(STAGING_POLICY.contains("target/ouroforge/exports/<run-id>/"));
    assert!(STAGING_POLICY.contains("ignored by default"));
    assert!(STAGING_POLICY.contains("fixture-scoped"));
    assert!(STAGING_POLICY.contains("git status --short --ignored"));
    assert!(STAGING_POLICY.to_lowercase().contains("stale"));
    assert!(STAGING_POLICY.contains("#1 remains"));
    assert!(STAGING_POLICY.contains("#23 remains"));
}

// ===========================================================================
// export_target_matrix_contract (#720)
// ===========================================================================

const MATRIX: &str = include_str!("../../../docs/export-target-matrix-v1.md");
const SCOPE: &str = include_str!("../../../docs/build-export-packaging-v1.md");

#[test]
fn matrix_declares_allowed_v1_web_targets() {
    assert!(MATRIX.contains("`web-local`"));
    assert!(MATRIX.contains("`web-static-bundle`"));
    // Both allowed targets appear in an `allowed` row of the matrix table.
    assert!(MATRIX.contains("| `web-local` | allowed |"));
    assert!(MATRIX.contains("| `web-static-bundle` | allowed |"));
}

#[test]
fn matrix_gates_desktop_wrapper_as_future_not_implemented() {
    assert!(MATRIX.contains("| `desktop-wrapper`"));
    assert!(MATRIX.contains("future"));
    assert!(MATRIX.contains("design-gated") || MATRIX.contains("design gate"));
    // The future target must not be treated as authorized: it is validated like
    // a blocked target until a separate design-gate issue scopes it.
    assert!(
        MATRIX.contains("treats it the same as a blocked target")
            || MATRIX.contains("the same as a blocked target")
    );
}

#[test]
fn matrix_blocks_every_publish_release_target() {
    for target in [
        "mobile",
        "console",
        "app-store",
        "steam",
        "itch",
        "hosted-deploy",
        "signed-release",
        "ci-release",
    ] {
        assert!(
            MATRIX.contains(&format!("| `{target}`")),
            "matrix is missing a row for blocked target `{target}`"
        );
    }
    assert!(MATRIX.contains("blocked"));
}

#[test]
fn matrix_blocks_publish_deploy_sign_credentials_and_ci_release() {
    let lower = MATRIX.to_lowercase();
    for term in [
        "publish",
        "deploy",
        "signing",
        "notariz",
        "credential",
        "upload",
        "ci release",
    ] {
        assert!(lower.contains(term), "matrix must address `{term}`");
    }
    assert!(MATRIX.contains("fail closed") || MATRIX.contains("fail-closed"));
}

#[test]
fn matrix_distinguishes_local_package_from_public_release() {
    assert!(MATRIX.contains("local, evidence-backed") || MATRIX.contains("local, evidence backed"));
    assert!(MATRIX.contains("public release"));
    assert!(MATRIX.contains("Producing a local artifact is not releasing it"));
}

#[test]
fn matrix_narrows_prior_release_export_mutation_blocker() {
    assert!(MATRIX.contains("source-apply"));
    assert!(MATRIX.contains("Now allowed:"));
    assert!(MATRIX.contains("Still blocked:"));
    assert!(MATRIX.contains("release/publish mutation"));
}

#[test]
fn matrix_keeps_governance_anchors_open() {
    assert!(MATRIX.contains("#1 remains"));
    assert!(MATRIX.contains("#23 remains"));
    assert!(MATRIX.contains("remains open"));
}

#[test]
fn scope_doc_references_the_matrix() {
    assert!(SCOPE.contains("docs/export-target-matrix-v1.md"));
}

// ===========================================================================
// export_verification_contract (#728)
// ===========================================================================

fn verify_staging(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ouroforge-verify-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn verify_plan() -> ExportPlan {
    let json = std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .unwrap();
    ExportPlan::from_profile_json(&json).unwrap()
}

fn assembled(name: &str) -> PathBuf {
    let dir = verify_staging(name);
    assemble_web_bundle(&verify_plan(), &repo_root(), &dir).unwrap();
    dir
}

#[test]
fn assembled_bundle_passes_verification() {
    let bundle = assembled("ok");
    let report = verify_export_bundle(&verify_plan(), &bundle, ExportProbeMode::DevProbeEnabled);
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
    ensure_export_verified(&verify_plan(), &bundle, ExportProbeMode::DevProbeEnabled).expect("verified");
    std::fs::remove_dir_all(&bundle).ok();
}

#[test]
fn scenario_smoke_is_skipped_not_passed_by_static_runner() {
    // The static runner cannot execute declared scenarios, so it must not
    // report scenario-smoke as a Pass derived only from bundle presence (#728).
    let bundle = assembled("scenario-skip");
    let report = verify_export_bundle(&verify_plan(), &bundle, ExportProbeMode::DevProbeEnabled);
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
    let missing = verify_staging("missing");
    let report = verify_export_bundle(&verify_plan(), &missing, ExportProbeMode::DevProbeEnabled);
    assert!(!report.passed());
    let err = ensure_export_verified(&verify_plan(), &missing, ExportProbeMode::DevProbeEnabled)
        .expect_err("missing bundle fails");
    assert!(err.to_string().contains("bundle-present"));
}

#[test]
fn blocked_target_fails_closed() {
    let bundle = assembled("blocked");
    let mut p = verify_plan();
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
    let report = verify_export_bundle(&verify_plan(), &bundle, ExportProbeMode::DevProbeEnabled);
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
    let report = verify_export_bundle(&verify_plan(), &bundle, ExportProbeMode::DevProbeEnabled);
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
fn verification_report_serializes_as_evidence() {
    let bundle = assembled("evidence");
    let report = verify_export_bundle(&verify_plan(), &bundle, ExportProbeMode::PackagedProbeLimited);
    let json = report.to_json().unwrap();
    assert!(json.contains("\"verdict\": \"pass\""));
    assert!(json.contains("runtime-probe-compatibility"));
    std::fs::remove_dir_all(&bundle).ok();
}
