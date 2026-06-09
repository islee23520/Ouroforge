//! Diagnosis correction and intervention feedback contract (#2070).
//!
//! This module is the Rust data-plane contract for Milestone 79. It records a
//! human diagnosis/attribution correction as intervention evidence, validates
//! the reused gates, and applies a transparent heuristic prior update for later
//! attribution. It never grants Studio/Phoenix artifact authority, performs raw
//! writes, introduces opaque ML, automates fun/taste, or requires a human for the
//! autonomous loop to complete.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const DIAGNOSIS_CORRECTION_SCHEMA_VERSION: &str = "ouroforge.diagnosis-correction.v1";
pub const DIAGNOSIS_CORRECTION_BOUNDARY: &str = "diagnosis correction and intervention feedback; intervention-as-evidence; read + gated-write; Rust data plane validates, records, and re-attributes; Elixir/Phoenix control + presentation only; review/apply, scene/source-apply, evaluator, evidence/provenance gates required; transparent heuristic prior update; no opaque ML; no raw bypass; local-first CLI fallback; loop completes without human; fun/taste and release go/no-go remain human; #1 and #23 remain open";

const REQUIRED_GATES: &[DiagnosisCorrectionGateKind] = &[
    DiagnosisCorrectionGateKind::ReviewApply,
    DiagnosisCorrectionGateKind::SceneSourceApply,
    DiagnosisCorrectionGateKind::Evaluator,
    DiagnosisCorrectionGateKind::EvidenceProvenance,
];

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosisCorrectionGateKind {
    ReviewApply,
    SceneSourceApply,
    Evaluator,
    EvidenceProvenance,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosisCorrectionGateStatus {
    Passed,
    Failed,
    Blocked,
    Stale,
    Missing,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosisCorrectionStatus {
    Recorded,
    Blocked,
    Stale,
    Rejected,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosisCorrectionCaptureSurface {
    Cli,
    StudioPhoenixLiveView,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagnosisCorrectionGateResult {
    pub kind: DiagnosisCorrectionGateKind,
    pub status: DiagnosisCorrectionGateStatus,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagnosisCorrectionArtifact {
    pub schema_version: String,
    pub correction_id: String,
    pub diagnosis_id: String,
    pub run_id: String,
    pub original_attribution: String,
    pub corrected_attribution: String,
    pub human_actor: String,
    pub correction_rationale: String,
    pub captured_via: DiagnosisCorrectionCaptureSurface,
    pub intervention_as_evidence: bool,
    pub base_evidence_refs: Vec<String>,
    pub correction_evidence_refs: Vec<String>,
    pub provenance_refs: Vec<String>,
    pub gate_results: Vec<DiagnosisCorrectionGateResult>,
    pub status: DiagnosisCorrectionStatus,
    pub heuristic_prior_delta: i32,
    pub opaque_ml_update: bool,
    pub automated_fun_taste_inference: bool,
    pub raw_bypass_requested: bool,
    pub studio_trusted_write_authority: bool,
    pub human_required_for_autonomous_loop: bool,
    pub cli_fallback_supported: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagnosisSignal {
    pub attribution: String,
    pub score: i32,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagnosisAttributionInput {
    pub diagnosis_id: String,
    pub run_id: String,
    pub signals: Vec<DiagnosisSignal>,
    pub priors: BTreeMap<String, i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagnosisAttributionDecision {
    pub diagnosis_id: String,
    pub run_id: String,
    pub selected_attribution: String,
    pub score: i32,
    pub evidence_refs: Vec<String>,
    pub applied_correction_refs: Vec<String>,
    pub priors: BTreeMap<String, i32>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagnosisCorrectionReadModel {
    pub correction_id: String,
    pub diagnosis_id: String,
    pub run_id: String,
    pub status: DiagnosisCorrectionStatus,
    pub recorded: bool,
    pub original_attribution: String,
    pub corrected_attribution: String,
    pub heuristic_prior_delta: i32,
    pub gate_count: usize,
    pub passed_gate_count: usize,
    pub blocked_reasons: Vec<String>,
    pub provenance_refs: Vec<String>,
    pub boundary: String,
}

impl DiagnosisCorrectionArtifact {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("diagnosis correction artifact is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != DIAGNOSIS_CORRECTION_SCHEMA_VERSION {
            return Err(anyhow!(
                "schemaVersion must be {DIAGNOSIS_CORRECTION_SCHEMA_VERSION}"
            ));
        }
        require_text("correctionId", &self.correction_id)?;
        require_text("diagnosisId", &self.diagnosis_id)?;
        require_text("runId", &self.run_id)?;
        require_text("originalAttribution", &self.original_attribution)?;
        require_text("correctedAttribution", &self.corrected_attribution)?;
        if self.original_attribution == self.corrected_attribution {
            return Err(anyhow!(
                "correctedAttribution must differ from originalAttribution"
            ));
        }
        require_text("humanActor", &self.human_actor)?;
        require_text("correctionRationale", &self.correction_rationale)?;
        require_refs("baseEvidenceRefs", &self.base_evidence_refs)?;
        require_refs("correctionEvidenceRefs", &self.correction_evidence_refs)?;
        require_refs("provenanceRefs", &self.provenance_refs)?;
        require_boundary(&self.boundary)?;

        if !self.intervention_as_evidence {
            return Err(anyhow!(
                "diagnosis correction must be recorded as intervention-as-evidence"
            ));
        }
        if self.heuristic_prior_delta <= 0 {
            return Err(anyhow!(
                "heuristicPriorDelta must be positive for transparent re-attribution"
            ));
        }
        if self.opaque_ml_update
            || self.automated_fun_taste_inference
            || self.raw_bypass_requested
            || self.studio_trusted_write_authority
            || self.human_required_for_autonomous_loop
            || !self.cli_fallback_supported
        {
            return Err(anyhow!(
                "correction must not use opaque ML, infer fun/taste, request raw bypass, grant Studio trusted writes, require humans, or break CLI fallback"
            ));
        }

        let mut kinds = BTreeSet::new();
        for result in &self.gate_results {
            result.validate()?;
            if !kinds.insert(result.kind) {
                return Err(anyhow!("duplicate gate result for {:?}", result.kind));
            }
        }
        for required in REQUIRED_GATES {
            if !kinds.contains(required) {
                return Err(anyhow!("missing required gate result for {required:?}"));
            }
        }

        let all_required_passed = REQUIRED_GATES.iter().all(|kind| {
            self.gate_results.iter().any(|result| {
                result.kind == *kind && result.status == DiagnosisCorrectionGateStatus::Passed
            })
        });
        match self.status {
            DiagnosisCorrectionStatus::Recorded if all_required_passed => Ok(()),
            DiagnosisCorrectionStatus::Recorded => Err(anyhow!(
                "recorded correction requires review/apply, scene/source-apply, evaluator, and evidence/provenance gates to pass"
            )),
            DiagnosisCorrectionStatus::Rejected => {
                if self.gate_results.iter().any(|result| {
                    matches!(
                        result.status,
                        DiagnosisCorrectionGateStatus::Failed
                            | DiagnosisCorrectionGateStatus::Blocked
                    )
                }) {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "rejected correction must keep failed or blocked gate evidence visible"
                    ))
                }
            }
            DiagnosisCorrectionStatus::Stale => {
                if self
                    .gate_results
                    .iter()
                    .any(|result| result.status == DiagnosisCorrectionGateStatus::Stale)
                {
                    Ok(())
                } else {
                    Err(anyhow!("stale correction must keep stale gate evidence visible"))
                }
            }
            DiagnosisCorrectionStatus::Blocked => {
                if !all_required_passed {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "blocked correction cannot declare every required gate passed"
                    ))
                }
            }
        }
    }

    pub fn recorded(&self) -> bool {
        self.validate().is_ok() && self.status == DiagnosisCorrectionStatus::Recorded
    }

    pub fn read_model(&self) -> DiagnosisCorrectionReadModel {
        let blocked_reasons = self
            .gate_results
            .iter()
            .filter(|result| result.status != DiagnosisCorrectionGateStatus::Passed)
            .map(|result| format!("{:?}:{:?}", result.kind, result.status))
            .collect();
        DiagnosisCorrectionReadModel {
            correction_id: self.correction_id.clone(),
            diagnosis_id: self.diagnosis_id.clone(),
            run_id: self.run_id.clone(),
            status: self.status,
            recorded: self.recorded(),
            original_attribution: self.original_attribution.clone(),
            corrected_attribution: self.corrected_attribution.clone(),
            heuristic_prior_delta: self.heuristic_prior_delta,
            gate_count: self.gate_results.len(),
            passed_gate_count: self
                .gate_results
                .iter()
                .filter(|result| result.status == DiagnosisCorrectionGateStatus::Passed)
                .count(),
            blocked_reasons,
            provenance_refs: self.provenance_refs.clone(),
            boundary: DIAGNOSIS_CORRECTION_BOUNDARY.to_string(),
        }
    }
}

impl DiagnosisCorrectionGateResult {
    fn validate(&self) -> Result<()> {
        require_text("gate evidenceRef", &self.evidence_ref)
    }
}

pub fn validate_diagnosis_correction_json(text: &str) -> Result<DiagnosisCorrectionReadModel> {
    let artifact = DiagnosisCorrectionArtifact::from_json_str(text)?;
    artifact.validate()?;
    Ok(artifact.read_model())
}

pub fn attribute_with_diagnosis_corrections(
    input: &DiagnosisAttributionInput,
    corrections: &[DiagnosisCorrectionArtifact],
) -> Result<DiagnosisAttributionDecision> {
    require_text("diagnosisId", &input.diagnosis_id)?;
    require_text("runId", &input.run_id)?;
    if input.signals.is_empty() {
        return Err(anyhow!("signals must not be empty"));
    }

    let mut priors = input.priors.clone();
    let mut applied_correction_refs = Vec::new();
    for correction in corrections {
        correction.validate()?;
        if correction.status != DiagnosisCorrectionStatus::Recorded {
            continue;
        }
        *priors
            .entry(correction.corrected_attribution.clone())
            .or_default() += correction.heuristic_prior_delta;
        *priors
            .entry(correction.original_attribution.clone())
            .or_default() -= correction.heuristic_prior_delta;
        applied_correction_refs.push(correction.correction_id.clone());
    }

    let mut best: Option<(&DiagnosisSignal, i32)> = None;
    let mut evidence_refs = Vec::new();
    for signal in &input.signals {
        require_text("signal attribution", &signal.attribution)?;
        require_text("signal evidenceRef", &signal.evidence_ref)?;
        let score = signal.score + priors.get(&signal.attribution).copied().unwrap_or(0);
        evidence_refs.push(signal.evidence_ref.clone());
        match best {
            Some((current, current_score))
                if score < current_score
                    || (score == current_score && signal.attribution >= current.attribution) => {}
            _ => best = Some((signal, score)),
        }
    }
    let (selected, score) = best.ok_or_else(|| anyhow!("signals must not be empty"))?;

    Ok(DiagnosisAttributionDecision {
        diagnosis_id: input.diagnosis_id.clone(),
        run_id: input.run_id.clone(),
        selected_attribution: selected.attribution.clone(),
        score,
        evidence_refs,
        applied_correction_refs,
        priors,
        boundary: DIAGNOSIS_CORRECTION_BOUNDARY.to_string(),
    })
}

fn require_refs(label: &str, refs: &[String]) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    let mut unique = BTreeSet::new();
    for value in refs {
        require_text(label, value)?;
        if !unique.insert(value.as_str()) {
            return Err(anyhow!("{label} contains duplicate ref {value}"));
        }
    }
    Ok(())
}

fn require_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    if value.contains("raw_write_bypass") || value.contains("raw_apply_bypass") {
        return Err(anyhow!("{label} must not reference raw bypass authority"));
    }
    Ok(())
}

fn require_boundary(boundary: &str) -> Result<()> {
    for token in [
        "diagnosis correction",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust data plane",
        "Elixir/Phoenix control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "transparent heuristic prior update",
        "no opaque ML",
        "no raw bypass",
        "local-first CLI fallback",
        "loop completes without human",
        "fun/taste and release go/no-go remain human",
        "#1 and #23 remain open",
    ] {
        if !boundary.contains(token) {
            return Err(anyhow!("boundary must contain `{token}`"));
        }
    }
    Ok(())
}
