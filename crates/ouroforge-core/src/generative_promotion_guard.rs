//! Engine-Room Promotion Guard v1 (#1594).
//!
//! Part of Generative Front Door v1 (#1592) under #1 Era F Milestone 30. A
//! generated proposal (the generative front door, #1593) cannot be promoted
//! unless it passes the ENGINE ROOM: the deterministic grid-puzzle solver
//! (#1580) and over-solution detector (#1581) produce intent-satisfaction and
//! over-solution facts, the Milestone 28 design-integrity gate (#1583) turns
//! those facts into a declared verdict, and that verdict is ANDed into the
//! existing four-gate `declared-gate-and` aggregation. This module is a
//! PROMOTION PRECONDITION over the existing review/apply/trust-gradient path —
//! it is not a new writer, solver, runtime, evaluator, or bypass, and it adds no
//! parallel aggregator: it reuses the over-solution detector
//! ([`crate::puzzle_oversolution`]), the evaluator's design-integrity gate
//! ([`ouroforge_evaluator::design_integrity_gate`]), and the evaluator's
//! `declared-gate-and` gate-category composition.
//!
//! Boundary: the guard never performs a trusted write, auto-apply,
//! self-approval, reviewer bypass, or auto-fix. A passing proposal is certified
//! *eligible* to enter the existing review/apply/trust-gradient path; it remains
//! proposal-only (proposed/pending/unverified) and is handed to that path
//! UNCHANGED, where the existing reviewer and trust-gradient classification
//! still govern whether it is ever applied. A failing proposal is BLOCKED with
//! an evidence-linked reason and withheld from the path. The guard fails closed:
//! an unsatisfiable intent, an over-solution, an inconclusive (exhausted) search,
//! or malformed evidence all block — never a false pass.

use crate::generative_intake::GenerativeProposal;
use crate::puzzle_oversolution;
use crate::puzzle_solver::SolveBudget;
use crate::MutationProposal;
use anyhow::Result;
use ouroforge_evaluator::design_integrity_gate::{
    compose_design_integrity_into_categories, evaluate_design_integrity_check,
    DesignIntegrityCheck, DesignIntegrityGateState, DesignIntegrityGateVerdict,
    DESIGN_INTEGRITY_GATE_SCHEMA_VERSION,
};
use serde::Serialize;
use serde_json::{json, Value};

/// Schema version for the promotion-guard verdict artifact.
pub const GENERATIVE_PROMOTION_GUARD_SCHEMA_VERSION: &str =
    "ouroforge.generative-promotion-guard.v1";

/// The decision of the promotion guard: whether a generated proposal may enter
/// the existing review/apply/trust-gradient path.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "decision", rename_all = "kebab-case")]
pub enum PromotionGuardOutcome {
    /// The proposal passed the engine room. It is eligible to enter the existing
    /// review/apply/trust-gradient path. This is NOT an apply, an approval, or a
    /// reviewer bypass: the proposal stays proposal-only until that path acts.
    Promotable {
        /// Evidence references backing the passing engine-room verdict.
        gate_evidence_refs: Vec<String>,
    },
    /// The proposal did not pass the engine room. It is blocked with an
    /// evidence-linked reason and withheld from the apply path.
    Blocked {
        /// Conservative, human-readable reason for the block.
        reason: String,
        /// Evidence references substantiating the block (the detector report and
        /// any replayable counterexample traces).
        evidence_refs: Vec<String>,
    },
}

/// The promotion-guard verdict: a read-only evidence record certifying whether a
/// generated proposal passed the engine room. It confers no apply or approval
/// authority on its own; promotion still flows through the existing
/// review/apply/trust-gradient path.
#[derive(Debug, Clone, Serialize)]
pub struct PromotionGuardVerdict {
    /// Schema version of this verdict artifact.
    pub schema_version: String,
    /// The id of the evaluated proposal.
    pub proposal_id: String,
    /// The brief id recorded in the proposal's generation provenance.
    pub brief_id: String,
    /// The grid-puzzle level id the engine room evaluated.
    pub level_id: String,
    /// The Milestone 28 design-integrity gate verdict (the engine-room gate over
    /// the solver/over-solution facts).
    pub design_integrity: DesignIntegrityGateVerdict,
    /// The composed gate-categories object: the design-integrity gate ANDed into
    /// the existing four-gate `declared-gate-and` aggregation (the four
    /// mechanical/runtime/visual/semantic gates stay neutral when undeclared for
    /// a freshly generated proposal). Carried as engine-room evidence.
    pub engine_room_categories: Value,
    /// The guard's promotion decision.
    pub outcome: PromotionGuardOutcome,
    /// The proposal eligible to enter the existing review/apply/trust-gradient
    /// path — present only when promotable, and carried UNCHANGED (still
    /// proposed/pending/unverified). `None` when blocked: a blocked proposal is
    /// withheld from the path. This is how an accepted proposal is *routed* into
    /// the existing path; the guard performs no trusted write itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligible_proposal: Option<MutationProposal>,
}

impl PromotionGuardVerdict {
    /// True only when the proposal passed the engine room and may enter the
    /// existing review/apply/trust-gradient path.
    pub fn is_promotable(&self) -> bool {
        matches!(self.outcome, PromotionGuardOutcome::Promotable { .. })
    }

    /// The proposal that may enter the existing review/apply/trust-gradient
    /// path — `Some` only when promotable, returned unchanged. `None` when
    /// blocked.
    pub fn promotable_proposal(&self) -> Option<&MutationProposal> {
        match self.outcome {
            PromotionGuardOutcome::Promotable { .. } => self.eligible_proposal.as_ref(),
            PromotionGuardOutcome::Blocked { .. } => None,
        }
    }

    /// The engine-room (design-integrity) gate state.
    pub fn gate_state(&self) -> DesignIntegrityGateState {
        self.design_integrity.state
    }

    /// Serialize the verdict to canonical JSON for evidence recording.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

/// Evaluate the promotion guard over a generated proposal.
///
/// The guard extracts the grid-puzzle spec and the designer intent from the
/// proposal's assembled artifact, runs the over-solution detector (which reuses
/// the deterministic solver and validates intent satisfaction) to produce the
/// design-integrity facts, evaluates the Milestone 28 design-integrity gate over
/// those facts, and composes that verdict into the existing four-gate
/// `declared-gate-and` aggregation. The proposal is `Promotable` iff the gate
/// passes; otherwise it is `Blocked` with an evidence-linked reason. Promotion
/// of an accepted proposal still flows through the existing
/// review/apply/trust-gradient path via
/// [`PromotionGuardVerdict::promotable_proposal`].
///
/// Fails closed: a structurally invalid proposal is rejected with an error, and
/// every non-`Pass` engine-room outcome (unsatisfiable intent, over-solution,
/// inconclusive search, malformed evidence) yields a `Blocked` verdict — never a
/// false pass.
pub fn evaluate_promotion(
    generative: &GenerativeProposal,
    budget: SolveBudget,
) -> Result<PromotionGuardVerdict> {
    // Fail closed on a structurally invalid generated proposal.
    generative.validate()?;

    let proposal = &generative.proposal;

    // The proposal's `to` payload is the assembled grid-puzzle artifact (the
    // spec). After `validate` it is well-formed JSON; we still fail closed by
    // routing a null spec through the detector, which surfaces as malformed
    // evidence below.
    let spec: Value = serde_json::from_str(&proposal.to).unwrap_or(Value::Null);
    let intended = spec.get("intendedSolution").cloned().unwrap_or(Value::Null);
    let intent = json!({ "intendedSolution": intended.clone() });
    let declared_length = intended.as_array().map(|a| a.len()).unwrap_or(0);
    let level_id = spec
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or(proposal.id.as_str())
        .to_string();

    // Run the over-solution detector (solver + over-solution) to produce the
    // design-integrity facts. A "does not reach the win state" error is an
    // unsatisfiable intent (a gate failure, not malformed input); any other
    // error means the artifact is unverifiable and fails closed as malformed.
    let evidence_base = format!("evidence/generative-promotion-guard/{level_id}");
    let (intent_satisfied, intended_length, over_solution_count, search_exhausted, evidence_refs) =
        match puzzle_oversolution::detect_oversolutions(&spec, &intent, budget) {
            Ok(report) => {
                let mut refs = vec![format!("{evidence_base}/oversolution-report.json")];
                for i in 0..report.counterexamples.len() {
                    refs.push(format!("{evidence_base}/counterexample-{i}.json"));
                }
                (
                    true,
                    report.intended_length,
                    report.counterexamples.len(),
                    report.exhausted,
                    refs,
                )
            }
            Err(err) if err.contains("does not reach the win state") => (
                false,
                declared_length.max(1),
                0,
                false,
                vec![format!("{evidence_base}/captured-intent.json")],
            ),
            // Unexpected detector error: treat as malformed evidence (intended
            // length 0 forces the gate's MalformedEvidence verdict). Fail closed.
            Err(_) => (
                false,
                0,
                0,
                false,
                vec![format!("{evidence_base}/detector-error.json")],
            ),
        };

    // Build the declared design-integrity check and evaluate the engine-room
    // gate. Evidence references are always present (the gate fails closed on an
    // empty set).
    let check = DesignIntegrityCheck {
        schema_version: DESIGN_INTEGRITY_GATE_SCHEMA_VERSION.to_string(),
        level_id: level_id.clone(),
        scenario_id: Some(generative.provenance.brief_id.clone()),
        intent_satisfied,
        intended_length,
        over_solution_count,
        search_exhausted,
        evidence_refs,
    };
    let design_integrity = evaluate_design_integrity_check(&check);

    // Compose the design-integrity gate into the existing four-gate
    // `declared-gate-and` aggregation. The four mechanical/runtime/visual/
    // semantic gates are undeclared (neutral) for a freshly generated proposal,
    // so the aggregate reduces to the single declared design-integrity gate.
    let mut engine_room_categories = json!({
        "aggregation": {
            "operator": "declared-gate-and",
            "undeclaredGatePolicy": "neutral"
        }
    });
    compose_design_integrity_into_categories(
        &mut engine_room_categories,
        std::slice::from_ref(&design_integrity),
    );

    // The promotion decision is the existing `declared-gate-and` aggregation read
    // off the composed categories: the design-integrity gate (which embeds the
    // solver and over-solution facts) must be DECLARED and `pass`. This makes the
    // four-gate composition the explicit decision authority and fails closed —
    // absent design-integrity evidence yields no declared gate and is never
    // promotable. The four mechanical/runtime/visual/semantic gates are
    // undeclared for a static proposal and stay neutral per the existing
    // `undeclaredGatePolicy`; they are enforced later when the routed-unchanged
    // proposal flows through the existing review/apply/verification path. This
    // reads the category the existing composition already produced; it adds no
    // parallel aggregator.
    let engine_room_pass = engine_room_categories
        .get("designIntegrity")
        .map(|category| {
            category.get("declared").and_then(Value::as_bool) == Some(true)
                && category.get("status").and_then(Value::as_str) == Some("pass")
        })
        .unwrap_or(false);
    debug_assert_eq!(engine_room_pass, design_integrity.state.is_pass());

    let (outcome, eligible_proposal) = if engine_room_pass {
        (
            PromotionGuardOutcome::Promotable {
                gate_evidence_refs: design_integrity.evidence_refs.clone(),
            },
            // Route the accepted proposal into the existing path UNCHANGED. The
            // guard removes only the engine-room block; it does not approve,
            // apply, or alter the proposal's proposal-only status.
            Some(proposal.clone()),
        )
    } else {
        (
            PromotionGuardOutcome::Blocked {
                reason: format!(
                    "blocked: proposal failed the engine room ({}): {}",
                    gate_state_label(design_integrity.state),
                    design_integrity.reason
                ),
                evidence_refs: design_integrity.evidence_refs.clone(),
            },
            None,
        )
    };

    Ok(PromotionGuardVerdict {
        schema_version: GENERATIVE_PROMOTION_GUARD_SCHEMA_VERSION.to_string(),
        proposal_id: proposal.id.clone(),
        brief_id: generative.provenance.brief_id.clone(),
        level_id,
        design_integrity,
        engine_room_categories,
        outcome,
        eligible_proposal,
    })
}

/// A short, stable label for a non-passing engine-room gate state, used in the
/// conservative block reason.
fn gate_state_label(state: DesignIntegrityGateState) -> &'static str {
    match state {
        DesignIntegrityGateState::Pass => "pass",
        DesignIntegrityGateState::IntentUnsatisfied => "intent-unsatisfied",
        DesignIntegrityGateState::OverSolution => "over-solution",
        DesignIntegrityGateState::Inconclusive => "inconclusive",
        DesignIntegrityGateState::MalformedEvidence => "malformed-evidence",
    }
}
