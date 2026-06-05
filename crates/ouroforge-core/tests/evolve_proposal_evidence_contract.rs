use ouroforge_core::{
    add_evidence_artifact, create_run, evolve_run, list_mutation_proposals,
    MutationProposalBoundedMutationType, MutationProposalEvidenceState,
    MutationProposalGateCategory, MutationProposalRationaleConfidence,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

const SEED: &str = r#"
id: evolve.proposal.evidence.contract
title: Evolve Proposal Evidence Contract
goal: Prove failed evidence creates bounded, evidence-linked mutation proposals.
constraints:
  target: local-fixture
acceptance:
  - Mutation proposal cites a failing gate and evidence.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed failure scenario.
"#;

#[test]
fn evolve_proposal_cites_gate_and_evidence_for_each_four_gate_failure() {
    for case in [
        GateCase {
            name: "mechanical",
            failure: json!({
                "kind": "scenario_failed",
                "path": "evidence/scenarios/collect-and-exit/scenario-result.json"
            }),
            evidence_id: "scenario-result",
            evidence_path: "evidence/scenarios/collect-and-exit/scenario-result.json",
            artifact_metadata: json!({"artifact":"scenario_result"}),
            expected_gate: MutationProposalGateCategory::Mechanical,
            expected_bounded_type: MutationProposalBoundedMutationType::Scenario,
            expected_confidence: MutationProposalRationaleConfidence::High,
        },
        GateCase {
            name: "runtime",
            failure: json!({
                "kind": "behavior_assertion_failed",
                "evidence_ref": "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json"
            }),
            evidence_id: "runtime-probe",
            evidence_path: "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
            artifact_metadata: json!({"artifact":"runtime_probe"}),
            expected_gate: MutationProposalGateCategory::Runtime,
            expected_bounded_type: MutationProposalBoundedMutationType::Data,
            expected_confidence: MutationProposalRationaleConfidence::High,
        },
        GateCase {
            name: "visual",
            failure: json!({
                "kind": "visual_gate_failed",
                "state": "fail",
                "path": "evidence/scenarios/collect-and-exit/visual/visual-comparison.json"
            }),
            evidence_id: "visual-comparison",
            evidence_path: "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
            artifact_metadata: json!({"artifact":"visual_comparison_evidence", "gate":"visual", "declaredAcceptance": true}),
            expected_gate: MutationProposalGateCategory::Visual,
            expected_bounded_type: MutationProposalBoundedMutationType::Scene,
            expected_confidence: MutationProposalRationaleConfidence::High,
        },
        GateCase {
            name: "semantic",
            failure: json!({
                "kind": "semantic_gate_failed",
                "state": "fail",
                "model_ref": "evidence/scenarios/collect-and-exit/semantic/runtime-invariant-model.json",
                "world_state_ref": "evidence/scenarios/collect-and-exit/world-state.json"
            }),
            evidence_id: "semantic-model",
            evidence_path:
                "evidence/scenarios/collect-and-exit/semantic/runtime-invariant-model.json",
            artifact_metadata: json!({"artifact":"runtime_invariant_model", "gate":"semantic", "declaredAcceptance": true}),
            expected_gate: MutationProposalGateCategory::Semantic,
            expected_bounded_type: MutationProposalBoundedMutationType::Data,
            expected_confidence: MutationProposalRationaleConfidence::High,
        },
    ] {
        let (root, run_dir) = create_fixture_run(&format!("evolve-proposal-{}", case.name));
        write_indexed_evidence(
            &run_dir,
            "scenario-result",
            "evidence/scenarios/collect-and-exit/scenario-result.json",
            json!({"artifact":"scenario_result"}),
        );
        if case.evidence_id != "scenario-result" {
            write_indexed_evidence(
                &run_dir,
                case.evidence_id,
                case.evidence_path,
                case.artifact_metadata.clone(),
            );
        }
        write_failed_verdict(
            &run_dir,
            case.failure.clone(),
            vec![
                "evidence/scenarios/collect-and-exit/scenario-result.json",
                case.evidence_path,
            ],
        );

        let summary = evolve_run(&run_dir).expect("evolve creates proposal");
        assert_eq!(summary.status, "proposed", "{}", case.name);
        assert_eq!(summary.proposals_created, 1, "{}", case.name);
        let proposals = list_mutation_proposals(&run_dir).expect("proposal list");
        let rationale = proposals[0].rationale.as_ref().expect("rationale");

        assert_eq!(proposals[0].evidence_id, case.evidence_id, "{}", case.name);
        assert_eq!(
            rationale.failing_gate_category,
            Some(case.expected_gate),
            "{}",
            case.name
        );
        assert_eq!(
            rationale.justifying_evidence_ref.as_deref(),
            Some(case.evidence_path),
            "{}",
            case.name
        );
        assert_eq!(
            rationale.evidence_state,
            Some(MutationProposalEvidenceState::Linked),
            "{}",
            case.name
        );
        assert_eq!(
            rationale.bounded_mutation_type,
            Some(case.expected_bounded_type),
            "{}",
            case.name
        );
        assert_eq!(
            rationale.confidence, case.expected_confidence,
            "{}",
            case.name
        );
        assert!(
            rationale.reasoning_summary.contains("evidence"),
            "{}",
            case.name
        );

        fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn evolve_proposal_fails_closed_when_justifying_evidence_is_missing() {
    let (root, run_dir) = create_fixture_run("evolve-proposal-missing-evidence");
    write_indexed_evidence(
        &run_dir,
        "unrelated",
        "evidence/scenarios/collect-and-exit/unrelated.json",
        json!({"artifact":"unrelated"}),
    );
    write_failed_verdict(
        &run_dir,
        json!({
            "kind": "visual_gate_failed",
            "state": "fail",
            "path": "evidence/scenarios/collect-and-exit/visual/missing-comparison.json"
        }),
        vec!["evidence/scenarios/collect-and-exit/visual/missing-comparison.json"],
    );

    let summary = evolve_run(&run_dir).expect("missing evidence is fail-closed summary");

    assert_eq!(summary.status, "missing-evidence");
    assert_eq!(summary.proposals_created, 0);
    assert!(summary
        .reason
        .contains("no mutation proposal was fabricated"));
    assert!(list_mutation_proposals(&run_dir)
        .expect("proposal list")
        .is_empty());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn evolve_proposal_fails_closed_when_justifying_evidence_is_stale() {
    let (root, run_dir) = create_fixture_run("evolve-proposal-stale-evidence");
    write_indexed_evidence(
        &run_dir,
        "unrelated",
        "evidence/scenarios/collect-and-exit/unrelated.json",
        json!({"artifact":"unrelated"}),
    );
    write_failed_verdict(
        &run_dir,
        json!({
            "kind": "visual_gate_failed",
            "state": "stale-ref",
            "path": "evidence/scenarios/collect-and-exit/visual/stale-comparison.json"
        }),
        vec!["evidence/scenarios/collect-and-exit/visual/stale-comparison.json"],
    );

    let summary = evolve_run(&run_dir).expect("stale evidence is fail-closed summary");

    assert_eq!(summary.status, "stale-ref");
    assert_eq!(summary.proposals_created, 0);
    assert!(summary
        .reason
        .contains("no mutation proposal was fabricated"));
    assert!(list_mutation_proposals(&run_dir)
        .expect("proposal list")
        .is_empty());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn evolve_proposal_fails_closed_for_unsupported_gate_even_with_linked_evidence() {
    let (root, run_dir) = create_fixture_run("evolve-proposal-unsupported-gate");
    write_indexed_evidence(
        &run_dir,
        "source-patch-request",
        "evidence/scenarios/collect-and-exit/source-patch-request.json",
        json!({"artifact":"source_patch_request"}),
    );
    write_failed_verdict(
        &run_dir,
        json!({
            "kind": "source_patch_requested",
            "gate_category": "source_patch",
            "path": "evidence/scenarios/collect-and-exit/source-patch-request.json"
        }),
        vec!["evidence/scenarios/collect-and-exit/source-patch-request.json"],
    );

    let summary = evolve_run(&run_dir).expect("unsupported gate is fail-closed summary");

    assert_eq!(summary.status, "unsupported");
    assert_eq!(summary.proposals_created, 0);
    assert!(summary
        .reason
        .contains("no mutation proposal was fabricated"));
    assert!(list_mutation_proposals(&run_dir)
        .expect("proposal list")
        .is_empty());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn mutation_proposal_rejects_unsupported_gate_category_and_bounded_type_drift() {
    let (root, run_dir) = create_fixture_run("evolve-proposal-invalid-rationale");
    fs::create_dir_all(run_dir.join("mutation")).unwrap();
    fs::write(
        run_dir.join("mutation/proposals.json"),
        serde_json::to_vec_pretty(&json!({
            "proposals": [{
                "id": "mutation-1",
                "reason": "invalid broad source mutation",
                "evidence_id": "evidence-1",
                "target": "src/lib.rs",
                "path": "source",
                "from": "old",
                "to": "new",
                "confidence": "medium",
                "status": "proposed",
                "verdict_status": "failed",
                "created_at_unix_ms": 1,
                "rationale": {
                    "schema_version": "1",
                    "failure_classification": "semantic_root_cause",
                    "evidence_artifact_ids": ["evidence-1"],
                    "scenario_result_refs": ["evidence/scenarios/collect-and-exit/scenario-result.json"],
                    "verdict_refs": ["verdict.json"],
                    "expected_effect": "invalid source patch",
                    "confidence": "medium",
                    "reasoning_summary": "invalid",
                    "allowed_mutation_type": "data_only",
                    "failing_gate_category": "source_patch",
                    "justifying_evidence_ref": "evidence/source.json",
                    "evidence_state": "linked",
                    "bounded_mutation_type": "source"
                }
            }]
        }))
        .unwrap(),
    )
    .unwrap();

    let error = list_mutation_proposals(&run_dir).expect_err("unsupported enums fail");

    assert!(error
        .to_string()
        .contains("failed to parse mutation proposals"));
    fs::remove_dir_all(root).unwrap();
}

struct GateCase {
    name: &'static str,
    failure: Value,
    evidence_id: &'static str,
    evidence_path: &'static str,
    artifact_metadata: Value,
    expected_gate: MutationProposalGateCategory,
    expected_bounded_type: MutationProposalBoundedMutationType,
    expected_confidence: MutationProposalRationaleConfidence,
}

fn create_fixture_run(prefix: &str) -> (PathBuf, PathBuf) {
    let root = std::env::temp_dir().join(format!("ouroforge-{prefix}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let seed_path = root.join("seed.yaml");
    fs::write(&seed_path, SEED).unwrap();
    let artifacts = create_run(&seed_path, root.join("runs")).unwrap();
    (root, artifacts.run_dir)
}

fn write_indexed_evidence(run_dir: &Path, id: &str, rel: &str, metadata: Value) {
    let path = run_dir.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, "{}\n").unwrap();
    add_evidence_artifact(run_dir, id, "application/json", rel, metadata).unwrap();
}

fn write_failed_verdict(run_dir: &Path, failure: Value, evidence_refs: Vec<&str>) {
    fs::write(
        run_dir.join("verdict.json"),
        serde_json::to_vec_pretty(&json!({
            "status": "failed",
            "summary": "fixture failed with evidence-linked gate",
            "failures": [failure],
            "evidence_refs": evidence_refs,
            "metadata": {}
        }))
        .unwrap(),
    )
    .unwrap();
}
