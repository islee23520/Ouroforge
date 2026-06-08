//! Whole-Game Difficulty-Curve Verification v1 (#1651).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. This module authors and verifies a **whole-game** difficulty
//! curve across an ordered campaign — not only per-level — by reading existing
//! per-stage measurements and checking the realized curve against declared
//! tolerances.
//!
//! Reuse, not a new engine: each stage's scalar difficulty is *derived* from
//! existing evidence — the Milestone 28 difficulty metric
//! ([`crate::puzzle_difficulty_metric::DifficultyMetric`], grid-puzzle solution
//! length) or the Milestone 32 balance report (`ouroforge.balance-report.v1`,
//! win rate and average turns). This module introduces no simulation, solver, or
//! balance engine; it aggregates existing measurements into a curve and verifies
//! its shape.
//!
//! The verification is descriptive: a "spike" is a jump larger than the declared
//! tolerance, a "regression" is a drop larger than the declared tolerance.
//! Neither is a fun, quality, or balance guarantee — they are measurements
//! against declared, evidence-backed thresholds. Missing or malformed evidence
//! for a declared stage fails closed.

use crate::puzzle_difficulty_metric::DifficultyMetric;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Schema version of the curve-verification input document.
pub const DIFFICULTY_CURVE_INPUT_SCHEMA: &str = "ouroforge.difficulty-curve-input.v1";
/// Schema version of the produced curve report.
pub const DIFFICULTY_CURVE_REPORT_SCHEMA: &str = "ouroforge.difficulty-curve.v1";
/// Schema the Milestone 32 balance report must declare to be read as evidence.
pub const BALANCE_REPORT_SCHEMA: &str = "ouroforge.balance-report.v1";

/// Source label: difficulty derived from a Milestone 28 difficulty metric.
pub const SOURCE_M28_DIFFICULTY: &str = "m28-difficulty";
/// Source label: difficulty derived from a Milestone 32 balance report.
pub const SOURCE_M32_BALANCE: &str = "m32-balance";

/// Weight applied to the inverse win-rate term when deriving balance difficulty.
const BALANCE_LOSS_WEIGHT: f64 = 10.0;

/// Per-stage evidence declared in the curve input, in campaign order.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CurveStageInput {
    #[serde(rename = "stageId")]
    pub stage_id: String,
    /// One of [`SOURCE_M28_DIFFICULTY`] or [`SOURCE_M32_BALANCE`].
    pub source: String,
    /// Milestone 28 difficulty-metric fields (required when `source` is
    /// `m28-difficulty`).
    #[serde(
        rename = "difficultyMetric",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub difficulty_metric: Option<Value>,
    /// Milestone 32 balance report (required when `source` is `m32-balance`).
    #[serde(
        rename = "balanceReport",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_report: Option<Value>,
}

/// Declared tolerances for the whole-game curve.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CurveTolerances {
    /// Maximum allowed increase between consecutive stages before it is flagged
    /// as a spike.
    pub spike: f64,
    /// Maximum allowed decrease between consecutive stages before it is flagged
    /// as a regression.
    pub regression: f64,
}

/// The curve-verification input document.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CurveInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    pub tolerances: CurveTolerances,
    pub stages: Vec<CurveStageInput>,
}

impl CurveInput {
    /// Parse a curve input from JSON, failing closed on malformed JSON.
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("difficulty-curve input is not valid JSON: {err}"))
    }
}

/// A realized stage on the verified curve.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CurvePoint {
    #[serde(rename = "stageId")]
    pub stage_id: String,
    pub source: String,
    pub difficulty: f64,
}

/// A flagged deviation from a monotonic-enough curve.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CurveFinding {
    /// `"spike"` or `"regression"`.
    pub kind: String,
    #[serde(rename = "fromStage")]
    pub from_stage: String,
    #[serde(rename = "toStage")]
    pub to_stage: String,
    /// The signed difficulty delta (`to - from`).
    pub delta: f64,
    /// The tolerance the delta exceeded.
    pub tolerance: f64,
}

/// The whole-game difficulty-curve verification report.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CurveReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    #[serde(rename = "stageCount")]
    pub stage_count: usize,
    /// The realized curve, in campaign order.
    pub curve: Vec<CurvePoint>,
    pub findings: Vec<CurveFinding>,
    pub spikes: usize,
    pub regressions: usize,
    /// True iff there are no spikes or regressions: the curve is monotonic-enough
    /// within the declared tolerances.
    pub passed: bool,
}

/// Derive a per-stage difficulty scalar from a Milestone 28 difficulty metric.
/// Uses the solution-length signal (longer solution = harder); the other metric
/// fields are available for future refinement.
pub fn difficulty_from_metric(metric: &DifficultyMetric) -> f64 {
    metric.solution_length as f64
}

/// Parse the Milestone 28 difficulty-metric fields from the input JSON into the
/// existing [`DifficultyMetric`] type, failing closed on missing fields.
fn metric_from_value(value: &Value) -> Result<DifficultyMetric> {
    let solution_length = value
        .get("solutionLength")
        .and_then(Value::as_u64)
        .ok_or_else(|| anyhow!("difficulty metric requires a non-negative solutionLength"))?
        as usize;
    let branching_factor = value
        .get("branchingFactor")
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let dead_end_density = value
        .get("deadEndDensity")
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let reachable_states = value
        .get("reachableStates")
        .and_then(Value::as_u64)
        .unwrap_or(0) as usize;
    Ok(DifficultyMetric {
        solution_length,
        branching_factor,
        dead_end_density,
        mechanic_introduction_order: Vec::new(),
        reachable_states,
    })
}

/// Derive a per-stage difficulty scalar from a Milestone 32 balance report.
/// Harder stages are those the synthetic players win less often and take more
/// turns to clear: `avg_turns + (1 - win_rate) * BALANCE_LOSS_WEIGHT`. Fails
/// closed on a malformed report.
pub fn difficulty_from_balance_report(report: &Value) -> Result<f64> {
    if report.get("schemaVersion").and_then(Value::as_str) != Some(BALANCE_REPORT_SCHEMA) {
        return Err(anyhow!(
            "balance report must declare schemaVersion \"{BALANCE_REPORT_SCHEMA}\""
        ));
    }
    let win = report
        .pointer("/winRate/wins")
        .and_then(Value::as_u64)
        .ok_or_else(|| anyhow!("balance report requires winRate.wins"))?;
    let total = report
        .pointer("/winRate/total")
        .and_then(Value::as_u64)
        .filter(|t| *t > 0)
        .ok_or_else(|| anyhow!("balance report requires a positive winRate.total"))?;
    if win > total {
        return Err(anyhow!(
            "balance report winRate.wins ({win}) must not exceed winRate.total ({total})"
        ));
    }
    let curve = report
        .get("difficultyCurve")
        .and_then(Value::as_array)
        .filter(|c| !c.is_empty())
        .ok_or_else(|| anyhow!("balance report requires a non-empty difficultyCurve"))?;
    let mut turns_total = 0u64;
    for entry in curve {
        let turns = entry
            .get("turns")
            .and_then(Value::as_u64)
            .ok_or_else(|| anyhow!("balance report difficultyCurve entry requires turns"))?;
        turns_total += turns;
    }
    let avg_turns = turns_total as f64 / curve.len() as f64;
    let win_rate = win as f64 / total as f64;
    Ok(avg_turns + (1.0 - win_rate) * BALANCE_LOSS_WEIGHT)
}

/// Derive the scalar difficulty for one declared stage from its evidence,
/// failing closed when the evidence for the declared source is missing or
/// malformed.
fn stage_difficulty(stage: &CurveStageInput) -> Result<f64> {
    match stage.source.as_str() {
        SOURCE_M28_DIFFICULTY => {
            let value = stage.difficulty_metric.as_ref().ok_or_else(|| {
                anyhow!(
                    "stage \"{}\" declares source {SOURCE_M28_DIFFICULTY} but has no difficultyMetric evidence",
                    stage.stage_id
                )
            })?;
            let metric = metric_from_value(value)
                .map_err(|err| anyhow!("stage \"{}\": {err}", stage.stage_id))?;
            Ok(difficulty_from_metric(&metric))
        }
        SOURCE_M32_BALANCE => {
            let report = stage.balance_report.as_ref().ok_or_else(|| {
                anyhow!(
                    "stage \"{}\" declares source {SOURCE_M32_BALANCE} but has no balanceReport evidence",
                    stage.stage_id
                )
            })?;
            difficulty_from_balance_report(report)
                .map_err(|err| anyhow!("stage \"{}\": {err}", stage.stage_id))
        }
        other => Err(anyhow!(
            "stage \"{}\" has unsupported difficulty source \"{other}\"",
            stage.stage_id
        )),
    }
}

/// Verify a whole-game difficulty curve from a curve-input document, deriving
/// each stage's difficulty from existing M28/M32 evidence and flagging spikes
/// and regressions against the declared tolerances. Fails closed on a malformed
/// input, an unsupported source, or missing/malformed stage evidence.
pub fn verify_curve(input: &CurveInput) -> Result<CurveReport> {
    if input.schema_version != DIFFICULTY_CURVE_INPUT_SCHEMA {
        return Err(anyhow!(
            "difficulty-curve input schemaVersion must be \"{DIFFICULTY_CURVE_INPUT_SCHEMA}\""
        ));
    }
    crate::require_text("difficulty-curve input campaignId", &input.campaign_id)?;
    if input.tolerances.spike < 0.0 || input.tolerances.regression < 0.0 {
        return Err(anyhow!("difficulty-curve tolerances must be non-negative"));
    }
    if input.stages.is_empty() {
        return Err(anyhow!(
            "difficulty-curve input must declare at least one stage"
        ));
    }

    let mut curve = Vec::with_capacity(input.stages.len());
    for stage in &input.stages {
        let difficulty = stage_difficulty(stage)?;
        curve.push(CurvePoint {
            stage_id: stage.stage_id.clone(),
            source: stage.source.clone(),
            difficulty,
        });
    }

    let mut findings = Vec::new();
    for window in curve.windows(2) {
        let (prev, next) = (&window[0], &window[1]);
        let delta = next.difficulty - prev.difficulty;
        if delta > input.tolerances.spike {
            findings.push(CurveFinding {
                kind: "spike".to_string(),
                from_stage: prev.stage_id.clone(),
                to_stage: next.stage_id.clone(),
                delta,
                tolerance: input.tolerances.spike,
            });
        } else if -delta > input.tolerances.regression {
            findings.push(CurveFinding {
                kind: "regression".to_string(),
                from_stage: prev.stage_id.clone(),
                to_stage: next.stage_id.clone(),
                delta,
                tolerance: input.tolerances.regression,
            });
        }
    }

    let spikes = findings.iter().filter(|f| f.kind == "spike").count();
    let regressions = findings.iter().filter(|f| f.kind == "regression").count();
    Ok(CurveReport {
        schema_version: DIFFICULTY_CURVE_REPORT_SCHEMA.to_string(),
        campaign_id: input.campaign_id.clone(),
        stage_count: curve.len(),
        curve,
        passed: findings.is_empty(),
        spikes,
        regressions,
        findings,
    })
}
