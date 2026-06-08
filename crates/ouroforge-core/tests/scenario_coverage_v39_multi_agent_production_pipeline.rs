//! Scenario Coverage v39 — Multi-Agent Production Pipeline Regression Suite (#1680).
//!
//! Locks Multi-Agent Production Pipeline v1 behavior: the role-agent/ownership
//! model (#1675), handoff artifacts and deterministic conflict resolution
//! (#1676), and reviewer/critic promotion gates (#1678), plus the
//! backward-compatibility guarantee that the existing single-agent evolve/apply
//! flows remain valid. State/shape assertions only — no flaky or timing-based
//! checks — so a breaking change fails CI.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ouroforge_core::evolve_campaign::{read_evolve_campaign_artifact, validate_evolve_campaign};
use ouroforge_core::production_handoff::ProductionHandoffLedger;
use ouroforge_core::production_review_gates::ProductionReviewGateLedger;
use ouroforge_core::production_roles::ProductionRoleOwnershipModel;
use ouroforge_core::safe_source_apply_demo::SafeSourceApplyDemoArtifact;
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

fn v39(name: &str) -> String {
    read_text(&format!(
        "examples/production-pipeline-v1/scenario-coverage-v39/{name}"
    ))
}

const SYSTEMS: [&str; 5] = ["roles", "handoff", "gates", "demo", "backcompat"];

#[test]
fn v39_matrix_is_enumerated() {
    let matrix: Value = serde_json::from_str(&v39("matrix.fixture.json")).expect("matrix parses");
    assert_eq!(matrix["schemaVersion"], "scenario-coverage-v39");
    let scenarios = matrix["scenarios"].as_array().expect("scenarios array");
    assert!(
        scenarios.len() >= 12,
        "v39 enumerates the multi-agent production pipeline behaviors"
    );
    let mut ids = BTreeSet::new();
    let mut systems = BTreeSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().expect("scenario id");
        assert!(ids.insert(id.to_string()), "duplicate scenario id {id}");
        systems.insert(scenario["system"].as_str().expect("system").to_string());
        assert!(scenario["kind"].is_string(), "{id} has a kind");
        assert!(scenario["expect"].is_string(), "{id} has an expect");
    }
    for system in SYSTEMS {
        assert!(systems.contains(system), "missing coverage for {system}");
    }
    assert!(ids.contains("backcompat-single-agent-evolve"));
    assert!(ids.contains("backcompat-single-agent-apply"));
}

#[test]
fn v39_roles_ownership_regression() {
    let model =
        ProductionRoleOwnershipModel::from_json_str(&v39("roles.fixture.json")).expect("roles");
    let read = model.read_model();
    // Single owner per class; designer owns two.
    assert_eq!(read.ownership_count, 3);
    assert_eq!(
        read.ownership_by_role["designer"],
        vec!["design-brief", "requirement-extraction"]
    );
    // One authorized proposal; a non-owner proposal and a trusted write rejected.
    assert_eq!(read.authorized_count, 1);
    assert_eq!(read.rejected_count, 2);
    assert!(read
        .rejections
        .iter()
        .any(|r| r.reason.contains("is not the owning role")));
    assert!(read.rejections.iter().any(|r| r
        .reason
        .contains("direct trusted write is never authorized")));
}

#[test]
fn v39_handoff_conflict_regression() {
    let ledger =
        ProductionHandoffLedger::from_json_str(&v39("handoff.fixture.json")).expect("handoff");
    let read = ledger.read_model();
    assert_eq!(read.handoff_count, 4);
    assert_eq!(read.clean_count, 1);
    assert_eq!(read.conflict_count, 2);
    assert_eq!(read.stale_count, 1);
    // Resolutions: 1 accepted (clean), 2 blocked (conflict), 1 needs-fix (stale).
    assert_eq!(read.accepted_count, 1);
    assert_eq!(read.blocked_count, 2);
    assert_eq!(read.needs_fix_count, 1);
    // The two conflicting handoffs reference each other deterministically.
    let h2 = read
        .observations
        .iter()
        .find(|o| o.handoff_id == "v39-handoff-002")
        .expect("v39-handoff-002");
    assert_eq!(h2.resolution, "blocked");
    assert_eq!(h2.conflicts_with, vec!["v39-handoff-003".to_string()]);
    // The unresolved set is the conflict pair plus the stale handoff.
    assert_eq!(read.unresolved.len(), 3);
}

#[test]
fn v39_review_gates_regression() {
    let ledger =
        ProductionReviewGateLedger::from_json_str(&v39("gates.fixture.json")).expect("gates");
    let read = ledger.read_model();
    assert_eq!(read.gate_count, 5);
    assert_eq!(read.blocked_count, 3);
    assert_eq!(read.promote_allowed_count, 2);
    assert_eq!(read.veto_count, 1);
    // The high-risk pending-critic gate stays blocked; high-risk fully-approved clears.
    let g3 = read
        .audit
        .iter()
        .find(|r| r.gate_id == "v39-gate-003")
        .unwrap();
    assert_eq!(g3.outcome, "blocked");
    assert!(g3.reason.contains("requires an explicit critic approval"));
    let g5 = read
        .audit
        .iter()
        .find(|r| r.gate_id == "v39-gate-005")
        .unwrap();
    assert_eq!(g5.outcome, "promote-allowed");
    assert!(g5.reason.contains("never auto-applied"));
}

#[test]
fn v39_demo_progression_is_locked() {
    let before = ProductionReviewGateLedger::from_json_str(&read_text(
        "examples/production-pipeline-v1/demo/review-gate.before.fixture.json",
    ))
    .expect("demo before");
    let after = ProductionReviewGateLedger::from_json_str(&read_text(
        "examples/production-pipeline-v1/demo/review-gate.after.fixture.json",
    ))
    .expect("demo after");
    assert_eq!(before.read_model().promote_allowed_count, 0);
    assert_eq!(after.read_model().promote_allowed_count, 1);
}

#[test]
fn v39_backcompat_single_agent_evolve_apply_remains_valid() {
    let golden: Value =
        serde_json::from_str(&v39("backcompat.single-agent.golden.json")).expect("golden parses");
    assert_eq!(golden["schemaVersion"], "scenario-coverage-v39-backcompat");
    let references = golden["references"].as_array().expect("references array");
    let mut checked = BTreeSet::new();

    for reference in references {
        let id = reference["id"].as_str().expect("reference id");
        let contract = reference["contract"].as_str().expect("contract");
        let rel = reference["ref"].as_str().expect("ref");
        let expect_valid = reference["expectValid"].as_bool().expect("expectValid");
        assert!(
            expect_valid,
            "{id}: the golden expects the flow to stay valid"
        );

        match contract {
            "evolve-campaign-v1" => {
                let artifact =
                    read_evolve_campaign_artifact(repo_root().join(rel)).expect("evolve artifact");
                validate_evolve_campaign(&artifact).unwrap_or_else(|error| {
                    panic!("{id} single-agent evolve regressed: {error:#}")
                });
            }
            "safe-source-apply-demo-v1" => {
                SafeSourceApplyDemoArtifact::from_json_str(&read_text(rel))
                    .unwrap_or_else(|error| panic!("{id} single-agent apply regressed: {error:#}"));
            }
            other => panic!("{id}: unknown backcompat contract {other}"),
        }
        checked.insert(id.to_string());
    }

    assert!(checked.contains("backcompat-single-agent-evolve"));
    assert!(checked.contains("backcompat-single-agent-apply"));
}

#[test]
fn v39_fixtures_stay_read_only() {
    let roles =
        ProductionRoleOwnershipModel::from_json_str(&v39("roles.fixture.json")).expect("roles");
    assert!(roles.dashboard_compat.read_only);
    let handoff =
        ProductionHandoffLedger::from_json_str(&v39("handoff.fixture.json")).expect("handoff");
    assert!(handoff.dashboard_compat.read_only);
    let gates =
        ProductionReviewGateLedger::from_json_str(&v39("gates.fixture.json")).expect("gates");
    assert!(gates.dashboard_compat.read_only);
}
