//! Scenario Coverage v41 — Scaled Trust and Release Provenance Regression Suite (#1695).
//!
//! Locks release-scale auto-apply, rollback, kill-switch, provenance bundle,
//! compliance, and Milestone 22/25 backward-compatibility state/shape behavior.
//! The suite composes existing contracts only and introduces no timing, browser,
//! network, trusted-write, release-authority, or subjective quality assertions.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ouroforge_core::provenance_bundle::{ProvenanceBundleArtifact, ProvenanceBundleStatus};
use ouroforge_core::release_auto_apply::{decide_release_auto_apply, ReleaseAutoApplyRequest};
use ouroforge_core::release_compliance_gate::{
    evaluate_release_compliance, ComplianceVerdictStatus, HumanGoNoGo, ReleaseComplianceGateInput,
};
use ouroforge_core::release_provenance_bundle::{ReleaseProvenanceBundle, ReleaseProvenanceStatus};
use ouroforge_core::trust_gradient_audit::AutoApplyAuditLog;
use ouroforge_core::trust_gradient_auto_apply::{
    decide_auto_apply, AutoApplyOutcome, AutoApplyRequest,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn read_text(relative: &str) -> String {
    let path = repo_root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative))
        .unwrap_or_else(|error| panic!("parse {relative}: {error}"))
}

fn assert_repo_file(relative: &str) {
    assert!(
        repo_root().join(relative).is_file(),
        "stale ref: {relative}"
    );
}

const MATRIX: &str =
    "examples/release-trust-provenance-v1/scenario-coverage-v41/matrix.fixture.json";
const BACKCOMPAT: &str =
    "examples/release-trust-provenance-v1/scenario-coverage-v41/backcompat-golden.fixture.json";
const DOC: &str = "docs/scenario-coverage-v41.md";

#[test]
fn v41_matrix_enumerates_scaled_trust_release_regressions() {
    let matrix = read_json(MATRIX);
    assert_eq!(matrix["schemaVersion"], "scenario-coverage-v41");
    assert_eq!(matrix["issue"], 1695);
    assert_eq!(matrix["fixtureScoped"], true);

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "rust/local owned",
        "browser/studio surfaces are read-only",
        "state/shape regression coverage",
        "no timing",
        "no network/live browser",
        "no trusted writes",
        "no auto-merge",
        "no self-approval",
        "production-ready",
        "quality/fun",
        "godot replacement/parity",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "missing boundary: {required}");
    }

    let scenarios = matrix["scenarios"].as_array().expect("scenarios array");
    assert!(
        scenarios.len() >= 10,
        "v41 enumerates all required surfaces"
    );
    let mut ids = BTreeSet::new();
    let mut systems = BTreeSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().expect("id");
        assert!(ids.insert(id.to_string()), "duplicate scenario id {id}");
        systems.insert(scenario["system"].as_str().expect("system").to_string());
        assert!(scenario["kind"].is_string(), "{id} kind");
        let fixture_ref = scenario["fixtureRef"].as_str().expect("fixtureRef");
        assert_repo_file(fixture_ref);
        assert!(scenario["expect"].is_string(), "{id} expect");
    }

    for system in [
        "release-auto-apply",
        "release-provenance",
        "release-compliance",
        "backcompat-m22",
        "backcompat-m25",
    ] {
        assert!(systems.contains(system), "missing coverage for {system}");
    }
    assert!(ids.contains("v41-release-auto-apply-kill-switch"));
    assert!(ids.contains("v41-backcompat-m25-per-change-provenance"));
}

#[test]
fn v41_release_auto_apply_rollback_and_kill_switch_states_are_locked() {
    let eligible = ReleaseAutoApplyRequest::from_json_str(&read_text(
        "examples/release-auto-apply-v1/fixtures/eligible-auto-apply.json",
    ))
    .expect("eligible release fixture parses");
    let decision =
        decide_release_auto_apply(&eligible, &AutoApplyAuditLog::new()).expect("decision");
    assert_eq!(decision.outcome, AutoApplyOutcome::AutoApplied);
    assert_eq!(decision.trust_decision.budget_after.remaining, 3);
    assert!(decision
        .rollback_command
        .as_deref()
        .expect("rollback command")
        .starts_with("ouroforge rollback --transaction txn-release-eligible-001"));

    let high_risk = ReleaseAutoApplyRequest::from_json_str(&read_text(
        "examples/release-auto-apply-v1/fixtures/ineligible-high-risk-manual.json",
    ))
    .expect("manual fallback fixture parses");
    let manual =
        decide_release_auto_apply(&high_risk, &AutoApplyAuditLog::new()).expect("decision");
    assert_eq!(manual.outcome, AutoApplyOutcome::ManualFallback);
    assert!(manual.rollback_command.is_none());
    assert!(manual
        .reasons
        .iter()
        .any(|reason| reason.contains("risk tier is not low")));

    let kill_log = AutoApplyAuditLog::from_json_str(&read_text(
        "examples/release-auto-apply-v1/fixtures/kill-switch-audit-log.json",
    ))
    .expect("kill switch log parses");
    assert!(kill_log.is_autonomy_halted());
    let halted = decide_release_auto_apply(&eligible, &kill_log).expect("halted decision");
    assert_eq!(halted.outcome, AutoApplyOutcome::ManualFallback);
    assert!(halted.rollback_command.is_none());
    assert!(halted
        .reasons
        .iter()
        .any(|reason| reason.contains("kill switch engaged")));
}

#[test]
fn v41_release_provenance_bundle_states_are_locked() {
    let root = repo_root();
    let complete = ReleaseProvenanceBundle::from_json_str(&read_text(
        "examples/release-provenance-bundle-v1/bundle.complete.fixture.json",
    ))
    .expect("complete release bundle parses");
    let complete_eval = complete.evaluate_with_root(&root);
    assert_eq!(
        complete_eval.computed_status,
        ReleaseProvenanceStatus::Complete
    );
    assert!(complete_eval.status_consistent, "{complete_eval:#?}");
    assert!(complete_eval.replayable, "{complete_eval:#?}");

    let missing = ReleaseProvenanceBundle::from_json_str(&read_text(
        "examples/release-provenance-bundle-v1/bundle.missing-link.fixture.json",
    ))
    .expect("missing-link release bundle parses");
    let missing_eval = missing.evaluate_with_root(&root);
    assert_eq!(
        missing_eval.computed_status,
        ReleaseProvenanceStatus::Incomplete
    );
    assert_eq!(
        missing_eval
            .link_states
            .get("compliance")
            .map(String::as_str),
        Some("missing")
    );
    assert!(missing_eval
        .issues
        .iter()
        .any(|issue| issue.contains("missing release chain link")));

    let not_replayable = ReleaseProvenanceBundle::from_json_str(&read_text(
        "examples/release-provenance-bundle-v1/bundle.not-replayable.fixture.json",
    ))
    .expect("not replayable bundle parses");
    let not_replayable_eval = not_replayable.evaluate_with_root(&root);
    assert_eq!(
        not_replayable_eval.computed_status,
        ReleaseProvenanceStatus::Incomplete
    );
    assert!(!not_replayable_eval.replayable);
    assert!(not_replayable_eval
        .issues
        .iter()
        .any(|issue| issue.contains("release replayability requires deterministic inputs")));
}

#[test]
fn v41_release_compliance_outcomes_are_locked() {
    let pass = ReleaseComplianceGateInput::from_json_str(&read_text(
        "examples/release-compliance-gate-v1/fixtures/compliant-release.json",
    ))
    .expect("compliant fixture parses");
    let pass_verdict = evaluate_release_compliance(&pass).expect("pass verdict");
    assert_eq!(pass_verdict.status, ComplianceVerdictStatus::Pass);
    assert_eq!(pass_verdict.human_go_no_go, HumanGoNoGo::Pending);
    assert!(pass_verdict.reasons.is_empty());

    let blocked = ReleaseComplianceGateInput::from_json_str(&read_text(
        "examples/release-compliance-gate-v1/fixtures/violation-release.json",
    ))
    .expect("blocked fixture parses");
    let blocked_verdict = evaluate_release_compliance(&blocked).expect("blocked verdict");
    assert_eq!(blocked_verdict.status, ComplianceVerdictStatus::Blocked);
    for expected in [
        "policy violation",
        "age-rating signals",
        "missing license",
        "missing provenance",
    ] {
        assert!(
            blocked_verdict
                .reasons
                .iter()
                .any(|reason| reason.contains(expected)),
            "missing compliance reason {expected}: {blocked_verdict:#?}"
        );
    }
    assert!(blocked_verdict.boundary.contains("no release authority"));
}

#[test]
fn v41_backward_compatibility_golden_preserves_m22_auto_apply_and_m25_provenance() {
    let golden = read_json(BACKCOMPAT);
    assert_eq!(
        golden["schemaVersion"],
        "scenario-coverage-v41-backcompat-golden"
    );
    assert_eq!(golden["fixtureScoped"], true);
    let boundary = golden["boundary"].as_str().expect("boundary");
    assert!(boundary.contains("no auto-merge"));
    assert!(boundary.contains("#1 and #23 remain open"));

    let auto_apply_ref = golden["milestone22"]["fixtureRef"].as_str().unwrap();
    let auto_apply = AutoApplyRequest::from_json_str(&read_text(auto_apply_ref))
        .expect("m22 auto apply fixture parses");
    let auto_apply_decision = decide_auto_apply(&auto_apply).expect("m22 decision");
    assert_eq!(auto_apply_decision.outcome, AutoApplyOutcome::AutoApplied);
    assert_eq!(
        auto_apply_decision.rollback_command.as_deref(),
        golden["milestone22"]["expectedRollbackCommand"].as_str()
    );
    assert_eq!(
        auto_apply_decision.budget_after.remaining,
        golden["milestone22"]["expectedBudgetRemaining"]
            .as_u64()
            .unwrap() as u32
    );

    let bundle_ref = golden["milestone25"]["fixtureRef"].as_str().unwrap();
    let bundle_root = repo_root().join(golden["milestone25"]["fixtureRoot"].as_str().unwrap());
    let bundle = ProvenanceBundleArtifact::from_json_str(&read_text(bundle_ref))
        .expect("m25 provenance fixture parses");
    let evaluation = bundle.evaluate_with_root(&bundle_root);
    assert_eq!(evaluation.computed_status, ProvenanceBundleStatus::Complete);
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert_eq!(
        evaluation.link_states.len(),
        golden["milestone25"]["expectedLinkCount"].as_u64().unwrap() as usize
    );
}

#[test]
fn v41_docs_preserve_generated_state_wording_compatibility_and_governance() {
    let docs = read_text(DOC);
    for required in [
        "Scenario Coverage v41 locks",
        "state/shape regressions only",
        "Milestone 22 per-change auto-apply",
        "Milestone 25 per-change provenance",
        "Rust/local owned",
        "Browser, dashboard, and Studio surfaces may inspect read-only evidence only",
        "fixture-scoped",
        "Issues #1 and #23 remain open",
    ] {
        assert!(docs.contains(required), "docs missing {required}");
    }
    for forbidden_boundary in [
        "no auto-merge",
        "no self-approval",
        "no reviewer bypass",
        "no production-ready claim",
        "no quality/fun guarantee",
        "no Godot replacement/parity",
        "no autonomous shipping claim",
    ] {
        assert!(
            docs.contains(forbidden_boundary),
            "docs must explicitly forbid {forbidden_boundary}"
        );
    }
}
