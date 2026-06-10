//! Sandbox Worktree Source Apply Protocol v1 (#2378, #1 M126).
//!
//! This module defines the data-only contract for applying an already reviewed
//! Safe Source Apply v1 transaction inside an isolated temporary worktree. It is
//! intentionally not a second authority path: the protocol references existing
//! Safe Source Apply preview/review/transaction/rollback evidence, records the
//! sandbox worktree context, and evaluates whether the sandbox-only apply may be
//! considered valid evidence. It never mutates the trusted/main worktree, runs
//! commands, merges, self-approves, or bypasses review gates.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const SOURCE_APPLY_SANDBOX_WORKTREE_PROTOCOL_SCHEMA_VERSION: &str =
    "source-apply-sandbox-worktree-protocol-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplySandboxFileClass {
    RustSource,
    RustTest,
    SpecDocument,
    Fixture,
    ExampleData,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplySandboxApplyState {
    Applied,
    Blocked,
    Failed,
    NotAttempted,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplySandboxCleanupState {
    Complete,
    Incomplete,
    NotRequired,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplySandboxProtocolStatus {
    Valid,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxWorktreeContext {
    #[serde(rename = "sandboxWorktreePath")]
    pub sandbox_worktree_path: String,
    #[serde(rename = "trustedWorktreePath")]
    pub trusted_worktree_path: String,
    #[serde(rename = "baseRevision")]
    pub base_revision: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "evidenceRoot")]
    pub evidence_root: String,
    #[serde(rename = "cargoTargetDir")]
    pub cargo_target_dir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxStatusSnapshot {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
    #[serde(rename = "worktreePath")]
    pub worktree_path: String,
    #[serde(rename = "gitStatusShort")]
    pub git_status_short: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxTarget {
    pub path: String,
    #[serde(rename = "fileClass")]
    pub file_class: SourceApplySandboxFileClass,
    #[serde(rename = "beforeHash")]
    pub before_hash: String,
    #[serde(rename = "expectedAfterHash")]
    pub expected_after_hash: String,
    #[serde(rename = "observedAfterHash")]
    pub observed_after_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxWorktreeProtocol {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "protocolId")]
    pub protocol_id: String,
    #[serde(rename = "patchPreviewId")]
    pub patch_preview_id: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "reviewDecisionRef")]
    pub review_decision_ref: String,
    #[serde(rename = "rollbackSnapshotRef")]
    pub rollback_snapshot_ref: String,
    #[serde(rename = "safeSourceApplyVersion")]
    pub safe_source_apply_version: String,
    pub context: SourceApplySandboxWorktreeContext,
    #[serde(rename = "mainStatusBefore")]
    pub main_status_before: SourceApplySandboxStatusSnapshot,
    #[serde(rename = "mainStatusAfter")]
    pub main_status_after: SourceApplySandboxStatusSnapshot,
    pub targets: Vec<SourceApplySandboxTarget>,
    #[serde(rename = "applyState")]
    pub apply_state: SourceApplySandboxApplyState,
    #[serde(rename = "cleanupState")]
    pub cleanup_state: SourceApplySandboxCleanupState,
    #[serde(rename = "auditLedgerRef")]
    pub audit_ledger_ref: String,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxWorktreeProtocolEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "protocolId")]
    pub protocol_id: String,
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    pub status: SourceApplySandboxProtocolStatus,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "allowedFileClasses")]
    pub allowed_file_classes: Vec<SourceApplySandboxFileClass>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplySandboxWorktreeProtocol {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply sandbox worktree protocol JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_SANDBOX_WORKTREE_PROTOCOL_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply sandbox worktree protocol schemaVersion must be {SOURCE_APPLY_SANDBOX_WORKTREE_PROTOCOL_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "source apply sandbox worktree protocol protocolId",
            &self.protocol_id,
        )?;
        require_local_id(
            "source apply sandbox worktree protocol patchPreviewId",
            &self.patch_preview_id,
        )?;
        require_local_id(
            "source apply sandbox worktree protocol applyTransactionId",
            &self.apply_transaction_id,
        )?;
        require_local_ref(
            "source apply sandbox worktree protocol reviewDecisionRef",
            &self.review_decision_ref,
        )?;
        require_local_ref(
            "source apply sandbox worktree protocol rollbackSnapshotRef",
            &self.rollback_snapshot_ref,
        )?;
        if self.safe_source_apply_version != "safe-source-mutation-apply-v1" {
            return Err(anyhow!(
                "source apply sandbox worktree protocol must reuse Safe Source Apply v1"
            ));
        }
        self.context.validate()?;
        self.main_status_before.validate()?;
        self.main_status_after.validate()?;
        if self.targets.is_empty() {
            return Err(anyhow!(
                "source apply sandbox worktree protocol targets must not be empty"
            ));
        }
        let mut seen = BTreeSet::new();
        for target in &self.targets {
            target.validate()?;
            if !seen.insert(target.path.clone()) {
                return Err(anyhow!(
                    "source apply sandbox worktree protocol has duplicate target `{}`",
                    target.path
                ));
            }
        }
        require_local_ref(
            "source apply sandbox worktree protocol auditLedgerRef",
            &self.audit_ledger_ref,
        )?;
        require_nonempty(
            "source apply sandbox worktree protocol guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text(
                "source apply sandbox worktree protocol guardrails",
                guardrail,
            )?;
        }
        Ok(())
    }

    pub fn evaluate(&self) -> SourceApplySandboxWorktreeProtocolEvaluation {
        let mut blocked = Vec::new();
        if self.context.sandbox_worktree_path == self.context.trusted_worktree_path {
            blocked
                .push("sandbox worktree must be isolated from trusted/main worktree".to_string());
        }
        if !self.context.evidence_root.starts_with("target/")
            && !self
                .context
                .evidence_root
                .starts_with(".ouroforge/generated/")
        {
            blocked.push("evidence root must stay in an ignored/generated root".to_string());
        }
        if self.context.cargo_target_dir.trim().is_empty()
            || self.context.cargo_target_dir == "target"
        {
            blocked.push("sandbox protocol requires a dedicated CARGO_TARGET_DIR".to_string());
        }
        if self.apply_state != SourceApplySandboxApplyState::Applied {
            blocked.push(format!(
                "sandbox apply state is not applied: {:?}",
                self.apply_state
            ));
        }
        if self.cleanup_state == SourceApplySandboxCleanupState::Incomplete {
            blocked.push("sandbox cleanup is incomplete".to_string());
        }
        if self.main_status_before.git_status_short != self.main_status_after.git_status_short {
            blocked.push(
                "trusted/main worktree status changed between before/after snapshots".to_string(),
            );
        }
        for target in &self.targets {
            if !target.file_class.is_allowed() {
                blocked.push(format!("target `{}` has a blocked file class", target.path));
            }
            if target.observed_after_hash != target.expected_after_hash {
                blocked.push(format!(
                    "target `{}` observed hash does not match expected after hash",
                    target.path
                ));
            }
        }
        let status = if blocked.is_empty() {
            SourceApplySandboxProtocolStatus::Valid
        } else {
            SourceApplySandboxProtocolStatus::Blocked
        };
        SourceApplySandboxWorktreeProtocolEvaluation {
            schema_version: SOURCE_APPLY_SANDBOX_WORKTREE_PROTOCOL_SCHEMA_VERSION.to_string(),
            protocol_id: self.protocol_id.clone(),
            apply_transaction_id: self.apply_transaction_id.clone(),
            status,
            evidence_summary: vec![
                format!("safe-source-apply:{}", self.safe_source_apply_version),
                format!("sandbox:{}", self.context.sandbox_worktree_path),
                format!("main-status-before:{}", self.main_status_before.snapshot_id),
                format!("main-status-after:{}", self.main_status_after.snapshot_id),
                format!("targets:{}", self.targets.len()),
                format!("audit:{}", self.audit_ledger_ref),
            ],
            blocked_reasons: blocked,
            allowed_file_classes: SourceApplySandboxFileClass::allowed_file_classes(),
            forbidden_actions: vec![
                "mutate_trusted_main_worktree".to_string(),
                "create_second_apply_authority".to_string(),
                "browser_trusted_write".to_string(),
                "execute_unreviewed_script".to_string(),
                "self_approve".to_string(),
                "auto_merge".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize source apply sandbox worktree protocol evaluation JSON")
    }
}

impl SourceApplySandboxFileClass {
    pub fn is_allowed(self) -> bool {
        matches!(
            self,
            SourceApplySandboxFileClass::RustSource
                | SourceApplySandboxFileClass::RustTest
                | SourceApplySandboxFileClass::SpecDocument
                | SourceApplySandboxFileClass::Fixture
                | SourceApplySandboxFileClass::ExampleData
        )
    }

    pub fn allowed_file_classes() -> Vec<Self> {
        vec![
            Self::RustSource,
            Self::RustTest,
            Self::SpecDocument,
            Self::Fixture,
            Self::ExampleData,
        ]
    }
}

impl SourceApplySandboxWorktreeContext {
    fn validate(&self) -> Result<()> {
        require_text(
            "source apply sandbox worktree protocol sandboxWorktreePath",
            &self.sandbox_worktree_path,
        )?;
        require_text(
            "source apply sandbox worktree protocol trustedWorktreePath",
            &self.trusted_worktree_path,
        )?;
        require_text(
            "source apply sandbox worktree protocol baseRevision",
            &self.base_revision,
        )?;
        require_text(
            "source apply sandbox worktree protocol createdAt",
            &self.created_at,
        )?;
        require_local_ref(
            "source apply sandbox worktree protocol evidenceRoot",
            &self.evidence_root,
        )?;
        require_text(
            "source apply sandbox worktree protocol cargoTargetDir",
            &self.cargo_target_dir,
        )?;
        Ok(())
    }
}

impl SourceApplySandboxStatusSnapshot {
    fn validate(&self) -> Result<()> {
        require_local_id(
            "source apply sandbox worktree protocol snapshotId",
            &self.snapshot_id,
        )?;
        require_text(
            "source apply sandbox worktree protocol worktreePath",
            &self.worktree_path,
        )?;
        Ok(())
    }
}

impl SourceApplySandboxTarget {
    fn validate(&self) -> Result<()> {
        require_local_ref(
            "source apply sandbox worktree protocol target path",
            &self.path,
        )?;
        for (field, value) in [
            ("beforeHash", &self.before_hash),
            ("expectedAfterHash", &self.expected_after_hash),
            ("observedAfterHash", &self.observed_after_hash),
        ] {
            require_text(
                &format!("source apply sandbox worktree protocol target {field}"),
                value,
            )?;
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
        return Err(anyhow!("{field} must stay inside the local worktree"));
    }
    if value.starts_with(".git/") || value.contains("/.git/") {
        return Err(anyhow!("{field} must not reference git internals"));
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
        "command bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "self-approval",
        "secure sandbox guarantee",
        "production-ready",
        "godot replacement",
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
