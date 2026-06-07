//! Asset-QA Gate v1 (#1636).
//!
//! Part of Asset Generation and Asset-QA v1 (#1634) under #1 Era G Milestone 36.
//! The asset-QA gate is the function-specific verification gate for generated
//! visual assets. It checks four dimensions — style-consistency,
//! format/resolution validity, visual-regression vs baseline, and
//! license/provenance completeness — and **composes with** the existing
//! evaluator gates via the `declared-gate-and` aggregation
//! (`undeclaredGatePolicy: neutral`). It is **not** a new evaluator and it adds
//! **no** fifth parallel aggregator: the asset-QA gate is one additional
//! declared category ANDed with the existing gates.
//!
//! Reuse: the visual dimensions (style-consistency and visual-regression) are
//! expressed with the existing [`crate::VisualGateState`] vocabulary, reusing
//! the visual gate rather than re-implementing image comparison. A missing or
//! non-comparable baseline never silently passes — it produces an explicit
//! `insufficient-data` outcome. The gate **fails closed**: any non-`Pass`
//! outcome blocks promotion.
//!
//! Boundary: this gate evaluates evidence and emits a verdict; it performs no
//! trusted write, auto-apply, self-approval, or promotion, and it asserts
//! conformance (license/format/style-baseline/regression), never that an asset
//! is "good", on-brand-by-taste, or fun.

use crate::VisualGateState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Schema version for an asset-QA check record.
pub const ASSET_QA_GATE_SCHEMA_VERSION: &str = "ouroforge.asset-qa-gate.v1";

/// Outcome of the asset-QA gate for a single asset. Fail-closed: any non-`Pass`
/// outcome blocks promotion.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AssetQaGateState {
    /// All four dimensions pass; the asset is promotable past this gate.
    Pass,
    /// The asset does not match the declared style baseline.
    StyleInconsistent,
    /// The asset's format or resolution is invalid for its target slot.
    FormatInvalid,
    /// The asset regresses against the prior accepted baseline.
    VisualRegression,
    /// License/provenance is incomplete (missing license, attribution, or
    /// source chain).
    MissingProvenance,
    /// The asset-QA evidence is structurally malformed; the gate fails closed.
    MalformedEvidence,
    /// A baseline is missing, stale, or non-comparable; the regression or style
    /// verdict cannot be determined. Never coerced into a pass.
    InsufficientData,
}

impl AssetQaGateState {
    /// True iff this outcome permits promotion past the asset-QA gate.
    pub fn is_pass(self) -> bool {
        matches!(self, AssetQaGateState::Pass)
    }
}

/// A single asset-QA check: the declared evidence for one asset. The visual
/// dimensions reuse the [`VisualGateState`] vocabulary; the format/resolution
/// and license/provenance dimensions are declared booleans backed by the
/// existing manifest validation and provenance bundle.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetQaCheck {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    /// Style-consistency vs the declared style baseline (reuses the visual
    /// gate). `Pass` = in-style; any non-`Pass` blocks.
    #[serde(rename = "styleConsistency")]
    pub style_consistency: VisualGateState,
    /// Visual-regression vs the prior accepted baseline (reuses the visual
    /// gate). `Pass` = no regression; `Fail` = regression; a missing/stale
    /// baseline = insufficient-data.
    #[serde(rename = "visualRegression")]
    pub visual_regression: VisualGateState,
    /// Whether format and resolution are valid for the target manifest slot.
    #[serde(rename = "formatResolutionValid")]
    pub format_resolution_valid: bool,
    /// Whether license/provenance is complete (license, required attribution,
    /// and source chain present and verifiable).
    #[serde(rename = "licenseProvenanceComplete")]
    pub license_provenance_complete: bool,
    /// Evidence references backing this check (visual gate verdict, `compare`
    /// artifact, provenance bundle). A check with no evidence is malformed.
    #[serde(rename = "evidenceRefs", default)]
    pub evidence_refs: Vec<String>,
}

impl AssetQaCheck {
    /// Parse a check from JSON, failing closed on malformed JSON.
    pub fn from_json_str(text: &str) -> anyhow::Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow::anyhow!("asset-QA check is not valid JSON: {err}"))
    }
}

/// The asset-QA gate verdict for a single asset.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AssetQaGateVerdict {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub state: AssetQaGateState,
    pub reason: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

/// Classify a visual dimension: `None` means the dimension passes; `Some(state)`
/// is the fail-closed outcome. A non-comparable baseline maps to
/// `InsufficientData`, never a silent pass.
fn classify_visual(value: VisualGateState, regression: bool) -> Option<AssetQaGateState> {
    match value {
        VisualGateState::Pass => None,
        VisualGateState::Fail => Some(if regression {
            AssetQaGateState::VisualRegression
        } else {
            AssetQaGateState::StyleInconsistent
        }),
        // Missing/stale/unsupported baseline: cannot determine, fail closed as
        // insufficient-data.
        VisualGateState::MissingBaseline
        | VisualGateState::MissingScreenshot
        | VisualGateState::UnsupportedFormat
        | VisualGateState::ThresholdNotDeclared
        | VisualGateState::StaleRef => Some(AssetQaGateState::InsufficientData),
    }
}

/// Evaluate a single asset-QA check, failing closed. Evaluation order:
/// malformed evidence, then license/provenance, then format/resolution, then
/// style-consistency, then visual-regression. The first failing dimension
/// determines the verdict.
pub fn evaluate_asset_qa_check(check: &AssetQaCheck) -> AssetQaGateVerdict {
    let (state, reason) = if check.schema_version != ASSET_QA_GATE_SCHEMA_VERSION {
        (
            AssetQaGateState::MalformedEvidence,
            format!("asset-QA check schemaVersion must be \"{ASSET_QA_GATE_SCHEMA_VERSION}\""),
        )
    } else if check.asset_id.trim().is_empty() {
        (
            AssetQaGateState::MalformedEvidence,
            "asset-QA check has an empty assetId".to_string(),
        )
    } else if check.evidence_refs.is_empty() {
        (
            AssetQaGateState::MalformedEvidence,
            "asset-QA check has no evidence references".to_string(),
        )
    } else if !check.license_provenance_complete {
        (
            AssetQaGateState::MissingProvenance,
            "license/provenance is incomplete; no unlicensed asset is promoted".to_string(),
        )
    } else if !check.format_resolution_valid {
        (
            AssetQaGateState::FormatInvalid,
            "asset format/resolution is invalid for its target slot".to_string(),
        )
    } else if let Some(style_state) = classify_visual(check.style_consistency, false) {
        let reason = match style_state {
            AssetQaGateState::StyleInconsistent => {
                "asset does not match the declared style baseline".to_string()
            }
            _ => "style-consistency baseline is missing or non-comparable".to_string(),
        };
        (style_state, reason)
    } else if let Some(regression_state) = classify_visual(check.visual_regression, true) {
        let reason = match regression_state {
            AssetQaGateState::VisualRegression => {
                "asset regresses against the prior accepted baseline".to_string()
            }
            _ => "visual-regression baseline is missing or non-comparable".to_string(),
        };
        (regression_state, reason)
    } else {
        (
            AssetQaGateState::Pass,
            "asset-QA checks pass: style-consistent, valid format/resolution, no regression, license/provenance complete".to_string(),
        )
    };

    AssetQaGateVerdict {
        asset_id: check.asset_id.clone(),
        state,
        reason,
        evidence_refs: check.evidence_refs.clone(),
    }
}

/// Evaluate a set of asset-QA checks, one verdict per check.
pub fn evaluate_asset_qa_gate(checks: &[AssetQaCheck]) -> Vec<AssetQaGateVerdict> {
    checks.iter().map(evaluate_asset_qa_check).collect()
}

/// Build the `assetQa` gate category in the existing gate vocabulary
/// (`declared`/`status`/`resultCount`/`failureCount`). Returns `None` when no
/// asset-QA checks were declared — an undeclared gate stays neutral and the
/// caller omits the key, preserving `undeclaredGatePolicy: neutral`.
pub fn asset_qa_gate_category(verdicts: &[AssetQaGateVerdict]) -> Option<Value> {
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

/// Compose the asset-QA gate into an existing gate-categories object (as
/// produced by [`crate::evaluation_gate_categories`]). The `assetQa` category is
/// ANDed with the existing gates under the same `declared-gate-and` aggregation.
/// Returns `true` if the category was added (asset-QA was declared), `false`
/// when undeclared (neutral) or the categories value is not an object.
pub fn compose_asset_qa_into_categories(
    categories: &mut Value,
    verdicts: &[AssetQaGateVerdict],
) -> bool {
    let Some(category) = asset_qa_gate_category(verdicts) else {
        return false;
    };
    if let Some(object) = categories.as_object_mut() {
        object.insert("assetQa".to_string(), category);
        true
    } else {
        false
    }
}
