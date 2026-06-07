//! Contract tests for Asset Replay-Proof and Provenance Binding v1 (#1614).
//!
//! Validates that, on consume, an asset's bound replay proof re-runs and
//! verifies, a tampered replay or provenance ref is detected fail-closed, and
//! the provenance lineage is traceable. Reuses the Milestone 25 provenance
//! bundle (#1500) and the deterministic replay (#1502); no parallel engine.

use ouroforge_core::evidence_marketplace_proof::{
    verify_asset_proof, AssetProofStatus, EVIDENCE_MARKETPLACE_PROOF_SCHEMA_VERSION,
};
use ouroforge_core::evidence_marketplace_registry::MarketplaceAsset;
use ouroforge_core::provenance_bundle::ProvenanceBundleStatus;
use ouroforge_core::provenance_replay::ProvenanceReplayStatus;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn fixture_root() -> PathBuf {
    repo_root().join("examples/evidence-marketplace-proof-v1614")
}

fn read_asset() -> MarketplaceAsset {
    let path = fixture_root().join("asset.proof.fixture.json");
    MarketplaceAsset::from_json_str(
        &fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}")),
    )
    .unwrap_or_else(|error| panic!("parse {path:?}: {error}"))
}

fn unique_temp(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "ouroforge-marketplace-proof-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("clear temp dir");
    }
    path
}

fn workspace(name: &str) -> PathBuf {
    unique_temp(&format!("workspace-{name}"))
}

fn copy_dir_all(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create destination");
    for entry in fs::read_dir(source).expect("read source dir") {
        let entry = entry.expect("dir entry");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("file type").is_dir() {
            copy_dir_all(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).expect("copy file");
        }
    }
}

/// Copy the committed fixture evidence into a writable temp root so tamper
/// cases never mutate the tracked fixtures.
fn tampered_root(name: &str) -> PathBuf {
    let root = unique_temp(&format!("root-{name}"));
    copy_dir_all(&fixture_root(), &root);
    root
}

#[test]
fn consume_re_runs_proof_and_verifies() {
    let asset = read_asset();
    let report = verify_asset_proof(&asset, fixture_root(), workspace("verifies"));

    assert_eq!(
        report.schema_version,
        EVIDENCE_MARKETPLACE_PROOF_SCHEMA_VERSION
    );
    assert_eq!(
        report.status,
        AssetProofStatus::Verified,
        "{:?}",
        report.issues
    );
    assert!(report.is_verified());
    assert_eq!(
        report.replay_status,
        Some(ProvenanceReplayStatus::Reproduced)
    );
    assert_eq!(report.provenance_status, ProvenanceBundleStatus::Complete);
    assert!(report.lineage_traceable);
    assert!(report.issues.is_empty(), "{:?}", report.issues);
}

#[test]
fn provenance_lineage_is_traceable_in_order() {
    let asset = read_asset();
    let report = verify_asset_proof(&asset, fixture_root(), workspace("lineage"));

    let order = [
        "intent-design-brief",
        "generated-edited-artifact",
        "validation-result",
        "runtime-observation",
        "evaluator-verdict",
        "regression-comparison",
        "journal-review-decision",
        "promotion-rollback-record",
    ];
    assert_eq!(report.lineage.len(), order.len());
    for (step, expected_kind) in report.lineage.iter().zip(order.iter()) {
        assert_eq!(&step.kind, expected_kind);
        assert_eq!(
            step.state, "present",
            "{} should resolve present",
            step.kind
        );
        assert!(!step.artifact_id.is_empty());
        assert!(!step.reference.is_empty());
    }
    assert!(report.lineage_traceable);
}

#[test]
fn tampered_replay_run_is_detected_fail_closed() {
    let asset = read_asset();
    let root = tampered_root("replay");
    // Mutate run evidence so the re-run verdict diverges from the bound
    // expectation. Run evidence is not digest-bound, so this surfaces as a
    // replay divergence rather than a provenance digest mismatch.
    let scenario_result =
        root.join("runs/replay-pass/evidence/scenarios/collect-and-exit/scenario-result.json");
    let original = fs::read_to_string(&scenario_result).expect("read scenario result");
    let tampered = original.replace("\"status\": \"passed\"", "\"status\": \"failed\"");
    assert_ne!(original, tampered, "tamper must change the run evidence");
    fs::write(&scenario_result, tampered).expect("write tampered scenario result");

    let report = verify_asset_proof(&asset, &root, workspace("replay-tamper"));

    assert!(!report.is_verified());
    assert_eq!(report.status, AssetProofStatus::ReplayDiverged);
    assert_eq!(report.replay_status, Some(ProvenanceReplayStatus::Diverged));
    assert!(report.issues.iter().any(|issue| issue.contains("diverged")));
}

#[test]
fn tampered_provenance_ref_is_detected_fail_closed() {
    let asset = read_asset();
    let root = tampered_root("provenance");
    // Mutate a digest-bound provenance ref; the bundle's existing digest
    // evaluation must report it stale and refuse to verify.
    let intent_ref = root.join("refs/intent-design-brief.json");
    let mut bytes = fs::read_to_string(&intent_ref).expect("read intent ref");
    bytes.push_str("\n{ \"tampered\": true }\n");
    fs::write(&intent_ref, bytes).expect("write tampered intent ref");

    let report = verify_asset_proof(&asset, &root, workspace("provenance-tamper"));

    assert!(!report.is_verified());
    assert_eq!(report.status, AssetProofStatus::ProvenanceTampered);
    assert_ne!(report.provenance_status, ProvenanceBundleStatus::Complete);
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.contains("stale") || issue.contains("not intact")));
    // The replay never runs once tampering is detected.
    assert!(report.replay_status.is_none());
}

#[test]
fn proof_unbound_from_lineage_is_detected_fail_closed() {
    let mut asset = read_asset();
    // Break the binding: point the asset replay proof at a different run than
    // the lineage records. Re-verification must refuse before any re-run.
    let proof = asset.replay_proof.as_mut().expect("replay proof");
    proof.run_ref = "runs/some-other-run".to_string();

    let report = verify_asset_proof(&asset, fixture_root(), workspace("unbound"));

    assert!(!report.is_verified());
    assert_eq!(report.status, AssetProofStatus::ProofUnbound);
    assert!(report.replay_status.is_none());
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.contains("not bound")));
}

#[test]
fn proof_docs_and_fixtures_preserve_wording_compatibility_and_governance() {
    let docs = fs::read_to_string(repo_root().join("docs/evidence-marketplace-proof-v1.md"))
        .expect("read proof docs");
    let asset = fs::read_to_string(fixture_root().join("asset.proof.fixture.json"))
        .expect("read asset fixture");
    let all = format!("{docs}\n{asset}");

    // Anchored to the reused surfaces, not a new engine.
    assert!(all.contains("#1614"));
    assert!(all.contains("#1500"));
    assert!(all.contains("#1502"));
    assert!(
        docs.contains("not** a new provenance engine")
            || docs.contains("not a new provenance engine")
    );
    assert!(docs.contains("evaluate_run"));
    assert!(docs.contains("review/apply/trust-gradient"));
    assert!(docs.contains("Generated replay outputs remain untracked unless fixture-scoped"));

    // Governance markers.
    assert!(all.contains("#1 remains open"));
    assert!(all.contains("#23 remains open"));

    // Fixture stays fixture-scoped.
    let value: serde_json::Value = serde_json::from_str(&asset).expect("fixture JSON");
    assert_eq!(value["generatedState"]["fixtureScoped"], true);
    assert_eq!(value["provenance"]["generatedState"]["fixtureScoped"], true);

    // Conservative wording: no overclaim, no auto-close of #1/#23. (Disclaimer
    // language such as "no auto-merge" is allowed; only positive overclaims and
    // close-directives are forbidden.)
    let lower = all.to_ascii_lowercase();
    assert!(!lower.contains("godot replacement"));
    assert!(!lower.contains("godot parity"));
    assert!(!lower.contains("production-grade"));
    assert!(!lower.contains("close #1"));
    assert!(!lower.contains("close #23"));
}
