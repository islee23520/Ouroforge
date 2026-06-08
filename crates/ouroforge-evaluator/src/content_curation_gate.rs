//! Content Curation Gate v1 (#1652).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. The curation gate is the **campaign-level promotion guard**: a
//! campaign is admitted only when every level is solvable, the campaign is
//! balanced, it clears the novelty threshold, and its whole-game difficulty
//! curve is verified — and only when all four evidence dimensions are declared.
//!
//! It **composes with** the existing evaluator gates via the `declared-gate-and`
//! aggregation (`undeclaredGatePolicy: neutral`). It is **not** a new evaluator
//! and adds **no** parallel aggregator: the curation gate is one additional
//! declared category ANDed with the existing mechanical, runtime, visual,
//! semantic, design-integrity, and asset-QA gates.
//!
//! Reuse and boundary: the solvability, balance, novelty, and curve facts are
//! produced by the existing Rust/local surfaces — the engine-room solver
//! (Milestone 28), synthetic-player balance telemetry (Milestone 32), the
//! content-novelty metrics (`ouroforge_core::content_novelty`, #1650), and the
//! whole-game difficulty-curve verifier
//! (`ouroforge_core::content_difficulty_curve`, #1651). `ouroforge-core` depends
//! on this evaluator crate, so those producers cannot live here; instead the gate
//! consumes their declared result evidence and emits a verdict — exactly as the
//! visual gate consumes a precomputed `compare` artifact and the design-integrity
//! gate consumes the over-solution detector's report. This gate evaluates
//! evidence and emits a verdict; it performs no trusted write, auto-apply,
//! self-approval, or promotion. It never claims a campaign is "good", balanced in
//! a subjective sense, or fun.
//!
//! The gate **fails closed**: any non-`Pass` outcome blocks, and malformed or
//! missing evidence — including a missing solver, balance, novelty, or curve
//! evidence dimension — is never coerced into a pass.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Schema version for a content-curation check record.
pub const CONTENT_CURATION_GATE_SCHEMA_VERSION: &str = "ouroforge.content-curation-gate.v1";

/// Outcome of the curation gate for a single campaign. Fail-closed: any
/// non-`Pass` outcome blocks.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ContentCurationGateState {
    /// Every level is solvable, the campaign is balanced, it clears the novelty
    /// threshold, and the difficulty curve is verified.
    Pass,
    /// At least one level is not solvable (engine-room solver evidence).
    Unsolvable,
    /// The campaign is not balanced (Milestone 32 balance telemetry).
    Imbalanced,
    /// The admitted content falls below the declared novelty threshold (#1650).
    LowNovelty,
    /// The whole-game difficulty curve has an unresolved spike/regression (#1651).
    CurveViolation,
    /// The curation evidence is structurally malformed or a required evidence
    /// dimension is missing; the gate fails closed.
    MalformedEvidence,
}

impl ContentCurationGateState {
    /// True iff this outcome permits passing the curation gate.
    pub fn is_pass(self) -> bool {
        matches!(self, ContentCurationGateState::Pass)
    }
}

/// The four required evidence dimensions for a curation check. Each must be
/// declared with at least one reference; a campaign cannot be admitted without
/// solver, balance, novelty, and curve evidence.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CurationEvidence {
    /// Engine-room solver evidence (Milestone 28).
    #[serde(default)]
    pub solver: Vec<String>,
    /// Balance telemetry evidence (Milestone 32).
    #[serde(default)]
    pub balance: Vec<String>,
    /// Novelty metric evidence (#1650).
    #[serde(default)]
    pub novelty: Vec<String>,
    /// Whole-game difficulty-curve evidence (#1651).
    #[serde(default)]
    pub curve: Vec<String>,
}

impl CurationEvidence {
    /// The first missing dimension (empty ref list), in a fixed order, if any.
    fn missing_dimension(&self) -> Option<&'static str> {
        if self.solver.is_empty() {
            Some("solver")
        } else if self.balance.is_empty() {
            Some("balance")
        } else if self.novelty.is_empty() {
            Some("novelty")
        } else if self.curve.is_empty() {
            Some("curve")
        } else {
            None
        }
    }

    /// All evidence references across the four dimensions, in dimension order.
    pub fn all_refs(&self) -> Vec<String> {
        let mut refs = Vec::new();
        refs.extend(self.solver.iter().cloned());
        refs.extend(self.balance.iter().cloned());
        refs.extend(self.novelty.iter().cloned());
        refs.extend(self.curve.iter().cloned());
        refs
    }
}

/// A single curation check: the declared evidence for one campaign. The facts
/// are summaries produced by the existing solver/balance/novelty/curve surfaces;
/// this gate composes them into a declared verdict. The `evidence` block must
/// declare a reference for each of the four dimensions so any reviewer can replay
/// them.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContentCurationCheck {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    /// Total number of levels in the campaign. A campaign with no levels is
    /// malformed.
    #[serde(rename = "levelsTotal")]
    pub levels_total: usize,
    /// Number of levels the engine-room solver demonstrated solvable. Must not
    /// exceed `levels_total`.
    #[serde(rename = "levelsSolvable")]
    pub levels_solvable: usize,
    /// Whether the Milestone 32 balance telemetry judged the campaign balanced.
    pub balanced: bool,
    /// Whether the #1650 novelty metric flagged the set as low-novelty.
    #[serde(rename = "lowNovelty")]
    pub low_novelty: bool,
    /// Whether the #1651 whole-game difficulty curve was verified (no unresolved
    /// spike/regression).
    #[serde(rename = "curvePassed")]
    pub curve_passed: bool,
    /// Count of curve findings (spikes + regressions); carried for the reason
    /// text. Defaults to `0`.
    #[serde(rename = "curveFindingCount", default)]
    pub curve_finding_count: usize,
    /// Evidence references for the four required dimensions. A check missing any
    /// dimension is malformed and fails closed.
    pub evidence: CurationEvidence,
}

impl ContentCurationCheck {
    /// Parse a check from JSON, failing closed on malformed JSON.
    pub fn from_json_str(text: &str) -> anyhow::Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow::anyhow!("content-curation check is not valid JSON: {err}"))
    }
}

/// The curation gate verdict for a single campaign.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContentCurationGateVerdict {
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    pub state: ContentCurationGateState,
    pub reason: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

/// Evaluate a single curation check, failing closed. Evaluation order: malformed
/// evidence (schema, campaign id, a missing evidence dimension, level counts),
/// then solvability, then balance, then novelty, then curve. The first failing
/// dimension determines the verdict.
pub fn evaluate_content_curation_check(check: &ContentCurationCheck) -> ContentCurationGateVerdict {
    let (state, reason) = if check.schema_version != CONTENT_CURATION_GATE_SCHEMA_VERSION {
        (
            ContentCurationGateState::MalformedEvidence,
            format!(
                "content-curation check schemaVersion must be \"{CONTENT_CURATION_GATE_SCHEMA_VERSION}\""
            ),
        )
    } else if check.campaign_id.trim().is_empty() {
        (
            ContentCurationGateState::MalformedEvidence,
            "content-curation check has an empty campaignId".to_string(),
        )
    } else if let Some(dimension) = check.evidence.missing_dimension() {
        (
            ContentCurationGateState::MalformedEvidence,
            format!("content-curation check is missing {dimension} evidence"),
        )
    } else if check.levels_total == 0 {
        (
            ContentCurationGateState::MalformedEvidence,
            "content-curation check declares no levels".to_string(),
        )
    } else if check.levels_solvable > check.levels_total {
        (
            ContentCurationGateState::MalformedEvidence,
            format!(
                "content-curation check levelsSolvable ({}) exceeds levelsTotal ({})",
                check.levels_solvable, check.levels_total
            ),
        )
    } else if check.levels_solvable < check.levels_total {
        (
            ContentCurationGateState::Unsolvable,
            format!(
                "{} of {} levels are not solvable",
                check.levels_total - check.levels_solvable,
                check.levels_total
            ),
        )
    } else if !check.balanced {
        (
            ContentCurationGateState::Imbalanced,
            "campaign balance telemetry did not judge the campaign balanced".to_string(),
        )
    } else if check.low_novelty {
        (
            ContentCurationGateState::LowNovelty,
            "admitted content falls below the declared novelty threshold".to_string(),
        )
    } else if !check.curve_passed {
        (
            ContentCurationGateState::CurveViolation,
            format!(
                "whole-game difficulty curve has {} unresolved spike/regression finding(s)",
                check.curve_finding_count
            ),
        )
    } else {
        (
            ContentCurationGateState::Pass,
            "campaign curated: all levels solvable, balanced, novel, and curve verified"
                .to_string(),
        )
    };

    ContentCurationGateVerdict {
        campaign_id: check.campaign_id.clone(),
        state,
        reason,
        evidence_refs: check.evidence.all_refs(),
    }
}

/// Evaluate a set of curation checks, one verdict per check.
pub fn evaluate_content_curation_gate(
    checks: &[ContentCurationCheck],
) -> Vec<ContentCurationGateVerdict> {
    checks.iter().map(evaluate_content_curation_check).collect()
}

/// Build the `contentCuration` gate category in the existing gate vocabulary
/// (`declared`/`status`/`resultCount`/`failureCount`). Returns `None` when no
/// curation checks were declared — an undeclared gate stays neutral and the
/// caller omits the key, preserving `undeclaredGatePolicy: neutral`.
pub fn content_curation_gate_category(verdicts: &[ContentCurationGateVerdict]) -> Option<Value> {
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

/// Compose the curation gate into an existing gate-categories object. The
/// `contentCuration` category is ANDed with the existing gates under the same
/// `declared-gate-and` aggregation. Returns `true` if the category was added
/// (curation was declared), `false` when undeclared (neutral) or the categories
/// value is not an object.
pub fn compose_content_curation_into_categories(
    categories: &mut Value,
    verdicts: &[ContentCurationGateVerdict],
) -> bool {
    let Some(category) = content_curation_gate_category(verdicts) else {
        return false;
    };
    if let Some(object) = categories.as_object_mut() {
        object.insert("contentCuration".to_string(), category);
        true
    } else {
        false
    }
}
