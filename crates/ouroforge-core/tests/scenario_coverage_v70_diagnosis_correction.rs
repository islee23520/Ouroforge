//! Scenario Coverage v70 regression suite for #2072 / Era M M79.
//!
//! The executable Studio demo lives in the local Elixir executor while Rust
//! owns diagnosis validation and re-attribution. These assertions lock the
//! fixture, prior-update behavior, and no-bypass boundary so future changes
//! cannot turn diagnosis correction into raw writes, opaque ML, or mandatory
//! human dependencies.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ouroforge_core::{
    attribute_with_diagnosis_corrections, DiagnosisAttributionInput, DiagnosisCorrectionArtifact,
    DiagnosisCorrectionCaptureSurface, DiagnosisCorrectionGateKind, DiagnosisCorrectionGateResult,
    DiagnosisCorrectionGateStatus, DiagnosisCorrectionStatus, DiagnosisSignal,
    DIAGNOSIS_CORRECTION_BOUNDARY, DIAGNOSIS_CORRECTION_SCHEMA_VERSION,
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

fn matrix() -> Value {
    read_json(
        "examples/diagnosis-correction-intervention-feedback-v1/scenario-coverage-v70/matrix.fixture.json",
    )
}

fn gate(kind: DiagnosisCorrectionGateKind) -> DiagnosisCorrectionGateResult {
    DiagnosisCorrectionGateResult {
        kind,
        status: DiagnosisCorrectionGateStatus::Passed,
        evidence_ref: format!("runs/v70/gates/{kind:?}.json"),
    }
}

fn correction() -> DiagnosisCorrectionArtifact {
    DiagnosisCorrectionArtifact {
        schema_version: DIAGNOSIS_CORRECTION_SCHEMA_VERSION.to_string(),
        correction_id: "corr-v70-threshold".to_string(),
        diagnosis_id: "diag-v70-001".to_string(),
        run_id: "run-v70-001".to_string(),
        original_attribution: "asset-provenance-gap".to_string(),
        corrected_attribution: "evaluator-threshold-drift".to_string(),
        human_actor: "human://local-operator".to_string(),
        correction_rationale: "Evidence shows provenance was complete and evaluator threshold drift caused the failure.".to_string(),
        captured_via: DiagnosisCorrectionCaptureSurface::StudioPhoenixLiveView,
        intervention_as_evidence: true,
        base_evidence_refs: vec!["runs/v70/evidence/original-diagnosis.json".to_string()],
        correction_evidence_refs: vec!["runs/v70/evidence/correction.json".to_string()],
        provenance_refs: vec!["runs/v70/provenance/human-correction.json".to_string()],
        gate_results: vec![
            gate(DiagnosisCorrectionGateKind::ReviewApply),
            gate(DiagnosisCorrectionGateKind::SceneSourceApply),
            gate(DiagnosisCorrectionGateKind::Evaluator),
            gate(DiagnosisCorrectionGateKind::EvidenceProvenance),
        ],
        status: DiagnosisCorrectionStatus::Recorded,
        heuristic_prior_delta: 4,
        opaque_ml_update: false,
        automated_fun_taste_inference: false,
        raw_bypass_requested: false,
        studio_trusted_write_authority: false,
        human_required_for_autonomous_loop: false,
        cli_fallback_supported: true,
        boundary: DIAGNOSIS_CORRECTION_BOUNDARY.to_string(),
    }
}

#[test]
fn v70_matrix_records_diagnosis_correction_rows_and_boundaries() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v70-diagnosis-correction-intervention-feedback-v1"
    );
    assert_eq!(matrix["coverageVersion"], 70);
    assert_eq!(matrix["issueRef"], "#2072");
    assert_eq!(matrix["milestone"], "Era M M79");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();

    for required in [
        "diagnosis-correction-recorded-through-existing-gates",
        "corrected-attribution-improves-subsequent-run",
        "no-raw-bypass-from-elixir-diagnosis-correction-surface",
        "loop-completes-without-human-input",
        "mandatory-human-and-opaque-inference-regressions-fail-closed",
        "coverage-v70-boundaries",
    ] {
        assert!(ids.contains(required), "missing v70 row {required}");
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
fn v70_rust_records_correction_and_improves_attribution() {
    let correction = correction();
    correction.validate().expect("valid v70 correction");
    let read = correction.read_model();
    assert!(read.recorded);
    assert_eq!(read.passed_gate_count, 4);
    assert_eq!(read.corrected_attribution, "evaluator-threshold-drift");

    let input = DiagnosisAttributionInput {
        diagnosis_id: "diag-v70-next".to_string(),
        run_id: "run-v70-next".to_string(),
        signals: vec![
            DiagnosisSignal {
                attribution: "asset-provenance-gap".to_string(),
                score: 7,
                evidence_ref: "runs/v70/signals/asset.json".to_string(),
            },
            DiagnosisSignal {
                attribution: "evaluator-threshold-drift".to_string(),
                score: 5,
                evidence_ref: "runs/v70/signals/evaluator.json".to_string(),
            },
        ],
        priors: BTreeMap::new(),
    };

    let before = attribute_with_diagnosis_corrections(&input, &[]).expect("before");
    assert_eq!(before.selected_attribution, "asset-provenance-gap");

    let after = attribute_with_diagnosis_corrections(&input, &[correction]).expect("after");
    assert_eq!(after.selected_attribution, "evaluator-threshold-drift");
    assert_eq!(
        after.applied_correction_refs,
        vec!["corr-v70-threshold".to_string()]
    );
    assert!(after
        .boundary
        .contains("transparent heuristic prior update"));
    assert!(after.boundary.contains("no opaque ML"));
}

#[test]
fn v70_autonomy_two_plane_and_inference_invariants_fail_closed() {
    let invariants = matrix()["autonomyInvariants"].clone();
    assert_eq!(invariants["humanInputRequired"], false);
    assert_eq!(invariants["loopCompletesWithoutHuman"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["elixirOwnsArtifactSemantics"], false);
    assert_eq!(invariants["rustDataPlaneOwnsValidation"], true);
    assert_eq!(invariants["hostedCollaborativeStudioDeferred"], true);
    assert_eq!(invariants["cliFallbackIntact"], true);
    assert_eq!(invariants["opaqueMlUpdate"], false);
    assert_eq!(invariants["automatedFunTasteInference"], false);

    let mut invalid = correction();
    invalid.raw_bypass_requested = true;
    assert!(format!("{:#}", invalid.validate().unwrap_err()).contains("raw bypass"));

    let mut invalid = correction();
    invalid.human_required_for_autonomous_loop = true;
    assert!(format!("{:#}", invalid.validate().unwrap_err()).contains("require humans"));

    let mut invalid = correction();
    invalid.opaque_ml_update = true;
    assert!(format!("{:#}", invalid.validate().unwrap_err()).contains("opaque ML"));

    let mut invalid = correction();
    invalid.automated_fun_taste_inference = true;
    assert!(format!("{:#}", invalid.validate().unwrap_err()).contains("fun/taste"));
}

#[test]
fn v70_elixir_diagnosis_correction_surface_has_no_raw_artifact_write_calls() {
    let lib_dir = repo_root().join("studio/executor/lib/ouroforge_executor");
    let mut offenders = Vec::new();

    for filename in [
        "diagnosis_correction_surface.ex",
        "diagnosis_correction_demo.ex",
    ] {
        let path = lib_dir.join(filename);
        let text = std::fs::read_to_string(&path).expect("read diagnosis correction elixir source");
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
            "elixirOwnsDiagnosisSemantics: true",
            "opaqueMlUpdate: true",
            "automatedFunTasteInference: true",
        ] {
            if text.contains(raw_write) {
                offenders.push(format!("{} contains {raw_write}", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "Elixir diagnosis correction raw-write bypass candidates: {offenders:#?}"
    );
}

#[test]
fn v70_docs_record_coverage_and_guardrails() {
    let doc = read_text("docs/scenario-coverage-v70-diagnosis-correction-intervention-feedback.md")
        .to_ascii_lowercase();

    for required in [
        "coverage v70",
        "agent-first default",
        "zero human input",
        "validated, recorded",
        "diagnosis correction",
        "transparent heuristic priors",
        "no opaque ml",
        "no fun/taste automation",
        "control and presentation",
        "rust remains the data plane",
        "no raw bypass",
        "no hosted studio",
        "no multi-user",
        "no mandatory human",
        "cli fallback remains intact",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}
