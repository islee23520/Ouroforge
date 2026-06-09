//! Milestone acceptance meta-evaluation tests for #2030 / Era L M69.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    evaluate_self_audit_acceptance, SelfAuditAcceptanceStatus, SelfAuditAttributionContract,
    SelfAuditEvidenceDocument, SELF_AUDIT_ACCEPTANCE_EVALUATOR_SCHEMA_VERSION,
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

fn contract() -> SelfAuditAttributionContract {
    SelfAuditAttributionContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
    ))
    .expect("contract fixture validates")
}

fn real_evidence() -> Vec<SelfAuditEvidenceDocument> {
    vec![
        doc("examples/real-title-dogfood-v1/run/verdict.json"),
        doc("examples/real-title-dogfood-v1/release-provenance.complete.json"),
        doc("examples/real-title-dogfood-v1/demo/friction-summary.fixture.json"),
        doc("examples/real-title-dogfood-v1/demo/audit-trail.fixture.json"),
        doc("examples/real-title-dogfood-v1/scenario-coverage-v60/matrix.fixture.json"),
        // Existing-pipeline anchors required by the meta-evaluator boundary.
        SelfAuditEvidenceDocument {
            source_ref: "examples/real-title-dogfood-v1/run/ledger.jsonl".to_string(),
            document: serde_json::json!({"kind":"existing-ledger-jsonl-anchor"}),
        },
        SelfAuditEvidenceDocument {
            source_ref: "examples/real-title-dogfood-v1/run/journal.md".to_string(),
            document: serde_json::json!({"kind":"existing-journal-anchor"}),
        },
        SelfAuditEvidenceDocument {
            source_ref: "examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json"
                .to_string(),
            document: read_json(
                "examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json",
            ),
        },
    ]
}

fn doc(path: &str) -> SelfAuditEvidenceDocument {
    SelfAuditEvidenceDocument {
        source_ref: path.to_string(),
        document: read_json(path),
    }
}

#[test]
fn each_milestone_gets_evidence_backed_acceptance_verdict() {
    let report = evaluate_self_audit_acceptance(&contract(), &real_evidence()).expect("report");
    assert_eq!(
        report.schema_version,
        SELF_AUDIT_ACCEPTANCE_EVALUATOR_SCHEMA_VERSION
    );
    assert_eq!(report.declared_gate_operator, "declared-gate-and");
    assert_eq!(report.status, SelfAuditAcceptanceStatus::Satisfied);
    assert_eq!(report.milestone_verdicts.len(), 3);

    for verdict in &report.milestone_verdicts {
        assert_eq!(verdict.declared_gate, "self-audit-acceptance");
        assert_eq!(verdict.status, SelfAuditAcceptanceStatus::Satisfied);
        assert!(verdict.success_criterion_ref.starts_with("#1:"));
        assert!(!verdict.evidence_refs.is_empty());
        assert!(verdict
            .predicate_results
            .iter()
            .all(|predicate| predicate.passed));
        for reference in &verdict.evidence_refs {
            assert!(
                repo_root().join(reference).is_file(),
                "missing evidence ref {reference}"
            );
        }
    }
}

#[test]
fn regressed_milestone_is_flagged_automatically() {
    let mut evidence = real_evidence();
    let friction = evidence
        .iter_mut()
        .find(|document| {
            document
                .source_ref
                .ends_with("friction-summary.fixture.json")
        })
        .expect("friction evidence");
    friction.document["hiddenFriction"] = Value::Bool(true);

    let report = evaluate_self_audit_acceptance(&contract(), &evidence).expect("report");
    assert_eq!(report.status, SelfAuditAcceptanceStatus::Regressed);
    let demo = report
        .milestone_verdicts
        .iter()
        .find(|verdict| verdict.milestone_id == "m68-demo")
        .expect("demo verdict");
    assert_eq!(demo.status, SelfAuditAcceptanceStatus::Regressed);
    assert!(demo
        .predicate_results
        .iter()
        .any(|predicate| predicate.path == "hiddenFriction" && !predicate.passed));
}

#[test]
fn missing_evidence_is_insufficient_not_a_silent_pass() {
    let evidence: Vec<_> = real_evidence()
        .into_iter()
        .filter(|document| !document.source_ref.ends_with("matrix.fixture.json"))
        .collect();
    let report = evaluate_self_audit_acceptance(&contract(), &evidence).expect("report");
    let coverage = report
        .milestone_verdicts
        .iter()
        .find(|verdict| verdict.milestone_id == "m68-coverage-v60")
        .expect("coverage verdict");
    assert_eq!(
        coverage.status,
        SelfAuditAcceptanceStatus::InsufficientEvidence
    );
    assert_eq!(
        report.status,
        SelfAuditAcceptanceStatus::InsufficientEvidence
    );
}

#[test]
fn report_preserves_autonomy_and_no_new_store_boundaries() {
    let report = evaluate_self_audit_acceptance(&contract(), &real_evidence()).expect("report");
    let lower = report.boundary.to_ascii_lowercase();
    for required in [
        "read-only",
        "ledger.jsonl",
        "journal.md",
        "verdict",
        "loop-coverage",
        "no new verification engine",
        "no new data plane",
        "no human input",
        "source-apply",
        "trust-gradient",
        "never auto-applied",
        "human ring 2",
    ] {
        assert!(lower.contains(required), "missing boundary {required}");
    }
}
