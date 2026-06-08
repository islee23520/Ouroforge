//! Source Apply Evidence Bundle v1 (#711, #1 Milestone 15).
//!
//! Bundles all source apply evidence references into one artifact for review,
//! audit, and governance handoff, and validates completeness: missing refs,
//! stale artifacts, inconsistent statuses, unresolved rollback gaps, missing
//! verification, and unresolved regressions are all surfaced. The bundle is a
//! read-only aggregation; it executes no commands and applies no patches.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SOURCE_APPLY_EVIDENCE_BUNDLE_SCHEMA_VERSION: &str = "source-apply-evidence-bundle-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyBundleStatus {
    Complete,
    Partial,
    Blocked,
    Failed,
}

/// A recorded component status, used to detect stale or inconsistent artifacts.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyBundleComponentStatus {
    pub component: String,
    /// e.g. `ok`, `stale`, `failed`, `missing`.
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyEvidenceBundle {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "previewRef")]
    pub preview_ref: String,
    #[serde(rename = "fileClassReportRef")]
    pub file_class_report_ref: String,
    #[serde(rename = "diffIntegrityReportRef")]
    pub diff_integrity_report_ref: String,
    #[serde(rename = "sandboxReportRef")]
    pub sandbox_report_ref: String,
    #[serde(rename = "reviewDecisionRef")]
    pub review_decision_ref: String,
    #[serde(rename = "applyTransactionRef")]
    pub apply_transaction_ref: String,
    #[serde(rename = "worktreeContextRef")]
    pub worktree_context_ref: String,
    #[serde(rename = "staleGuardRef")]
    pub stale_guard_ref: String,
    #[serde(rename = "sandboxPromotionRef")]
    pub sandbox_promotion_ref: String,
    #[serde(rename = "rollbackSnapshotRef")]
    pub rollback_snapshot_ref: String,
    #[serde(rename = "auditLedgerRef")]
    pub audit_ledger_ref: String,
    #[serde(rename = "verificationLogRefs")]
    pub verification_log_refs: Vec<String>,
    #[serde(
        rename = "rerunComparisonRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rerun_comparison_ref: Option<String>,
    #[serde(
        rename = "blockerEvidenceRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocker_evidence_ref: Option<String>,
    #[serde(rename = "rollbackResolved")]
    pub rollback_resolved: bool,
    #[serde(rename = "regressionResolved")]
    pub regression_resolved: bool,
    #[serde(rename = "componentStatuses")]
    pub component_statuses: Vec<SourceApplyBundleComponentStatus>,
    #[serde(rename = "declaredFinalStatus")]
    pub declared_final_status: SourceApplyBundleStatus,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyEvidenceBundleEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "computedStatus")]
    pub computed_status: SourceApplyBundleStatus,
    #[serde(rename = "statusConsistent")]
    pub status_consistent: bool,
    #[serde(rename = "issues")]
    pub issues: Vec<String>,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplyEvidenceBundle {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply evidence bundle JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_EVIDENCE_BUNDLE_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply evidence bundle schemaVersion must be {SOURCE_APPLY_EVIDENCE_BUNDLE_SCHEMA_VERSION}"
            ));
        }
        require_local_id("source apply evidence bundle bundleId", &self.bundle_id)?;
        require_local_id(
            "source apply evidence bundle applyTransactionId",
            &self.apply_transaction_id,
        )?;
        for reference in self.required_refs() {
            require_local_ref("source apply evidence bundle ref", reference)?;
        }
        for reference in &self.verification_log_refs {
            require_local_ref(
                "source apply evidence bundle verificationLogRefs",
                reference,
            )?;
        }
        for reference in [&self.rerun_comparison_ref, &self.blocker_evidence_ref]
            .into_iter()
            .flatten()
        {
            require_local_ref("source apply evidence bundle optional ref", reference)?;
        }
        for component in &self.component_statuses {
            component.validate()?;
        }
        require_nonempty(
            "source apply evidence bundle guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text("source apply evidence bundle guardrails", guardrail)?;
        }
        Ok(())
    }

    /// Aggregate-level evaluation. Surfaces missing/stale/inconsistent evidence
    /// and computes the bundle's effective status, flagging any mismatch with
    /// the declared status.
    pub fn evaluate(&self) -> SourceApplyEvidenceBundleEvaluation {
        let mut issues = Vec::new();

        if self.verification_log_refs.is_empty() {
            issues.push("missing verification evidence".to_string());
        }
        if !self.rollback_resolved {
            issues.push("unresolved rollback gap".to_string());
        }
        if !self.regression_resolved {
            issues.push("unresolved regression".to_string());
        }
        for component in &self.component_statuses {
            match component.status.to_ascii_lowercase().as_str() {
                "stale" => issues.push(format!("stale artifact: {}", component.component)),
                "missing" => issues.push(format!("missing artifact: {}", component.component)),
                "failed" => issues.push(format!("failed component: {}", component.component)),
                _ => {}
            }
        }

        let computed_status = if issues.is_empty() {
            SourceApplyBundleStatus::Complete
        } else if self
            .component_statuses
            .iter()
            .any(|c| c.status.eq_ignore_ascii_case("failed"))
        {
            SourceApplyBundleStatus::Failed
        } else if !self.rollback_resolved
            || !self.regression_resolved
            || self
                .component_statuses
                .iter()
                .any(|c| c.status.eq_ignore_ascii_case("stale"))
        {
            SourceApplyBundleStatus::Blocked
        } else {
            SourceApplyBundleStatus::Partial
        };

        let status_consistent = computed_status == self.declared_final_status;
        if !status_consistent {
            issues.push(format!(
                "inconsistent status: declared `{}` but evidence implies `{}`",
                status_label(&self.declared_final_status),
                status_label(&computed_status)
            ));
        }

        let mut evidence_summary = vec![
            format!("bundle:{}", self.bundle_id),
            format!("transaction:{}", self.apply_transaction_id),
            format!("verificationLogs:{}", self.verification_log_refs.len()),
            format!("components:{}", self.component_statuses.len()),
        ];
        if issues.is_empty() {
            evidence_summary
                .push("end-to-end source apply evidence is complete and consistent".to_string());
        }

        SourceApplyEvidenceBundleEvaluation {
            schema_version: SOURCE_APPLY_EVIDENCE_BUNDLE_SCHEMA_VERSION.to_string(),
            bundle_id: self.bundle_id.clone(),
            apply_transaction_id: self.apply_transaction_id.clone(),
            computed_status,
            status_consistent,
            issues,
            evidence_summary,
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "merge_branch".to_string(),
                "execute_command".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize source apply evidence bundle evaluation JSON")
    }

    pub fn is_complete(&self) -> bool {
        let evaluation = self.evaluate();
        evaluation.computed_status == SourceApplyBundleStatus::Complete
            && evaluation.status_consistent
    }

    fn required_refs(&self) -> [&String; 11] {
        [
            &self.preview_ref,
            &self.file_class_report_ref,
            &self.diff_integrity_report_ref,
            &self.sandbox_report_ref,
            &self.review_decision_ref,
            &self.apply_transaction_ref,
            &self.worktree_context_ref,
            &self.stale_guard_ref,
            &self.sandbox_promotion_ref,
            &self.rollback_snapshot_ref,
            &self.audit_ledger_ref,
        ]
    }
}

impl SourceApplyBundleComponentStatus {
    fn validate(&self) -> Result<()> {
        require_local_id("source apply evidence bundle component", &self.component)?;
        require_text(
            "source apply evidence bundle component status",
            &self.status,
        )?;
        Ok(())
    }
}

fn status_label(status: &SourceApplyBundleStatus) -> &'static str {
    match status {
        SourceApplyBundleStatus::Complete => "complete",
        SourceApplyBundleStatus::Partial => "partial",
        SourceApplyBundleStatus::Blocked => "blocked",
        SourceApplyBundleStatus::Failed => "failed",
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
