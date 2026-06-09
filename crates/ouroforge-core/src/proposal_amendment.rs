//! Proposal amendment capture and re-verify contract (#2054).
//!
//! This module is the Rust data-plane contract for amend-before-approve human
//! intervention. It records a human edit as intervention evidence, verifies that
//! the amended proposal passed the existing gates, and exposes whether it may be
//! routed onward to review/apply. It never writes trusted artifacts, bypasses
//! gates, grants Studio/Phoenix artifact authority, or requires a human for the
//! autonomous loop to complete.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PROPOSAL_AMENDMENT_SCHEMA_VERSION: &str = "ouroforge.proposal-amendment.v1";
pub const PROPOSAL_AMENDMENT_BOUNDARY: &str = "intervention-as-evidence; read + gated-write; Rust data plane validates and records; Elixir/Phoenix control + presentation only; review/apply, scene/source-apply, evaluator, evidence/provenance gates required; no raw bypass; local-first CLI fallback; #1 and #23 remain open";

const REQUIRED_GATES: &[ProposalAmendmentGateKind] = &[
    ProposalAmendmentGateKind::ReviewApply,
    ProposalAmendmentGateKind::SceneSourceApply,
    ProposalAmendmentGateKind::Evaluator,
    ProposalAmendmentGateKind::DesignIntegrity,
];

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ProposalAmendmentGateKind {
    ReviewApply,
    SceneSourceApply,
    Evaluator,
    DesignIntegrity,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProposalAmendmentGateStatus {
    Passed,
    Failed,
    Blocked,
    Stale,
    Missing,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProposalAmendmentStatus {
    ReadyForReviewApply,
    Blocked,
    Stale,
    Rejected,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProposalAmendmentArtifact {
    pub schema_version: String,
    pub amendment_id: String,
    pub proposal_id: String,
    pub base_proposal_ref: String,
    pub amended_proposal_ref: String,
    pub human_actor: String,
    pub edit_summary: String,
    pub captured_via: ProposalAmendmentCaptureSurface,
    pub intervention_as_evidence: bool,
    pub before_evidence_refs: Vec<String>,
    pub after_evidence_refs: Vec<String>,
    pub provenance_refs: Vec<String>,
    pub gate_results: Vec<ProposalAmendmentGateResult>,
    pub status: ProposalAmendmentStatus,
    pub review_apply_ref: String,
    pub auto_apply_performed: bool,
    pub raw_bypass_requested: bool,
    pub studio_trusted_write_authority: bool,
    pub human_required_for_autonomous_loop: bool,
    pub cli_fallback_supported: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProposalAmendmentCaptureSurface {
    Cli,
    StudioPhoenixLiveView,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProposalAmendmentGateResult {
    pub kind: ProposalAmendmentGateKind,
    pub status: ProposalAmendmentGateStatus,
    pub evidence_ref: String,
    pub before_ref: String,
    pub after_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProposalAmendmentReadModel {
    pub amendment_id: String,
    pub proposal_id: String,
    pub status: ProposalAmendmentStatus,
    pub ready_for_review_apply: bool,
    pub captured_via: ProposalAmendmentCaptureSurface,
    pub gate_count: usize,
    pub passed_gate_count: usize,
    pub blocked_reasons: Vec<String>,
    pub before_evidence_refs: Vec<String>,
    pub after_evidence_refs: Vec<String>,
    pub provenance_refs: Vec<String>,
    pub review_apply_ref: String,
    pub boundary: String,
}

impl ProposalAmendmentArtifact {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("proposal amendment artifact is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PROPOSAL_AMENDMENT_SCHEMA_VERSION {
            return Err(anyhow!(
                "schemaVersion must be {PROPOSAL_AMENDMENT_SCHEMA_VERSION}"
            ));
        }
        require_text("amendmentId", &self.amendment_id)?;
        require_text("proposalId", &self.proposal_id)?;
        require_text("baseProposalRef", &self.base_proposal_ref)?;
        require_text("amendedProposalRef", &self.amended_proposal_ref)?;
        if self.base_proposal_ref == self.amended_proposal_ref {
            return Err(anyhow!(
                "amendedProposalRef must differ from baseProposalRef to record a human edit"
            ));
        }
        require_text("humanActor", &self.human_actor)?;
        require_text("editSummary", &self.edit_summary)?;
        require_refs("beforeEvidenceRefs", &self.before_evidence_refs)?;
        require_refs("afterEvidenceRefs", &self.after_evidence_refs)?;
        require_refs("provenanceRefs", &self.provenance_refs)?;
        require_text("reviewApplyRef", &self.review_apply_ref)?;
        require_boundary(&self.boundary)?;

        if !self.intervention_as_evidence {
            return Err(anyhow!(
                "proposal amendment must be recorded as intervention-as-evidence"
            ));
        }
        if self.auto_apply_performed
            || self.raw_bypass_requested
            || self.studio_trusted_write_authority
            || self.human_required_for_autonomous_loop
            || !self.cli_fallback_supported
        {
            return Err(anyhow!(
                "amendment must not auto-apply, request raw bypass, grant Studio trusted writes, require humans, or break CLI fallback"
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
                result.kind == *kind && result.status == ProposalAmendmentGateStatus::Passed
            })
        });
        match self.status {
            ProposalAmendmentStatus::ReadyForReviewApply if all_required_passed => Ok(()),
            ProposalAmendmentStatus::ReadyForReviewApply => Err(anyhow!(
                "ready amended proposal requires review/apply, scene/source-apply, evaluator, and design-integrity gates to pass"
            )),
            ProposalAmendmentStatus::Rejected => {
                if self.gate_results.iter().any(|result| {
                    matches!(
                        result.status,
                        ProposalAmendmentGateStatus::Failed | ProposalAmendmentGateStatus::Blocked
                    )
                }) {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "rejected amendment must keep a failed or blocked gate visible"
                    ))
                }
            }
            ProposalAmendmentStatus::Stale => {
                if self
                    .gate_results
                    .iter()
                    .any(|result| result.status == ProposalAmendmentGateStatus::Stale)
                {
                    Ok(())
                } else {
                    Err(anyhow!("stale amendment must keep stale gate evidence visible"))
                }
            }
            ProposalAmendmentStatus::Blocked => {
                if !all_required_passed {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "blocked amendment cannot declare every required gate passed"
                    ))
                }
            }
        }
    }

    pub fn ready_for_review_apply(&self) -> bool {
        self.validate().is_ok() && self.status == ProposalAmendmentStatus::ReadyForReviewApply
    }

    pub fn read_model(&self) -> ProposalAmendmentReadModel {
        let blocked_reasons = self
            .gate_results
            .iter()
            .filter(|result| result.status != ProposalAmendmentGateStatus::Passed)
            .map(|result| format!("{:?}:{:?}", result.kind, result.status))
            .collect();
        ProposalAmendmentReadModel {
            amendment_id: self.amendment_id.clone(),
            proposal_id: self.proposal_id.clone(),
            status: self.status,
            ready_for_review_apply: self.ready_for_review_apply(),
            captured_via: self.captured_via,
            gate_count: self.gate_results.len(),
            passed_gate_count: self
                .gate_results
                .iter()
                .filter(|result| result.status == ProposalAmendmentGateStatus::Passed)
                .count(),
            blocked_reasons,
            before_evidence_refs: self.before_evidence_refs.clone(),
            after_evidence_refs: self.after_evidence_refs.clone(),
            provenance_refs: self.provenance_refs.clone(),
            review_apply_ref: self.review_apply_ref.clone(),
            boundary: PROPOSAL_AMENDMENT_BOUNDARY.to_string(),
        }
    }
}

impl ProposalAmendmentGateResult {
    fn validate(&self) -> Result<()> {
        require_text("gate evidenceRef", &self.evidence_ref)?;
        require_text("gate beforeRef", &self.before_ref)?;
        require_text("gate afterRef", &self.after_ref)?;
        Ok(())
    }
}

pub fn validate_proposal_amendment_json(text: &str) -> Result<ProposalAmendmentReadModel> {
    let artifact = ProposalAmendmentArtifact::from_json_str(text)?;
    artifact.validate()?;
    Ok(artifact.read_model())
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
        "intervention-as-evidence",
        "read + gated-write",
        "Rust data plane",
        "Elixir/Phoenix control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "no raw bypass",
        "local-first CLI fallback",
        "#1 and #23 remain open",
    ] {
        if !boundary.contains(token) {
            return Err(anyhow!("boundary must contain `{token}`"));
        }
    }
    Ok(())
}
