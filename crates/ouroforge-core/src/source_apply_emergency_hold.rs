//! Source Apply Emergency Hold and Kill Switch v1 (#715, #1 Milestone 15).
//!
//! Provides an explicit, local hold mechanism to block source apply globally or
//! by scope when risk is detected. Apply readiness fails closed while a matching
//! hold is active, and a hold cannot be bypassed by a force flag. This is a
//! local control only: there is no remote kill switch or cloud control plane.
//! The hold records and evaluates; it applies and reverts nothing itself.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SOURCE_APPLY_EMERGENCY_HOLD_SCHEMA_VERSION: &str = "source-apply-emergency-hold-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceApplyHoldScopeKind {
    /// Blocks all source apply.
    Global,
    /// Blocks apply that touches a given file class.
    FileClass,
    /// Blocks a specific apply transaction.
    TransactionId,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyHoldScope {
    pub kind: SourceApplyHoldScopeKind,
    /// Empty for `global`; required for `file-class` / `transaction-id`.
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyHold {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "holdId")]
    pub hold_id: String,
    pub disabled: bool,
    pub reason: String,
    pub actor: String,
    #[serde(rename = "recordedAt")]
    pub recorded_at: String,
    /// RFC3339-like expiry; when `<= now` the hold is stale/inactive.
    #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub scopes: Vec<SourceApplyHoldScope>,
    #[serde(rename = "requiresReviewToLift")]
    pub requires_review_to_lift: bool,
    #[serde(rename = "auditLedgerRef")]
    pub audit_ledger_ref: String,
    pub guardrails: Vec<String>,
}

/// What an apply attempt is asking to do, checked against the hold.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyHoldQuery {
    /// Current RFC3339-like time, used to evaluate expiry.
    pub now: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "fileClasses", default)]
    pub file_classes: Vec<String>,
    /// A bypass attempt; must be ignored (a hold cannot be forced).
    #[serde(rename = "forceApply", default)]
    pub force_apply: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceApplyHoldEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "holdId")]
    pub hold_id: String,
    pub active: bool,
    #[serde(rename = "applyBlocked")]
    pub apply_blocked: bool,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl SourceApplyHold {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse source apply emergency hold JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SOURCE_APPLY_EMERGENCY_HOLD_SCHEMA_VERSION {
            return Err(anyhow!(
                "source apply emergency hold schemaVersion must be {SOURCE_APPLY_EMERGENCY_HOLD_SCHEMA_VERSION}"
            ));
        }
        require_local_id("source apply emergency hold holdId", &self.hold_id)?;
        require_text("source apply emergency hold actor", &self.actor)?;
        require_text("source apply emergency hold recordedAt", &self.recorded_at)?;
        require_local_ref(
            "source apply emergency hold auditLedgerRef",
            &self.audit_ledger_ref,
        )?;
        if let Some(expiry) = &self.expires_at {
            require_text("source apply emergency hold expiresAt", expiry)?;
        }
        for scope in &self.scopes {
            scope.validate()?;
        }
        if self.disabled {
            // An active hold must explain itself and declare at least one scope.
            require_text("source apply emergency hold reason", &self.reason)?;
            if self.scopes.is_empty() {
                return Err(anyhow!(
                    "an enabled source apply hold must declare at least one scope (conflicting hold state)"
                ));
            }
        }
        require_nonempty(
            "source apply emergency hold guardrails",
            self.guardrails.len(),
        )?;
        for guardrail in &self.guardrails {
            require_text("source apply emergency hold guardrails", guardrail)?;
        }
        Ok(())
    }

    /// True when the hold is enabled and not expired relative to `now`.
    pub fn is_active(&self, now: &str) -> bool {
        if !self.disabled {
            return false;
        }
        match &self.expires_at {
            // RFC3339 timestamps sort lexicographically; expired when expiry <= now.
            Some(expiry) => expiry.as_str() > now,
            None => true,
        }
    }

    /// Fail-closed evaluation against an apply attempt. A matching active hold
    /// blocks the apply; `forceApply` is ignored.
    pub fn evaluate_against(&self, query: &SourceApplyHoldQuery) -> SourceApplyHoldEvaluation {
        let active = self.is_active(&query.now);
        let mut blocked = Vec::new();

        if active {
            for scope in &self.scopes {
                match scope.kind {
                    SourceApplyHoldScopeKind::Global => {
                        blocked.push(format!("source apply globally held: {}", self.reason));
                    }
                    SourceApplyHoldScopeKind::FileClass => {
                        if query.file_classes.iter().any(|fc| fc == &scope.value) {
                            blocked.push(format!(
                                "source apply held for file class `{}`: {}",
                                scope.value, self.reason
                            ));
                        }
                    }
                    SourceApplyHoldScopeKind::TransactionId => {
                        if query.transaction_id == scope.value {
                            blocked.push(format!(
                                "source apply held for transaction `{}`: {}",
                                scope.value, self.reason
                            ));
                        }
                    }
                }
            }
        }

        if query.force_apply && !blocked.is_empty() {
            blocked.push("forceApply cannot bypass an active source apply hold".to_string());
        }

        let apply_blocked = !blocked.is_empty();
        let mut evidence_summary = vec![
            format!("hold:{}", self.hold_id),
            format!("active:{active}"),
            format!("scopes:{}", self.scopes.len()),
        ];
        if !active && self.disabled {
            evidence_summary.push("hold is stale/expired and no longer blocks apply".to_string());
        }

        SourceApplyHoldEvaluation {
            schema_version: SOURCE_APPLY_EMERGENCY_HOLD_SCHEMA_VERSION.to_string(),
            hold_id: self.hold_id.clone(),
            active,
            apply_blocked,
            blocked_reasons: blocked,
            evidence_summary,
            forbidden_actions: vec![
                "bypass_hold".to_string(),
                "apply_patch".to_string(),
                "remote_kill_switch".to_string(),
            ],
        }
    }

    pub fn evaluation_json(&self, query: &SourceApplyHoldQuery) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate_against(query))
            .context("failed to serialize source apply emergency hold evaluation JSON")
    }
}

impl SourceApplyHoldScope {
    fn validate(&self) -> Result<()> {
        match self.kind {
            SourceApplyHoldScopeKind::Global => {
                if !self.value.trim().is_empty() {
                    return Err(anyhow!(
                        "global source apply hold scope must not carry a value"
                    ));
                }
            }
            SourceApplyHoldScopeKind::FileClass | SourceApplyHoldScopeKind::TransactionId => {
                require_text("source apply emergency hold scope value", &self.value)?;
            }
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
