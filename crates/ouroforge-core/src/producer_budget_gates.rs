//! Producer Budgets, Stop Conditions and Human Approval Gates v1 (#1685).
//!
//! Adds game-scale budget/stop accounting around the whole-game producer state.
//! This reuses the evolve-campaign/fuzz budget + stop-condition shape and stays
//! Rust/local, descriptive, bounded, proposal-only, and read-only for browser or
//! Studio consumers. It is not a new control engine and it never grants trusted
//! write, auto-apply, auto-merge, self-approval, reviewer-bypass, release, or
//! quality/fun authority.

use crate::{
    evolve_campaign::EvolveCampaignBudget,
    producer_orchestration::{ProducerOrchestrationState, PRODUCER_ORCHESTRATION_SCHEMA_VERSION},
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PRODUCER_BUDGET_GATES_SCHEMA_VERSION: &str = "producer-budget-gates-v1";

const REQUIRED_HUMAN_GATE_KINDS: &[&str] = ["vision", "legal", "release"].as_slice();

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ProducerStopReason {
    BudgetExhausted,
    HumanApprovalRequired,
    NoProgress,
}

impl ProducerStopReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BudgetExhausted => "budget-exhausted",
            Self::HumanApprovalRequired => "human-approval-required",
            Self::NoProgress => "no-progress",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProducerGateStatus {
    Pending,
    Approved,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProducerStopCondition {
    pub condition_id: String,
    pub reason: ProducerStopReason,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProducerHumanApprovalGate {
    pub gate_id: String,
    pub gate_kind: String,
    pub status: ProducerGateStatus,
    pub evidence_ref: String,
    pub approver_role: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProducerBudgetUsage {
    pub iteration_count: u32,
    pub cost_units: u64,
    pub no_progress_steps: u32,
    pub last_evidence_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProducerBudgetGatePolicy {
    pub schema_version: String,
    pub policy_id: String,
    pub orchestration_ref: String,
    pub budget: EvolveCampaignBudget,
    pub usage: ProducerBudgetUsage,
    pub stop_conditions: Vec<ProducerStopCondition>,
    pub human_approval_gates: Vec<ProducerHumanApprovalGate>,
    pub evidence_refs: Vec<String>,
    pub generated_state_policy: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProducerBudgetGateReadModel {
    pub schema_version: String,
    pub policy_id: String,
    pub orchestration_id: String,
    pub status: ProducerBudgetGateStatus,
    pub stop_reason: Option<ProducerStopReason>,
    pub condition_id: Option<String>,
    pub diagnosis: String,
    pub iteration_count: u32,
    pub max_iterations: u32,
    pub cost_units: u64,
    pub max_cost_units: u64,
    pub no_progress_steps: u32,
    pub no_progress_window: u32,
    pub pending_human_gate_ids: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub validation_summary: Vec<String>,
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProducerBudgetGateStatus {
    Continue,
    HaltedBudgetExhausted,
    BlockedHumanGate,
    StoppedNoProgress,
}

impl ProducerBudgetGatePolicy {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let policy: Self = serde_json::from_str(input)
            .context("failed to parse Producer Budget Gates policy JSON")?;
        policy.validate()?;
        Ok(policy)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCER_BUDGET_GATES_SCHEMA_VERSION {
            return Err(anyhow!(
                "producer budget gates schemaVersion must be {PRODUCER_BUDGET_GATES_SCHEMA_VERSION}"
            ));
        }
        require_id("producer budget gates policyId", &self.policy_id)?;
        require_ref(
            "producer budget gates orchestrationRef",
            &self.orchestration_ref,
        )?;
        validate_budget(&self.budget)?;
        validate_usage(&self.usage)?;
        validate_stop_conditions(&self.stop_conditions)?;
        validate_human_gates(&self.human_approval_gates)?;
        validate_refs(
            "producer budget gates evidenceRefs",
            &self.evidence_refs,
            true,
        )?;
        require_text(
            "producer budget gates generatedStatePolicy",
            &self.generated_state_policy,
        )?;
        let policy = self.generated_state_policy.to_ascii_lowercase();
        for required in ["untracked", "fixture-scoped"] {
            if !policy.contains(required) {
                return Err(anyhow!(
                    "producer budget gates generatedStatePolicy must mention `{required}`"
                ));
            }
        }
        require_boundary(&self.boundary)?;
        Ok(())
    }
}

pub fn evaluate_producer_budget_gates(
    orchestration: &ProducerOrchestrationState,
    policy: &ProducerBudgetGatePolicy,
) -> Result<ProducerBudgetGateReadModel> {
    orchestration.validate()?;
    policy.validate()?;
    if orchestration.schema_version != PRODUCER_ORCHESTRATION_SCHEMA_VERSION {
        return Err(anyhow!(
            "producer budget gates require v1 orchestration state"
        ));
    }
    if policy.orchestration_ref != orchestration.orchestration_id {
        return Err(anyhow!(
            "producer budget gates orchestrationRef must match orchestrationId"
        ));
    }

    let (status, stop_reason, condition_id, diagnosis) = classify(policy)?;
    let mut pending_human_gate_ids: Vec<String> = policy
        .human_approval_gates
        .iter()
        .filter(|gate| gate.status == ProducerGateStatus::Pending)
        .map(|gate| gate.gate_id.clone())
        .collect();
    pending_human_gate_ids.sort();

    Ok(ProducerBudgetGateReadModel {
        schema_version: policy.schema_version.clone(),
        policy_id: policy.policy_id.clone(),
        orchestration_id: orchestration.orchestration_id.clone(),
        status,
        stop_reason,
        condition_id,
        diagnosis,
        iteration_count: policy.usage.iteration_count,
        max_iterations: policy.budget.max_iterations,
        cost_units: policy.usage.cost_units,
        max_cost_units: policy.budget.max_cost_units,
        no_progress_steps: policy.usage.no_progress_steps,
        no_progress_window: policy.budget.no_progress_window,
        pending_human_gate_ids,
        evidence_refs: policy.evidence_refs.clone(),
        validation_summary: vec![
            "producer game-scale budgets are hard caps and fail closed with diagnosis".to_string(),
            "human approval gates are mandatory and pending gates block further producer progress".to_string(),
            "no-progress uses the reused campaign/fuzz stop-condition window; producer never loops unbounded".to_string(),
        ],
        compatibility_notes: vec![
            "Reuses evolve-campaign/fuzz budget and stop-condition shape; no new control engine".to_string(),
            "Builds on producer orchestration state; Rust/local owns validation and browser/Studio surfaces remain read-only".to_string(),
            "Generated runs, assets, content, and local artifacts remain untracked unless fixture-scoped".to_string(),
        ],
        boundary: policy.boundary.clone(),
    })
}

fn classify(
    policy: &ProducerBudgetGatePolicy,
) -> Result<(
    ProducerBudgetGateStatus,
    Option<ProducerStopReason>,
    Option<String>,
    String,
)> {
    if policy.usage.iteration_count >= policy.budget.max_iterations
        || policy.usage.cost_units >= policy.budget.max_cost_units
    {
        return Ok((
            ProducerBudgetGateStatus::HaltedBudgetExhausted,
            Some(ProducerStopReason::BudgetExhausted),
            Some(condition_id_for(
                policy,
                ProducerStopReason::BudgetExhausted,
            )?),
            format!(
                "budget exhausted: iterations {}/{} cost {}/{}; diagnosis evidence {}",
                policy.usage.iteration_count,
                policy.budget.max_iterations,
                policy.usage.cost_units,
                policy.budget.max_cost_units,
                policy.usage.last_evidence_ref
            ),
        ));
    }

    let pending: Vec<&ProducerHumanApprovalGate> = policy
        .human_approval_gates
        .iter()
        .filter(|gate| gate.status == ProducerGateStatus::Pending)
        .collect();
    if !pending.is_empty() {
        let ids = pending
            .iter()
            .map(|gate| gate.gate_id.as_str())
            .collect::<Vec<_>>()
            .join(",");
        return Ok((
            ProducerBudgetGateStatus::BlockedHumanGate,
            Some(ProducerStopReason::HumanApprovalRequired),
            Some(condition_id_for(
                policy,
                ProducerStopReason::HumanApprovalRequired,
            )?),
            format!(
                "human approval required: pending gates {ids}; diagnosis evidence {}",
                policy.usage.last_evidence_ref
            ),
        ));
    }

    if policy.usage.no_progress_steps >= policy.budget.no_progress_window {
        return Ok((
            ProducerBudgetGateStatus::StoppedNoProgress,
            Some(ProducerStopReason::NoProgress),
            Some(condition_id_for(policy, ProducerStopReason::NoProgress)?),
            format!(
                "no progress: trailing {} steps reached window {}; diagnosis evidence {}",
                policy.usage.no_progress_steps,
                policy.budget.no_progress_window,
                policy.usage.last_evidence_ref
            ),
        ));
    }

    Ok((
        ProducerBudgetGateStatus::Continue,
        None,
        None,
        "within budget, all mandatory human approval gates approved, and no-progress window not reached".to_string(),
    ))
}

fn condition_id_for(
    policy: &ProducerBudgetGatePolicy,
    reason: ProducerStopReason,
) -> Result<String> {
    policy
        .stop_conditions
        .iter()
        .find(|condition| condition.reason == reason)
        .map(|condition| condition.condition_id.clone())
        .ok_or_else(|| {
            anyhow!(
                "producer budget gates missing stop condition for {}",
                reason.as_str()
            )
        })
}

fn validate_budget(budget: &EvolveCampaignBudget) -> Result<()> {
    if budget.max_iterations == 0 {
        return Err(anyhow!(
            "producer budget gates maxIterations must be at least 1"
        ));
    }
    if budget.max_cost_units == 0 {
        return Err(anyhow!(
            "producer budget gates maxCostUnits must be at least 1"
        ));
    }
    if budget.no_progress_window == 0 {
        return Err(anyhow!(
            "producer budget gates noProgressWindow must be at least 1"
        ));
    }
    if budget.no_progress_window > budget.max_iterations {
        return Err(anyhow!(
            "producer budget gates noProgressWindow must not exceed maxIterations"
        ));
    }
    Ok(())
}

fn validate_usage(usage: &ProducerBudgetUsage) -> Result<()> {
    require_ref(
        "producer budget gates usage.lastEvidenceRef",
        &usage.last_evidence_ref,
    )?;
    Ok(())
}

fn validate_stop_conditions(conditions: &[ProducerStopCondition]) -> Result<()> {
    if conditions.is_empty() {
        return Err(anyhow!(
            "producer budget gates require declared stop conditions"
        ));
    }
    let mut ids = BTreeSet::new();
    let mut reasons = BTreeSet::new();
    for condition in conditions {
        require_id("producer budget gates conditionId", &condition.condition_id)?;
        require_text(
            "producer budget gates stop condition description",
            &condition.description,
        )?;
        if !ids.insert(condition.condition_id.as_str()) {
            return Err(anyhow!(
                "producer budget gates duplicate stop condition `{}`",
                condition.condition_id
            ));
        }
        reasons.insert(condition.reason);
    }
    for required in [
        ProducerStopReason::BudgetExhausted,
        ProducerStopReason::HumanApprovalRequired,
        ProducerStopReason::NoProgress,
    ] {
        if !reasons.contains(&required) {
            return Err(anyhow!(
                "producer budget gates must declare `{}` stop condition",
                required.as_str()
            ));
        }
    }
    Ok(())
}

fn validate_human_gates(gates: &[ProducerHumanApprovalGate]) -> Result<()> {
    if gates.is_empty() {
        return Err(anyhow!(
            "producer budget gates require mandatory human approval gates"
        ));
    }
    let mut ids = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    for gate in gates {
        require_id("producer budget gates human gate gateId", &gate.gate_id)?;
        require_id("producer budget gates human gate gateKind", &gate.gate_kind)?;
        require_ref(
            "producer budget gates human gate evidenceRef",
            &gate.evidence_ref,
        )?;
        require_text(
            "producer budget gates human gate approverRole",
            &gate.approver_role,
        )?;
        if !ids.insert(gate.gate_id.as_str()) {
            return Err(anyhow!(
                "producer budget gates duplicate human approval gate `{}`",
                gate.gate_id
            ));
        }
        kinds.insert(gate.gate_kind.as_str());
    }
    for required in REQUIRED_HUMAN_GATE_KINDS {
        if !kinds.contains(required) {
            return Err(anyhow!(
                "producer budget gates missing mandatory human approval gate `{required}`"
            ));
        }
    }
    Ok(())
}

fn validate_refs(field: &str, values: &[String], required: bool) -> Result<()> {
    if required && values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = BTreeSet::new();
    for value in values {
        require_ref(field, value)?;
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{field} contains duplicate ref `{value}`"));
        }
    }
    Ok(())
}

fn require_boundary(value: &str) -> Result<()> {
    require_text("producer budget gates boundary", value)?;
    let lower = value.to_ascii_lowercase();
    for required in [
        "reuses evolve-campaign/fuzz budget and stop-condition shape",
        "no new control engine",
        "game-scale budgets",
        "human approval gates mandatory",
        "rust/local",
        "browser/studio read-only",
        "proposal-only",
        "no direct trusted writes",
        "no auto-apply",
        "no auto-merge",
        "no self-approval",
        "no reviewer bypass",
        "no production-ready",
        "no engine replacement",
    ] {
        if !lower.contains(required) {
            return Err(anyhow!(
                "producer budget gates boundary must state `{required}`"
            ));
        }
    }
    Ok(())
}

fn require_id(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('.')
        || value.starts_with('/')
        || value.contains("..")
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must be a local stable id using alphanumeric, dash, underscore, or dot characters"
        ));
    }
    Ok(())
}

fn require_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with("http://")
        || value.starts_with("https://")
        || value.starts_with('/')
        || value.contains("..")
    {
        return Err(anyhow!(
            "{field} must be a local fixture/reference path or stable ref"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn reject_forbidden_wording(field: &str, value: &str) -> Result<()> {
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "reviewer bypass enabled",
        "direct trusted write enabled",
        "browser trusted write enabled",
        "production-ready engine",
        "godot replacement enabled",
        "godot parity enabled",
        "finished game shipping",
        "quality/fun guaranteed",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden producer budget gates wording `{forbidden}`"
            ));
        }
    }
    Ok(())
}
