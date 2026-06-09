use std::collections::BTreeMap;

use ouroforge_core::{
    attribute_with_diagnosis_corrections, DiagnosisAttributionInput, DiagnosisCorrectionArtifact,
    DiagnosisCorrectionCaptureSurface, DiagnosisCorrectionGateKind, DiagnosisCorrectionGateResult,
    DiagnosisCorrectionGateStatus, DiagnosisCorrectionStatus, DiagnosisSignal,
    DIAGNOSIS_CORRECTION_BOUNDARY, DIAGNOSIS_CORRECTION_SCHEMA_VERSION,
};

fn gate(kind: DiagnosisCorrectionGateKind) -> DiagnosisCorrectionGateResult {
    DiagnosisCorrectionGateResult {
        kind,
        status: DiagnosisCorrectionGateStatus::Passed,
        evidence_ref: format!("evidence://gate/{kind:?}"),
    }
}

fn valid_correction() -> DiagnosisCorrectionArtifact {
    DiagnosisCorrectionArtifact {
        schema_version: DIAGNOSIS_CORRECTION_SCHEMA_VERSION.to_string(),
        correction_id: "corr-m79-001".to_string(),
        diagnosis_id: "diag-era-l-017".to_string(),
        run_id: "run-era-l-017".to_string(),
        original_attribution: "asset-provenance-gap".to_string(),
        corrected_attribution: "evaluator-threshold-drift".to_string(),
        human_actor: "human://local-operator".to_string(),
        correction_rationale: "The cited evidence shows all asset provenance present; the failing gate was the evaluator threshold.".to_string(),
        captured_via: DiagnosisCorrectionCaptureSurface::StudioPhoenixLiveView,
        intervention_as_evidence: true,
        base_evidence_refs: vec!["evidence://diagnosis/original".to_string()],
        correction_evidence_refs: vec!["evidence://diagnosis/correction".to_string()],
        provenance_refs: vec!["provenance://human/corr-m79-001".to_string()],
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
fn corrected_diagnosis_is_recorded_and_auditable_through_existing_gates() {
    let correction = valid_correction();

    correction.validate().expect("valid correction");
    let read = correction.read_model();

    assert!(read.recorded);
    assert_eq!(read.corrected_attribution, "evaluator-threshold-drift");
    assert_eq!(read.passed_gate_count, 4);
    assert!(read
        .provenance_refs
        .contains(&"provenance://human/corr-m79-001".to_string()));
    assert!(read.boundary.contains("intervention-as-evidence"));
    assert!(read.boundary.contains("read + gated-write"));
    assert!(read.boundary.contains("transparent heuristic prior update"));
}

#[test]
fn correction_improves_subsequent_attribution_with_transparent_priors() {
    let input = DiagnosisAttributionInput {
        diagnosis_id: "diag-era-l-next".to_string(),
        run_id: "run-era-l-next".to_string(),
        signals: vec![
            DiagnosisSignal {
                attribution: "asset-provenance-gap".to_string(),
                score: 7,
                evidence_ref: "evidence://signal/asset".to_string(),
            },
            DiagnosisSignal {
                attribution: "evaluator-threshold-drift".to_string(),
                score: 5,
                evidence_ref: "evidence://signal/evaluator".to_string(),
            },
        ],
        priors: BTreeMap::new(),
    };

    let before = attribute_with_diagnosis_corrections(&input, &[]).expect("baseline attribution");
    assert_eq!(before.selected_attribution, "asset-provenance-gap");

    let after = attribute_with_diagnosis_corrections(&input, &[valid_correction()])
        .expect("corrected attribution");
    assert_eq!(after.selected_attribution, "evaluator-threshold-drift");
    assert_eq!(
        after.applied_correction_refs,
        vec!["corr-m79-001".to_string()]
    );
    assert_eq!(after.priors["evaluator-threshold-drift"], 4);
    assert_eq!(after.priors["asset-provenance-gap"], -4);
    assert!(after.boundary.contains("no opaque ML"));
}

#[test]
fn raw_bypass_or_studio_truth_authority_is_rejected() {
    let mut correction = valid_correction();
    correction.raw_bypass_requested = true;
    let err = correction.validate().expect_err("raw bypass rejected");
    assert!(format!("{err:#}").contains("raw bypass"));

    let mut correction = valid_correction();
    correction.studio_trusted_write_authority = true;
    let err = correction
        .validate()
        .expect_err("studio authority rejected");
    assert!(format!("{err:#}").contains("Studio trusted writes"));
}

#[test]
fn opaque_ml_fun_taste_and_mandatory_human_regressions_fail_closed() {
    let mut correction = valid_correction();
    correction.opaque_ml_update = true;
    assert!(format!("{:#}", correction.validate().unwrap_err()).contains("opaque ML"));

    let mut correction = valid_correction();
    correction.automated_fun_taste_inference = true;
    assert!(format!("{:#}", correction.validate().unwrap_err()).contains("fun/taste"));

    let mut correction = valid_correction();
    correction.human_required_for_autonomous_loop = true;
    assert!(format!("{:#}", correction.validate().unwrap_err()).contains("require humans"));
}

#[test]
fn missing_or_failed_gates_do_not_record_corrections() {
    let mut correction = valid_correction();
    correction.gate_results.pop();
    let err = correction.validate().expect_err("missing provenance gate");
    assert!(format!("{err:#}").contains("missing required gate"));

    let mut correction = valid_correction();
    correction.gate_results[0].status = DiagnosisCorrectionGateStatus::Failed;
    correction.status = DiagnosisCorrectionStatus::Rejected;
    correction
        .validate()
        .expect("rejected correction keeps failed evidence");
    assert!(!correction.recorded());
}
