//! Deterministic demo smoke test for Production-Scale QA Matrix v1 (#1670):
//! a game build exercised by the matrix catches a planted cross-variant
//! regression with replayable evidence, and the consolidated production-QA
//! verdict fails closed because of it. Fixture-scoped; no network, no browser.

use ouroforge_core::production_qa_matrix::ProductionQaMatrixArtifact;
use ouroforge_core::production_qa_verdict::ProductionQaVerdictArtifact;
use std::{fs, path::PathBuf};

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn read(rel: &str) -> String {
    fs::read_to_string(repo_path(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

#[test]
fn demo_matrix_catches_planted_regression_with_replayable_evidence() {
    let matrix = ProductionQaMatrixArtifact::from_json_str(&read(
        "examples/production-qa-matrix-v1/demo/demo.matrix.fixture.json",
    ))
    .expect("demo matrix validates");
    let read_model = matrix.read_model();
    assert_eq!(read_model.status, "complete");
    assert_eq!(
        read_model.regression_count, 1,
        "the demo plants exactly one cross-variant regression"
    );
    let regression = &read_model.detected_regressions[0];
    assert_eq!(regression.content_variant, "candidate");
    assert_eq!(regression.seed, "seed-1");
    assert_eq!(regression.target, "web");
    assert_eq!(regression.baseline_verdict, "passed");
    assert_eq!(regression.variant_verdict, "failed");
    // Replayable: the regression carries both baseline and candidate evidence.
    assert!(regression
        .evidence_refs
        .iter()
        .any(|r| r.contains("candidate-s1-web")));
    assert!(regression
        .evidence_refs
        .iter()
        .any(|r| r.contains("base-s1-web")));
}

#[test]
fn demo_consolidated_verdict_fails_closed_on_the_regression() {
    let verdict = ProductionQaVerdictArtifact::from_json_str(&read(
        "examples/production-qa-matrix-v1/demo/demo.verdict.fixture.json",
    ))
    .expect("demo verdict validates");
    let read_model = verdict.read_model();
    assert_eq!(
        read_model.verdict, "fail",
        "a single failing declared check propagates to the consolidated verdict"
    );
    assert_eq!(read_model.failing_checks.len(), 1);
    assert_eq!(read_model.failing_checks[0].kind, "regressionMatrix");
    // Reuse anchor: the failing check references the demo matrix fixture.
    let matrix_check = verdict
        .checks
        .iter()
        .find(|c| c.kind == "regressionMatrix")
        .expect("regressionMatrix check present");
    assert!(matrix_check
        .evidence_ref
        .contains("demo/demo.matrix.fixture.json"));
    // Composition reuses the evaluator declared-gate-and aggregation.
    assert_eq!(read_model.aggregation_operator, "declared-gate-and");
    assert_eq!(read_model.undeclared_gate_policy, "neutral");
}

#[test]
fn demo_doc_documents_the_catch_and_the_verdict() {
    let doc = read("docs/production-qa-matrix-v1-demo.md");
    assert!(doc.contains("#1670"));
    assert!(doc.contains("planted cross-variant regression"));
    assert!(doc.contains("declared-gate-and"));
    assert!(doc.contains("descriptive"));
    assert!(doc.contains("#1 and #23 remain open"));
    for forbidden in [
        "auto-merge enabled",
        "production-ready claim enabled",
        "current Godot replacement is implemented",
    ] {
        assert!(!doc.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
