//! Scenario Coverage v69 regression suite for #2068 / Era M M78.
//!
//! The executable Studio demo lives in the local Elixir executor while Rust
//! owns the data-plane gate. These assertions lock the fixture, gate behavior,
//! and no-bypass boundary so future changes cannot turn human constraints into
//! raw writes or mandatory human dependencies.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_evaluator::human_constraint_gate::{
    compose_human_constraints_into_categories, evaluate_human_constraint_gate,
    CandidateConstraintEvidence, HumanConstraintGateInput, HumanConstraintGateState,
    HumanConstraintKind, HumanConstraintRecord, HumanConstraintStatus,
    HUMAN_CONSTRAINT_GATE_BOUNDARY, HUMAN_CONSTRAINT_GATE_SCHEMA_VERSION,
};
use serde_json::{json, Value};

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

fn matrix() -> Value {
    read_json(
        "examples/human-constraints-first-class-gates-v1/scenario-coverage-v69/matrix.fixture.json",
    )
}

fn candidate() -> CandidateConstraintEvidence {
    CandidateConstraintEvidence {
        candidate_id: "candidate-v69-dash-card".to_string(),
        target_ref: "runs/v69/candidates/card.json".to_string(),
        mechanics: vec!["dash".to_string(), "burn".to_string()],
        style: "pixel-art".to_string(),
        budget: 8,
        evidence_refs: vec!["runs/v69/evidence/candidate.json".to_string()],
    }
}

fn constraint() -> HumanConstraintRecord {
    HumanConstraintRecord {
        constraint_id: "constraint-v69-no-dash".to_string(),
        kind: HumanConstraintKind::ForbiddenMechanic,
        status: HumanConstraintStatus::Active,
        author: "human:local-designer".to_string(),
        author_provenance_ref: "runs/v69/provenance/human.json".to_string(),
        target_ref: "runs/v69/candidates/card.json".to_string(),
        target_base_ref: "hash:v69-before".to_string(),
        normalized_constraint_ref: "runs/v69/constraints/no-dash.normalized.json".to_string(),
        review_apply_ref: "runs/v69/review/no-dash.decision.json".to_string(),
        evaluator_evidence_ref: "runs/v69/evaluator/human-constraint.json".to_string(),
        evidence_refs: vec!["runs/v69/evidence/no-dash-capture.json".to_string()],
        forbidden_mechanic: Some("dash".to_string()),
        required_style: None,
        budget_cap: None,
        intervention_as_evidence: true,
        read_gated_write: true,
        raw_bypass_requested: false,
        direct_artifact_write: false,
        studio_trusted_write_authority: false,
        human_required_for_autonomous_loop: false,
        cli_fallback_supported: true,
    }
}

fn gate_input(mut constraints: Vec<HumanConstraintRecord>) -> HumanConstraintGateInput {
    HumanConstraintGateInput {
        schema_version: HUMAN_CONSTRAINT_GATE_SCHEMA_VERSION.to_string(),
        gate_id: "v69-human-constraint-gate".to_string(),
        candidate: candidate(),
        constraints: std::mem::take(&mut constraints),
        boundary: HUMAN_CONSTRAINT_GATE_BOUNDARY.to_string(),
    }
}

#[test]
fn v69_matrix_records_human_constraint_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v69-human-constraints-first-class-gates-v1"
    );
    assert_eq!(matrix["coverageVersion"], 69);
    assert_eq!(matrix["issueRef"], "#2068");
    assert_eq!(matrix["milestone"], "Era M M78");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();

    for required in [
        "constraints-recorded-through-existing-gates",
        "violating-output-blocked-with-evidence",
        "no-raw-bypass-from-elixir-human-constraint-surface",
        "loop-completes-without-human-input",
        "mandatory-human-regression-fails-closed",
        "coverage-v69-boundaries",
    ] {
        assert!(ids.contains(required), "missing v69 row {required}");
    }

    for row in matrix["rows"].as_array().expect("rows") {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidenceRef");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence {evidence_ref}"
        );
        assert!(row["locks"].as_str().expect("locks").len() > 90);
    }
}

#[test]
fn v69_rust_gate_blocks_violating_output_with_evidence_and_composes() {
    let verdicts = evaluate_human_constraint_gate(&gate_input(vec![constraint()]));
    assert_eq!(verdicts.len(), 1);
    assert_eq!(verdicts[0].state, HumanConstraintGateState::Violation);
    assert!(verdicts[0].reason.contains("forbidden mechanic dash"));
    assert!(verdicts[0]
        .evidence_refs
        .contains(&"runs/v69/evaluator/human-constraint.json".to_string()));

    let mut categories = json!({
        "operator": "declared-gate-and",
        "scenario": {"declared": true, "status": "pass", "resultCount": 1, "failureCount": 0},
        "visual": {"declared": true, "status": "pass", "resultCount": 1, "failureCount": 0},
        "semantic": {"declared": true, "status": "pass", "resultCount": 1, "failureCount": 0},
        "reviewApply": {"declared": true, "status": "pass", "resultCount": 1, "failureCount": 0}
    });
    assert!(compose_human_constraints_into_categories(
        &mut categories,
        &verdicts
    ));
    assert_eq!(categories["operator"], "declared-gate-and");
    assert_eq!(categories["humanConstraints"]["status"], "fail");
    assert_eq!(categories["humanConstraints"]["failureCount"], 1);
}

#[test]
fn v69_autonomy_and_two_plane_invariants_fail_closed() {
    let invariants = matrix()["autonomyInvariants"].clone();
    assert_eq!(invariants["humanInputRequired"], false);
    assert_eq!(invariants["loopCompletesWithoutHuman"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["rustDataPlaneOwnsValidation"], true);
    assert_eq!(invariants["hostedCollaborativeStudioDeferred"], true);
    assert_eq!(invariants["cliFallbackIntact"], true);

    let boundary = matrix()["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();

    for required in [
        "opt-in",
        "validated, recorded",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "elixir/phoenix is local control and presentation only",
        "never writes artifacts",
        "rust remains the data plane",
        "zero human input",
        "no raw bypass",
        "no new write path",
        "no new data store",
        "no hosted",
        "no mandatory human",
        "cli fallback remains intact",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v69_elixir_human_constraint_surface_has_no_raw_artifact_write_calls() {
    let lib_dir = repo_root().join("studio/executor/lib/ouroforge_executor");
    let mut offenders = Vec::new();

    for filename in ["human_constraint_surface.ex", "human_constraint_demo.ex"] {
        let path = lib_dir.join(filename);
        let text = std::fs::read_to_string(&path).expect("read human constraint elixir source");
        for raw_write in [
            "File.write",
            ":file.write",
            ":file.open",
            "File.open",
            "File.rm",
            "File.cp",
            "File.rename",
            "File.touch",
            ":file.delete",
            ":file.rename",
            "directArtifactWrite: true",
            "studioTrustedWriteAuthority: true",
            "rawBypassRequested: true",
        ] {
            if text.contains(raw_write) {
                offenders.push(format!("{} contains {raw_write}", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "Elixir human constraint raw-write bypass candidates: {offenders:#?}"
    );
}

#[test]
fn v69_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v69-human-constraints-first-class-gates.md")
        .to_ascii_lowercase();

    for required in [
        "coverage v69",
        "agent-first default",
        "zero human input",
        "validated, recorded proposals",
        "constraints",
        "rust evaluator",
        "control and presentation",
        "rust remains the data plane",
        "no raw bypass",
        "no hosted",
        "no new data store",
        "cli fallback",
        "fun/taste",
        "release go/no-go",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
