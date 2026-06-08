//! Source Apply Audit Ledger v1 (#710, #1 Milestone 15).
//!
//! Records every trusted source apply attempt — including blocked and failed
//! attempts — in an append-only audit ledger. It enforces append-only semantics
//! (no silent rewrite of recorded history), rejects duplicate attempt ids, and
//! validates transaction references and entry shape. The ledger only records;
//! it never applies patches or performs rollback itself.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SOURCE_APPLY_AUDIT_LEDGER_SCHEMA_VERSION: &str = "source-apply-audit-ledger-v1";

/// Recorded outcome of a single apply attempt.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyAuditApplyStatus {
    Blocked,
    PreconditionFailed,
    ApplyFailed,
    Applied,
    VerificationFailed,
    RollbackRequired,
    Held,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyAuditEntry {
    #[serde(rename = "attemptId")]
    pub attempt_id: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    pub actor: String,
    #[serde(rename = "recordedAt")]
    pub recorded_at: String,
    #[serde(rename = "reviewDecisionRef")]
    pub review_decision_ref: String,
    #[serde(rename = "sandboxReportRef")]
    pub sandbox_report_ref: String,
    #[serde(rename = "staleGuardRef")]
    pub stale_guard_ref: String,
    #[serde(rename = "rollbackSnapshotRef")]
    pub rollback_snapshot_ref: String,
    #[serde(rename = "verificationLogRef")]
    pub verification_log_ref: String,
    #[serde(
        rename = "rerunComparisonRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rerun_comparison_ref: Option<String>,
    #[serde(rename = "applyStatus")]
    pub apply_status: SourceApplyAuditApplyStatus,
    #[serde(
        rename = "failureReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_reason: Option<String>,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyAuditLedger {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "ledgerId")]
    pub ledger_id: String,
    pub entries: Vec<SourceApplyAuditEntry>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyAuditLedgerReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "ledgerId")]
    pub ledger_id: String,
    #[serde(rename = "entryCount")]
    pub entry_count: usize,
    #[serde(rename = "appliedCount")]
    pub applied_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "failedCount")]
    pub failed_count: usize,
    #[serde(rename = "appendOnly")]
    pub append_only: bool,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplyAuditLedger {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply audit ledger JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_AUDIT_LEDGER_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply audit ledger schemaVersion must be {SOURCE_APPLY_AUDIT_LEDGER_SCHEMA_VERSION}"
            ));
        }
        require_local_id("source apply audit ledger ledgerId", &self.ledger_id)?;
        let mut seen = std::collections::BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !seen.insert(entry.attempt_id.clone()) {
                return Err(anyhow!(
                    "source apply audit ledger has a duplicate attemptId `{}`",
                    entry.attempt_id
                ));
            }
        }
        require_nonempty(
            "source apply audit ledger guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text("source apply audit ledger guardrails", guardrail)?;
        }
        Ok(())
    }

    /// Append a new entry, enforcing unique attempt ids. Returns a new ledger;
    /// the original is never mutated in place.
    pub fn append_entry(&self, entry: SourceApplyAuditEntry) -> Result<Self> {
        entry.validate()?;
        if self
            .entries
            .iter()
            .any(|existing| existing.attempt_id == entry.attempt_id)
        {
            return Err(anyhow!(
                "cannot append duplicate attemptId `{}` to the audit ledger",
                entry.attempt_id
            ));
        }
        let mut next = self.clone();
        next.entries.push(entry);
        Ok(next)
    }

    /// Enforce append-only semantics: `self` must extend `previous` by appending
    /// only — every previously recorded entry must be byte-identical and in the
    /// same order. Any rewrite, reorder, or truncation is rejected.
    pub fn validate_is_append_of(&self, previous: &Self) -> Result<()> {
        if self.ledger_id != previous.ledger_id {
            return Err(anyhow!("audit ledger id changed; not an append"));
        }
        if self.entries.len() < previous.entries.len() {
            return Err(anyhow!(
                "audit ledger lost entries; append-only history cannot shrink"
            ));
        }
        for (index, prior) in previous.entries.iter().enumerate() {
            if &self.entries[index] != prior {
                return Err(anyhow!(
                    "audit ledger rewrote entry {index}; recorded history is append-only"
                ));
            }
        }
        Ok(())
    }

    pub fn read_model(&self) -> SourceApplyAuditLedgerReadModel {
        use SourceApplyAuditApplyStatus::*;
        let applied_count = self
            .entries
            .iter()
            .filter(|e| e.apply_status == Applied)
            .count();
        let blocked_count = self
            .entries
            .iter()
            .filter(|e| matches!(e.apply_status, Blocked | Held))
            .count();
        let failed_count = self
            .entries
            .iter()
            .filter(|e| {
                matches!(
                    e.apply_status,
                    PreconditionFailed | ApplyFailed | VerificationFailed | RollbackRequired
                )
            })
            .count();
        SourceApplyAuditLedgerReadModel {
            schema_version: SOURCE_APPLY_AUDIT_LEDGER_SCHEMA_VERSION.to_string(),
            ledger_id: self.ledger_id.clone(),
            entry_count: self.entries.len(),
            applied_count,
            blocked_count,
            failed_count,
            append_only: true,
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "perform_rollback".to_string(),
                "rewrite_history".to_string(),
            ],
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize source apply audit ledger read model JSON")
    }
}

impl SourceApplyAuditEntry {
    fn validate(&self) -> Result<()> {
        require_local_id("source apply audit entry attemptId", &self.attempt_id)?;
        require_local_id(
            "source apply audit entry transactionId",
            &self.transaction_id,
        )?;
        require_text("source apply audit entry actor", &self.actor)?;
        require_text("source apply audit entry recordedAt", &self.recorded_at)?;
        for (field, value) in [
            ("reviewDecisionRef", &self.review_decision_ref),
            ("sandboxReportRef", &self.sandbox_report_ref),
            ("staleGuardRef", &self.stale_guard_ref),
            ("rollbackSnapshotRef", &self.rollback_snapshot_ref),
            ("verificationLogRef", &self.verification_log_ref),
        ] {
            require_local_ref(&format!("source apply audit entry {field}"), value)?;
        }
        if let Some(reference) = &self.rerun_comparison_ref {
            require_local_ref("source apply audit entry rerunComparisonRef", reference)?;
        }
        if let Some(reason) = &self.failure_reason {
            require_text("source apply audit entry failureReason", reason)?;
        }
        // A non-applied terminal status must explain itself.
        if matches!(
            self.apply_status,
            SourceApplyAuditApplyStatus::Blocked | SourceApplyAuditApplyStatus::Held
        ) && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!(
                "blocked/held audit entry `{}` must record blockedReasons",
                self.attempt_id
            ));
        }
        for reason in &self.blocked_reasons {
            require_text("source apply audit entry blockedReasons", reason)?;
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
