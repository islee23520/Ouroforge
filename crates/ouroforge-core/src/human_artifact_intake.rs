//! Human-authored artifact intake contract (#2058).
//!
//! Human-authored content enters Ouroforge only as intervention evidence. The
//! Rust data plane validates and records the candidate through the same existing
//! gates used for agent output; Studio/Phoenix may capture and route, but never
//! owns artifact semantics or performs raw writes.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const HUMAN_ARTIFACT_INTAKE_SCHEMA_VERSION: &str = "ouroforge.human-artifact-intake.v1";
pub const HUMAN_ARTIFACT_INTAKE_BOUNDARY: &str = "human-authored artifact intake; intervention-as-evidence; read + gated-write; Rust = data plane; Elixir/OTP + Phoenix LiveView = control + presentation; review/apply, scene/source-apply, evaluator, evidence/provenance gates reused; author=human provenance; no raw bypass; local-first CLI fallback; loop completes without human; #1 and #23 remain open";

const REQUIRED_GATES: &[HumanArtifactIntakeGateKind] = &[
    HumanArtifactIntakeGateKind::ReviewApply,
    HumanArtifactIntakeGateKind::SceneSourceApply,
    HumanArtifactIntakeGateKind::Evaluator,
    HumanArtifactIntakeGateKind::EvidenceProvenance,
];

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum HumanArtifactKind {
    Card,
    Scene,
    Tuning,
    Asset,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum HumanArtifactIntakeGateKind {
    ReviewApply,
    SceneSourceApply,
    Evaluator,
    EvidenceProvenance,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HumanArtifactIntakeGateStatus {
    Passed,
    Failed,
    Blocked,
    Stale,
    Missing,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HumanArtifactIntakeStatus {
    ReadyForReviewApply,
    Blocked,
    Stale,
    Rejected,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HumanArtifactIntakeSurface {
    Cli,
    StudioPhoenixLiveView,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HumanArtifactIntakeGateResult {
    pub kind: HumanArtifactIntakeGateKind,
    pub status: HumanArtifactIntakeGateStatus,
    pub evidence_ref: String,
    pub before_ref: String,
    pub after_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HumanArtifactIntakeArtifact {
    pub schema_version: String,
    pub intake_id: String,
    pub artifact_id: String,
    pub artifact_kind: HumanArtifactKind,
    pub captured_via: HumanArtifactIntakeSurface,
    pub author: String,
    pub author_provenance_ref: String,
    pub human_provenance: bool,
    pub original_artifact_ref: String,
    pub normalized_candidate_ref: String,
    pub target_ref: String,
    pub target_base_ref: String,
    pub validation_report_ref: String,
    pub review_apply_ref: String,
    pub gate_results: Vec<HumanArtifactIntakeGateResult>,
    pub status: HumanArtifactIntakeStatus,
    pub intervention_as_evidence: bool,
    pub read_gated_write: bool,
    pub raw_bypass_requested: bool,
    pub direct_artifact_write: bool,
    pub studio_trusted_write_authority: bool,
    pub human_required_for_autonomous_loop: bool,
    pub cli_fallback_supported: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HumanArtifactIntakeReadModel {
    pub intake_id: String,
    pub artifact_id: String,
    pub artifact_kind: HumanArtifactKind,
    pub author: String,
    pub status: HumanArtifactIntakeStatus,
    pub ready_for_review_apply: bool,
    pub captured_via: HumanArtifactIntakeSurface,
    pub gate_count: usize,
    pub passed_gate_count: usize,
    pub blocked_reasons: Vec<String>,
    pub original_artifact_ref: String,
    pub normalized_candidate_ref: String,
    pub validation_report_ref: String,
    pub author_provenance_ref: String,
    pub review_apply_ref: String,
    pub boundary: String,
}

impl HumanArtifactIntakeArtifact {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("human artifact intake artifact is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != HUMAN_ARTIFACT_INTAKE_SCHEMA_VERSION {
            return Err(anyhow!(
                "schemaVersion must be {HUMAN_ARTIFACT_INTAKE_SCHEMA_VERSION}"
            ));
        }
        require_text("intakeId", &self.intake_id)?;
        require_text("artifactId", &self.artifact_id)?;
        require_text("author", &self.author)?;
        if self.author != "human" && !self.author.starts_with("human:") {
            return Err(anyhow!("author must record author=human provenance"));
        }
        require_text("authorProvenanceRef", &self.author_provenance_ref)?;
        require_text("originalArtifactRef", &self.original_artifact_ref)?;
        require_text("normalizedCandidateRef", &self.normalized_candidate_ref)?;
        require_text("targetRef", &self.target_ref)?;
        require_text("targetBaseRef", &self.target_base_ref)?;
        require_text("validationReportRef", &self.validation_report_ref)?;
        require_text("reviewApplyRef", &self.review_apply_ref)?;
        require_boundary(&self.boundary)?;

        if self.original_artifact_ref == self.normalized_candidate_ref {
            return Err(anyhow!(
                "normalizedCandidateRef must differ from originalArtifactRef so untrusted human input is normalized before gates"
            ));
        }
        if !self.human_provenance || !self.intervention_as_evidence || !self.read_gated_write {
            return Err(anyhow!(
                "human artifact intake must be author=human provenance, intervention-as-evidence, and read + gated-write"
            ));
        }
        if self.raw_bypass_requested
            || self.direct_artifact_write
            || self.studio_trusted_write_authority
            || self.human_required_for_autonomous_loop
            || !self.cli_fallback_supported
        {
            return Err(anyhow!(
                "intake must not request raw bypass, direct artifact write, Studio trusted writes, mandatory humans, or broken CLI fallback"
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
                result.kind == *kind && result.status == HumanArtifactIntakeGateStatus::Passed
            })
        });
        match self.status {
            HumanArtifactIntakeStatus::ReadyForReviewApply if all_required_passed => Ok(()),
            HumanArtifactIntakeStatus::ReadyForReviewApply => Err(anyhow!(
                "ready human-authored artifact requires review/apply, scene/source-apply, evaluator, and evidence/provenance gates to pass"
            )),
            HumanArtifactIntakeStatus::Rejected => {
                if self.gate_results.iter().any(|result| {
                    matches!(
                        result.status,
                        HumanArtifactIntakeGateStatus::Failed
                            | HumanArtifactIntakeGateStatus::Blocked
                    )
                }) {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "rejected intake must keep a failed or blocked gate visible"
                    ))
                }
            }
            HumanArtifactIntakeStatus::Stale => {
                if self
                    .gate_results
                    .iter()
                    .any(|result| result.status == HumanArtifactIntakeGateStatus::Stale)
                {
                    Ok(())
                } else {
                    Err(anyhow!("stale intake must keep stale gate evidence visible"))
                }
            }
            HumanArtifactIntakeStatus::Blocked => {
                if !all_required_passed {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "blocked intake cannot declare every required gate passed"
                    ))
                }
            }
        }
    }

    pub fn ready_for_review_apply(&self) -> bool {
        self.validate().is_ok() && self.status == HumanArtifactIntakeStatus::ReadyForReviewApply
    }

    pub fn read_model(&self) -> HumanArtifactIntakeReadModel {
        let blocked_reasons = self
            .gate_results
            .iter()
            .filter(|result| result.status != HumanArtifactIntakeGateStatus::Passed)
            .map(|result| format!("{:?}:{:?}", result.kind, result.status))
            .collect();
        HumanArtifactIntakeReadModel {
            intake_id: self.intake_id.clone(),
            artifact_id: self.artifact_id.clone(),
            artifact_kind: self.artifact_kind,
            author: self.author.clone(),
            status: self.status,
            ready_for_review_apply: self.ready_for_review_apply(),
            captured_via: self.captured_via,
            gate_count: self.gate_results.len(),
            passed_gate_count: self
                .gate_results
                .iter()
                .filter(|result| result.status == HumanArtifactIntakeGateStatus::Passed)
                .count(),
            blocked_reasons,
            original_artifact_ref: self.original_artifact_ref.clone(),
            normalized_candidate_ref: self.normalized_candidate_ref.clone(),
            validation_report_ref: self.validation_report_ref.clone(),
            author_provenance_ref: self.author_provenance_ref.clone(),
            review_apply_ref: self.review_apply_ref.clone(),
            boundary: HUMAN_ARTIFACT_INTAKE_BOUNDARY.to_string(),
        }
    }
}

impl HumanArtifactIntakeGateResult {
    fn validate(&self) -> Result<()> {
        require_text("gate evidenceRef", &self.evidence_ref)?;
        require_text("gate beforeRef", &self.before_ref)?;
        require_text("gate afterRef", &self.after_ref)?;
        Ok(())
    }
}

pub fn validate_human_artifact_intake_json(text: &str) -> Result<HumanArtifactIntakeReadModel> {
    let artifact = HumanArtifactIntakeArtifact::from_json_str(text)?;
    artifact.validate()?;
    Ok(artifact.read_model())
}

fn require_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    let lowered = value.to_ascii_lowercase();
    if lowered.contains("raw_write_bypass")
        || lowered.contains("raw_apply_bypass")
        || lowered.contains("trusted_studio_write")
    {
        return Err(anyhow!("{label} must not reference raw bypass authority"));
    }
    Ok(())
}

fn require_boundary(boundary: &str) -> Result<()> {
    for token in [
        "human-authored artifact intake",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust = data plane",
        "Elixir/OTP + Phoenix LiveView = control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "author=human provenance",
        "no raw bypass",
        "local-first CLI fallback",
        "loop completes without human",
        "#1 and #23 remain open",
    ] {
        if !boundary.contains(token) {
            return Err(anyhow!("boundary must contain `{token}`"));
        }
    }
    Ok(())
}
