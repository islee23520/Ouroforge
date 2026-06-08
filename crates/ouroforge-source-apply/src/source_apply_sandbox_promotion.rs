//! Source Apply Sandbox-to-Trusted Promotion v1 (#705, #1 Milestone 15).
//!
//! Promotes only sandbox-validated patches to trusted apply readiness by
//! proving the sandbox dry-run applied the exact diff, ran only allowlisted
//! commands, left no forbidden command evidence, cleaned up, kept generated
//! state isolated, and produced sandbox target hashes that match the trusted
//! target expectations. It is a fail-closed readiness evaluation only: it does
//! not apply patches, run commands, or guarantee a secure sandbox.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION: &str = "source-apply-sandbox-promotion-v1";

/// Recorded outcome of the sandbox dry-run report.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplySandboxReportState {
    Passed,
    Failed,
    Missing,
}

/// Recorded sandbox cleanup state.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplySandboxCleanupState {
    Complete,
    Incomplete,
    Missing,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplySandboxPromotionStatus {
    Ready,
    Blocked,
}

/// One target the patch touches, with the trusted expectation and the sandbox
/// observation. Promotion requires `sandboxBefore == trustedBefore` and
/// `sandboxAfter == expectedAfter`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxTargetExpectation {
    pub path: String,
    #[serde(rename = "trustedBeforeHash")]
    pub trusted_before_hash: String,
    #[serde(rename = "expectedAfterHash")]
    pub expected_after_hash: String,
    #[serde(rename = "sandboxBeforeHash")]
    pub sandbox_before_hash: String,
    #[serde(rename = "sandboxAfterHash")]
    pub sandbox_after_hash: String,
}

/// Evidence that an allowlisted verification command ran in the sandbox.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxCommandEvidence {
    pub command: String,
    #[serde(rename = "allowlistPolicyId")]
    pub allowlist_policy_id: String,
    /// `passed` or any other recorded status.
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxPromotion {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "patchPreviewId")]
    pub patch_preview_id: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "expectedDiffHash")]
    pub expected_diff_hash: String,
    #[serde(rename = "sandboxDiffHash")]
    pub sandbox_diff_hash: String,
    #[serde(rename = "transactionBaseRevision")]
    pub transaction_base_revision: String,
    #[serde(rename = "sandboxBaseRevision")]
    pub sandbox_base_revision: String,
    #[serde(rename = "reportState")]
    pub report_state: SourceApplySandboxReportState,
    pub targets: Vec<SourceApplySandboxTargetExpectation>,
    #[serde(rename = "allowlistedCommands")]
    pub allowlisted_commands: Vec<SourceApplySandboxCommandEvidence>,
    #[serde(rename = "forbiddenCommandsObserved")]
    pub forbidden_commands_observed: Vec<String>,
    #[serde(rename = "cleanupState")]
    pub cleanup_state: SourceApplySandboxCleanupState,
    #[serde(rename = "generatedStateIsolated")]
    pub generated_state_isolated: bool,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxPromotionEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "patchPreviewId")]
    pub patch_preview_id: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    pub status: SourceApplySandboxPromotionStatus,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplySandboxPromotion {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply sandbox promotion JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply sandbox promotion schemaVersion must be {SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "source apply sandbox promotion patchPreviewId",
            &self.patch_preview_id,
        )?;
        require_local_id(
            "source apply sandbox promotion applyTransactionId",
            &self.apply_transaction_id,
        )?;
        require_text(
            "source apply sandbox promotion expectedDiffHash",
            &self.expected_diff_hash,
        )?;
        require_text(
            "source apply sandbox promotion transactionBaseRevision",
            &self.transaction_base_revision,
        )?;
        if self.targets.is_empty() {
            return Err(anyhow!(
                "source apply sandbox promotion targets must not be empty"
            ));
        }
        for target in &self.targets {
            target.validate()?;
        }
        for command in &self.allowlisted_commands {
            command.validate()?;
        }
        for forbidden in &self.forbidden_commands_observed {
            require_text(
                "source apply sandbox promotion forbiddenCommandsObserved",
                forbidden,
            )?;
        }
        require_nonempty(
            "source apply sandbox promotion guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text("source apply sandbox promotion guardrails", guardrail)?;
        }
        Ok(())
    }

    pub fn evaluate(&self) -> SourceApplySandboxPromotionEvaluation {
        let mut blocked = Vec::new();

        match self.report_state {
            SourceApplySandboxReportState::Missing => {
                blocked.push("sandbox dry-run report is missing".to_string())
            }
            SourceApplySandboxReportState::Failed => {
                blocked.push("sandbox dry-run report did not pass".to_string())
            }
            SourceApplySandboxReportState::Passed => {}
        }

        if self.sandbox_diff_hash != self.expected_diff_hash {
            blocked.push("sandbox applied a different diff than the apply transaction".to_string());
        }
        if self.sandbox_base_revision != self.transaction_base_revision {
            blocked.push(
                "sandbox report is stale: built against a different base revision".to_string(),
            );
        }

        if self.allowlisted_commands.is_empty() {
            blocked.push("no allowlisted verification command evidence was recorded".to_string());
        }
        for command in &self.allowlisted_commands {
            if command.status != "passed" {
                blocked.push(format!(
                    "allowlisted command `{}` did not pass in the sandbox",
                    command.command
                ));
            }
        }
        for forbidden in &self.forbidden_commands_observed {
            blocked.push(format!(
                "forbidden command observed in the sandbox: `{forbidden}`"
            ));
        }

        match self.cleanup_state {
            SourceApplySandboxCleanupState::Complete => {}
            SourceApplySandboxCleanupState::Incomplete => {
                blocked.push("sandbox cleanup is incomplete".to_string())
            }
            SourceApplySandboxCleanupState::Missing => {
                blocked.push("sandbox cleanup evidence is missing".to_string())
            }
        }
        if !self.generated_state_isolated {
            blocked.push("sandbox generated state was not isolated".to_string());
        }

        for target in &self.targets {
            if target.sandbox_before_hash != target.trusted_before_hash {
                blocked.push(format!(
                    "sandbox/trusted before-state mismatch for `{}`",
                    target.path
                ));
            }
            if target.sandbox_after_hash != target.expected_after_hash {
                blocked.push(format!(
                    "sandbox after-state does not match the expected trusted target for `{}`",
                    target.path
                ));
            }
        }

        let status = if blocked.is_empty() {
            SourceApplySandboxPromotionStatus::Ready
        } else {
            SourceApplySandboxPromotionStatus::Blocked
        };

        let mut evidence_summary = vec![
            format!("preview:{}", self.patch_preview_id),
            format!("transaction:{}", self.apply_transaction_id),
            format!("targets:{}", self.targets.len()),
            format!("commands:{}", self.allowlisted_commands.len()),
        ];
        if status == SourceApplySandboxPromotionStatus::Ready {
            evidence_summary
                .push("sandbox evidence matches trusted target expectations".to_string());
        }

        SourceApplySandboxPromotionEvaluation {
            schema_version: SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION.to_string(),
            patch_preview_id: self.patch_preview_id.clone(),
            apply_transaction_id: self.apply_transaction_id.clone(),
            status,
            evidence_summary,
            blocked_reasons: blocked,
            allowed_actions: vec![
                "inspect_sandbox_promotion".to_string(),
                "inspect_blocked_reasons".to_string(),
            ],
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "merge_branch".to_string(),
                "execute_command".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize source apply sandbox promotion evaluation JSON")
    }

    pub fn is_ready(&self) -> bool {
        self.evaluate().status == SourceApplySandboxPromotionStatus::Ready
    }
}

impl SourceApplySandboxTargetExpectation {
    fn validate(&self) -> Result<()> {
        require_local_ref("source apply sandbox promotion target path", &self.path)?;
        for (field, value) in [
            ("trustedBeforeHash", &self.trusted_before_hash),
            ("expectedAfterHash", &self.expected_after_hash),
            ("sandboxBeforeHash", &self.sandbox_before_hash),
            ("sandboxAfterHash", &self.sandbox_after_hash),
        ] {
            require_text(
                &format!("source apply sandbox promotion target {field}"),
                value,
            )?;
        }
        Ok(())
    }
}

impl SourceApplySandboxCommandEvidence {
    fn validate(&self) -> Result<()> {
        require_text("source apply sandbox promotion command", &self.command)?;
        require_local_id(
            "source apply sandbox promotion allowlistPolicyId",
            &self.allowlist_policy_id,
        )?;
        require_text(
            "source apply sandbox promotion command status",
            &self.status,
        )?;
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
        "secure sandbox guarantee",
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
