//! Scenario Coverage v111: M130 production usability gate (#2391-#2394).

use ouroforge_protocols::production_usability_gate::{
    ComparisonVerdict, ManualGapLedgerEntry, ProductionUsabilityGate, ProductionUsabilityPhase,
    ProductionUsabilityPhaseKind, ProductionUsabilityVerdict,
    PRODUCTION_USABILITY_GATE_SCHEMA_VERSION, PRODUCTION_USABILITY_SCENARIO_COVERAGE,
};
use std::{fs, path::PathBuf};

fn repo_file(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn valid_gate() -> ProductionUsabilityGate {
    let text = fs::read_to_string(repo_file(
        "examples/production-usability-gate-v111/gate.fixture.json",
    ))
    .expect("read gate fixture");
    serde_json::from_str(&text).expect("parse gate fixture")
}

#[test]
fn scenario_coverage_v111_locks_final_m130_gate_contract() {
    let gate = valid_gate();
    gate.validate().expect("gate validates");
    assert_eq!(
        gate.schema_version,
        PRODUCTION_USABILITY_GATE_SCHEMA_VERSION
    );
    assert_eq!(
        gate.scenario_coverage_suite,
        PRODUCTION_USABILITY_SCENARIO_COVERAGE
    );
    assert_eq!(
        gate.phases.iter().map(|p| p.issue).collect::<Vec<_>>(),
        vec![2391, 2392, 2393, 2394]
    );
    assert_eq!(gate.anchors_remain_open, vec![1, 23]);
}

#[test]
fn scenario_coverage_v111_requires_2391_transcript_screenshots_and_manual_gap_ledger() {
    let mut gate = valid_gate();
    let phase = gate.phases.iter_mut().find(|p| p.issue == 2391).unwrap();
    phase.workflow_transcript_refs.clear();
    assert!(gate
        .validate()
        .unwrap_err()
        .to_string()
        .contains("#2391 requires workflow transcript"));

    let mut gate = valid_gate();
    gate.phases
        .iter_mut()
        .find(|p| p.issue == 2391)
        .unwrap()
        .manual_gaps
        .clear();
    assert!(gate
        .validate()
        .unwrap_err()
        .to_string()
        .contains("#2391 must enumerate every manual step"));
}

#[test]
fn scenario_coverage_v111_requires_2392_comparison_verdict() {
    let mut gate = valid_gate();
    gate.phases
        .iter_mut()
        .find(|p| p.issue == 2392)
        .unwrap()
        .comparison_verdict = None;
    assert!(gate
        .validate()
        .unwrap_err()
        .to_string()
        .contains("#2392 requires an improvement or regression"));
}

#[test]
fn scenario_coverage_v111_requires_2393_local_package_refs() {
    let mut gate = valid_gate();
    gate.phases
        .iter_mut()
        .find(|p| p.issue == 2393)
        .unwrap()
        .package_refs
        .clear();
    assert!(gate
        .validate()
        .unwrap_err()
        .to_string()
        .contains("#2393 requires local package"));
}

#[test]
fn scenario_coverage_v111_rejects_missing_governance_handoff() {
    let mut gate = valid_gate();
    gate.phases
        .iter_mut()
        .find(|p| p.issue == 2394)
        .unwrap()
        .governance_refs
        .clear();
    assert!(gate
        .validate()
        .unwrap_err()
        .to_string()
        .contains("#2394 requires #1/roadmap/backlog"));
}

#[test]
fn scenario_coverage_v111_manual_fixture_can_record_honest_regression() {
    let phase = ProductionUsabilityPhase {
        issue: 2392,
        kind: ProductionUsabilityPhaseKind::StudioEditRerun,
        verdict: ProductionUsabilityVerdict::RegressionRecorded,
        evidence_refs: vec!["runs/m130/2392/comparison.json".to_string()],
        screenshot_refs: vec![],
        workflow_transcript_refs: vec![],
        manual_gaps: vec![ManualGapLedgerEntry {
            gap_id: "regression-recorded".to_string(),
            owner_issue: "#2394".to_string(),
            summary: "comparison verdict regressed and must stay visible".to_string(),
            follow_up: "keep in postmortem backlog".to_string(),
        }],
        comparison_verdict: Some(ComparisonVerdict::Regression),
        package_refs: vec![],
        governance_refs: vec![],
    };
    phase
        .validate()
        .expect("regression is honest when comparison verdict is explicit");
}
