//! Multi-Iteration Evolve Campaigns v1 — campaign model and stop conditions.
//!
//! A campaign is an ordered, finite sequence of bounded evolve iterations
//! directed at one Seed's acceptance criteria. This module owns the campaign
//! model and its termination (stop conditions), reusing the adversarial-input
//! fuzzing stop-condition + budget shape and the four-gate verdict vocabulary.
//! It introduces no new evolve, verdict, or journal engine.
//!
//! Scope boundary: this is a descriptive audit model, not a quality,
//! correctness, or production-readiness guarantee. See
//! `docs/evolve-campaign-v1.md`.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const EVOLVE_CAMPAIGN_SCHEMA_VERSION: &str = "evolve-campaign-v1";

/// The canonical four gates, in order. Each iteration records exactly one
/// verdict per gate.
pub const EVOLVE_CAMPAIGN_FOUR_GATES: &[&str] = &["mechanical", "runtime", "visual", "semantic"];

/// Per-gate verdict status for one iteration. Descriptive only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvolveGateStatus {
    Pass,
    Fail,
    Unsupported,
}
impl EvolveGateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Unsupported => "unsupported",
        }
    }
}

/// The terminal stop reason for a campaign. Reuses the fuzz stop-condition
/// vocabulary: acceptance-reached, budget-exhausted, no-progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvolveCampaignStopReason {
    AcceptanceReached,
    BudgetExhausted,
    NoProgress,
}
impl EvolveCampaignStopReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcceptanceReached => "acceptance-reached",
            Self::BudgetExhausted => "budget-exhausted",
            Self::NoProgress => "no-progress",
        }
    }
}

/// Per-iteration trust-gradient decision. Iterations are manual-review unless
/// the mutation falls within the Milestone 22 bounded auto-apply budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvolveCampaignDecision {
    ManualReview,
    AutoApply,
}
impl EvolveCampaignDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManualReview => "manual-review",
            Self::AutoApply => "auto-apply",
        }
    }
}

/// Hard iteration/cost budget. Both limits are inclusive caps; a campaign never
/// runs more iterations or spends more cost than declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvolveCampaignBudget {
    pub max_iterations: u32,
    pub max_cost_units: u64,
    /// Size of the trailing window inspected for the no-progress stop condition.
    pub no_progress_window: u32,
}

/// A declared stop condition. Reuses the fuzz stop-condition shape (conditionId
/// + description) and adds a typed reason so termination is enumerable.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvolveCampaignStopCondition {
    pub condition_id: String,
    pub reason: EvolveCampaignStopReason,
    pub description: String,
}

/// One gate verdict within an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvolveGateVerdict {
    pub gate: String,
    pub status: EvolveGateStatus,
}

/// One bounded evolve iteration: hypothesis, the mutation it produced, the
/// four-gate verdict for its rerun, and the trust-gradient decision.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvolveCampaignIteration {
    pub index: u32,
    pub hypothesis: String,
    pub mutation_ref: String,
    pub cost_units: u64,
    pub four_gate: Vec<EvolveGateVerdict>,
    pub decision: EvolveCampaignDecision,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// The recorded terminal state of a campaign.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvolveCampaignTermination {
    pub reason: EvolveCampaignStopReason,
    pub condition_id: String,
    /// The iteration index that satisfied acceptance, when reason is
    /// acceptance-reached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_iteration: Option<u32>,
    /// Required when the campaign did not reach acceptance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<String>,
}

/// The campaign artifact. Additive and backward-compatible; generated runs are
/// untracked unless fixture-scoped.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvolveCampaignArtifact {
    pub schema_version: String,
    pub campaign_id: String,
    pub seed_ref: String,
    /// Gates that must all be `pass` for acceptance. Non-empty subset of the
    /// four canonical gates.
    pub acceptance_target: Vec<String>,
    pub budget: EvolveCampaignBudget,
    pub stop_conditions: Vec<EvolveCampaignStopCondition>,
    pub iterations: Vec<EvolveCampaignIteration>,
    pub termination: EvolveCampaignTermination,
    pub boundary: String,
}

/// Read model returned by [`validate_evolve_campaign`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolveCampaignReadModel {
    pub schema_version: String,
    pub campaign_id: String,
    pub seed_ref: String,
    pub acceptance_target: Vec<String>,
    pub iteration_count: usize,
    pub total_cost_units: u64,
    pub stop_reason: EvolveCampaignStopReason,
    pub stop_condition_id: String,
    pub accepted_iteration: Option<u32>,
    pub diagnosis: Option<String>,
    pub boundary: String,
}

pub fn read_evolve_campaign_artifact(
    path: impl AsRef<std::path::Path>,
) -> Result<EvolveCampaignArtifact> {
    let path = path.as_ref();
    let input = std::fs::read_to_string(path).map_err(|err| {
        anyhow!(
            "failed to read evolve campaign artifact {}: {err}",
            path.display()
        )
    })?;
    serde_json::from_str(&input).map_err(|err| {
        anyhow!(
            "failed to parse evolve campaign artifact {}: {err}",
            path.display()
        )
    })
}

/// Validate a campaign artifact and return its read model. Rejects missing stop
/// conditions, malformed budgets, zero-iteration campaigns, stale refs, and
/// terminations inconsistent with the recorded iterations.
pub fn validate_evolve_campaign(
    artifact: &EvolveCampaignArtifact,
) -> Result<EvolveCampaignReadModel> {
    if artifact.schema_version != EVOLVE_CAMPAIGN_SCHEMA_VERSION {
        return Err(anyhow!(
            "evolve campaign schemaVersion must be {EVOLVE_CAMPAIGN_SCHEMA_VERSION}"
        ));
    }
    validate_id("campaignId", &artifact.campaign_id)?;
    validate_ref("seedRef", &artifact.seed_ref)?;
    validate_boundary(&artifact.boundary)?;
    validate_acceptance_target(&artifact.acceptance_target)?;
    validate_budget(&artifact.budget)?;
    validate_stop_conditions(&artifact.stop_conditions)?;

    if artifact.iterations.is_empty() {
        return Err(anyhow!(
            "evolve campaign requires at least one iteration (baseline)"
        ));
    }
    if artifact.iterations.len() as u64 > artifact.budget.max_iterations as u64 {
        return Err(anyhow!(
            "evolve campaign iteration count {} exceeds budget maxIterations {}",
            artifact.iterations.len(),
            artifact.budget.max_iterations
        ));
    }

    let mut total_cost: u64 = 0;
    for (position, iteration) in artifact.iterations.iter().enumerate() {
        validate_iteration(iteration, position)?;
        total_cost = total_cost
            .checked_add(iteration.cost_units)
            .ok_or_else(|| anyhow!("evolve campaign total cost overflow"))?;
    }
    if total_cost > artifact.budget.max_cost_units {
        return Err(anyhow!(
            "evolve campaign total cost {total_cost} exceeds budget maxCostUnits {}",
            artifact.budget.max_cost_units
        ));
    }

    validate_termination(artifact, total_cost)?;

    Ok(EvolveCampaignReadModel {
        schema_version: EVOLVE_CAMPAIGN_SCHEMA_VERSION.to_string(),
        campaign_id: artifact.campaign_id.clone(),
        seed_ref: artifact.seed_ref.clone(),
        acceptance_target: artifact.acceptance_target.clone(),
        iteration_count: artifact.iterations.len(),
        total_cost_units: total_cost,
        stop_reason: artifact.termination.reason,
        stop_condition_id: artifact.termination.condition_id.clone(),
        accepted_iteration: artifact.termination.accepted_iteration,
        diagnosis: artifact.termination.diagnosis.clone(),
        boundary: artifact.boundary.clone(),
    })
}

pub fn evolve_campaign_cli_summary(artifact: &EvolveCampaignArtifact) -> Result<String> {
    let read_model = validate_evolve_campaign(artifact)?;
    Ok(format!(
        "Evolve campaign: stop={} iterations={} cost={} accepted-iteration={} boundary=descriptive-read-only",
        read_model.stop_reason.as_str(),
        read_model.iteration_count,
        read_model.total_cost_units,
        read_model
            .accepted_iteration
            .map(|index| index.to_string())
            .unwrap_or_else(|| "none".to_string()),
    ))
}

/// Whether an iteration satisfies the acceptance target: every targeted gate is
/// recorded `pass`.
pub fn iteration_reaches_acceptance(
    iteration: &EvolveCampaignIteration,
    acceptance_target: &[String],
) -> bool {
    acceptance_target.iter().all(|target| {
        iteration
            .four_gate
            .iter()
            .any(|verdict| &verdict.gate == target && verdict.status == EvolveGateStatus::Pass)
    })
}

fn validate_acceptance_target(target: &[String]) -> Result<()> {
    if target.is_empty() {
        return Err(anyhow!(
            "evolve campaign acceptanceTarget must name at least one gate"
        ));
    }
    let mut seen = BTreeSet::new();
    for gate in target {
        if !EVOLVE_CAMPAIGN_FOUR_GATES.contains(&gate.as_str()) {
            return Err(anyhow!(
                "evolve campaign acceptanceTarget gate `{gate}` is not one of the four gates"
            ));
        }
        if !seen.insert(gate.as_str()) {
            return Err(anyhow!(
                "evolve campaign acceptanceTarget contains duplicate gate `{gate}`"
            ));
        }
    }
    Ok(())
}

fn validate_budget(budget: &EvolveCampaignBudget) -> Result<()> {
    if budget.max_iterations == 0 {
        return Err(anyhow!(
            "evolve campaign budget maxIterations must be at least 1"
        ));
    }
    if budget.max_cost_units == 0 {
        return Err(anyhow!(
            "evolve campaign budget maxCostUnits must be at least 1"
        ));
    }
    if budget.no_progress_window == 0 {
        return Err(anyhow!(
            "evolve campaign budget noProgressWindow must be at least 1"
        ));
    }
    if budget.no_progress_window > budget.max_iterations {
        return Err(anyhow!(
            "evolve campaign budget noProgressWindow must not exceed maxIterations"
        ));
    }
    Ok(())
}

fn validate_stop_conditions(conditions: &[EvolveCampaignStopCondition]) -> Result<()> {
    if conditions.is_empty() {
        return Err(anyhow!(
            "evolve campaign requires at least one declared stop condition"
        ));
    }
    let mut ids = BTreeSet::new();
    let mut reasons = BTreeSet::new();
    for condition in conditions {
        validate_id("stopConditions.conditionId", &condition.condition_id)?;
        validate_text("stopConditions.description", &condition.description)?;
        if !ids.insert(condition.condition_id.as_str()) {
            return Err(anyhow!(
                "evolve campaign stop conditions contain duplicate conditionId `{}`",
                condition.condition_id
            ));
        }
        reasons.insert(condition.reason);
    }
    if !reasons.contains(&EvolveCampaignStopReason::AcceptanceReached) {
        return Err(anyhow!(
            "evolve campaign must declare an acceptance-reached stop condition"
        ));
    }
    Ok(())
}

fn validate_iteration(iteration: &EvolveCampaignIteration, position: usize) -> Result<()> {
    if iteration.index as usize != position {
        return Err(anyhow!(
            "evolve campaign iteration index {} must equal its position {position}",
            iteration.index
        ));
    }
    validate_text("iteration.hypothesis", &iteration.hypothesis)?;
    validate_ref("iteration.mutationRef", &iteration.mutation_ref)?;
    validate_refs("iteration.evidenceRefs", &iteration.evidence_refs)?;

    let mut gates = BTreeSet::new();
    for verdict in &iteration.four_gate {
        if !EVOLVE_CAMPAIGN_FOUR_GATES.contains(&verdict.gate.as_str()) {
            return Err(anyhow!(
                "evolve campaign iteration {position} four_gate gate `{}` is not one of the four gates",
                verdict.gate
            ));
        }
        if !gates.insert(verdict.gate.as_str()) {
            return Err(anyhow!(
                "evolve campaign iteration {position} repeats gate `{}`",
                verdict.gate
            ));
        }
    }
    if gates.len() != EVOLVE_CAMPAIGN_FOUR_GATES.len() {
        return Err(anyhow!(
            "evolve campaign iteration {position} must record all four gates"
        ));
    }
    Ok(())
}

fn validate_termination(artifact: &EvolveCampaignArtifact, total_cost: u64) -> Result<()> {
    let termination = &artifact.termination;
    validate_id("termination.conditionId", &termination.condition_id)?;
    let matching = artifact
        .stop_conditions
        .iter()
        .find(|condition| condition.condition_id == termination.condition_id)
        .ok_or_else(|| {
            anyhow!(
                "evolve campaign termination conditionId `{}` is not a declared stop condition",
                termination.condition_id
            )
        })?;
    if matching.reason != termination.reason {
        return Err(anyhow!(
            "evolve campaign termination reason `{}` does not match stop condition `{}` reason `{}`",
            termination.reason.as_str(),
            matching.condition_id,
            matching.reason.as_str()
        ));
    }

    let last_index = (artifact.iterations.len() - 1) as u32;
    let final_reaches = artifact
        .iterations
        .last()
        .map(|iteration| iteration_reaches_acceptance(iteration, &artifact.acceptance_target))
        .unwrap_or(false);

    match termination.reason {
        EvolveCampaignStopReason::AcceptanceReached => {
            let accepted = termination.accepted_iteration.ok_or_else(|| {
                anyhow!("evolve campaign acceptance-reached termination requires acceptedIteration")
            })?;
            if accepted != last_index {
                return Err(anyhow!(
                    "evolve campaign acceptance-reached termination must accept the final iteration"
                ));
            }
            if !final_reaches {
                return Err(anyhow!(
                    "evolve campaign acceptance-reached termination requires the final iteration to pass every acceptance gate"
                ));
            }
            if termination.diagnosis.is_some() {
                return Err(anyhow!(
                    "evolve campaign acceptance-reached termination must not carry a non-convergence diagnosis"
                ));
            }
        }
        EvolveCampaignStopReason::BudgetExhausted => {
            require_diagnosis(termination)?;
            if final_reaches {
                return Err(anyhow!(
                    "evolve campaign budget-exhausted termination must not have a final iteration that reached acceptance"
                ));
            }
            let iteration_cap_reached =
                artifact.iterations.len() as u32 == artifact.budget.max_iterations;
            let cost_cap_reached = total_cost == artifact.budget.max_cost_units;
            if !iteration_cap_reached && !cost_cap_reached {
                return Err(anyhow!(
                    "evolve campaign budget-exhausted termination requires the iteration or cost budget to be reached"
                ));
            }
            if termination.accepted_iteration.is_some() {
                return Err(anyhow!(
                    "evolve campaign budget-exhausted termination must not record an acceptedIteration"
                ));
            }
        }
        EvolveCampaignStopReason::NoProgress => {
            require_diagnosis(termination)?;
            if final_reaches {
                return Err(anyhow!(
                    "evolve campaign no-progress termination must not have a final iteration that reached acceptance"
                ));
            }
            if termination.accepted_iteration.is_some() {
                return Err(anyhow!(
                    "evolve campaign no-progress termination must not record an acceptedIteration"
                ));
            }
            verify_no_progress_window(artifact)?;
        }
    }
    Ok(())
}

fn require_diagnosis(termination: &EvolveCampaignTermination) -> Result<()> {
    match termination.diagnosis.as_deref() {
        Some(text) => validate_text("termination.diagnosis", text),
        None => Err(anyhow!(
            "evolve campaign non-converged termination requires an evidence-linked diagnosis"
        )),
    }
}

/// A no-progress stop is only valid if the trailing window of iterations shows
/// no gate improving from fail/unsupported to pass.
fn verify_no_progress_window(artifact: &EvolveCampaignArtifact) -> Result<()> {
    let window = artifact.budget.no_progress_window as usize;
    if artifact.iterations.len() < window + 1 {
        return Err(anyhow!(
            "evolve campaign no-progress termination requires at least noProgressWindow+1 iterations"
        ));
    }
    let start = artifact.iterations.len() - window - 1;
    for pair in artifact.iterations[start..].windows(2) {
        if gate_improved(&pair[0], &pair[1]) {
            return Err(anyhow!(
                "evolve campaign no-progress termination is invalid: a gate improved within the trailing window"
            ));
        }
    }
    Ok(())
}

fn gate_improved(previous: &EvolveCampaignIteration, current: &EvolveCampaignIteration) -> bool {
    for gate in EVOLVE_CAMPAIGN_FOUR_GATES {
        let before = gate_status(previous, gate);
        let after = gate_status(current, gate);
        if before != Some(EvolveGateStatus::Pass) && after == Some(EvolveGateStatus::Pass) {
            return true;
        }
    }
    false
}

fn gate_status(iteration: &EvolveCampaignIteration, gate: &str) -> Option<EvolveGateStatus> {
    iteration
        .four_gate
        .iter()
        .find(|verdict| verdict.gate == gate)
        .map(|verdict| verdict.status)
}

fn validate_refs(label: &str, refs: &[String]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in refs {
        validate_ref(label, value)?;
        if !seen.insert(value) {
            return Err(anyhow!("{label} contains duplicate ref `{value}`"));
        }
    }
    Ok(())
}

fn validate_ref(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.starts_with('/')
        || value.contains("..")
        || value.contains('\\')
    {
        return Err(anyhow!(
            "{label} must be a safe repo-relative or run-relative ref"
        ));
    }
    validate_tracked_fixture_ref(label, value)?;
    Ok(())
}

fn validate_tracked_fixture_ref(label: &str, value: &str) -> Result<()> {
    if !requires_tracked_fixture_ref(value) {
        return Ok(());
    }

    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = workspace_root.join(value);
    if !path.exists() {
        return Err(anyhow!(
            "{label} points to missing repo-relative ref `{value}`"
        ));
    }
    Ok(())
}

fn requires_tracked_fixture_ref(value: &str) -> bool {
    value.starts_with("examples/evolve-campaign-v1/contract/")
}

fn validate_id(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!("{label} must be a non-empty stable id"));
    }
    Ok(())
}

fn validate_text(label: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > 512 {
        return Err(anyhow!("{label} must be non-empty bounded text"));
    }
    for forbidden in [
        "production-ready",
        "Godot replacement",
        "auto-fix",
        "reviewer bypass",
        "quality claim",
    ] {
        if trimmed.contains(forbidden) {
            return Err(anyhow!(
                "{label} contains forbidden evolve campaign authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn validate_boundary(value: &str) -> Result<()> {
    validate_text("boundary", value)?;
    for required in [
        "descriptive",
        "read-only",
        "manual-review",
        "bounded",
        "no auto-merge",
        "not quality",
    ] {
        if !value.contains(required) {
            return Err(anyhow!("evolve campaign boundary must state `{required}`"));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Convergence tracking and budget outcome (#1488)
//
// Convergence is a descriptive running state computed from the per-iteration
// four-gate verdict deltas. It is never a quality or correctness guarantee. The
// outcome reuses the campaign model and termination from #1487; it adds no new
// engine.
// ---------------------------------------------------------------------------

/// The terminal convergence outcome of a campaign.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvolveCampaignOutcomeState {
    Converged,
    NotConverged,
}
impl EvolveCampaignOutcomeState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Converged => "converged",
            Self::NotConverged => "not-converged",
        }
    }
}

/// How a single gate moved between two adjacent iterations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvolveGateTransition {
    Improved,
    Regressed,
    Unchanged,
}
impl EvolveGateTransition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Improved => "improved",
            Self::Regressed => "regressed",
            Self::Unchanged => "unchanged",
        }
    }
}

/// The delta for one gate between the previous iteration and the current one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolveGateDelta {
    pub gate: String,
    pub before: EvolveGateStatus,
    pub after: EvolveGateStatus,
    pub transition: EvolveGateTransition,
}

/// The four-gate verdict delta for one iteration versus its predecessor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolveCampaignIterationDelta {
    pub index: u32,
    /// True for the baseline iteration (index 0), which has no predecessor; its
    /// deltas are reported against an implicit all-unsupported baseline.
    pub is_baseline: bool,
    pub gate_deltas: Vec<EvolveGateDelta>,
    pub improved_gates: usize,
    pub regressed_gates: usize,
    pub reaches_acceptance: bool,
}

/// The descriptive convergence outcome artifact for a campaign.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolveCampaignOutcome {
    pub schema_version: String,
    pub campaign_id: String,
    pub state: EvolveCampaignOutcomeState,
    pub stop_reason: EvolveCampaignStopReason,
    /// The iteration that reached acceptance, when converged.
    pub converged_iteration: Option<u32>,
    pub iteration_count: usize,
    pub total_cost_units: u64,
    pub iteration_deltas: Vec<EvolveCampaignIterationDelta>,
    /// The final iteration's gate deltas (the "last deltas" for a diagnosis).
    pub last_deltas: Vec<EvolveGateDelta>,
    /// Required when not converged.
    pub diagnosis: Option<String>,
    pub boundary: String,
}

/// Compute the convergence outcome for a campaign. Validates the campaign first
/// (reusing #1487), then derives per-iteration four-gate deltas and a
/// converged/not-converged state consistent with the recorded termination.
/// Budgets are hard: an over-budget campaign is rejected before any outcome is
/// produced.
pub fn compute_evolve_campaign_outcome(
    artifact: &EvolveCampaignArtifact,
) -> Result<EvolveCampaignOutcome> {
    let read_model = validate_evolve_campaign(artifact)?;

    if artifact.iterations.is_empty() {
        return Err(anyhow!(
            "evolve campaign convergence requires a baseline iteration"
        ));
    }

    let mut iteration_deltas = Vec::with_capacity(artifact.iterations.len());
    for (position, iteration) in artifact.iterations.iter().enumerate() {
        let previous = position
            .checked_sub(1)
            .map(|prev| &artifact.iterations[prev]);
        let gate_deltas = gate_deltas_for(previous, iteration);
        let improved_gates = gate_deltas
            .iter()
            .filter(|delta| delta.transition == EvolveGateTransition::Improved)
            .count();
        let regressed_gates = gate_deltas
            .iter()
            .filter(|delta| delta.transition == EvolveGateTransition::Regressed)
            .count();
        iteration_deltas.push(EvolveCampaignIterationDelta {
            index: iteration.index,
            is_baseline: position == 0,
            gate_deltas,
            improved_gates,
            regressed_gates,
            reaches_acceptance: iteration_reaches_acceptance(
                iteration,
                &artifact.acceptance_target,
            ),
        });
    }

    let last_deltas = iteration_deltas
        .last()
        .map(|delta| delta.gate_deltas.clone())
        .unwrap_or_default();

    let (state, converged_iteration, diagnosis) = match artifact.termination.reason {
        EvolveCampaignStopReason::AcceptanceReached => (
            EvolveCampaignOutcomeState::Converged,
            artifact.termination.accepted_iteration,
            None,
        ),
        EvolveCampaignStopReason::BudgetExhausted | EvolveCampaignStopReason::NoProgress => {
            let diagnosis = artifact.termination.diagnosis.clone().ok_or_else(|| {
                anyhow!("evolve campaign not-converged outcome requires a diagnosis")
            })?;
            (
                EvolveCampaignOutcomeState::NotConverged,
                None,
                Some(diagnosis),
            )
        }
    };

    Ok(EvolveCampaignOutcome {
        schema_version: EVOLVE_CAMPAIGN_SCHEMA_VERSION.to_string(),
        campaign_id: read_model.campaign_id,
        state,
        stop_reason: artifact.termination.reason,
        converged_iteration,
        iteration_count: read_model.iteration_count,
        total_cost_units: read_model.total_cost_units,
        iteration_deltas,
        last_deltas,
        diagnosis,
        boundary: read_model.boundary,
    })
}

pub fn evolve_campaign_outcome_cli_summary(artifact: &EvolveCampaignArtifact) -> Result<String> {
    let outcome = compute_evolve_campaign_outcome(artifact)?;
    Ok(format!(
        "Evolve campaign outcome: state={} stop={} converged-iteration={} iterations={} cost={} boundary=descriptive-read-only",
        outcome.state.as_str(),
        outcome.stop_reason.as_str(),
        outcome
            .converged_iteration
            .map(|index| index.to_string())
            .unwrap_or_else(|| "none".to_string()),
        outcome.iteration_count,
        outcome.total_cost_units,
    ))
}

/// Compute the four-gate deltas for `current` versus `previous`. When
/// `previous` is `None` (the baseline iteration), each gate is compared against
/// an implicit `unsupported` baseline so the trajectory has a defined origin.
fn gate_deltas_for(
    previous: Option<&EvolveCampaignIteration>,
    current: &EvolveCampaignIteration,
) -> Vec<EvolveGateDelta> {
    EVOLVE_CAMPAIGN_FOUR_GATES
        .iter()
        .filter_map(|gate| {
            let after = gate_status(current, gate)?;
            let before = match previous {
                Some(prev) => gate_status(prev, gate).unwrap_or(EvolveGateStatus::Unsupported),
                None => EvolveGateStatus::Unsupported,
            };
            Some(EvolveGateDelta {
                gate: (*gate).to_string(),
                before,
                after,
                transition: classify_transition(before, after),
            })
        })
        .collect()
}

fn classify_transition(before: EvolveGateStatus, after: EvolveGateStatus) -> EvolveGateTransition {
    match (
        before == EvolveGateStatus::Pass,
        after == EvolveGateStatus::Pass,
    ) {
        (false, true) => EvolveGateTransition::Improved,
        (true, false) => EvolveGateTransition::Regressed,
        _ => EvolveGateTransition::Unchanged,
    }
}

// ---------------------------------------------------------------------------
// Campaign journal narrative (#1489)
//
// Extends the journal-to-mutation narrative across a campaign: per iteration
// (hypothesis -> failing gate -> evidence -> mutation -> rerun delta) plus a
// final converged/not-converged summary. Reuses Journal v2's narrative shape
// and the campaign outcome from #1488; it adds no new journal engine.
// ---------------------------------------------------------------------------

/// One narrative entry for a single campaign iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolveCampaignJournalEntry {
    pub index: u32,
    pub is_baseline: bool,
    pub hypothesis: String,
    /// Gates that are not `pass` for this iteration, in canonical order.
    pub failing_gates: Vec<String>,
    pub four_gate: Vec<EvolveGateVerdict>,
    pub gate_deltas: Vec<EvolveGateDelta>,
    pub mutation_ref: String,
    pub decision: EvolveCampaignDecision,
    pub evidence_refs: Vec<String>,
    pub reaches_acceptance: bool,
}

/// The final campaign summary appended to the narrative.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolveCampaignJournalSummary {
    pub state: EvolveCampaignOutcomeState,
    pub stop_reason: EvolveCampaignStopReason,
    pub converged_iteration: Option<u32>,
    pub diagnosis: Option<String>,
    pub iteration_count: usize,
    pub total_cost_units: u64,
}

/// The structured campaign journal: an ordered per-iteration narrative and a
/// final summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolveCampaignJournal {
    pub schema_version: String,
    pub campaign_id: String,
    pub seed_ref: String,
    pub acceptance_target: Vec<String>,
    pub entries: Vec<EvolveCampaignJournalEntry>,
    pub summary: EvolveCampaignJournalSummary,
    pub boundary: String,
}

/// Build the structured campaign journal. Reuses the campaign outcome (which
/// validates the artifact and computes per-iteration deltas) and additionally
/// requires every iteration to link justifying evidence and the iteration
/// sequence to be gap-free.
pub fn build_evolve_campaign_journal(
    artifact: &EvolveCampaignArtifact,
) -> Result<EvolveCampaignJournal> {
    let outcome = compute_evolve_campaign_outcome(artifact)?;

    let mut entries = Vec::with_capacity(artifact.iterations.len());
    for (position, iteration) in artifact.iterations.iter().enumerate() {
        // Gap-free sequence: index must match position (also enforced by
        // validation; restated here so the journal owns its narrative ordering).
        if iteration.index as usize != position {
            return Err(anyhow!(
                "evolve campaign journal requires a gap-free iteration sequence; iteration index {} is out of order at position {position}",
                iteration.index
            ));
        }
        // Missing iteration evidence is a journal error: a narrative entry must
        // link the evidence that justifies it.
        if iteration.evidence_refs.is_empty() {
            return Err(anyhow!(
                "evolve campaign journal iteration {position} is missing linked evidence"
            ));
        }
        let delta = &outcome.iteration_deltas[position];
        let failing_gates = EVOLVE_CAMPAIGN_FOUR_GATES
            .iter()
            .filter(|gate| gate_status(iteration, gate) != Some(EvolveGateStatus::Pass))
            .map(|gate| (*gate).to_string())
            .collect();
        entries.push(EvolveCampaignJournalEntry {
            index: iteration.index,
            is_baseline: delta.is_baseline,
            hypothesis: iteration.hypothesis.clone(),
            failing_gates,
            four_gate: iteration.four_gate.clone(),
            gate_deltas: delta.gate_deltas.clone(),
            mutation_ref: iteration.mutation_ref.clone(),
            decision: iteration.decision,
            evidence_refs: iteration.evidence_refs.clone(),
            reaches_acceptance: delta.reaches_acceptance,
        });
    }

    Ok(EvolveCampaignJournal {
        schema_version: EVOLVE_CAMPAIGN_SCHEMA_VERSION.to_string(),
        campaign_id: artifact.campaign_id.clone(),
        seed_ref: artifact.seed_ref.clone(),
        acceptance_target: artifact.acceptance_target.clone(),
        entries,
        summary: EvolveCampaignJournalSummary {
            state: outcome.state,
            stop_reason: outcome.stop_reason,
            converged_iteration: outcome.converged_iteration,
            diagnosis: outcome.diagnosis,
            iteration_count: outcome.iteration_count,
            total_cost_units: outcome.total_cost_units,
        },
        boundary: artifact.boundary.clone(),
    })
}

/// Render the campaign journal as a Markdown narrative: a header, one section
/// per iteration (hypothesis -> failing gates -> four-gate verdict -> rerun
/// delta -> mutation -> evidence), and a final converged/not-converged summary.
pub fn render_evolve_campaign_journal_markdown(
    artifact: &EvolveCampaignArtifact,
) -> Result<String> {
    let journal = build_evolve_campaign_journal(artifact)?;
    let mut out = String::new();
    out.push_str(&format!(
        "# Evolve Campaign Journal: {}\n\n",
        journal.campaign_id
    ));
    out.push_str(&format!("- Seed: `{}`\n", journal.seed_ref));
    out.push_str(&format!(
        "- Acceptance target: {}\n",
        journal.acceptance_target.join(", ")
    ));
    out.push_str(&format!("- Boundary: {}\n\n", journal.boundary));

    for entry in &journal.entries {
        let suffix = if entry.is_baseline { " (baseline)" } else { "" };
        out.push_str(&format!("## Iteration {}{}\n", entry.index, suffix));
        out.push_str(&format!("- Hypothesis: {}\n", entry.hypothesis));
        let failing = if entry.failing_gates.is_empty() {
            "none".to_string()
        } else {
            entry.failing_gates.join(", ")
        };
        out.push_str(&format!("- Failing gates: {failing}\n"));
        let verdicts: Vec<String> = entry
            .four_gate
            .iter()
            .map(|verdict| format!("{}={}", verdict.gate, verdict.status.as_str()))
            .collect();
        out.push_str(&format!("- Four-gate verdict: {}\n", verdicts.join(", ")));
        if entry.is_baseline {
            out.push_str("- Rerun delta: baseline (no predecessor)\n");
        } else {
            let moves: Vec<String> = entry
                .gate_deltas
                .iter()
                .filter(|delta| delta.transition != EvolveGateTransition::Unchanged)
                .map(|delta| {
                    format!(
                        "{} {} ({} -> {})",
                        delta.gate,
                        delta.transition.as_str(),
                        delta.before.as_str(),
                        delta.after.as_str()
                    )
                })
                .collect();
            let rendered = if moves.is_empty() {
                "no gate movement".to_string()
            } else {
                moves.join(", ")
            };
            out.push_str(&format!("- Rerun delta: {rendered}\n"));
        }
        out.push_str(&format!("- Mutation: `{}`\n", entry.mutation_ref));
        out.push_str(&format!("- Decision: {}\n", entry.decision.as_str()));
        out.push_str(&format!(
            "- Evidence: {}\n\n",
            entry
                .evidence_refs
                .iter()
                .map(|reference| format!("`{reference}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    out.push_str("## Campaign summary\n");
    out.push_str(&format!("- Outcome: {}\n", journal.summary.state.as_str()));
    out.push_str(&format!(
        "- Stop reason: {}\n",
        journal.summary.stop_reason.as_str()
    ));
    match journal.summary.converged_iteration {
        Some(index) => out.push_str(&format!("- Converged at iteration: {index}\n")),
        None => out.push_str("- Converged at iteration: none\n"),
    }
    if let Some(diagnosis) = &journal.summary.diagnosis {
        out.push_str(&format!("- Diagnosis: {diagnosis}\n"));
    }
    out.push_str(&format!(
        "- Iterations: {} (total cost {})\n",
        journal.summary.iteration_count, journal.summary.total_cost_units
    ));
    out.push_str(
        "- Boundary: descriptive narrative over Journal v2; not a quality or correctness guarantee\n",
    );

    Ok(out)
}
