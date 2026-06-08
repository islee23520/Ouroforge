//! Source Apply Post-Apply Rerun and Comparison v1 (#708, #1 Milestone 15).
//!
//! Connects trusted source apply to rerun and comparison evidence so outcomes
//! are evaluated before any promotion claim. It records before/after run ids and
//! per-dimension verdict/performance/evidence/runtime changes, classifies the
//! overall state, and links the comparison to the transaction, review,
//! verification logs, rollback, QA backlog, and audit ledger. Missing and
//! unsupported rerun states are explicit; a promotion claim requires evidence.
//! This module evaluates recorded evidence only; it never runs or applies.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION: &str = "source-apply-rerun-comparison-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyRerunStatus {
    Completed,
    Unsupported,
    Missing,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyComparisonState {
    Improved,
    Regressed,
    Unchanged,
    Inconclusive,
    MissingBefore,
    MissingAfter,
    Unsupported,
    Flaky,
}

/// One compared dimension (scenario verdict, performance, visual, runtime, …).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyComparisonDimension {
    pub name: String,
    pub state: SourceApplyComparisonState,
    pub detail: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyRerunComparison {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "reviewDecisionRef")]
    pub review_decision_ref: String,
    #[serde(rename = "verificationLogRef")]
    pub verification_log_ref: String,
    #[serde(rename = "rollbackSnapshotRef")]
    pub rollback_snapshot_ref: String,
    #[serde(rename = "auditLedgerRef")]
    pub audit_ledger_ref: String,
    #[serde(
        rename = "qaBacklogRefs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub qa_backlog_refs: Vec<String>,
    #[serde(rename = "rerunStatus")]
    pub rerun_status: SourceApplyRerunStatus,
    #[serde(
        rename = "beforeRunId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub before_run_id: Option<String>,
    #[serde(
        rename = "afterRunId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_run_id: Option<String>,
    /// Generated, untracked output root for rerun artifacts.
    #[serde(rename = "generatedOutputRoot")]
    pub generated_output_root: String,
    pub dimensions: Vec<SourceApplyComparisonDimension>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyRerunComparisonEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "overallState")]
    pub overall_state: SourceApplyComparisonState,
    #[serde(rename = "promotionClaimAllowed")]
    pub promotion_claim_allowed: bool,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplyRerunComparison {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply rerun comparison JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply rerun comparison schemaVersion must be {SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "source apply rerun comparison applyTransactionId",
            &self.apply_transaction_id,
        )?;
        for (field, value) in [
            ("reviewDecisionRef", &self.review_decision_ref),
            ("verificationLogRef", &self.verification_log_ref),
            ("rollbackSnapshotRef", &self.rollback_snapshot_ref),
            ("auditLedgerRef", &self.audit_ledger_ref),
            ("generatedOutputRoot", &self.generated_output_root),
        ] {
            require_local_ref(&format!("source apply rerun comparison {field}"), value)?;
        }
        for backlog in &self.qa_backlog_refs {
            require_local_ref("source apply rerun comparison qaBacklogRefs", backlog)?;
        }
        for id in [&self.before_run_id, &self.after_run_id]
            .into_iter()
            .flatten()
        {
            require_local_id("source apply rerun comparison runId", id)?;
        }
        if self.rerun_status == SourceApplyRerunStatus::Completed && self.dimensions.is_empty() {
            return Err(anyhow!(
                "completed source apply rerun comparison must record at least one dimension"
            ));
        }
        for dimension in &self.dimensions {
            dimension.validate()?;
        }
        require_nonempty(
            "source apply rerun comparison guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text("source apply rerun comparison guardrails", guardrail)?;
        }
        Ok(())
    }

    /// Classify the overall state and decide whether a promotion claim is
    /// permitted. A promotion claim requires a completed rerun with both run
    /// ids and an Improved/Unchanged outcome and no regression.
    pub fn evaluate(&self) -> SourceApplyRerunComparisonEvaluation {
        let mut blocked = Vec::new();

        let overall = match self.rerun_status {
            SourceApplyRerunStatus::Unsupported => SourceApplyComparisonState::Unsupported,
            SourceApplyRerunStatus::Missing => {
                if self.before_run_id.is_none() {
                    SourceApplyComparisonState::MissingBefore
                } else {
                    SourceApplyComparisonState::MissingAfter
                }
            }
            SourceApplyRerunStatus::Completed => self.classify_dimensions(),
        };

        if self.rerun_status == SourceApplyRerunStatus::Completed {
            if self.before_run_id.is_none() {
                blocked.push("completed rerun is missing the before run id".to_string());
            }
            if self.after_run_id.is_none() {
                blocked.push("completed rerun is missing the after run id".to_string());
            }
        }

        match overall {
            SourceApplyComparisonState::Regressed => {
                blocked.push("post-apply comparison shows a regression".to_string())
            }
            SourceApplyComparisonState::Inconclusive | SourceApplyComparisonState::Flaky => blocked
                .push(
                    "post-apply comparison is inconclusive/flaky; promotion needs more evidence"
                        .to_string(),
                ),
            SourceApplyComparisonState::Unsupported => {
                blocked.push("post-apply rerun is unsupported for this transaction".to_string())
            }
            SourceApplyComparisonState::MissingBefore
            | SourceApplyComparisonState::MissingAfter => {
                blocked.push("post-apply rerun evidence is missing".to_string())
            }
            SourceApplyComparisonState::Improved | SourceApplyComparisonState::Unchanged => {}
        }

        let promotion_claim_allowed = blocked.is_empty();

        let mut evidence_summary = vec![
            format!("transaction:{}", self.apply_transaction_id),
            format!("rerun:{}", rerun_status_label(&self.rerun_status)),
            format!("dimensions:{}", self.dimensions.len()),
            format!("overall:{}", comparison_state_label(&overall)),
        ];
        if promotion_claim_allowed {
            evidence_summary.push("post-apply evidence supports a promotion claim".to_string());
        }

        SourceApplyRerunComparisonEvaluation {
            schema_version: SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION.to_string(),
            apply_transaction_id: self.apply_transaction_id.clone(),
            overall_state: overall,
            promotion_claim_allowed,
            evidence_summary,
            blocked_reasons: blocked,
            allowed_actions: vec![
                "inspect_rerun_comparison".to_string(),
                "inspect_generated_runs".to_string(),
            ],
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "merge_branch".to_string(),
                "execute_command".to_string(),
                "claim_promotion_without_evidence".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize source apply rerun comparison evaluation JSON")
    }

    pub fn promotion_claim_allowed(&self) -> bool {
        self.evaluate().promotion_claim_allowed
    }

    fn classify_dimensions(&self) -> SourceApplyComparisonState {
        use SourceApplyComparisonState::*;
        if self.dimensions.iter().any(|d| d.state == Regressed) {
            return Regressed;
        }
        if self
            .dimensions
            .iter()
            .any(|d| matches!(d.state, MissingBefore))
        {
            return MissingBefore;
        }
        if self
            .dimensions
            .iter()
            .any(|d| matches!(d.state, MissingAfter))
        {
            return MissingAfter;
        }
        if self.dimensions.iter().any(|d| d.state == Flaky) {
            return Flaky;
        }
        if self
            .dimensions
            .iter()
            .any(|d| matches!(d.state, Inconclusive | Unsupported))
        {
            return Inconclusive;
        }
        if self.dimensions.iter().any(|d| d.state == Improved) {
            Improved
        } else {
            Unchanged
        }
    }
}

impl SourceApplyComparisonDimension {
    fn validate(&self) -> Result<()> {
        require_local_id("source apply rerun comparison dimension name", &self.name)?;
        require_text(
            "source apply rerun comparison dimension detail",
            &self.detail,
        )?;
        Ok(())
    }
}

fn rerun_status_label(status: &SourceApplyRerunStatus) -> &'static str {
    match status {
        SourceApplyRerunStatus::Completed => "completed",
        SourceApplyRerunStatus::Unsupported => "unsupported",
        SourceApplyRerunStatus::Missing => "missing",
    }
}

fn comparison_state_label(state: &SourceApplyComparisonState) -> &'static str {
    match state {
        SourceApplyComparisonState::Improved => "improved",
        SourceApplyComparisonState::Regressed => "regressed",
        SourceApplyComparisonState::Unchanged => "unchanged",
        SourceApplyComparisonState::Inconclusive => "inconclusive",
        SourceApplyComparisonState::MissingBefore => "missing-before",
        SourceApplyComparisonState::MissingAfter => "missing-after",
        SourceApplyComparisonState::Unsupported => "unsupported",
        SourceApplyComparisonState::Flaky => "flaky",
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
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
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
    Ok(())
}
