//! Self-audit attribution contract tests for #2028 / Era L M69.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    SelfAuditAcceptanceStatus, SelfAuditAttributionContract, SelfAuditTrendDirection,
    SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

#[test]
fn contract_fixture_is_machine_checkable_and_extends_loop_coverage() {
    let contract = SelfAuditAttributionContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
    ))
    .expect("contract validates");

    assert_eq!(
        contract.schema_version,
        SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION
    );
    assert_eq!(
        contract.extends_schema_version,
        "loop-coverage-attribution-v1"
    );
    assert_eq!(contract.title_id, "era-i-engine-builder-deckbuilder");
    assert!(contract.anchor_issue_refs.contains(&"#1".to_string()));
    assert!(contract.anchor_issue_refs.contains(&"#23".to_string()));

    let pipeline: BTreeSet<_> = contract
        .evidence_pipeline
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        "openchrome",
        "scenario-verdicts",
        "four-gates",
        "design-integrity",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage-attribution",
        "evolve",
        "source-apply",
        "trust-gradient",
    ] {
        assert!(
            pipeline.contains(required),
            "missing pipeline ref {required}"
        );
    }
}

#[test]
fn mappings_link_real_evidence_to_milestones_gates_and_loop_stages() {
    let contract = SelfAuditAttributionContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
    ))
    .expect("contract validates");

    let milestone_ids: BTreeSet<_> = contract
        .milestone_mappings
        .iter()
        .map(|mapping| mapping.milestone_id.as_str())
        .collect();
    for required in ["m68-real-title-run", "m68-demo", "m68-coverage-v60"] {
        assert!(
            milestone_ids.contains(required),
            "missing mapping {required}"
        );
    }

    for mapping in &contract.milestone_mappings {
        assert!(mapping.issue_ref.starts_with('#'));
        assert!(contract.loop_stages.contains(&mapping.loop_stage));
        assert!(mapping
            .attribution_refs
            .iter()
            .any(|reference| reference.contains("loop-coverage")));
        assert!(!mapping.failure_signal_kinds.is_empty());
        for reference in &mapping.evidence_refs {
            assert!(
                repo_root().join(reference).is_file(),
                "missing evidence ref {reference}"
            );
        }
    }
}

#[test]
fn acceptance_audits_point_to_issue_1_success_criteria_and_trends() {
    let contract = SelfAuditAttributionContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
    ))
    .expect("contract validates");
    let trend_ids: BTreeSet<_> = contract
        .trend_definitions
        .iter()
        .map(|trend| trend.trend_id.as_str())
        .collect();

    for audit in &contract.acceptance_audits {
        assert!(audit.success_criterion_ref.starts_with("#1:"));
        assert_eq!(audit.status, SelfAuditAcceptanceStatus::Satisfied);
        assert!(trend_ids.contains(audit.trend_ref.as_str()));
        for predicate in &audit.evidence_predicates {
            assert!(repo_root().join(&predicate.source_ref).is_file());
            assert!(["equals", "exists", "not-empty", "contains", "lte", "gte"]
                .contains(&predicate.operator.as_str()));
        }
    }
}

#[test]
fn trend_definitions_are_bounded_regression_rules_not_a_new_verifier() {
    let contract = SelfAuditAttributionContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
    ))
    .expect("contract validates");
    assert!(contract
        .trend_definitions
        .iter()
        .any(|trend| trend.direction == SelfAuditTrendDirection::Unchanged));
    for trend in &contract.trend_definitions {
        assert!(repo_root().join(&trend.baseline_ref).is_file());
        assert!(repo_root().join(&trend.current_ref).is_file());
        assert!(trend
            .regression_when
            .iter()
            .any(|rule| rule.to_ascii_lowercase().contains("regress")));
    }
}

#[test]
fn docs_and_fixture_preserve_autonomy_boundaries() {
    let doc = read_text("docs/self-audit-bottleneck-attribution-contract-v1.md");
    let lower = doc.to_ascii_lowercase();
    for required in [
        "loop-coverage-attribution-v1",
        "ledger.jsonl",
        "journal.md",
        "verdict",
        "no new verification engine",
        "no new data plane",
        "human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(lower.contains(required), "doc missing {required}");
    }

    let fixture =
        read_json("examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json");
    assert_eq!(
        fixture["schemaVersion"],
        SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION
    );
    assert_eq!(
        fixture["extendsSchemaVersion"],
        "loop-coverage-attribution-v1"
    );
    assert!(fixture["boundary"]
        .as_str()
        .expect("boundary")
        .contains("no new data plane"));
}
