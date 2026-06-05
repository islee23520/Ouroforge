use ouroforge_core::{
    add_evidence_artifact, create_run, evolve_run, list_mutation_proposals,
    write_mutation_backlog_artifact, MutationBacklogArtifact, MutationBacklogItem,
    MutationBacklogSeverity, MutationClassificationCategory, MutationProposalBoundedMutationType,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

const SEED: &str = r#"
id: evolve.proposal.selection.contract
title: Evolve Proposal Selection Contract
goal: Prove classified failures select bounded proposals from a read-only backlog.
constraints:
  target: local-fixture
acceptance:
  - Classification-driven proposal selection is bounded and review-only.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed failure scenario.
"#;

#[test]
fn classification_taxonomy_maps_to_bounded_proposal_types() {
    for case in mapped_cases() {
        let (root, run_dir) = create_fixture_run(&format!("evolve-selection-{}", case.name));
        write_indexed_evidence(
            &run_dir,
            "failure-evidence",
            case.evidence_path,
            case.metadata,
        );
        write_failed_verdict(&run_dir, case.failure, vec![case.evidence_path]);
        write_backlog_item(
            &run_dir,
            "backlog-1",
            "classification-1",
            case.category.clone(),
            case.expected_type.clone(),
            MutationBacklogSeverity::Medium,
            vec![case.evidence_path],
        );

        let summary = evolve_run(&run_dir).expect("mapped class creates proposal");

        assert_eq!(summary.status, "proposed", "{}", case.name);
        assert_eq!(summary.proposals_created, 1, "{}", case.name);
        let proposals = list_mutation_proposals(&run_dir).expect("proposal list");
        let rationale = proposals[0].rationale.as_ref().expect("rationale");
        assert_eq!(
            rationale.bounded_mutation_type,
            Some(case.expected_type.clone()),
            "{}",
            case.name
        );
        assert_eq!(
            rationale.failure_classification, case.expected_label,
            "{}",
            case.name
        );
        assert_eq!(
            rationale.selection_backlog_item_id.as_deref(),
            Some("backlog-1"),
            "{}",
            case.name
        );
        assert_eq!(
            rationale.selection_source.as_deref(),
            Some("mutation/backlog.json")
        );
        assert_eq!(rationale.backlog_read_only, Some(true));

        fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn backlog_only_classes_do_not_fabricate_proposals() {
    for (name, marker, category, label) in [
        (
            "flaky",
            "flaky rerun marker",
            MutationClassificationCategory::Flaky,
            "flaky",
        ),
        (
            "unsupported",
            "unsupported mechanic marker",
            MutationClassificationCategory::Unsupported,
            "unsupported",
        ),
        (
            "unknown",
            "opaque marker",
            MutationClassificationCategory::Unknown,
            "unknown",
        ),
    ] {
        let (root, run_dir) = create_fixture_run(&format!("evolve-selection-{name}"));
        write_indexed_evidence(
            &run_dir,
            "failure-evidence",
            "evidence/scenarios/collect-and-exit/scenario-result.json",
            json!({"artifact":"scenario_result", "marker": marker}),
        );
        write_failed_verdict(
            &run_dir,
            json!({
                "kind": "classified_failure",
                "classification": label,
                "summary": marker,
                "path": "evidence/scenarios/collect-and-exit/scenario-result.json"
            }),
            vec!["evidence/scenarios/collect-and-exit/scenario-result.json"],
        );
        if category != MutationClassificationCategory::Unknown {
            write_backlog_item(
                &run_dir,
                "backlog-only",
                "classification-1",
                category,
                MutationProposalBoundedMutationType::Data,
                MutationBacklogSeverity::High,
                vec!["evidence/scenarios/collect-and-exit/scenario-result.json"],
            );
        }

        let summary = evolve_run(&run_dir).expect("backlog-only class is handled");

        assert_eq!(summary.status, "backlog-only", "{name}");
        assert_eq!(summary.proposals_created, 0, "{name}");
        assert!(list_mutation_proposals(&run_dir)
            .expect("proposal list")
            .is_empty());
        fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn selection_validates_missing_stale_and_bounded_backlog_refs() {
    for case in [
        InvalidBacklogCase {
            name: "missing-backlog-ref",
            classification_id: "classification-missing",
            backlog_category: MutationClassificationCategory::GameplayLogic,
            backlog_type: MutationProposalBoundedMutationType::Data,
            evidence_refs: vec![
                "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
            ],
            expected_error: "missing-backlog-ref",
        },
        InvalidBacklogCase {
            name: "stale-ref",
            classification_id: "classification-1",
            backlog_category: MutationClassificationCategory::VisualMismatch,
            backlog_type: MutationProposalBoundedMutationType::Scene,
            evidence_refs: vec!["evidence/scenarios/collect-and-exit/visual/stale.json"],
            expected_error: "stale-ref",
        },
        InvalidBacklogCase {
            name: "missing-classification",
            classification_id: "classification-missing",
            backlog_category: MutationClassificationCategory::VisualMismatch,
            backlog_type: MutationProposalBoundedMutationType::Scene,
            evidence_refs: vec![
                "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
            ],
            expected_error: "missing-classification",
        },
        InvalidBacklogCase {
            name: "bounded-type-violation",
            classification_id: "classification-1",
            backlog_category: MutationClassificationCategory::VisualMismatch,
            backlog_type: MutationProposalBoundedMutationType::Data,
            evidence_refs: vec![
                "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
            ],
            expected_error: "bounded-type-violation",
        },
    ] {
        let (root, run_dir) = create_fixture_run(&format!("evolve-selection-{}", case.name));
        write_indexed_evidence(
            &run_dir,
            "visual-evidence",
            "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
            json!({"artifact":"visual_comparison_evidence", "gate":"visual"}),
        );
        write_failed_verdict(
            &run_dir,
            json!({
                "kind": "visual_gate_failed",
                "path": "evidence/scenarios/collect-and-exit/visual/visual-comparison.json"
            }),
            vec!["evidence/scenarios/collect-and-exit/visual/visual-comparison.json"],
        );
        write_backlog_item(
            &run_dir,
            "bad-backlog",
            case.classification_id,
            case.backlog_category.clone(),
            case.backlog_type.clone(),
            MutationBacklogSeverity::Critical,
            case.evidence_refs,
        );

        let error = evolve_run(&run_dir).expect_err("invalid backlog blocks proposal");

        assert!(
            error.to_string().contains(case.expected_error),
            "{:#}",
            error
        );
        assert!(list_mutation_proposals(&run_dir)
            .expect("proposal list")
            .is_empty());
        fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn backlog_selection_is_read_only_and_prefers_severity_with_repro_context() {
    let (root, run_dir) = create_fixture_run("evolve-selection-read-only-backlog");
    write_indexed_evidence(
        &run_dir,
        "runtime-evidence",
        "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
        json!({"artifact":"runtime_probe"}),
    );
    write_indexed_evidence(
        &run_dir,
        "scenario-result",
        "evidence/scenarios/collect-and-exit/scenario-result.json",
        json!({"artifact":"scenario_result"}),
    );
    write_failed_verdict(
        &run_dir,
        json!({
            "kind": "runtime_probe_failed",
            "path": "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json"
        }),
        vec![
            "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
            "evidence/scenarios/collect-and-exit/scenario-result.json",
        ],
    );
    write_backlog_items(
        &run_dir,
        vec![
            backlog_item(
                "medium-item",
                "classification-1",
                MutationClassificationCategory::ProbeFailure,
                MutationProposalBoundedMutationType::Data,
                MutationBacklogSeverity::Medium,
                vec![
                    "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
                    "evidence/scenarios/collect-and-exit/scenario-result.json",
                ],
            ),
            backlog_item(
                "critical-item",
                "classification-1",
                MutationClassificationCategory::ProbeFailure,
                MutationProposalBoundedMutationType::Data,
                MutationBacklogSeverity::Critical,
                vec![
                    "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
                    "evidence/scenarios/collect-and-exit/scenario-result.json",
                ],
            ),
        ],
    );
    let before = fs::read(run_dir.join("mutation/backlog.json")).unwrap();

    let summary = evolve_run(&run_dir).expect("runtime backlog selects proposal");

    assert_eq!(summary.status, "proposed");
    let after = fs::read(run_dir.join("mutation/backlog.json")).unwrap();
    assert_eq!(before, after, "selection must consume backlog read-only");
    let proposals = list_mutation_proposals(&run_dir).expect("proposal list");
    let rationale = proposals[0].rationale.as_ref().expect("rationale");
    assert_eq!(
        rationale.selection_backlog_item_id.as_deref(),
        Some("critical-item")
    );
    assert_eq!(rationale.backlog_read_only, Some(true));
    assert!(rationale
        .selection_reason
        .as_deref()
        .unwrap_or_default()
        .contains("without mutating backlog state"));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn backlog_selection_prefers_higher_severity_across_later_classifications() {
    let (root, run_dir) = create_fixture_run("evolve-selection-global-severity");
    write_indexed_evidence(
        &run_dir,
        "probe-evidence",
        "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
        json!({"artifact":"runtime_probe"}),
    );
    write_indexed_evidence(
        &run_dir,
        "gameplay-evidence",
        "evidence/scenarios/collect-and-exit/gameplay/gameplay-failure.json",
        json!({"artifact":"gameplay_failure"}),
    );
    write_indexed_evidence(
        &run_dir,
        "scenario-result",
        "evidence/scenarios/collect-and-exit/scenario-result.json",
        json!({"artifact":"scenario_result"}),
    );
    // A verdict with TWO failures yields classification-1 (probe) and classification-2
    // (gameplay), in that order.
    fs::write(
        run_dir.join("verdict.json"),
        serde_json::to_vec_pretty(&json!({
            "status": "failed",
            "summary": "two distinct classified failures",
            "failures": [
                {
                    "kind": "probe_failed",
                    "classification_category": "probe_failure",
                    "path": "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json"
                },
                {
                    "kind": "gameplay_logic_failed",
                    "classification_category": "gameplay_logic",
                    "path": "evidence/scenarios/collect-and-exit/gameplay/gameplay-failure.json"
                }
            ],
            "evidence_refs": [
                "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
                "evidence/scenarios/collect-and-exit/gameplay/gameplay-failure.json"
            ],
            "metadata": {}
        }))
        .unwrap(),
    )
    .unwrap();
    // classification-1 (probe) carries only a Medium backlog item, while the LATER
    // classification-2 (gameplay) carries a higher-severity Critical item.
    write_backlog_items(
        &run_dir,
        vec![
            backlog_item(
                "probe-medium",
                "classification-1",
                MutationClassificationCategory::ProbeFailure,
                MutationProposalBoundedMutationType::Data,
                MutationBacklogSeverity::Medium,
                vec!["evidence/scenarios/collect-and-exit/runtime/runtime-probe.json"],
            ),
            backlog_item(
                "gameplay-critical",
                "classification-2",
                MutationClassificationCategory::GameplayLogic,
                MutationProposalBoundedMutationType::Data,
                MutationBacklogSeverity::Critical,
                vec!["evidence/scenarios/collect-and-exit/gameplay/gameplay-failure.json"],
            ),
        ],
    );

    let summary = evolve_run(&run_dir).expect("multi-classification backlog selects a proposal");
    assert_eq!(summary.status, "proposed");

    let proposals = list_mutation_proposals(&run_dir).expect("proposal list");
    let rationale = proposals[0].rationale.as_ref().expect("rationale");
    // The globally highest-severity backlog item must win even though it belongs to the
    // later classification-2 rather than the first classification.
    assert_eq!(
        rationale.selection_backlog_item_id.as_deref(),
        Some("gameplay-critical")
    );

    fs::remove_dir_all(root).unwrap();
}

#[derive(Clone)]
struct MappedCase {
    name: &'static str,
    failure: Value,
    evidence_path: &'static str,
    metadata: Value,
    category: MutationClassificationCategory,
    expected_label: &'static str,
    expected_type: MutationProposalBoundedMutationType,
}

fn mapped_cases() -> Vec<MappedCase> {
    vec![
        mapped(
            "gameplay",
            "gameplay logic assertion",
            MutationClassificationCategory::GameplayLogic,
            "gameplay_logic",
            MutationProposalBoundedMutationType::Data,
        ),
        mapped(
            "level",
            "level design route",
            MutationClassificationCategory::LevelDesign,
            "level_design",
            MutationProposalBoundedMutationType::Scene,
        ),
        mapped(
            "asset",
            "asset sprite missing",
            MutationClassificationCategory::Asset,
            "asset",
            MutationProposalBoundedMutationType::Data,
        ),
        mapped(
            "physics",
            "physics/collision overlap",
            MutationClassificationCategory::PhysicsCollision,
            "physics_collision",
            MutationProposalBoundedMutationType::Data,
        ),
        mapped(
            "input",
            "input control dropped",
            MutationClassificationCategory::Input,
            "input",
            MutationProposalBoundedMutationType::Scenario,
        ),
        mapped(
            "performance",
            "performance metric over budget",
            MutationClassificationCategory::PerformanceRegression,
            "performance_regression",
            MutationProposalBoundedMutationType::Data,
        ),
        mapped(
            "visual",
            "visual screenshot mismatch",
            MutationClassificationCategory::VisualMismatch,
            "visual_mismatch",
            MutationProposalBoundedMutationType::Scene,
        ),
        mapped(
            "runtime-crash",
            "runtime crash stacktrace",
            MutationClassificationCategory::RuntimeCrash,
            "runtime_crash",
            MutationProposalBoundedMutationType::Data,
        ),
        mapped(
            "console",
            "console error emitted",
            MutationClassificationCategory::ConsoleError,
            "console_error",
            MutationProposalBoundedMutationType::Data,
        ),
        mapped(
            "probe",
            "probe failure evidence",
            MutationClassificationCategory::ProbeFailure,
            "probe_failure",
            MutationProposalBoundedMutationType::Data,
        ),
    ]
}

fn mapped(
    name: &'static str,
    summary: &'static str,
    category: MutationClassificationCategory,
    expected_label: &'static str,
    expected_type: MutationProposalBoundedMutationType,
) -> MappedCase {
    MappedCase {
        name,
        failure: json!({
            "kind": "classified_failure",
            "classification": expected_label,
            "summary": summary,
            "path": "evidence/scenarios/collect-and-exit/scenario-result.json"
        }),
        evidence_path: "evidence/scenarios/collect-and-exit/scenario-result.json",
        metadata: json!({"artifact":"scenario_result", "summary": summary}),
        category,
        expected_label,
        expected_type,
    }
}

struct InvalidBacklogCase {
    name: &'static str,
    classification_id: &'static str,
    backlog_category: MutationClassificationCategory,
    backlog_type: MutationProposalBoundedMutationType,
    evidence_refs: Vec<&'static str>,
    expected_error: &'static str,
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
            "summary": failure.get("summary").and_then(Value::as_str).unwrap_or("fixture failed"),
            "failures": [failure],
            "evidence_refs": evidence_refs,
            "metadata": {}
        }))
        .unwrap(),
    )
    .unwrap();
}

fn write_backlog_item(
    run_dir: &Path,
    id: &str,
    classification_id: &str,
    category: MutationClassificationCategory,
    bounded_type: MutationProposalBoundedMutationType,
    severity: MutationBacklogSeverity,
    evidence_refs: Vec<&str>,
) {
    write_backlog_items(
        run_dir,
        vec![backlog_item(
            id,
            classification_id,
            category,
            bounded_type,
            severity,
            evidence_refs,
        )],
    );
}

fn write_backlog_items(run_dir: &Path, items: Vec<MutationBacklogItem>) {
    let artifact = MutationBacklogArtifact {
        schema_version: "1".to_string(),
        run_id: "run-selection-fixture".to_string(),
        items,
    };
    write_mutation_backlog_artifact(run_dir, &artifact).unwrap();
}

fn backlog_item(
    id: &str,
    classification_id: &str,
    category: MutationClassificationCategory,
    bounded_type: MutationProposalBoundedMutationType,
    severity: MutationBacklogSeverity,
    evidence_refs: Vec<&str>,
) -> MutationBacklogItem {
    MutationBacklogItem {
        id: id.to_string(),
        classification_id: classification_id.to_string(),
        failure_class: category,
        bounded_mutation_type: bounded_type,
        severity,
        reproduction_context: "scenario collect-and-exit reproduces locally".to_string(),
        evidence_refs: evidence_refs.into_iter().map(str::to_string).collect(),
        suggested_next_investigation: "review linked evidence before any manual mutation"
            .to_string(),
        owner_lane: "evolve-depth".to_string(),
        review_status: "open".to_string(),
        blocked_reasons: Vec::new(),
    }
}
