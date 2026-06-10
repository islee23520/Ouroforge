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
        if self.apply_state == SourceApplySandboxApplyState::Applied
            && self.cleanup_state != SourceApplySandboxCleanupState::Complete
        {
            blocked.push(
                "applied sandbox mutations must record complete cleanup metadata".to_string(),
            );
        }
        if self.main_status_before.worktree_path != self.context.trusted_worktree_path
            || self.main_status_after.worktree_path != self.context.trusted_worktree_path
        {
            blocked.push(
                "trusted/main worktree status snapshots must reference the trusted worktree path"
                    .to_string(),
            );
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxMutation {
    pub path: String,
    #[serde(rename = "fileClass")]
    pub file_class: SourceApplySandboxFileClass,
    #[serde(rename = "expectedBeforeHash")]
    pub expected_before_hash: String,
    #[serde(rename = "expectedAfterHash")]
    pub expected_after_hash: String,
    #[serde(rename = "replacementUtf8")]
    pub replacement_utf8: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplySandboxApplyRequest {
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
    pub context: SourceApplySandboxWorktreeContext,
    #[serde(rename = "mainStatusBefore")]
    pub main_status_before: SourceApplySandboxStatusSnapshot,
    #[serde(rename = "mainStatusAfter")]
    pub main_status_after: SourceApplySandboxStatusSnapshot,
    pub mutations: Vec<SourceApplySandboxMutation>,
    #[serde(rename = "cleanupState")]
    pub cleanup_state: SourceApplySandboxCleanupState,
    #[serde(rename = "auditLedgerRef")]
    pub audit_ledger_ref: String,
    pub guardrails: Vec<String>,
}

pub fn fnv1a64_utf8_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

pub fn apply_sandbox_utf8_mutations(
    request: SourceApplySandboxApplyRequest,
) -> Result<SourceApplySandboxWorktreeProtocol> {
    request.validate()?;
    let sandbox_root = std::path::Path::new(&request.context.sandbox_worktree_path);
    let mut targets = Vec::new();
    for mutation in &request.mutations {
        mutation.validate()?;
        let target_path = sandbox_root.join(&mutation.path);
        let before = std::fs::read_to_string(&target_path).with_context(|| {
            format!(
                "failed to read sandbox target `{}` before apply",
                mutation.path
            )
        })?;
        let before_hash = fnv1a64_utf8_digest(&before);
        if before_hash != mutation.expected_before_hash {
            return Err(anyhow!(
                "sandbox target `{}` before hash is stale: expected {}, observed {}",
                mutation.path,
                mutation.expected_before_hash,
                before_hash
            ));
        }
        let after_hash = fnv1a64_utf8_digest(&mutation.replacement_utf8);
        if after_hash != mutation.expected_after_hash {
            return Err(anyhow!(
                "sandbox target `{}` replacement does not match expected after hash",
                mutation.path
            ));
        }
        std::fs::write(&target_path, &mutation.replacement_utf8)
            .with_context(|| format!("failed to write sandbox target `{}`", mutation.path))?;
        targets.push(SourceApplySandboxTarget {
            path: mutation.path.clone(),
            file_class: mutation.file_class,
            before_hash,
            expected_after_hash: mutation.expected_after_hash.clone(),
            observed_after_hash: after_hash,
        });
    }
    Ok(SourceApplySandboxWorktreeProtocol {
        schema_version: SOURCE_APPLY_SANDBOX_WORKTREE_PROTOCOL_SCHEMA_VERSION.to_string(),
        protocol_id: request.protocol_id,
        patch_preview_id: request.patch_preview_id,
        apply_transaction_id: request.apply_transaction_id,
        review_decision_ref: request.review_decision_ref,
        rollback_snapshot_ref: request.rollback_snapshot_ref,
        safe_source_apply_version: "safe-source-mutation-apply-v1".to_string(),
        context: request.context,
        main_status_before: request.main_status_before,
        main_status_after: request.main_status_after,
        targets,
        apply_state: SourceApplySandboxApplyState::Applied,
        cleanup_state: request.cleanup_state,
        audit_ledger_ref: request.audit_ledger_ref,
        guardrails: request.guardrails,
    })
}

impl SourceApplySandboxApplyRequest {
    fn validate(&self) -> Result<()> {
        require_local_id(
            "source apply sandbox apply request protocolId",
            &self.protocol_id,
        )?;
        require_local_id(
            "source apply sandbox apply request patchPreviewId",
            &self.patch_preview_id,
        )?;
        require_local_id(
            "source apply sandbox apply request applyTransactionId",
            &self.apply_transaction_id,
        )?;
        self.context.validate()?;
        self.main_status_before.validate()?;
        self.main_status_after.validate()?;
        if self.mutations.is_empty() {
            return Err(anyhow!(
                "source apply sandbox apply request mutations must not be empty"
            ));
        }
        if self.context.sandbox_worktree_path == self.context.trusted_worktree_path {
            return Err(anyhow!(
                "source apply sandbox apply request cannot target the trusted/main worktree"
            ));
        }
        require_local_ref(
            "source apply sandbox apply request reviewDecisionRef",
            &self.review_decision_ref,
        )?;
        require_local_ref(
            "source apply sandbox apply request rollbackSnapshotRef",
            &self.rollback_snapshot_ref,
        )?;
        require_local_ref(
            "source apply sandbox apply request auditLedgerRef",
            &self.audit_ledger_ref,
        )?;
        require_nonempty(
            "source apply sandbox apply request guardrails",
            self.guardrails.len(),
        )?;
        Ok(())
    }
}

impl SourceApplySandboxMutation {
    fn validate(&self) -> Result<()> {
        require_local_ref("source apply sandbox mutation path", &self.path)?;
        if !self.file_class.is_allowed() {
            return Err(anyhow!(
                "source apply sandbox mutation file class is blocked"
            ));
        }
        require_text(
            "source apply sandbox mutation expectedBeforeHash",
            &self.expected_before_hash,
        )?;
        require_text(
            "source apply sandbox mutation expectedAfterHash",
            &self.expected_after_hash,
        )?;
        require_text(
            "source apply sandbox mutation replacementUtf8",
            &self.replacement_utf8,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_apply_mutates_only_isolated_worktree_and_evaluates_valid() {
        let unique = format!(
            "ouroforge-sandbox-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let root = std::env::temp_dir().join(unique);
        let main = root.join("main");
        let sandbox = root.join("sandbox");
        std::fs::create_dir_all(main.join("docs")).unwrap();
        std::fs::create_dir_all(sandbox.join("docs")).unwrap();
        std::fs::write(main.join("docs/spec.md"), "before\n").unwrap();
        std::fs::write(sandbox.join("docs/spec.md"), "before\n").unwrap();

        let before_hash = fnv1a64_utf8_digest("before\n");
        let after_hash = fnv1a64_utf8_digest("after\n");
        let request = SourceApplySandboxApplyRequest {
            protocol_id: "sandbox-protocol-smoke".to_string(),
            patch_preview_id: "preview-smoke".to_string(),
            apply_transaction_id: "tx-smoke".to_string(),
            review_decision_ref: "evidence/review.json".to_string(),
            rollback_snapshot_ref: "evidence/rollback.json".to_string(),
            context: SourceApplySandboxWorktreeContext {
                sandbox_worktree_path: sandbox.display().to_string(),
                trusted_worktree_path: main.display().to_string(),
                base_revision: "abc123".to_string(),
                created_at: "2026-06-10T00:00:00Z".to_string(),
                evidence_root: "target/sandbox-smoke".to_string(),
                cargo_target_dir: "/tmp/ouroforge-sandbox-target-smoke".to_string(),
            },
            main_status_before: SourceApplySandboxStatusSnapshot {
                snapshot_id: "main-before".to_string(),
                worktree_path: main.display().to_string(),
                git_status_short: " M README.md".to_string(),
            },
            main_status_after: SourceApplySandboxStatusSnapshot {
                snapshot_id: "main-after".to_string(),
                worktree_path: main.display().to_string(),
                git_status_short: " M README.md".to_string(),
            },
            mutations: vec![SourceApplySandboxMutation {
                path: "docs/spec.md".to_string(),
                file_class: SourceApplySandboxFileClass::SpecDocument,
                expected_before_hash: before_hash,
                expected_after_hash: after_hash,
                replacement_utf8: "after\n".to_string(),
            }],
            cleanup_state: SourceApplySandboxCleanupState::Complete,
            audit_ledger_ref: "evidence/audit.json".to_string(),
            guardrails: vec!["reuse Safe Source Apply v1".to_string()],
        };

        let artifact = apply_sandbox_utf8_mutations(request).unwrap();
        assert_eq!(
            std::fs::read_to_string(main.join("docs/spec.md")).unwrap(),
            "before\n",
            "trusted/main worktree fixture remains unchanged"
        );
        assert_eq!(
            std::fs::read_to_string(sandbox.join("docs/spec.md")).unwrap(),
            "after\n",
            "sandbox worktree fixture receives the reviewed mutation"
        );
        assert_eq!(
            artifact.evaluate().status,
            SourceApplySandboxProtocolStatus::Valid
        );
        let _ = std::fs::remove_dir_all(root);
    }
    #[test]
    fn unsafe_path_is_blocked_before_sandbox_write() {
        let request = smoke_request("before\n", "after\n", "../trusted.md");
        let err = apply_sandbox_utf8_mutations(request)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("must stay inside the local worktree"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn stale_target_hash_is_blocked() {
        let mut request = smoke_request("before\n", "after\n", "docs/spec.md");
        request.mutations[0].expected_before_hash = fnv1a64_utf8_digest("older\n");
        let err = apply_sandbox_utf8_mutations(request)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("before hash is stale"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn evaluation_blocks_changed_main_status_and_incomplete_cleanup() {
        let mut request = smoke_request("before\n", "after\n", "docs/spec.md");
        request.main_status_after.git_status_short = " M docs/changed.md".to_string();
        request.cleanup_state = SourceApplySandboxCleanupState::Incomplete;
        let artifact = apply_sandbox_utf8_mutations(request).unwrap();
        let evaluation = artifact.evaluate();
        assert_eq!(evaluation.status, SourceApplySandboxProtocolStatus::Blocked);
        assert!(evaluation
            .blocked_reasons
            .iter()
            .any(|reason| reason.contains("trusted/main worktree status changed")));
        assert!(evaluation
            .blocked_reasons
            .iter()
            .any(|reason| reason.contains("complete cleanup metadata")));
        assert!(evaluation
            .evidence_summary
            .iter()
            .any(|line| line.contains("audit:")));
        assert!(artifact.rollback_snapshot_ref.ends_with("rollback.json"));
    }

    fn smoke_request(
        before: &str,
        after: &str,
        mutation_path: &str,
    ) -> SourceApplySandboxApplyRequest {
        let unique = format!(
            "ouroforge-sandbox-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let root = std::env::temp_dir().join(unique);
        let main = root.join("main");
        let sandbox = root.join("sandbox");
        std::fs::create_dir_all(main.join("docs")).unwrap();
        std::fs::create_dir_all(sandbox.join("docs")).unwrap();
        std::fs::write(main.join("docs/spec.md"), before).unwrap();
        std::fs::write(sandbox.join("docs/spec.md"), before).unwrap();

        SourceApplySandboxApplyRequest {
            protocol_id: "sandbox-protocol-smoke".to_string(),
            patch_preview_id: "preview-smoke".to_string(),
            apply_transaction_id: "tx-smoke".to_string(),
            review_decision_ref: "evidence/review.json".to_string(),
            rollback_snapshot_ref: "evidence/rollback.json".to_string(),
            context: SourceApplySandboxWorktreeContext {
                sandbox_worktree_path: sandbox.display().to_string(),
                trusted_worktree_path: main.display().to_string(),
                base_revision: "abc123".to_string(),
                created_at: "2026-06-10T00:00:00Z".to_string(),
                evidence_root: "target/sandbox-smoke".to_string(),
                cargo_target_dir: "/tmp/ouroforge-sandbox-target-smoke".to_string(),
            },
            main_status_before: SourceApplySandboxStatusSnapshot {
                snapshot_id: "main-before".to_string(),
                worktree_path: main.display().to_string(),
                git_status_short: " M README.md".to_string(),
            },
            main_status_after: SourceApplySandboxStatusSnapshot {
                snapshot_id: "main-after".to_string(),
                worktree_path: main.display().to_string(),
                git_status_short: " M README.md".to_string(),
            },
            mutations: vec![SourceApplySandboxMutation {
                path: mutation_path.to_string(),
                file_class: SourceApplySandboxFileClass::SpecDocument,
                expected_before_hash: fnv1a64_utf8_digest(before),
                expected_after_hash: fnv1a64_utf8_digest(after),
                replacement_utf8: after.to_string(),
            }],
            cleanup_state: SourceApplySandboxCleanupState::Complete,
            audit_ledger_ref: "evidence/audit.json".to_string(),
            guardrails: vec!["reuse Safe Source Apply v1".to_string()],
        }
    }
}
