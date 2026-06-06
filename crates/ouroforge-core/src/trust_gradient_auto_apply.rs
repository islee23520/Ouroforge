//! Rollback-Backed Bounded Auto-Apply v1 (#1478, #1 Era E Milestone 22).
//!
//! Decides whether an auto-apply-eligible (low-risk) mutation proposal may apply
//! within an explicit risk budget, always reversibly. Auto-apply is a governed
//! entry into the existing review-gated scene-only apply + rollback path
//! (#215-style): this module owns the decision and the rollback handle, not a
//! new writer. A proposal auto-applies only when it is low-risk and
//! auto-apply-eligible, at high confidence, with all four gates passing on
//! rerun, within an unexhausted risk budget, and backed by a one-command
//! rollback. Anything else falls back to manual review. It decides and records;
//! it executes nothing on its own.
//!
//! Authorized by the Trust Gradient design gate (docs/trust-gradient-design.md).
//! Auto-apply is off by default; the default everywhere remains "no auto-apply".

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub const TRUST_GRADIENT_AUTO_APPLY_SCHEMA_VERSION: &str = "trust-gradient-auto-apply-v1";

/// Inclusive high-confidence threshold, matching the risk-tier classifier.
pub const TRUST_GRADIENT_AUTO_APPLY_CONFIDENCE_THRESHOLD: f64 = 0.9;

const BOUNDARY: &str =
    "bounded, reversible, audited auto-apply: not auto-merge, not self-approval, \
not a quality guarantee; low-risk scene-only data only, Rust/local owned, browser read-only";

/// Risk tier carried into the auto-apply decision. Only [`RiskTier::Low`] is
/// ever apply-eligible; mirrors the design-gate T0/T1/T2 model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RiskTier {
    Low,
    Medium,
    High,
}

/// Whether the upstream classifier marked the proposal auto-apply eligible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AutoApplyEligibility {
    AutoApplyEligible,
    ManualOnly,
}

/// Outcome of a single gate on the post-apply rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GateOutcome {
    Pass,
    Fail,
    Missing,
    Stale,
}

impl GateOutcome {
    fn is_pass(self) -> bool {
        matches!(self, GateOutcome::Pass)
    }
}

/// Four-gate verdict re-run against the post-apply state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RerunGateVerdicts {
    pub mechanical: GateOutcome,
    pub runtime: GateOutcome,
    pub visual: GateOutcome,
    pub semantic: GateOutcome,
}

impl RerunGateVerdicts {
    fn all_pass(&self) -> bool {
        self.mechanical.is_pass()
            && self.runtime.is_pass()
            && self.visual.is_pass()
            && self.semantic.is_pass()
    }
}

/// Explicit, configurable risk budget for a run. Auto-apply consumes `cost`
/// from `remaining`; when `cost` exceeds `remaining`, auto-apply is refused and
/// the proposal falls back to manual review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RiskBudget {
    /// Remaining auto-apply allowance (count/scope units) before this proposal.
    pub remaining: u32,
    /// Cost this proposal would consume from the budget.
    pub cost: u32,
}

impl RiskBudget {
    fn is_exhausted_for(&self, _proposal: &str) -> bool {
        self.cost == 0 || self.cost > self.remaining
    }

    fn after_apply(&self) -> RiskBudget {
        RiskBudget {
            remaining: self.remaining.saturating_sub(self.cost),
            cost: self.cost,
        }
    }
}

/// Handle to the existing scene-only rollback metadata that makes an applied
/// change reversible. This module does not create rollback data; it references
/// the existing apply/rollback path so any auto-apply is one-command reversible.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackHandle {
    #[serde(rename = "applyTransactionId")]
    pub apply_transaction_id: String,
    #[serde(rename = "reverseRef")]
    pub reverse_ref: String,
}

/// Input request for an auto-apply decision.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AutoApplyRequest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    pub tier: RiskTier,
    pub eligibility: AutoApplyEligibility,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(rename = "rerunGates")]
    pub rerun_gates: RerunGateVerdicts,
    pub budget: RiskBudget,
    /// Whether autonomy is opted in for this run. Absent/false keeps the system
    /// at the "no auto-apply" default, equivalent to the kill switch engaged.
    #[serde(rename = "autonomyEnabled", default)]
    pub autonomy_enabled: bool,
    /// Rollback handle into the existing apply/rollback path. Absent means no
    /// reversible apply is possible, so auto-apply is refused.
    #[serde(
        rename = "rollbackHandle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rollback_handle: Option<RollbackHandle>,
}

impl AutoApplyRequest {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let request: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse auto-apply request: {err}"))?;
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != TRUST_GRADIENT_AUTO_APPLY_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected schema version: {}",
                self.schema_version
            ));
        }
        if self.proposal_ref.trim().is_empty() {
            return Err(anyhow!("proposalRef must not be empty"));
        }
        if let Some(confidence) = self.confidence {
            if !(0.0..=1.0).contains(&confidence) {
                return Err(anyhow!("confidence must be within [0, 1]: {confidence}"));
            }
        }
        Ok(())
    }
}

/// Outcome of an auto-apply decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AutoApplyOutcome {
    /// The proposal auto-applied within budget and is one-command reversible.
    AutoApplied,
    /// The proposal fell back to manual review-gated apply (the default).
    ManualFallback,
}

/// Decision artifact for an auto-apply request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AutoApplyDecision {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    pub outcome: AutoApplyOutcome,
    pub reasons: Vec<String>,
    /// Present only when `outcome` is `AutoApplied`: the one-command rollback.
    #[serde(
        rename = "rollbackCommand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rollback_command: Option<String>,
    #[serde(rename = "budgetAfter")]
    pub budget_after: RiskBudget,
    pub boundary: String,
}

/// Decide whether a proposal auto-applies or falls back to manual review.
///
/// Fail-closed: the outcome is [`AutoApplyOutcome::AutoApplied`] only when every
/// mandatory safety property holds — autonomy is enabled, eligibility is
/// `AutoApplyEligible`, tier is `Low`, confidence is present and at least the
/// high-confidence threshold, all four gates pass on rerun, the risk budget is
/// not exhausted, and a rollback handle is present. Otherwise the proposal falls
/// back to manual review and the budget is left unchanged.
pub fn decide_auto_apply(request: &AutoApplyRequest) -> Result<AutoApplyDecision> {
    request.validate()?;

    let mut reasons: Vec<String> = Vec::new();
    let mut apply = true;

    if !request.autonomy_enabled {
        apply = false;
        reasons.push("autonomy disabled (kill switch / default no-auto-apply)".to_string());
    }
    if request.eligibility != AutoApplyEligibility::AutoApplyEligible {
        apply = false;
        reasons.push("proposal is not auto-apply eligible".to_string());
    }
    if request.tier != RiskTier::Low {
        apply = false;
        reasons.push("risk tier is not low".to_string());
    }
    match request.confidence {
        Some(confidence) if confidence >= TRUST_GRADIENT_AUTO_APPLY_CONFIDENCE_THRESHOLD => {}
        _ => {
            apply = false;
            reasons.push("confidence missing or below high-confidence threshold".to_string());
        }
    }
    if !request.rerun_gates.all_pass() {
        apply = false;
        reasons.push("not all four gates pass on rerun".to_string());
    }
    if request.budget.is_exhausted_for(&request.proposal_ref) {
        apply = false;
        reasons.push("risk budget exhausted".to_string());
    }
    // A rollback handle must be present AND actionable: both fields non-empty
    // after trimming. An empty/whitespace handle would emit a non-actionable
    // `ouroforge rollback --transaction  --reverse ` command, breaking the
    // one-command-reversal guarantee (#1478).
    let rollback_present = request.rollback_handle.as_ref().is_some_and(|handle| {
        !handle.apply_transaction_id.trim().is_empty() && !handle.reverse_ref.trim().is_empty()
    });
    if !rollback_present {
        apply = false;
        reasons.push(
            "no actionable rollback handle (applyTransactionId and reverseRef must be non-empty); cannot guarantee one-command reversal"
                .to_string(),
        );
    }

    let (outcome, rollback_command, budget_after) = if apply {
        let handle = request
            .rollback_handle
            .as_ref()
            .expect("rollback handle present when applying");
        reasons.push(
            "low-risk, eligible, high confidence, all gates pass, in budget, rollback-backed"
                .to_string(),
        );
        (
            AutoApplyOutcome::AutoApplied,
            Some(format!(
                "ouroforge rollback --transaction {} --reverse {}",
                handle.apply_transaction_id, handle.reverse_ref
            )),
            request.budget.after_apply(),
        )
    } else {
        (AutoApplyOutcome::ManualFallback, None, request.budget)
    };

    Ok(AutoApplyDecision {
        schema_version: TRUST_GRADIENT_AUTO_APPLY_SCHEMA_VERSION.to_string(),
        proposal_ref: request.proposal_ref.clone(),
        outcome,
        reasons,
        rollback_command,
        budget_after,
        boundary: BOUNDARY.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn passing_gates() -> RerunGateVerdicts {
        RerunGateVerdicts {
            mechanical: GateOutcome::Pass,
            runtime: GateOutcome::Pass,
            visual: GateOutcome::Pass,
            semantic: GateOutcome::Pass,
        }
    }

    fn apply_request() -> AutoApplyRequest {
        AutoApplyRequest {
            schema_version: TRUST_GRADIENT_AUTO_APPLY_SCHEMA_VERSION.to_string(),
            proposal_ref: "proposal-low".to_string(),
            tier: RiskTier::Low,
            eligibility: AutoApplyEligibility::AutoApplyEligible,
            confidence: Some(0.95),
            rerun_gates: passing_gates(),
            budget: RiskBudget {
                remaining: 3,
                cost: 1,
            },
            autonomy_enabled: true,
            rollback_handle: Some(RollbackHandle {
                apply_transaction_id: "txn-1".to_string(),
                reverse_ref: "reverse/txn-1.json".to_string(),
            }),
        }
    }

    #[test]
    fn eligible_in_budget_auto_applies_reversibly() {
        let decision = decide_auto_apply(&apply_request()).unwrap();
        assert_eq!(decision.outcome, AutoApplyOutcome::AutoApplied);
        assert!(decision.rollback_command.is_some());
        assert_eq!(decision.budget_after.remaining, 2);
    }

    #[test]
    fn autonomy_disabled_falls_back_to_manual() {
        let mut request = apply_request();
        request.autonomy_enabled = false;
        let decision = decide_auto_apply(&request).unwrap();
        assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
        assert!(decision.rollback_command.is_none());
        assert_eq!(decision.budget_after.remaining, 3);
    }

    #[test]
    fn ineligible_falls_back_to_manual() {
        let mut request = apply_request();
        request.eligibility = AutoApplyEligibility::ManualOnly;
        let decision = decide_auto_apply(&request).unwrap();
        assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    }

    #[test]
    fn gate_regression_on_rerun_falls_back_to_manual() {
        let mut request = apply_request();
        request.rerun_gates.runtime = GateOutcome::Fail;
        let decision = decide_auto_apply(&request).unwrap();
        assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    }

    #[test]
    fn exhausted_budget_falls_back_to_manual() {
        let mut request = apply_request();
        request.budget = RiskBudget {
            remaining: 0,
            cost: 1,
        };
        let decision = decide_auto_apply(&request).unwrap();
        assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    }

    #[test]
    fn missing_rollback_handle_falls_back_to_manual() {
        let mut request = apply_request();
        request.rollback_handle = None;
        let decision = decide_auto_apply(&request).unwrap();
        assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    }
}
