//! Scenario Coverage v61 regression suite for #2032 / Era L M69.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    attribute_self_audit_bottlenecks, evaluate_self_audit_acceptance, run_self_audit_demo,
    self_audit_bottleneck_input_from_json_str, SelfAuditAcceptanceStatus,
    SelfAuditAttributionContract, SelfAuditEvidenceDocument,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).unwrap_or_else(|err| panic!("read {path}: {err}"))
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

fn bottleneck_input() -> ouroforge_core::SelfAuditBottleneckInput {
    self_audit_bottleneck_input_from_json_str(&read_text(
        "examples/real-title-dogfood-v1/bottleneck-attribution-v1/planted-failure.fixture.json",
    ))
    .expect("bottleneck input validates")
}

fn doc(path: &str) -> SelfAuditEvidenceDocument {
    SelfAuditEvidenceDocument {
        source_ref: path.to_string(),
        document: read_json(path),
    }
}

fn real_acceptance_evidence() -> Vec<SelfAuditEvidenceDocument> {
    vec![
        doc("examples/real-title-dogfood-v1/run/verdict.json"),
        doc("examples/real-title-dogfood-v1/release-provenance.complete.json"),
        doc("examples/real-title-dogfood-v1/demo/friction-summary.fixture.json"),
        doc("examples/real-title-dogfood-v1/demo/audit-trail.fixture.json"),
        doc("examples/real-title-dogfood-v1/scenario-coverage-v60/matrix.fixture.json"),
        SelfAuditEvidenceDocument {
            source_ref: "examples/real-title-dogfood-v1/run/ledger.jsonl".to_string(),
            document: serde_json::json!({"kind":"existing-ledger-jsonl-anchor"}),
        },
        SelfAuditEvidenceDocument {
            source_ref: "examples/real-title-dogfood-v1/run/journal.md".to_string(),
            document: serde_json::json!({"kind":"existing-journal-anchor"}),
        },
        doc("examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json"),
    ]
}

fn planted_acceptance_evidence() -> Vec<SelfAuditEvidenceDocument> {
    let mut evidence = real_acceptance_evidence();
    let verdict = evidence
        .iter_mut()
        .find(|document| document.source_ref.ends_with("run/verdict.json"))
        .expect("verdict evidence");
    verdict.document["status"] = Value::String("fail".to_string());
    let provenance = evidence
        .iter_mut()
        .find(|document| {
            document
                .source_ref
                .ends_with("release-provenance.complete.json")
        })
        .expect("provenance evidence");
    provenance.document["status"] = Value::String("incomplete".to_string());
    evidence
}

#[test]
fn v61_matrix_records_self_audit_regression_rows() {
    let matrix =
        read_json("examples/real-title-dogfood-v1/scenario-coverage-v61/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v61-self-audit-attribution-v1"
    );
    assert_eq!(matrix["coverageVersion"], 61);
    assert_eq!(matrix["titleId"], "era-i-engine-builder-deckbuilder");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "self-audit-contract-links",
        "planted-bottleneck-attribution",
        "acceptance-per-milestone-pass",
        "acceptance-regression-fail-closed",
        "self-audit-demo-planted-defect",
        "autonomy-and-existing-pipeline-boundaries",
    ] {
        assert!(ids.contains(required), "missing v61 row {required}");
    }

    for row in matrix["rows"].as_array().expect("rows") {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidenceRef");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence {evidence_ref}"
        );
        assert!(
            row["locks"].as_str().expect("locks").len() > 20,
            "row must explain locked behavior"
        );
    }

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "ledger.jsonl",
        "journal.md",
        "openchrome",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "test-only",
        "no new verification engine",
        "no new data plane",
        "never auto-applied",
        "human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v61_locks_contract_bottleneck_acceptance_and_demo_behavior() {
    let contract = contract();
    let input = bottleneck_input();

    let bottlenecks =
        attribute_self_audit_bottlenecks(&contract, &input).expect("bottleneck report");
    let top = bottlenecks
        .ranked_bottlenecks
        .first()
        .expect("top bottleneck");
    assert_eq!(top.milestone_id, "m68-real-title-run");
    assert_eq!(top.issue_ref, "#2025");
    assert!(top
        .signal_ids
        .contains(&"planted-four-gates-fail".to_string()));
    assert!(!bottlenecks
        .ranked_bottlenecks
        .iter()
        .flat_map(|rank| rank.signal_ids.iter())
        .any(|signal| signal == "coverage-v60-pass"));

    let acceptance =
        evaluate_self_audit_acceptance(&contract, &real_acceptance_evidence()).expect("acceptance");
    assert_eq!(acceptance.status, SelfAuditAcceptanceStatus::Satisfied);
    assert_eq!(
        acceptance.milestone_verdicts.len(),
        contract.acceptance_audits.len()
    );
    assert!(acceptance
        .milestone_verdicts
        .iter()
        .all(|verdict| verdict.status == SelfAuditAcceptanceStatus::Satisfied));

    let demo = run_self_audit_demo(
        &contract,
        &input,
        &planted_acceptance_evidence(),
        "planted-four-gates-fail",
    )
    .expect("self-audit demo");
    assert_eq!(
        demo.output_summary.acceptance_status,
        SelfAuditAcceptanceStatus::Regressed
    );
    assert_eq!(
        demo.output_summary.top_bottleneck_milestone_id,
        "m68-real-title-run"
    );
    assert!(demo
        .output_summary
        .regressed_milestone_ids
        .contains(&"m68-real-title-run".to_string()));
}

#[test]
fn v61_acceptance_regressions_and_missing_evidence_fail_closed() {
    let contract = contract();
    let regressed = evaluate_self_audit_acceptance(&contract, &planted_acceptance_evidence())
        .expect("regressed acceptance");
    assert_eq!(regressed.status, SelfAuditAcceptanceStatus::Regressed);
    let real_title = regressed
        .milestone_verdicts
        .iter()
        .find(|verdict| verdict.milestone_id == "m68-real-title-run")
        .expect("real-title verdict");
    assert_eq!(real_title.status, SelfAuditAcceptanceStatus::Regressed);
    assert!(real_title
        .predicate_results
        .iter()
        .any(|predicate| predicate.path == "status" && !predicate.passed));

    let missing: Vec<_> = real_acceptance_evidence()
        .into_iter()
        .filter(|document| !document.source_ref.ends_with("matrix.fixture.json"))
        .collect();
    let insufficient =
        evaluate_self_audit_acceptance(&contract, &missing).expect("insufficient acceptance");
    assert_eq!(
        insufficient.status,
        SelfAuditAcceptanceStatus::InsufficientEvidence
    );
    assert!(insufficient.milestone_verdicts.iter().any(|verdict| {
        verdict.milestone_id == "m68-coverage-v60"
            && verdict.status == SelfAuditAcceptanceStatus::InsufficientEvidence
    }));
}

#[test]
fn v61_docs_preserve_test_only_autonomy_boundaries() {
    let doc = read_text("docs/scenario-coverage-v61-self-audit-attribution.md");
    let lower = doc.to_ascii_lowercase();
    for required in [
        "cargo test --workspace --jobs 2",
        "test-only rust coverage",
        "openchrome",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage attribution",
        "source-apply",
        "trust-gradient",
        "does not introduce a verification engine",
        "data plane",
        "zero human input",
        "never auto-applied",
        "fun/taste and release go/no-go remain human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(lower.contains(required), "doc missing {required}");
    }
}
