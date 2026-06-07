//! Design-Integrity Gate v1 (#1583).
//!
//! Part of Puzzle Solver and Over-Solution Detection v1 (#1579) under #1 Era F
//! Milestone 28. The design-integrity gate is the moat verification for authored
//! grid-puzzle levels: solvability is table stakes, but a level only has design
//! integrity when it has *exactly* its intended solution — **intent is satisfied
//! AND no unintended over-solution exists** within the declared bound.
//!
//! It **composes with** the existing evaluator gates via the `declared-gate-and`
//! aggregation (`undeclaredGatePolicy: neutral`). It is **not** a new evaluator
//! and it adds **no** parallel aggregator: the design-integrity gate is one
//! additional declared category ANDed with the existing mechanical, runtime,
//! visual, and semantic gates.
//!
//! Reuse and boundary: the intent-satisfaction and over-solution facts are
//! produced by the Rust/local over-solution detector
//! (`ouroforge_core::puzzle_oversolution`, which reuses the deterministic
//! solver) and written as replayable evidence. `ouroforge-core` depends on this
//! evaluator crate, so the detector cannot live here; instead the gate consumes
//! the detector's declared result evidence and emits a verdict — exactly as the
//! visual gate consumes a precomputed `compare` artifact rather than
//! re-implementing image diffing. This gate evaluates evidence and emits a
//! verdict; it performs no trusted write, auto-apply, self-approval, promotion,
//! or auto-fix of detected over-solutions. It asserts design integrity against
//! the declared bound; it never claims a level is "good", balanced, or fun.
//!
//! The gate **fails closed**: any non-`Pass` outcome blocks. A bounded search
//! that was exhausted before the shorter-solution space was fully explored is
//! reported as `Inconclusive`, never coerced into a pass.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Schema version for a design-integrity check record.
pub const DESIGN_INTEGRITY_GATE_SCHEMA_VERSION: &str = "ouroforge.design-integrity-gate.v1";

/// Outcome of the design-integrity gate for a single level. Fail-closed: any
/// non-`Pass` outcome blocks.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DesignIntegrityGateState {
    /// Intent is satisfied and no over-solution exists within a fully explored
    /// bound; the level has design integrity past this gate.
    Pass,
    /// The captured intended solution does not reach the win state on the real
    /// stepper; the design is not even internally satisfiable as declared.
    IntentUnsatisfied,
    /// At least one unintended solution shorter than the intended one exists; the
    /// level can be bypassed.
    OverSolution,
    /// The detector's bounded search was exhausted before the shorter-solution
    /// space was fully explored and surfaced no over-solution; absence cannot be
    /// concluded. Never coerced into a pass.
    Inconclusive,
    /// The design-integrity evidence is structurally malformed; the gate fails
    /// closed.
    MalformedEvidence,
}

impl DesignIntegrityGateState {
    /// True iff this outcome permits passing the design-integrity gate.
    pub fn is_pass(self) -> bool {
        matches!(self, DesignIntegrityGateState::Pass)
    }
}

/// A single design-integrity check: the declared evidence for one authored
/// level. The facts (`intent_satisfied`, `over_solution_count`,
/// `search_exhausted`, `intended_length`) are produced by the Rust/local
/// over-solution detector and written as evidence; this gate composes them into
/// a declared verdict. The `evidence_refs` link to the detector report and the
/// replayable counterexample traces so any reviewer can replay a bypass.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DesignIntegrityCheck {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "levelId")]
    pub level_id: String,
    /// Optional scenario binding, carried through for evidence linkage.
    #[serde(
        rename = "scenarioId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_id: Option<String>,
    /// Whether the captured intended solution was validated to reach the win
    /// state on the real stepper.
    #[serde(rename = "intentSatisfied")]
    pub intent_satisfied: bool,
    /// Length of the captured intended solution, in steps. A check with no
    /// captured intent (`0`) is malformed.
    #[serde(rename = "intendedLength")]
    pub intended_length: usize,
    /// Count of distinct over-solutions (winning solutions strictly shorter than
    /// the intended one) the detector surfaced. `0` = none within the explored
    /// bound.
    #[serde(rename = "overSolutionCount")]
    pub over_solution_count: usize,
    /// Whether the detector's bounded search hit its budget before fully
    /// exploring the shorter-solution space. When `true` and no over-solution was
    /// found, absence is inconclusive — fail closed.
    #[serde(rename = "searchExhausted", default)]
    pub search_exhausted: bool,
    /// Evidence references backing this check (detector report, counterexample
    /// traces, captured-intent artifact). A check with no evidence is malformed.
    #[serde(rename = "evidenceRefs", default)]
    pub evidence_refs: Vec<String>,
}

impl DesignIntegrityCheck {
    /// Parse a check from JSON, failing closed on malformed JSON.
    pub fn from_json_str(text: &str) -> anyhow::Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow::anyhow!("design-integrity check is not valid JSON: {err}"))
    }
}

/// The design-integrity gate verdict for a single level.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DesignIntegrityGateVerdict {
    #[serde(rename = "levelId")]
    pub level_id: String,
    pub state: DesignIntegrityGateState,
    pub reason: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

/// Evaluate a single design-integrity check, failing closed. Evaluation order:
/// malformed evidence, then intent-satisfaction, then over-solution absence,
/// then search-exhaustion. The first failing dimension determines the verdict —
/// an unsatisfiable intent is reported before any over-solution, and a fully
/// explored clean search is required before `Pass`.
pub fn evaluate_design_integrity_check(check: &DesignIntegrityCheck) -> DesignIntegrityGateVerdict {
    let (state, reason) = if check.schema_version != DESIGN_INTEGRITY_GATE_SCHEMA_VERSION {
        (
            DesignIntegrityGateState::MalformedEvidence,
            format!(
                "design-integrity check schemaVersion must be \"{DESIGN_INTEGRITY_GATE_SCHEMA_VERSION}\""
            ),
        )
    } else if check.level_id.trim().is_empty() {
        (
            DesignIntegrityGateState::MalformedEvidence,
            "design-integrity check has an empty levelId".to_string(),
        )
    } else if check.evidence_refs.is_empty() {
        (
            DesignIntegrityGateState::MalformedEvidence,
            "design-integrity check has no evidence references".to_string(),
        )
    } else if check.intended_length == 0 {
        (
            DesignIntegrityGateState::MalformedEvidence,
            "design-integrity check has no captured intended solution".to_string(),
        )
    } else if !check.intent_satisfied {
        (
            DesignIntegrityGateState::IntentUnsatisfied,
            "captured intended solution does not reach the win state".to_string(),
        )
    } else if check.over_solution_count > 0 {
        (
            DesignIntegrityGateState::OverSolution,
            format!(
                "{} unintended over-solution(s) bypass the intended solution",
                check.over_solution_count
            ),
        )
    } else if check.search_exhausted {
        (
            DesignIntegrityGateState::Inconclusive,
            "over-solution search was exhausted before the shorter-solution space was fully explored; absence is inconclusive".to_string(),
        )
    } else {
        (
            DesignIntegrityGateState::Pass,
            "design integrity holds: intent satisfied and no over-solution within the explored bound".to_string(),
        )
    };

    DesignIntegrityGateVerdict {
        level_id: check.level_id.clone(),
        state,
        reason,
        evidence_refs: check.evidence_refs.clone(),
    }
}

/// Evaluate a set of design-integrity checks, one verdict per check.
pub fn evaluate_design_integrity_gate(
    checks: &[DesignIntegrityCheck],
) -> Vec<DesignIntegrityGateVerdict> {
    checks.iter().map(evaluate_design_integrity_check).collect()
}

/// Build the `designIntegrity` gate category in the existing gate vocabulary
/// (`declared`/`status`/`resultCount`/`failureCount`). Returns `None` when no
/// design-integrity checks were declared — an undeclared gate stays neutral and
/// the caller omits the key, preserving `undeclaredGatePolicy: neutral`.
pub fn design_integrity_gate_category(verdicts: &[DesignIntegrityGateVerdict]) -> Option<Value> {
    if verdicts.is_empty() {
        return None;
    }
    let failed = verdicts.iter().filter(|v| !v.state.is_pass()).count();
    Some(json!({
        "declared": true,
        "status": if failed == 0 { "pass" } else { "fail" },
        "resultCount": verdicts.len(),
        "failureCount": failed
    }))
}

/// Compose the design-integrity gate into an existing gate-categories object (as
/// produced by [`crate::evaluation_gate_categories`]). The `designIntegrity`
/// category is ANDed with the existing gates under the same `declared-gate-and`
/// aggregation. Returns `true` if the category was added (design-integrity was
/// declared), `false` when undeclared (neutral) or the categories value is not
/// an object.
pub fn compose_design_integrity_into_categories(
    categories: &mut Value,
    verdicts: &[DesignIntegrityGateVerdict],
) -> bool {
    let Some(category) = design_integrity_gate_category(verdicts) else {
        return false;
    };
    if let Some(object) = categories.as_object_mut() {
        object.insert("designIntegrity".to_string(), category);
        true
    } else {
        false
    }
}
