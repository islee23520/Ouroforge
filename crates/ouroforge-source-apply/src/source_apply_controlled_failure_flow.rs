//! Controlled failure to reviewed fix flow (#2379 / M126.2).
//!
//! This composes the existing review enforcement, sandbox promotion, and
//! before/after comparison contracts for a real sandbox file. It records and
//! evaluates evidence only; it never applies patches to the maintainer worktree,
//! executes commands, approves itself, or writes trusted state.

use crate::source_apply_post_apply_rerun::{
    SourceApplyComparisonState, SourceApplyRerunComparison,
};
use crate::source_apply_review_enforcement::{
    SourceApplyReviewEnforcement, SourceApplyReviewEnforcementStatus,
};
use crate::source_apply_sandbox_promotion::{
    SourceApplySandboxPromotion, SourceApplySandboxPromotionStatus,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const SOURCE_APPLY_CONTROLLED_FAILURE_FLOW_SCHEMA_VERSION: &str =
    "source-apply-controlled-failure-flow-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyControlledFailureFlow {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "flowId")]
    pub flow_id: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "sandboxFailureFile")]
    pub sandbox_failure_file: String,
    #[serde(rename = "beforeBundleRef")]
    pub before_bundle_ref: String,
    #[serde(rename = "afterBundleRef")]
    pub after_bundle_ref: String,
    #[serde(rename = "comparisonArtifactRef")]
    pub comparison_artifact_ref: String,
    #[serde(rename = "reviewArtifactRef")]
    pub review_artifact_ref: String,
    #[serde(rename = "sandboxArtifactRef")]
    pub sandbox_artifact_ref: String,
    #[serde(rename = "mainWorktreeStatusRef")]
    pub main_worktree_status_ref: String,
    #[serde(rename = "selfApprovalAttempted")]
    pub self_approval_attempted: bool,
    #[serde(rename = "selfApprovalBlocked")]
    pub self_approval_blocked: bool,
    pub review: SourceApplyReviewEnforcement,
    pub sandbox: SourceApplySandboxPromotion,
    pub comparison: SourceApplyRerunComparison,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyControlledFailureFlowStatus {
    ProductObservedReady,
    Regressed,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyControlledFailureFlowEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "flowId")]
    pub flow_id: String,
    pub status: SourceApplyControlledFailureFlowStatus,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplyControlledFailureFlow {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse controlled failure flow JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_CONTROLLED_FAILURE_FLOW_SCHEMA_VERSION {
            return Err(anyhow!(
                "controlled failure flow schemaVersion must be {SOURCE_APPLY_CONTROLLED_FAILURE_FLOW_SCHEMA_VERSION}"
            ));
        }
        require_local_id("controlled failure flow flowId", &self.flow_id)?;
        require_local_id(
            "controlled failure flow applyTransactionId",
            &self.apply_transaction_id,
        )?;
        if self.apply_transaction_id != self.review.apply_transaction_id
            || self.apply_transaction_id != self.sandbox.apply_transaction_id
            || self.apply_transaction_id != self.comparison.apply_transaction_id
        {
            return Err(anyhow!(
                "controlled failure flow applyTransactionId must match review, sandbox, and comparison artifacts"
            ));
        }
        require_local_path(
            "controlled failure flow sandboxFailureFile",
            &self.sandbox_failure_file,
        )?;
        for (field, value) in [
            ("beforeBundleRef", &self.before_bundle_ref),
            ("afterBundleRef", &self.after_bundle_ref),
            ("comparisonArtifactRef", &self.comparison_artifact_ref),
            ("reviewArtifactRef", &self.review_artifact_ref),
            ("sandboxArtifactRef", &self.sandbox_artifact_ref),
            ("mainWorktreeStatusRef", &self.main_worktree_status_ref),
        ] {
            require_local_ref(&format!("controlled failure flow {field}"), value)?;
        }
        self.review.validate()?;
        self.sandbox.validate()?;
        self.comparison.validate()?;
        if self.guardrails.is_empty() {
            return Err(anyhow!(
                "controlled failure flow guardrails must not be empty"
            ));
        }
        for guardrail in &self.guardrails {
            require_text("controlled failure flow guardrails", guardrail)?;
        }
        Ok(())
    }

    pub fn evaluate(
        &self,
        sandbox_root: impl AsRef<Path>,
    ) -> SourceApplyControlledFailureFlowEvaluation {
        let mut blocked = Vec::new();
        let sandbox_file = sandbox_root.as_ref().join(&self.sandbox_failure_file);
        if !sandbox_file.is_file() {
            blocked.push(format!(
                "controlled failure was not reproduced from a real sandbox file: {}",
                self.sandbox_failure_file
            ));
        }
        if !self.self_approval_attempted {
            blocked.push("self-approval rejection path was not exercised".to_string());
        }
        if !self.self_approval_blocked {
            blocked.push("self-approval was not blocked".to_string());
        }

        let review_eval = self.review.evaluate();
        if review_eval.status != SourceApplyReviewEnforcementStatus::Ready {
            blocked.extend(
                review_eval
                    .blocked_reasons
                    .into_iter()
                    .map(|reason| format!("review gate blocked: {reason}")),
            );
        }
        let sandbox_eval = self.sandbox.evaluate();
        if sandbox_eval.status != SourceApplySandboxPromotionStatus::Ready {
            blocked.extend(
                sandbox_eval
                    .blocked_reasons
                    .into_iter()
                    .map(|reason| format!("sandbox gate blocked: {reason}")),
            );
        }
        let comparison_eval = self.comparison.evaluate();
        if !comparison_eval.promotion_claim_allowed {
            blocked.extend(
                comparison_eval
                    .blocked_reasons
                    .iter()
                    .map(|reason| format!("comparison blocked: {reason}")),
            );
        }

        let status = if !blocked.is_empty() {
            SourceApplyControlledFailureFlowStatus::Blocked
        } else if comparison_eval.overall_state == SourceApplyComparisonState::Regressed {
            SourceApplyControlledFailureFlowStatus::Regressed
        } else {
            SourceApplyControlledFailureFlowStatus::ProductObservedReady
        };

        SourceApplyControlledFailureFlowEvaluation {
            schema_version: SOURCE_APPLY_CONTROLLED_FAILURE_FLOW_SCHEMA_VERSION.to_string(),
            flow_id: self.flow_id.clone(),
            status,
            evidence_summary: vec![
                format!("beforeBundle:{}", self.before_bundle_ref),
                format!("afterBundle:{}", self.after_bundle_ref),
                format!("comparison:{}", self.comparison_artifact_ref),
                format!("sandboxFile:{}", self.sandbox_failure_file),
                format!("mainWorktreeStatus:{}", self.main_worktree_status_ref),
            ],
            blocked_reasons: blocked,
            forbidden_actions: vec![
                "trusted_worktree_apply".to_string(),
                "self_approve".to_string(),
                "auto_merge".to_string(),
                "execute_untrusted_command".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self, sandbox_root: impl AsRef<Path>) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate(sandbox_root))
            .context("failed to serialize controlled failure flow evaluation JSON")
    }
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.contains('/') || value.contains('\\') || value.contains("..") {
        return Err(anyhow!("{field} must be a local id"));
    }
    Ok(())
}

fn require_local_path(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let path = Path::new(value);
    if path.is_absolute() || value.contains("..") {
        return Err(anyhow!("{field} must stay inside the sandbox root"));
    }
    Ok(())
}

fn require_local_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains("\\") {
        return Err(anyhow!("{field} must be a local evidence ref"));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}
