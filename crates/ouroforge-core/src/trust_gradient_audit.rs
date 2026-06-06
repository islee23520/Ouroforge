//! Auto-Apply Audit Log and Kill Switch v1 (#1479, #1 Era E Milestone 22).
//!
//! Records every auto-applied change in an append-only audit log and provides an
//! emergency kill switch that halts all autonomy. Each entry captures the
//! proposal ref, risk tier, gate verdicts, budget state, apply result, and a
//! rollback handle. The log is append-only and tamper-evident: sequence numbers
//! must be contiguous and strictly increasing, every entry must be complete, and
//! every rollback handle must be intact. When the kill switch is engaged, no new
//! auto-apply may be recorded — eligible proposals fall back to manual review,
//! restoring the default "no auto-apply" posture. This module records and
//! gates; it applies nothing and runs nothing.
//!
//! Authorized by the Trust Gradient design gate (docs/trust-gradient-design.md).
//! Rust/local owned; browser read-only.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub const TRUST_GRADIENT_AUDIT_SCHEMA_VERSION: &str = "trust-gradient-audit-v1";

const BOUNDARY: &str = "append-only auto-apply audit and kill switch: not auto-merge, \
not self-approval, not a quality guarantee; Rust/local owned, browser read-only";

/// Risk tier recorded with an audit entry (mirrors the design-gate T0/T1/T2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RiskTier {
    Low,
    Medium,
    High,
}

/// Outcome of a single gate, recorded with the entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GateOutcome {
    Pass,
    Fail,
    Missing,
    Stale,
}

/// Four-gate verdict recorded with an audit entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GateVerdicts {
    pub mechanical: GateOutcome,
    pub runtime: GateOutcome,
    pub visual: GateOutcome,
    pub semantic: GateOutcome,
}

/// Apply result recorded with an audit entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplyResult {
    AutoApplied,
    ManualFallback,
}

/// Rollback handle recorded with an auto-applied entry. Integrity requires both
/// fields to be present and non-empty so any auto-apply stays reversible.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackHandle {
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "reverseRef")]
    pub reverse_ref: String,
}

impl RollbackHandle {
    fn is_intact(&self) -> bool {
        !self.apply_transaction_id.trim().is_empty() && !self.reverse_ref.trim().is_empty()
    }

    fn rollback_command(&self) -> String {
        format!(
            "ouroforge rollback --transaction {} --reverse {}",
            self.apply_transaction_id, self.reverse_ref
        )
    }
}

/// One append-only audit entry for an auto-apply decision.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AutoApplyAuditEntry {
    /// Monotonic sequence number; entries must be contiguous from 0.
    pub sequence: u64,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    pub tier: RiskTier,
    pub gates: GateVerdicts,
    /// Remaining risk budget recorded at apply time.
    #[serde(rename = "budgetRemaining")]
    pub budget_remaining: u32,
    #[serde(rename = "applyResult")]
    pub apply_result: ApplyResult,
    #[serde(rename = "rollbackHandle")]
    pub rollback_handle: RollbackHandle,
}

impl AutoApplyAuditEntry {
    fn is_complete(&self) -> bool {
        !self.proposal_ref.trim().is_empty() && self.rollback_handle.is_intact()
    }
}

/// Emergency kill switch state.
///
/// Defaults to not engaged. The "no auto-apply" default is preserved elsewhere:
/// autonomy is opt-in, so a disengaged kill switch does not by itself enable
/// auto-apply.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KillSwitch {
    pub engaged: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Append-only auto-apply audit log with an emergency kill switch.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AutoApplyAuditLog {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(default)]
    pub entries: Vec<AutoApplyAuditEntry>,
    #[serde(rename = "killSwitch", default)]
    pub kill_switch: KillSwitch,
    #[serde(default = "default_boundary")]
    pub boundary: String,
}

fn default_boundary() -> String {
    BOUNDARY.to_string()
}

/// An engaged kill switch must record a non-empty operator reason; a missing,
/// empty, or whitespace-only reason is treated as blank (#1479).
fn kill_switch_reason_is_blank(reason: &Option<String>) -> bool {
    reason.as_deref().map(str::trim).unwrap_or("").is_empty()
}

impl AutoApplyAuditLog {
    /// A fresh, empty log with the kill switch disengaged.
    pub fn new() -> Self {
        AutoApplyAuditLog {
            schema_version: TRUST_GRADIENT_AUDIT_SCHEMA_VERSION.to_string(),
            entries: Vec::new(),
            kill_switch: KillSwitch::default(),
            boundary: BOUNDARY.to_string(),
        }
    }

    pub fn from_json_str(input: &str) -> Result<Self> {
        let log: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse auto-apply audit log: {err}"))?;
        log.validate()?;
        Ok(log)
    }

    /// Validate schema and append-only / tamper-evidence invariants: contiguous
    /// strictly-increasing sequences from 0, complete entries, and intact
    /// rollback handles.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != TRUST_GRADIENT_AUDIT_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected schema version: {}",
                self.schema_version
            ));
        }
        for (index, entry) in self.entries.iter().enumerate() {
            let expected = index as u64;
            if entry.sequence != expected {
                return Err(anyhow!(
                    "audit log tampered: entry {index} has sequence {} (expected {expected}); \
append-only sequences must be contiguous",
                    entry.sequence
                ));
            }
            if !entry.is_complete() {
                return Err(anyhow!(
                    "audit entry {index} is incomplete or has a broken rollback handle"
                ));
            }
        }
        if self.kill_switch.engaged && kill_switch_reason_is_blank(&self.kill_switch.reason) {
            return Err(anyhow!(
                "engaged kill switch must record a non-empty reason"
            ));
        }
        Ok(())
    }

    /// Whether autonomy is currently halted (kill switch engaged).
    pub fn is_autonomy_halted(&self) -> bool {
        self.kill_switch.engaged
    }

    /// Append a new auto-apply entry. Fails if the kill switch is engaged, if the
    /// entry is incomplete, or if its sequence is not the next contiguous value.
    pub fn append(&mut self, entry: AutoApplyAuditEntry) -> Result<()> {
        if self.kill_switch.engaged {
            return Err(anyhow!(
                "kill switch engaged: autonomy halted, no auto-apply may be recorded"
            ));
        }
        if !entry.is_complete() {
            return Err(anyhow!("refusing to append an incomplete audit entry"));
        }
        let expected = self.entries.len() as u64;
        if entry.sequence != expected {
            return Err(anyhow!(
                "non-monotonic append: got sequence {}, expected {expected}",
                entry.sequence
            ));
        }
        self.entries.push(entry);
        Ok(())
    }

    /// Engage the emergency kill switch, halting all further auto-apply.
    ///
    /// Fails closed if `reason` is empty or whitespace-only: an engaged switch
    /// must always record an operator reason (#1479).
    pub fn engage_kill_switch(&mut self, reason: impl Into<String>) -> Result<()> {
        let reason = reason.into();
        if reason.trim().is_empty() {
            return Err(anyhow!(
                "kill switch reason must be a non-empty operator reason"
            ));
        }
        self.kill_switch = KillSwitch {
            engaged: true,
            reason: Some(reason),
        };
        Ok(())
    }

    /// Resolve the one-command rollback for a recorded entry by sequence.
    pub fn rollback_command(&self, sequence: u64) -> Option<String> {
        self.entries
            .iter()
            .find(|entry| entry.sequence == sequence)
            .map(|entry| entry.rollback_handle.rollback_command())
    }
}

impl Default for AutoApplyAuditLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gates() -> GateVerdicts {
        GateVerdicts {
            mechanical: GateOutcome::Pass,
            runtime: GateOutcome::Pass,
            visual: GateOutcome::Pass,
            semantic: GateOutcome::Pass,
        }
    }

    fn entry(sequence: u64) -> AutoApplyAuditEntry {
        AutoApplyAuditEntry {
            sequence,
            proposal_ref: format!("proposal-{sequence}"),
            tier: RiskTier::Low,
            gates: gates(),
            budget_remaining: 2,
            apply_result: ApplyResult::AutoApplied,
            rollback_handle: RollbackHandle {
                apply_transaction_id: format!("txn-{sequence}"),
                reverse_ref: format!("reverse/txn-{sequence}.json"),
            },
        }
    }

    #[test]
    fn append_records_entries_and_resolves_rollback() {
        let mut log = AutoApplyAuditLog::new();
        log.append(entry(0)).unwrap();
        log.append(entry(1)).unwrap();
        log.validate().unwrap();
        assert_eq!(log.entries.len(), 2);
        assert!(log.rollback_command(1).unwrap().contains("txn-1"));
    }

    #[test]
    fn kill_switch_halts_further_auto_apply() {
        let mut log = AutoApplyAuditLog::new();
        log.append(entry(0)).unwrap();
        log.engage_kill_switch("operator halt").unwrap();
        assert!(log.is_autonomy_halted());
        assert!(log.append(entry(1)).is_err());
        // A blank reason is rejected at engage time.
        assert!(AutoApplyAuditLog::new().engage_kill_switch("   ").is_err());
    }

    #[test]
    fn tampered_sequence_gap_is_detected() {
        let mut log = AutoApplyAuditLog::new();
        log.entries.push(entry(0));
        log.entries.push(entry(2)); // gap: missing sequence 1
        assert!(log.validate().is_err());
    }

    #[test]
    fn broken_rollback_handle_is_detected() {
        let mut log = AutoApplyAuditLog::new();
        let mut bad = entry(0);
        bad.rollback_handle.reverse_ref = String::new();
        log.entries.push(bad);
        assert!(log.validate().is_err());
    }

    #[test]
    fn engaged_kill_switch_requires_reason() {
        let mut log = AutoApplyAuditLog::new();
        log.kill_switch = KillSwitch {
            engaged: true,
            reason: None,
        };
        assert!(log.validate().is_err());
    }
}
