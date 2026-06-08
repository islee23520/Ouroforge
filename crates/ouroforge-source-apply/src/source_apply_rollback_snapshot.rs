//! Source Apply Rollback Snapshot and Recovery v1 (#706, #1 Milestone 15).
//!
//! Makes every trusted source apply rollbackable by recording before-state
//! metadata and recovery guidance, and validating that the snapshot is complete
//! before apply readiness. This module records and validates metadata only; any
//! restore behavior remains out of scope here, so it implies no untested
//! automatic restore. It never applies patches or executes commands.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SOURCE_APPLY_ROLLBACK_SNAPSHOT_SCHEMA_VERSION: &str = "source-apply-rollback-snapshot-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyRollbackStatus {
    /// Rollback metadata is complete and links to the transaction.
    Complete,
    /// At least one completeness gap blocks apply readiness.
    Blocked,
}

/// One target the apply transaction will mutate, with the metadata needed to
/// reconstruct the before-state. At least one of `beforeContentRef` or
/// `reversePatchRef` must be present.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyRollbackTarget {
    pub path: String,
    #[serde(rename = "beforeContentHash")]
    pub before_content_hash: String,
    #[serde(rename = "expectedAfterHash")]
    pub expected_after_hash: String,
    #[serde(
        rename = "beforeContentRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub before_content_ref: Option<String>,
    #[serde(
        rename = "reversePatchRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reverse_patch_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyRollbackSnapshot {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "transactionBaseRevision")]
    pub transaction_base_revision: String,
    #[serde(rename = "snapshotBaseRevision")]
    pub snapshot_base_revision: String,
    pub actor: String,
    /// Recorded RFC3339-like timestamp string (data, not generated here).
    #[serde(rename = "recordedAt")]
    pub recorded_at: String,
    pub targets: Vec<SourceApplyRollbackTarget>,
    /// Copyable-only recovery guidance; never executed by this module.
    #[serde(rename = "recoveryGuidance")]
    pub recovery_guidance: Vec<String>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyRollbackEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    pub status: SourceApplyRollbackStatus,
    #[serde(rename = "targetCount")]
    pub target_count: usize,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "recoveryGaps")]
    pub recovery_gaps: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplyRollbackSnapshot {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply rollback snapshot JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_ROLLBACK_SNAPSHOT_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply rollback snapshot schemaVersion must be {SOURCE_APPLY_ROLLBACK_SNAPSHOT_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "source apply rollback snapshot applyTransactionId",
            &self.apply_transaction_id,
        )?;
        require_text(
            "source apply rollback snapshot transactionBaseRevision",
            &self.transaction_base_revision,
        )?;
        require_text(
            "source apply rollback snapshot snapshotBaseRevision",
            &self.snapshot_base_revision,
        )?;
        require_text("source apply rollback snapshot actor", &self.actor)?;
        require_text(
            "source apply rollback snapshot recordedAt",
            &self.recorded_at,
        )?;
        if self.targets.is_empty() {
            return Err(anyhow!(
                "source apply rollback snapshot targets must not be empty"
            ));
        }
        for target in &self.targets {
            target.validate()?;
        }
        for guidance in &self.recovery_guidance {
            require_text("source apply rollback snapshot recoveryGuidance", guidance)?;
        }
        require_nonempty(
            "source apply rollback snapshot guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text("source apply rollback snapshot guardrails", guardrail)?;
        }
        Ok(())
    }

    /// Fail-closed completeness evaluation. Any missing reverse data, hash, or
    /// recovery guidance, or a stale snapshot, blocks apply readiness.
    pub fn evaluate(&self) -> SourceApplyRollbackEvaluation {
        let mut gaps = Vec::new();

        if self.targets.is_empty() {
            gaps.push(
                "rollback snapshot records no targets, so it cannot roll back the apply transaction"
                    .to_string(),
            );
        }

        if self.snapshot_base_revision != self.transaction_base_revision {
            gaps.push(
                "rollback snapshot is stale: recorded against a different base revision"
                    .to_string(),
            );
        }

        for target in &self.targets {
            if target.before_content_hash.trim().is_empty() {
                gaps.push(format!(
                    "rollback target `{}` is missing a before content hash",
                    target.path
                ));
            }
            if target.expected_after_hash.trim().is_empty() {
                gaps.push(format!(
                    "rollback target `{}` is missing an expected after hash",
                    target.path
                ));
            }
            let has_before_ref = target
                .before_content_ref
                .as_deref()
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false);
            let has_reverse_patch = target
                .reverse_patch_ref
                .as_deref()
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false);
            if !has_before_ref && !has_reverse_patch {
                gaps.push(format!(
                    "rollback target `{}` is missing reverse data (before content ref or reverse patch)",
                    target.path
                ));
            }
        }

        if self.recovery_guidance.is_empty() {
            gaps.push("rollback snapshot is missing recovery guidance (recovery gap)".to_string());
        }

        let status = if gaps.is_empty() {
            SourceApplyRollbackStatus::Complete
        } else {
            SourceApplyRollbackStatus::Blocked
        };

        let mut evidence_summary = vec![
            format!("transaction:{}", self.apply_transaction_id),
            format!("targets:{}", self.targets.len()),
            format!("actor:{}", self.actor),
            format!("recoveryGuidance:{}", self.recovery_guidance.len()),
        ];
        if status == SourceApplyRollbackStatus::Complete {
            evidence_summary
                .push("rollback metadata is complete and transaction-linked".to_string());
        }

        SourceApplyRollbackEvaluation {
            schema_version: SOURCE_APPLY_ROLLBACK_SNAPSHOT_SCHEMA_VERSION.to_string(),
            apply_transaction_id: self.apply_transaction_id.clone(),
            status,
            target_count: self.targets.len(),
            evidence_summary,
            recovery_gaps: gaps,
            allowed_actions: vec![
                "inspect_rollback_snapshot".to_string(),
                "copy_recovery_guidance".to_string(),
            ],
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "merge_branch".to_string(),
                "execute_command".to_string(),
                "auto_restore".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize source apply rollback evaluation JSON")
    }

    pub fn is_complete(&self) -> bool {
        self.evaluate().status == SourceApplyRollbackStatus::Complete
    }
}

impl SourceApplyRollbackTarget {
    fn validate(&self) -> Result<()> {
        require_local_ref("source apply rollback snapshot target path", &self.path)?;
        require_text(
            "source apply rollback snapshot target beforeContentHash",
            &self.before_content_hash,
        )?;
        require_text(
            "source apply rollback snapshot target expectedAfterHash",
            &self.expected_after_hash,
        )?;
        for reference in [&self.before_content_ref, &self.reverse_patch_ref]
            .into_iter()
            .flatten()
        {
            require_local_ref("source apply rollback snapshot target ref", reference)?;
        }
        Ok(())
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
