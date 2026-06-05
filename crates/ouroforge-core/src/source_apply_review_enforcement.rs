//! Source Apply Review Decision Enforcement v1 (#704, #1 Milestone 15).
//!
//! Requires an accepted, independent review decision that matches the exact
//! apply transaction before trusted source apply can proceed. This module owns
//! a self-contained enforcement artifact and a fail-closed evaluation: it never
//! applies patches, merges branches, or executes commands. It only reports
//! whether the recorded review decision is sufficient to consider an apply
//! transaction review-gated, and emits explicit blocked reasons otherwise.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION: &str =
    "source-apply-review-enforcement-v1";

/// Recorded state of an independent review decision.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyReviewDecisionState {
    Accepted,
    Rejected,
    Deferred,
    Withdrawn,
    /// No usable decision was recorded for this transaction yet.
    Missing,
}

/// Outcome of enforcing the review decision against the apply transaction.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyReviewEnforcementStatus {
    /// The accepted, independent, exactly-matching decision permits readiness.
    Ready,
    /// At least one fail-closed condition blocks apply readiness.
    Blocked,
}

/// One file target the apply transaction expects to touch, with its exact
/// before/after content hashes. Coverage is matched exactly: the review
/// decision must cover the same path set with the same hashes.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyReviewTargetCoverage {
    pub path: String,
    #[serde(rename = "beforeHash")]
    pub before_hash: String,
    #[serde(rename = "afterHash")]
    pub after_hash: String,
}

/// The enforcement artifact: the apply transaction's expectations alongside the
/// independent review decision's recorded claims. Evaluation is deterministic.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyReviewEnforcement {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "patchPreviewId")]
    pub patch_preview_id: String,
    #[serde(rename = "expectedDiffHash")]
    pub expected_diff_hash: String,
    /// Revision the transaction is based on; the decision must match it to be fresh.
    #[serde(rename = "transactionBaseRevision")]
    pub transaction_base_revision: String,
    #[serde(rename = "expectedTargets")]
    pub expected_targets: Vec<SourceApplyReviewTargetCoverage>,
    /// Author/proposer of the patch; must differ from the reviewer.
    #[serde(rename = "proposerId")]
    pub proposer_id: String,
    /// Independent reviewer that produced the decision.
    #[serde(rename = "reviewerId")]
    pub reviewer_id: String,
    #[serde(rename = "decisionState")]
    pub decision_state: SourceApplyReviewDecisionState,
    #[serde(rename = "decisionTransactionId")]
    pub decision_transaction_id: String,
    #[serde(rename = "decisionPreviewId")]
    pub decision_preview_id: String,
    #[serde(rename = "decisionDiffHash")]
    pub decision_diff_hash: String,
    #[serde(rename = "decisionBaseRevision")]
    pub decision_base_revision: String,
    #[serde(rename = "decisionTargets")]
    pub decision_targets: Vec<SourceApplyReviewTargetCoverage>,
    pub guardrails: Vec<String>,
}

/// Display-only evaluation read model. It records the enforced status and the
/// fail-closed blocked reasons. It carries no apply/merge/command authority.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyReviewEnforcementEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "patchPreviewId")]
    pub patch_preview_id: String,
    pub status: SourceApplyReviewEnforcementStatus,
    #[serde(rename = "reviewState")]
    pub review_state: SourceApplyReviewDecisionState,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplyReviewEnforcement {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply review enforcement JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    /// Structural validation: shapes and bounded ids. The accept/block decision
    /// is computed separately by [`Self::evaluate`].
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply review enforcement schemaVersion must be {SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "source apply review enforcement applyTransactionId",
            &self.apply_transaction_id,
        )?;
        require_local_id(
            "source apply review enforcement patchPreviewId",
            &self.patch_preview_id,
        )?;
        require_text(
            "source apply review enforcement expectedDiffHash",
            &self.expected_diff_hash,
        )?;
        require_text(
            "source apply review enforcement transactionBaseRevision",
            &self.transaction_base_revision,
        )?;
        require_text(
            "source apply review enforcement proposerId",
            &self.proposer_id,
        )?;
        // reviewerId may be empty only when the decision is Missing.
        if self.decision_state != SourceApplyReviewDecisionState::Missing {
            require_text(
                "source apply review enforcement reviewerId",
                &self.reviewer_id,
            )?;
        }
        if self.expected_targets.is_empty() {
            return Err(anyhow!(
                "source apply review enforcement expectedTargets must not be empty"
            ));
        }
        for target in self
            .expected_targets
            .iter()
            .chain(self.decision_targets.iter())
        {
            target.validate()?;
        }
        require_nonempty(
            "source apply review enforcement guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text("source apply review enforcement guardrails", guardrail)?;
        }
        Ok(())
    }

    /// Fail-closed evaluation. Apply readiness requires an accepted, independent
    /// decision whose recorded coverage matches the transaction exactly.
    pub fn evaluate(&self) -> SourceApplyReviewEnforcementEvaluation {
        let mut blocked = Vec::new();

        if self.decision_state != SourceApplyReviewDecisionState::Accepted {
            blocked.push(format!(
                "review decision is not accepted (state: {})",
                decision_state_label(&self.decision_state)
            ));
        }

        if self.reviewer_id.trim().is_empty() {
            blocked.push("review decision has no recorded independent reviewer".to_string());
        } else if self.reviewer_id.trim() == self.proposer_id.trim() {
            blocked.push(
                "self-review/self-approval is not permitted: reviewer must differ from proposer"
                    .to_string(),
            );
        }

        if self.decision_transaction_id != self.apply_transaction_id {
            blocked.push(
                "review decision does not match the apply transaction id (mismatched decision)"
                    .to_string(),
            );
        }
        if self.decision_preview_id != self.patch_preview_id {
            blocked.push("review decision does not match the patch preview id".to_string());
        }
        if self.decision_diff_hash != self.expected_diff_hash {
            blocked
                .push("review decision diff hash does not match the transaction diff".to_string());
        }
        if self.decision_base_revision != self.transaction_base_revision {
            blocked.push(
                "review decision is stale: it was made against a different base revision"
                    .to_string(),
            );
        }

        self.push_coverage_gaps(&mut blocked);

        let status = if blocked.is_empty() {
            SourceApplyReviewEnforcementStatus::Ready
        } else {
            SourceApplyReviewEnforcementStatus::Blocked
        };

        let mut evidence_summary = vec![
            format!("transaction:{}", self.apply_transaction_id),
            format!("preview:{}", self.patch_preview_id),
            format!("reviewer:{}", reviewer_label(&self.reviewer_id)),
            format!("targets:{}", self.expected_targets.len()),
            format!("decision:{}", decision_state_label(&self.decision_state)),
        ];
        if status == SourceApplyReviewEnforcementStatus::Ready {
            evidence_summary.push("accepted independent decision matches transaction".to_string());
        }

        SourceApplyReviewEnforcementEvaluation {
            schema_version: SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION.to_string(),
            apply_transaction_id: self.apply_transaction_id.clone(),
            patch_preview_id: self.patch_preview_id.clone(),
            status,
            review_state: self.decision_state,
            evidence_summary,
            blocked_reasons: blocked,
            allowed_actions: vec![
                "inspect_review_enforcement".to_string(),
                "inspect_blocked_reasons".to_string(),
            ],
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "merge_branch".to_string(),
                "execute_command".to_string(),
                "self_approve".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize source apply review enforcement evaluation JSON")
    }

    pub fn is_ready(&self) -> bool {
        self.evaluate().status == SourceApplyReviewEnforcementStatus::Ready
    }

    fn push_coverage_gaps(&self, blocked: &mut Vec<String>) {
        for expected in &self.expected_targets {
            match self
                .decision_targets
                .iter()
                .find(|target| target.path == expected.path)
            {
                None => blocked.push(format!(
                    "review decision does not cover file target `{}` (partial coverage)",
                    expected.path
                )),
                Some(covered) => {
                    if covered.before_hash != expected.before_hash
                        || covered.after_hash != expected.after_hash
                    {
                        blocked.push(format!(
                            "review decision hash coverage for `{}` does not match the transaction",
                            expected.path
                        ));
                    }
                }
            }
        }
        for covered in &self.decision_targets {
            if !self
                .expected_targets
                .iter()
                .any(|expected| expected.path == covered.path)
            {
                blocked.push(format!(
                    "review decision covers unexpected file target `{}` outside the transaction",
                    covered.path
                ));
            }
        }
    }
}

impl SourceApplyReviewTargetCoverage {
    fn validate(&self) -> Result<()> {
        require_local_ref("source apply review enforcement target path", &self.path)?;
        require_text(
            "source apply review enforcement target beforeHash",
            &self.before_hash,
        )?;
        require_text(
            "source apply review enforcement target afterHash",
            &self.after_hash,
        )?;
        Ok(())
    }
}

fn decision_state_label(state: &SourceApplyReviewDecisionState) -> &'static str {
    match state {
        SourceApplyReviewDecisionState::Accepted => "accepted",
        SourceApplyReviewDecisionState::Rejected => "rejected",
        SourceApplyReviewDecisionState::Deferred => "deferred",
        SourceApplyReviewDecisionState::Withdrawn => "withdrawn",
        SourceApplyReviewDecisionState::Missing => "missing",
    }
}

fn reviewer_label(reviewer_id: &str) -> String {
    if reviewer_id.trim().is_empty() {
        "none".to_string()
    } else {
        reviewer_id.to_string()
    }
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 128
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, or dot"
        ));
    }
    Ok(())
}

fn require_local_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} must stay inside the local trusted worktree"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "eval(",
        "command bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "godot replacement",
        "production-ready",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden source-apply authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}
