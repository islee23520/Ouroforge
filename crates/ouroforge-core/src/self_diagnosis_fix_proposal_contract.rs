//! Self-diagnosis and fix-proposal contract v1 (#2033 / Era L M70).
//!
//! This contract turns an attributed self-audit failure into bounded root-cause
//! hypotheses and a source-apply patch preview. It is intentionally a contract
//! and read model only: no verifier, no persistent store, no source mutation,
//! and no self-application authority.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::{
    SourcePatchPreviewApplyStatus, SourcePatchPreviewArtifact, SourcePatchPreviewRiskLevel,
    SOURCE_PATCH_PREVIEW_SCHEMA_VERSION,
};

pub const SELF_DIAGNOSIS_FIX_PROPOSAL_CONTRACT_SCHEMA_VERSION: &str =
    "self-diagnosis-fix-proposal-contract-v1";

const REQUIRED_PIPELINE_REFS: &[&str] = &[
    "verdict",
    "journal.md",
    "ledger.jsonl",
    "loop-coverage",
    "self-audit-bottleneck-attribution",
    "source-apply",
    "trust-gradient",
];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfDiagnosisFixProposalContract {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    pub diagnosis: SelfDiagnosisRecord,
    #[serde(rename = "sourceApplyProposal")]
    pub source_apply_proposal: SourcePatchPreviewArtifact,
    pub boundary: String,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfDiagnosisRecord {
    #[serde(rename = "diagnosisId")]
    pub diagnosis_id: String,
    #[serde(rename = "attributedMilestoneId")]
    pub attributed_milestone_id: String,
    #[serde(rename = "attributedIssueRef")]
    pub attributed_issue_ref: String,
    #[serde(rename = "basedOnRefs")]
    pub based_on_refs: Vec<String>,
    #[serde(rename = "rootCauseHypotheses")]
    pub root_cause_hypotheses: Vec<SelfRootCauseHypothesis>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfRootCauseHypothesis {
    #[serde(rename = "hypothesisId")]
    pub hypothesis_id: String,
    #[serde(rename = "causalChain")]
    pub causal_chain: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "proposedFixScope")]
    pub proposed_fix_scope: String,
    pub confidence: String,
}

impl SelfDiagnosisFixProposalContract {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let contract: Self = serde_json::from_str(input).map_err(|err| {
            anyhow!("failed to parse self-diagnosis fix-proposal contract: {err}")
        })?;
        contract.validate()?;
        Ok(contract)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_DIAGNOSIS_FIX_PROPOSAL_CONTRACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-diagnosis contract schemaVersion must be {SELF_DIAGNOSIS_FIX_PROPOSAL_CONTRACT_SCHEMA_VERSION}"
            ));
        }
        require_id("titleId", &self.title_id)?;
        self.diagnosis.validate()?;
        validate_source_apply_proposal(&self.source_apply_proposal)?;
        validate_boundary(&self.boundary)?;
        validate_guardrails(&self.guardrails)?;
        Ok(())
    }
}

impl SelfDiagnosisRecord {
    fn validate(&self) -> Result<()> {
        require_id("diagnosisId", &self.diagnosis_id)?;
        require_id("attributedMilestoneId", &self.attributed_milestone_id)?;
        require_issue_ref("attributedIssueRef", &self.attributed_issue_ref)?;
        validate_refs("basedOnRefs", &self.based_on_refs, true)?;
        let joined = self.based_on_refs.join("\n").to_ascii_lowercase();
        for required in REQUIRED_PIPELINE_REFS {
            if !joined.contains(required) {
                return Err(anyhow!(
                    "basedOnRefs must include existing {required} evidence"
                ));
            }
        }
        if self.root_cause_hypotheses.is_empty() {
            return Err(anyhow!("rootCauseHypotheses must not be empty"));
        }
        let mut ids = BTreeSet::new();
        for hypothesis in &self.root_cause_hypotheses {
            hypothesis.validate()?;
            if !ids.insert(hypothesis.hypothesis_id.as_str()) {
                return Err(anyhow!(
                    "rootCauseHypotheses contains duplicate hypothesisId `{}`",
                    hypothesis.hypothesis_id
                ));
            }
        }
        Ok(())
    }
}

impl SelfRootCauseHypothesis {
    fn validate(&self) -> Result<()> {
        require_id("hypothesisId", &self.hypothesis_id)?;
        validate_texts("causalChain", &self.causal_chain, true)?;
        if self.causal_chain.len() < 3 {
            return Err(anyhow!(
                "causalChain must include detect, explain/trace, and proposal links"
            ));
        }
        validate_refs("evidenceRefs", &self.evidence_refs, true)?;
        require_text("proposedFixScope", &self.proposed_fix_scope)?;
        match self.confidence.as_str() {
            "low" | "medium" | "high" => Ok(()),
            _ => Err(anyhow!("confidence must be low, medium, or high")),
        }
    }
}

fn validate_source_apply_proposal(proposal: &SourcePatchPreviewArtifact) -> Result<()> {
    if proposal.schema_version != SOURCE_PATCH_PREVIEW_SCHEMA_VERSION {
        return Err(anyhow!(
            "sourceApplyProposal must reuse {SOURCE_PATCH_PREVIEW_SCHEMA_VERSION}"
        ));
    }
    if proposal.source_mutation_apply_status != SourcePatchPreviewApplyStatus::Blocked {
        return Err(anyhow!(
            "sourceApplyProposal sourceMutationApplyStatus must remain blocked at M70"
        ));
    }
    if proposal.targets.is_empty() {
        return Err(anyhow!("sourceApplyProposal targets must not be empty"));
    }
    if proposal.linked_evidence.is_empty() {
        return Err(anyhow!(
            "sourceApplyProposal linkedEvidence must cite diagnosis evidence"
        ));
    }
    if proposal.required_tests.is_empty() {
        return Err(anyhow!(
            "sourceApplyProposal requiredTests must name re-verification commands"
        ));
    }
    let evidence_text = proposal
        .linked_evidence
        .iter()
        .map(|evidence| evidence.path.as_str())
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase();
    for required in ["verdict", "journal", "ledger", "loop-coverage"] {
        if !evidence_text.contains(required) {
            return Err(anyhow!(
                "sourceApplyProposal linkedEvidence must include {required} evidence"
            ));
        }
    }
    let checklist = proposal.reviewer_checklist.join("\n").to_ascii_lowercase();
    if matches!(
        proposal.risk_level,
        SourcePatchPreviewRiskLevel::High | SourcePatchPreviewRiskLevel::Critical
    ) && !(checklist.contains("human go/no-go") && checklist.contains("no auto-apply"))
    {
        return Err(anyhow!(
            "high-risk sourceApplyProposal must keep thin human go/no-go and no auto-apply checklist items"
        ));
    }
    let forbidden_actions = proposal
        .read_model_prototype
        .as_ref()
        .map(|model| model.forbidden_actions.join("\n").to_ascii_lowercase())
        .unwrap_or_default();
    if !(forbidden_actions.contains("apply") && forbidden_actions.contains("merge")) {
        return Err(anyhow!(
            "sourceApplyProposal read model must forbid apply and merge at M70"
        ));
    }
    Ok(())
}

fn validate_boundary(boundary: &str) -> Result<()> {
    let lower = boundary.to_ascii_lowercase();
    for required in [
        "read-only",
        "verdict",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "no new verification engine",
        "no new data plane",
        "no self-application",
        "without a human",
        "never auto-applied",
        "human ring 2",
    ] {
        if !lower.contains(required) {
            return Err(anyhow!("boundary must mention {required}"));
        }
    }
    Ok(())
}

fn validate_guardrails(guardrails: &[String]) -> Result<()> {
    validate_texts("guardrails", guardrails, true)?;
    let joined = guardrails.join("\n").to_ascii_lowercase();
    for required in [
        "source-apply",
        "trust-gradient",
        "high-risk",
        "human go/no-go",
        "no new verification engine",
        "no new data plane",
    ] {
        if !joined.contains(required) {
            return Err(anyhow!("guardrails must mention {required}"));
        }
    }
    Ok(())
}

fn validate_refs(label: &str, refs: &[String], non_empty: bool) -> Result<()> {
    if non_empty && refs.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    let mut seen = BTreeSet::new();
    for reference in refs {
        require_ref(label, reference)?;
        if !seen.insert(reference) {
            return Err(anyhow!("{label} contains duplicate ref `{reference}`"));
        }
    }
    Ok(())
}

fn validate_texts(label: &str, values: &[String], non_empty: bool) -> Result<()> {
    if non_empty && values.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    for value in values {
        require_text(label, value)?;
    }
    Ok(())
}

fn require_issue_ref(label: &str, value: &str) -> Result<()> {
    if value.starts_with('#') && value[1..].chars().all(|ch| ch.is_ascii_digit()) {
        Ok(())
    } else {
        Err(anyhow!("{label} must be a GitHub issue ref like #2033"))
    }
}

fn require_ref(label: &str, value: &str) -> Result<()> {
    require_text(label, value)?;
    if value.starts_with('/')
        || value.contains("..")
        || value.contains('\\')
        || value.contains(';')
        || value.contains("&&")
        || value.contains('|')
    {
        return Err(anyhow!("{label} must be a safe local evidence ref"));
    }
    Ok(())
}

fn require_id(label: &str, value: &str) -> Result<()> {
    require_text(label, value)?;
    if value.len() <= 128
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        Ok(())
    } else {
        Err(anyhow!("{label} must be a bounded local id"))
    }
}

fn require_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{label} must not be empty"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_contract() -> SelfDiagnosisFixProposalContract {
        SelfDiagnosisFixProposalContract::from_json_str(include_str!(
            "../../../examples/real-title-dogfood-v1/self-diagnosis-fix-proposal-v1/contract.fixture.json"
        ))
        .expect("fixture contract validates")
    }

    #[test]
    fn fixture_contract_reuses_source_apply_preview_without_apply_authority() {
        let contract = fixture_contract();
        assert_eq!(
            contract.source_apply_proposal.schema_version,
            "patch-preview.v1"
        );
        assert_eq!(
            contract.source_apply_proposal.source_mutation_apply_status,
            SourcePatchPreviewApplyStatus::Blocked
        );
        assert_eq!(
            contract.source_apply_proposal.risk_level,
            SourcePatchPreviewRiskLevel::High
        );
    }

    #[test]
    fn high_risk_proposal_requires_human_go_no_go_checklist() {
        let mut contract = fixture_contract();
        contract.source_apply_proposal.reviewer_checklist = vec!["review evidence".to_string()];
        let error = contract.validate().expect_err("missing checklist rejected");
        assert!(error.to_string().contains("human go/no-go"));
    }
}
