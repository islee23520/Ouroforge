//! Scenario Coverage v54 regression suite for #1873.
//!
//! Locks Balance Tuning Co-Pilot and Release Readiness Go/No-Go state/shape
//! behavior, plus Milestone 25/44 provenance backward compatibility.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::balance_copilot::{
    BalanceCopilotHumanApproval, BalanceCopilotRecommendationSet,
    BalanceCopilotReverificationReport, BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED,
};
use ouroforge_core::provenance_bundle::{ProvenanceBundleArtifact, ProvenanceBundleStatus};
use ouroforge_core::release_provenance_bundle::{ReleaseProvenanceBundle, ReleaseProvenanceStatus};
use ouroforge_core::release_readiness::{
    ReleaseGoNoGoRecord, ReleaseReadinessBundle, ReleaseReadinessStatus,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path)).expect("fixture/doc exists")
}

fn matrix() -> Value {
    serde_json::from_str(&read_text(
        "examples/release-readiness-v1/scenario-coverage-v54/coverage-matrix.json",
    ))
    .expect("coverage matrix parses")
}

fn row_ids(matrix: &Value) -> BTreeSet<String> {
    matrix["rows"]
        .as_array()
        .expect("rows array")
        .iter()
        .map(|row| row["id"].as_str().expect("row id").to_string())
        .collect()
}

#[test]
fn v54_matrix_enumerates_release_readiness_regressions() {
    let matrix = matrix();
    assert_eq!(matrix["schemaVersion"], "ouroforge.scenario-coverage.v54");
    assert_eq!(matrix["assertionMode"], "state-and-shape-only");
    assert_eq!(matrix["networkRequired"], false);
    assert_eq!(matrix["liveBrowserRequired"], false);

    let ids = row_ids(&matrix);
    for required in [
        "v54-copilot-recommend",
        "v54-copilot-approve",
        "v54-copilot-reverify",
        "v54-readiness-ready",
        "v54-readiness-missing-gate",
        "v54-go-no-go-record",
        "v54-m25-provenance-backcompat",
        "v54-m44-release-provenance-backcompat",
    ] {
        assert!(ids.contains(required), "missing coverage row {required}");
    }

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "rust/local",
        "state-and-shape-only",
        "browser/studio read-only",
        "no network/live browser",
        "no auto-apply",
        "no auto-merge",
        "no release authority",
        "no automated fun score",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v54_balance_copilot_states_are_locked() {
    let recommendation = BalanceCopilotRecommendationSet::from_json_str(&read_text(
        "examples/balance-copilot-v1/recommendation-set-v1.json",
    ))
    .expect("recommendation fixture");
    recommendation.validate().expect("recommendation validates");
    assert_eq!(recommendation.recommendations.len(), 1);
    assert_eq!(
        recommendation.recommendations[0].target_build_id,
        "loop-engine"
    );
    assert!(recommendation.proposal_only);
    assert!(recommendation.human_approval_required);
    assert!(!recommendation.auto_apply_allowed);
    assert!(!recommendation.trusted_write_authority);

    let approval = BalanceCopilotHumanApproval::from_json_str(&read_text(
        "examples/balance-copilot-v1/human-approval-v1.json",
    ))
    .expect("approval fixture");
    approval.validate().expect("approval validates");
    assert_eq!(approval.decision, "approved-with-tweak");
    assert!(approval.reverify_required);
    assert!(!approval.auto_apply_requested);
    assert!(!approval.trusted_write_authority);

    let reverification = BalanceCopilotReverificationReport::from_json_str(&read_text(
        "examples/balance-copilot-v1/reverification-v1.json",
    ))
    .expect("reverification fixture");
    reverification.validate().expect("reverification validates");
    assert_eq!(
        reverification.status,
        BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED
    );
    assert!(!reverification.dominant_build_still_flagged);
    assert!(!reverification.auto_apply_performed);
    assert!(!reverification.trusted_write_authority);
}

#[test]
fn v54_release_readiness_states_are_locked() {
    let ready = ReleaseReadinessBundle::from_json_str(&read_text(
        "examples/release-readiness-v1/complete-ready-bundle.fixture.json",
    ))
    .expect("ready bundle");
    ready.validate().expect("ready validates");
    assert_eq!(ready.status, ReleaseReadinessStatus::Ready);
    assert!(ready.human_go_no_go_required);
    assert!(!ready.human_go_no_go_recorded);
    assert!(!ready.release_authority_granted);
    assert!(!ready.auto_merge_allowed);
    assert!(!ready.trusted_write_authority);

    let blocked = ReleaseReadinessBundle::from_json_str(&read_text(
        "examples/release-readiness-v1/missing-gate.fixture.json",
    ))
    .expect("blocked bundle");
    blocked.validate().expect("blocked validates");
    assert_eq!(blocked.status, ReleaseReadinessStatus::Blocked);
    assert!(blocked
        .missing_gate_kinds
        .contains(&"steam-export-readiness".to_string()));
    assert!(blocked
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("balance blocked")));

    let go = ReleaseGoNoGoRecord::from_json_str(&read_text(
        "examples/release-readiness-v1/go-no-go.fixture.json",
    ))
    .expect("go/no-go fixture");
    go.validate().expect("go/no-go validates");
    assert!(go.release_ready);
    assert!(go.human_confirmed);
    assert!(!go.release_authority_granted);
    assert!(!go.auto_merge_allowed);
    assert!(!go.trusted_write_authority);
}

#[test]
fn v54_milestone_25_and_44_provenance_remain_valid() {
    let m25 = ProvenanceBundleArtifact::from_json_str(&read_text(
        "examples/provenance-bundle-v1/bundle.complete.fixture.json",
    ))
    .expect("m25 provenance parses");
    let m25_eval = m25.evaluate_with_root(&repo_root().join("examples/provenance-bundle-v1"));
    assert_eq!(m25_eval.computed_status, ProvenanceBundleStatus::Complete);
    assert!(m25_eval.status_consistent, "{m25_eval:#?}");
    assert!(m25_eval.issues.is_empty(), "{m25_eval:#?}");
    assert_eq!(m25_eval.link_states.len(), 8);

    let m44 = ReleaseProvenanceBundle::from_json_str(&read_text(
        "examples/release-provenance-bundle-v1/bundle.complete.fixture.json",
    ))
    .expect("m44 release provenance parses");
    let m44_eval = m44.evaluate_with_root(&repo_root());
    assert_eq!(m44_eval.computed_status, ReleaseProvenanceStatus::Complete);
    assert!(m44_eval.status_consistent, "{m44_eval:#?}");
    assert!(m44_eval.replayable, "{m44_eval:#?}");
    assert!(m44_eval.issues.is_empty(), "{m44_eval:#?}");
}

#[test]
fn v54_docs_preserve_generated_state_and_conservative_wording() {
    let doc = read_text("docs/scenario-coverage-v54.md");
    let lower = doc.to_ascii_lowercase();
    for required in [
        "state-and-shape-only",
        "cargo test -p ouroforge-core --test scenario_coverage_v54_release_readiness --jobs 2",
        "generated runs/artifacts remain untracked unless fixture-scoped",
        "browser/studio surfaces remain read-only",
        "#1 and #23 remain open",
    ] {
        assert!(lower.contains(required), "doc missing {required}");
    }
    for forbidden in [
        "godot replacement",
        "production-ready",
        "automated fun score passes",
        "grants auto-merge authority",
        "trusted write is granted",
    ] {
        assert!(
            !lower.contains(forbidden),
            "forbidden wording present: {forbidden}"
        );
    }
}
