//! Non-Developer Accessibility Path v1 (#1595).
//!
//! Part of Generative Front Door v1 (#1592) under #1 Era F Milestone 30. This
//! module wires the non-developer-facing flow: a plain authoring brief becomes a
//! generated proposal (intake, #1593), the engine room verifies it (the
//! promotion guard, #1594), and the result — the proposal, its generation
//! provenance, the engine-room verdicts, and the solver trace — is assembled
//! into a single READ-ONLY view a non-developer can inspect.
//!
//! Boundary: this is read-only surfacing over the existing path, not a new write
//! surface. It reuses the intake ([`crate::generative_intake`]), the promotion
//! guard ([`crate::generative_promotion_guard`]), and the deterministic solver
//! ([`crate::puzzle_solver`]); it adds no new engine, runtime, writer, or
//! evaluator. It performs no trusted write, auto-apply, self-approval, or
//! reviewer bypass. A *verified* result (one that passed the engine room) carries
//! the proposal eligible to enter the existing review/apply/trust-gradient path,
//! unchanged and still proposal-only; an *unverified* brief is reported
//! read-only and is never promoted. "Verified" means only that the proposal
//! passed the engine room — never that the generated game is good, fun, or
//! shippable.

use crate::generative_intake::{intake_brief, GenerationProvenance, GenerativeBrief};
use crate::generative_promotion_guard::{
    evaluate_promotion, PromotionGuardOutcome, PromotionGuardVerdict,
};
use crate::puzzle_solver::{self, SolveBudget, SolveOutcome};
use crate::MutationProposal;
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

/// Schema version for the non-developer accessibility result artifact.
pub const GENERATIVE_ACCESSIBILITY_SCHEMA_VERSION: &str = "ouroforge.generative-accessibility.v1";

/// A read-only solver trace surfaced to a non-developer: the intended solution
/// the brief declared and the deterministic solver's own shortest witness.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AccessibilitySolverTrace {
    /// The intended solution the brief declared (the author's path).
    pub intended_solution: Vec<String>,
    /// Whether the deterministic solver found any winning path within budget.
    pub solvable: bool,
    /// The solver's shortest replayable winning path, when one was found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortest_witness: Option<Vec<String>>,
    /// True when a winning path strictly shorter than the intended solution
    /// exists (an over-solution that bypasses the intended design).
    pub over_solution: bool,
    /// Number of states the solver expanded.
    pub explored_states: usize,
    /// True when the solver hit its budget before deciding; reported explicitly,
    /// never treated as unsolvable.
    pub search_exhausted: bool,
}

/// The non-developer accessibility outcome.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum AccessibilityOutcome {
    /// The brief produced a verified-solvable proposal that passed the engine
    /// room. It is eligible to enter the existing review/apply/trust-gradient
    /// path; it is not applied or approved here.
    Verified {
        /// Evidence references backing the passing engine-room verdict.
        gate_evidence_refs: Vec<String>,
    },
    /// The brief did not produce a verified proposal. Reported read-only with an
    /// evidence-linked reason; never auto-promoted.
    Unverified {
        /// Conservative reason the proposal was not verified.
        reason: String,
        /// Evidence references substantiating the report.
        evidence_refs: Vec<String>,
    },
}

/// A read-only, non-developer-facing view of a generated proposal: the proposal
/// and its assembled artifact, the generation provenance, the engine-room
/// verdicts, and the solver trace. This view performs no trusted write;
/// promotion stays gated by the engine room and routed through the existing
/// review/apply/trust-gradient path.
#[derive(Debug, Clone, Serialize)]
pub struct AccessibilityResult {
    /// Schema version of this result artifact.
    pub schema_version: String,
    /// The brief id this result was produced from.
    pub brief_id: String,
    /// The brief title, surfaced for display.
    pub title: String,
    /// The grid-puzzle level id.
    pub level_id: String,
    /// A conservative, plain-language status line for a non-developer.
    /// "Verified" means only that the proposal passed the engine room.
    pub headline: String,
    /// True iff the proposal passed the engine room.
    pub verified: bool,
    /// The generation provenance linking the proposal to its brief.
    pub provenance: GenerationProvenance,
    /// The generated grid-puzzle artifact, surfaced read-only for inspection.
    pub artifact: Value,
    /// The deterministic solver trace for the level.
    pub solver_trace: AccessibilitySolverTrace,
    /// The engine-room promotion-guard verdict (the proposal plus verdicts).
    pub engine_room: PromotionGuardVerdict,
    /// The accessibility outcome (verified / unverified).
    pub outcome: AccessibilityOutcome,
}

impl AccessibilityResult {
    /// True iff the brief produced a verified-solvable proposal.
    pub fn is_verified(&self) -> bool {
        self.verified
    }

    /// The proposal eligible to enter the existing review/apply/trust-gradient
    /// path — `Some` only when verified, carried unchanged. `None` when
    /// unverified: an unverified brief is reported read-only, never promoted.
    pub fn verified_proposal(&self) -> Option<&MutationProposal> {
        self.engine_room.promotable_proposal()
    }

    /// Serialize the result to canonical JSON for read-only surfacing.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

/// Run the non-developer accessibility path: turn a plain authoring brief into a
/// read-only, verified-or-reported proposal view.
///
/// The brief is run through the front-door intake (#1593) to produce a proposal,
/// the engine room (the promotion guard, #1594) verifies it, and the
/// deterministic solver (#1580) produces a trace. The assembled result surfaces
/// the proposal, provenance, verdicts, and solver trace read-only. A verified
/// result carries the proposal eligible to enter the existing
/// review/apply/trust-gradient path; an unverified brief is reported and never
/// promoted. Fails closed on a malformed brief.
pub fn accessibility_intake(
    brief: &GenerativeBrief,
    budget: SolveBudget,
    now_unix_ms: u128,
) -> Result<AccessibilityResult> {
    // Front door: brief -> proposal (proposal-only; no trusted write).
    let generative = intake_brief(brief, now_unix_ms)?;
    // Engine room: verify the proposal (solver + over-solution + design gate,
    // ANDed into the four-gate aggregation).
    let engine_room = evaluate_promotion(&generative, budget)?;

    let artifact: Value = serde_json::from_str(&generative.proposal.to).unwrap_or(Value::Null);
    let intended_solution: Vec<String> = artifact
        .get("intendedSolution")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    // Solver trace: surface the deterministic solver's own shortest witness.
    let solve_outcome = puzzle_solver::solve(&artifact, budget).ok();
    let (solvable, shortest_witness, explored_states, search_exhausted) = match &solve_outcome {
        Some(SolveOutcome::Solvable { witness, explored }) => {
            (true, Some(witness.clone()), *explored, false)
        }
        Some(SolveOutcome::Unsolvable { explored }) => (false, None, *explored, false),
        Some(SolveOutcome::Exhausted { explored, .. }) => (false, None, *explored, true),
        None => (false, None, 0, false),
    };
    let over_solution = shortest_witness
        .as_ref()
        .map(|witness| witness.len() < intended_solution.len())
        .unwrap_or(false);
    let solver_trace = AccessibilitySolverTrace {
        intended_solution,
        solvable,
        shortest_witness,
        over_solution,
        explored_states,
        search_exhausted,
    };

    let verified = engine_room.is_promotable();
    let (headline, outcome) = match &engine_room.outcome {
        PromotionGuardOutcome::Promotable { gate_evidence_refs } => (
            "Verified solvable: this puzzle passed the engine room and is ready for review. \
             It has not been applied; \"verified\" means it passed the engine room, not that the \
             game is good, fun, or shippable."
                .to_string(),
            AccessibilityOutcome::Verified {
                gate_evidence_refs: gate_evidence_refs.clone(),
            },
        ),
        PromotionGuardOutcome::Blocked {
            reason,
            evidence_refs,
        } => (
            format!("Not verified: {reason}. Reported for review; it was not promoted."),
            AccessibilityOutcome::Unverified {
                reason: reason.clone(),
                evidence_refs: evidence_refs.clone(),
            },
        ),
    };

    Ok(AccessibilityResult {
        schema_version: GENERATIVE_ACCESSIBILITY_SCHEMA_VERSION.to_string(),
        brief_id: brief.brief_id.clone(),
        title: brief.title.clone(),
        level_id: engine_room.level_id.clone(),
        headline,
        verified,
        provenance: generative.provenance.clone(),
        artifact,
        solver_trace,
        engine_room,
        outcome,
    })
}
