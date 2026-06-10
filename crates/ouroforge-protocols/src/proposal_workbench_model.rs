//! Proposal Workbench Model v1 (#2375, #1 M125).
//!
//! Data model for evidence-linked agent gameplay proposals. Proposals reference
//! evidence by `runId` + artifact path + digest, declare a bounded diff scope,
//! mandatory risk and rollback, reviewer requirements, and non-goals. The model
//! is representable inside Safe Source Apply review artifacts and contains no
//! self-apply flag, hidden command authority, script execution, or browser
//! trusted-write surface.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PROPOSAL_WORKBENCH_SCHEMA_VERSION: &str = "proposal-workbench-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProposalFindingCategory {
    RuntimeUx,
    Gameplay,
    Authoring,
    Performance,
    Accessibility,
    Regression,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProposalDiffKind {
    SceneData,
    AssetReference,
    BehaviorData,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProposalRiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProposalEvidenceRef {
    #[serde(rename = "runId")]
    pub run_id: String,
    pub path: String,
    pub digest: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProposalWorkbenchModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    pub category: ProposalFindingCategory,
    #[serde(rename = "problemEvidenceRefs")]
    pub problem_evidence_refs: Vec<ProposalEvidenceRef>,
    pub hypothesis: String,
    #[serde(rename = "diffScope")]
    pub diff_scope: ProposalDiffScope,
    #[serde(rename = "expectedImpact")]
    pub expected_impact: String,
    pub risk: ProposalRisk,
    pub rollback: ProposalRollback,
    #[serde(rename = "reviewerRequirements")]
    pub reviewer_requirements: Vec<String>,
    #[serde(rename = "nonGoals")]
    pub non_goals: Vec<String>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProposalDiffScope {
    pub kind: ProposalDiffKind,
    #[serde(rename = "targetPaths")]
    pub target_paths: Vec<String>,
    #[serde(rename = "operationSummary")]
    pub operation_summary: String,
    #[serde(rename = "boundedChangeCount")]
    pub bounded_change_count: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProposalRisk {
    pub level: ProposalRiskLevel,
    pub rationale: String,
    #[serde(rename = "mitigations")]
    pub mitigations: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProposalRollback {
    #[serde(rename = "rollbackPlan")]
    pub rollback_plan: String,
    #[serde(rename = "rollbackRefs")]
    pub rollback_refs: Vec<ProposalEvidenceRef>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SafeSourceApplyReviewProposalSection {
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<ProposalEvidenceRef>,
    #[serde(rename = "targetPaths")]
    pub target_paths: Vec<String>,
    #[serde(rename = "reviewerRequirements")]
    pub reviewer_requirements: Vec<String>,
    #[serde(rename = "riskLevel")]
    pub risk_level: ProposalRiskLevel,
    #[serde(rename = "rollbackPlan")]
    pub rollback_plan: String,
    #[serde(rename = "forbiddenAuthority")]
    pub forbidden_authority: Vec<String>,
}

impl ProposalWorkbenchModel {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let proposal: Self =
            serde_json::from_str(input).context("failed to parse proposal workbench model JSON")?;
        proposal.validate()?;
        Ok(proposal)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PROPOSAL_WORKBENCH_SCHEMA_VERSION {
            return Err(anyhow!(
                "proposal workbench schemaVersion must be {PROPOSAL_WORKBENCH_SCHEMA_VERSION}"
            ));
        }
        require_local_id("proposal workbench proposalId", &self.proposal_id)?;
        require_nonempty(
            "proposal workbench problemEvidenceRefs",
            self.problem_evidence_refs.len(),
        )?;
        for reference in &self.problem_evidence_refs {
            reference.validate("proposal problem evidence ref")?;
        }
        require_boundary_text("proposal workbench hypothesis", &self.hypothesis)?;
        self.diff_scope.validate()?;
        require_boundary_text("proposal workbench expectedImpact", &self.expected_impact)?;
        self.risk.validate()?;
        self.rollback.validate()?;
        require_nonempty(
            "proposal workbench reviewerRequirements",
            self.reviewer_requirements.len(),
        )?;
        for requirement in &self.reviewer_requirements {
            require_boundary_text("proposal reviewer requirement", requirement)?;
        }
        require_nonempty("proposal workbench nonGoals", self.non_goals.len())?;
        for non_goal in &self.non_goals {
            require_boundary_text("proposal non-goal", non_goal)?;
        }
        require_nonempty("proposal workbench guardrails", self.guardrails.len())?;
        for guardrail in &self.guardrails {
            require_boundary_text("proposal guardrail", guardrail)?;
        }
        Ok(())
    }

    pub fn safe_source_apply_review_section(&self) -> Result<SafeSourceApplyReviewProposalSection> {
        self.validate()?;
        let mut evidence_refs = self.problem_evidence_refs.clone();
        evidence_refs.extend(self.rollback.rollback_refs.iter().cloned());
        evidence_refs
            .sort_by(|a, b| (&a.run_id, &a.path, &a.digest).cmp(&(&b.run_id, &b.path, &b.digest)));
        evidence_refs.dedup();
        Ok(SafeSourceApplyReviewProposalSection {
            proposal_id: self.proposal_id.clone(),
            evidence_refs,
            target_paths: self.diff_scope.target_paths.clone(),
            reviewer_requirements: self.reviewer_requirements.clone(),
            risk_level: self.risk.level,
            rollback_plan: self.rollback.rollback_plan.clone(),
            forbidden_authority: forbidden_authority(),
        })
    }
}

impl ProposalDiffScope {
    fn validate(&self) -> Result<()> {
        require_nonempty("proposal diffScope targetPaths", self.target_paths.len())?;
        if self.bounded_change_count == 0 || self.bounded_change_count > 20 {
            return Err(anyhow!(
                "proposal diffScope boundedChangeCount must be between 1 and 20"
            ));
        }
        require_boundary_text(
            "proposal diffScope operationSummary",
            &self.operation_summary,
        )?;
        let mut seen = BTreeSet::new();
        for path in &self.target_paths {
            require_relative_path("proposal diffScope target path", path)?;
            if !seen.insert(path.clone()) {
                return Err(anyhow!("proposal diffScope targetPaths must be unique"));
            }
            if blocked_target_path(path) {
                return Err(anyhow!(
                    "proposal diffScope target path `{path}` is outside proposal-owned scene/asset/behavior data"
                ));
            }
        }
        Ok(())
    }
}

impl ProposalRisk {
    fn validate(&self) -> Result<()> {
        require_boundary_text("proposal risk rationale", &self.rationale)?;
        require_nonempty("proposal risk mitigations", self.mitigations.len())?;
        for mitigation in &self.mitigations {
            require_boundary_text("proposal risk mitigation", mitigation)?;
        }
        Ok(())
    }
}

impl ProposalRollback {
    fn validate(&self) -> Result<()> {
        require_boundary_text("proposal rollbackPlan", &self.rollback_plan)?;
        require_nonempty("proposal rollbackRefs", self.rollback_refs.len())?;
        for reference in &self.rollback_refs {
            reference.validate("proposal rollback ref")?;
        }
        Ok(())
    }
}

impl ProposalEvidenceRef {
    fn validate(&self, label: &str) -> Result<()> {
        require_local_id(&format!("{label} runId"), &self.run_id)?;
        require_relative_path(&format!("{label} path"), &self.path)?;
        require_digest(&format!("{label} digest"), &self.digest)
    }
}

fn blocked_target_path(path: &str) -> bool {
    path.starts_with(".github/")
        || path.starts_with("scripts/")
        || path.contains("Cargo.toml")
        || path.contains("Cargo.lock")
        || path.contains("package.json")
        || path.contains("runtime.js")
        || path.contains("cockpit.js")
        || path.starts_with("crates/")
        || path.starts_with(".git/")
}

fn forbidden_authority() -> Vec<String> {
    vec![
        "self_apply".to_string(),
        "hidden_command".to_string(),
        "browser_trusted_write".to_string(),
        "auto_apply".to_string(),
        "auto_merge".to_string(),
        "dependency_install".to_string(),
        "publish_deploy_upload".to_string(),
    ]
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if value.len() > 160
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, dot, or colon"
        ));
    }
    Ok(())
}

fn require_relative_path(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} must stay inside the local artifact root"));
    }
    Ok(())
}

fn require_digest(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if !value.contains(':') || value.len() > 160 {
        return Err(anyhow!("{field} must include an algorithm prefix"));
    }
    Ok(())
}

fn require_boundary_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "command bridge",
        "hidden command",
        "browser trusted write",
        "self-apply",
        "self apply",
        "self-approval",
        "auto-merge",
        "auto-apply",
        "dependency install",
        "publish",
        "deploy",
        "upload",
        "production-ready",
        "godot replacement",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden proposal authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposal_review_section_is_safe_source_apply_compatible() {
        let proposal = accepted_proposal();
        proposal.validate().unwrap();
        let review = proposal.safe_source_apply_review_section().unwrap();
        assert_eq!(review.proposal_id, "proposal-runtime-exit");
        assert_eq!(
            review.target_paths,
            vec!["examples/playable-demo-v2/collect-and-exit/scene.patch.json"]
        );
        assert!(review
            .forbidden_authority
            .contains(&"self_apply".to_string()));
        assert!(review
            .evidence_refs
            .iter()
            .all(|reference| !reference.path.contains("..")));
    }

    #[test]
    fn proposal_without_evidence_refs_is_rejected() {
        let mut proposal = accepted_proposal();
        proposal.problem_evidence_refs.clear();
        let err = proposal.validate().unwrap_err().to_string();
        assert!(
            err.contains("problemEvidenceRefs"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn proposal_without_bounded_diff_scope_is_rejected() {
        let mut proposal = accepted_proposal();
        proposal.diff_scope.bounded_change_count = 0;
        let err = proposal.validate().unwrap_err().to_string();
        assert!(
            err.contains("boundedChangeCount"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn hidden_command_authority_is_rejected() {
        let mut proposal = accepted_proposal();
        proposal.hypothesis = "use hidden command to patch the scene".to_string();
        let err = proposal.validate().unwrap_err().to_string();
        assert!(
            err.contains("forbidden proposal authority text"),
            "unexpected error: {err}"
        );
    }

    fn accepted_proposal() -> ProposalWorkbenchModel {
        ProposalWorkbenchModel {
            schema_version: PROPOSAL_WORKBENCH_SCHEMA_VERSION.to_string(),
            proposal_id: "proposal-runtime-exit".to_string(),
            category: ProposalFindingCategory::Gameplay,
            problem_evidence_refs: vec![reference(
                "run-before",
                "evidence/runtime-events.json",
                "sha256:events",
            )],
            hypothesis: "exit trigger is missing from the collect-and-exit scene data".to_string(),
            diff_scope: ProposalDiffScope {
                kind: ProposalDiffKind::SceneData,
                target_paths: vec![
                    "examples/playable-demo-v2/collect-and-exit/scene.patch.json".to_string(),
                ],
                operation_summary: "add bounded exit trigger data".to_string(),
                bounded_change_count: 1,
            },
            expected_impact: "replay reaches the exit event and clears the known gap".to_string(),
            risk: ProposalRisk {
                level: ProposalRiskLevel::Low,
                rationale: "single scene-data target with deterministic replay evidence"
                    .to_string(),
                mitigations: vec!["reviewer checks before-after comparison".to_string()],
            },
            rollback: ProposalRollback {
                rollback_plan: "restore previous scene data from rollback ref".to_string(),
                rollback_refs: vec![reference(
                    "run-before",
                    "rollback/scene.json",
                    "sha256:rollback",
                )],
            },
            reviewer_requirements: vec![
                "independent reviewer must verify evidence refs and bounded target".to_string(),
            ],
            non_goals: vec!["no runtime code or dependency changes".to_string()],
            guardrails: vec!["Safe Source Apply review required before trusted apply".to_string()],
        }
    }

    fn reference(run_id: &str, path: &str, digest: &str) -> ProposalEvidenceRef {
        ProposalEvidenceRef {
            run_id: run_id.to_string(),
            path: path.to_string(),
            digest: digest.to_string(),
        }
    }
}
