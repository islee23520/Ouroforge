//! Design Regression Harness model and diff v1 (#1588).
//!
//! This is the trusted Rust/local owner of the design-regression harness for
//! Era F Milestone 29 (`docs/design-regression-harness-v1.md`): design
//! regression as CI for game design. On a content/rule edit, it re-runs the
//! Milestone 28 solver + over-solution detector + difficulty suite across the
//! affected levels, diffs the recomputed status against the recorded baseline,
//! and classifies each level as `unchanged`, `improved`, or `newly-broken`.
//!
//! It is an *orchestration* over existing pieces, not a new comparison engine:
//!
//! - solvability and the shortest replayable witness come from the existing
//!   [`puzzle_solver`](crate::puzzle_solver) bounded search (#1580);
//! - over-solution counterexamples come from the existing
//!   [`puzzle_oversolution`](crate::puzzle_oversolution) detector (#1581);
//! - descriptive difficulty measurements come from the existing
//!   [`puzzle_difficulty_metric`](crate::puzzle_difficulty_metric) suite
//!   (#1582), computed from the solver's own witness;
//! - the outcome vocabulary (improved / unchanged / regressed) and the
//!   promotion-blocking shape mirror the existing post-apply `compare` artifact
//!   ([`source_apply_post_apply_rerun`](crate::source_apply_post_apply_rerun)),
//!   and the verdict/stop shape mirrors `evolve_campaign`.
//!
//! Every flagged regression carries a *replayable trace*: either the shortest
//! over-solution counterexample (watch the bypass) or the previously-intended
//! solution that no longer wins (watch it break). The harness never applies,
//! promotes, or auto-fixes anything; it only re-runs deterministic computation
//! and reports a verdict. Stale, exhausted, or malformed inputs fail closed —
//! reported explicitly and never as a false "clean" or a false regression.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::puzzle_difficulty_metric::{self, EVIDENCE_SCHEMA};
use crate::puzzle_oversolution;
use crate::puzzle_solver::{self, SolveBudget, SolveOutcome, DEFAULT_MAX_STATES, DIRECTIONS};

/// Schema identifier of the design-regression harness artifact.
pub const DESIGN_REGRESSION_HARNESS_SCHEMA_VERSION: &str = "design-regression-harness-v1";

/// A design-regression harness artifact: an edit under test plus the affected
/// levels (current post-edit spec + designer intent + recorded baseline status).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DesignRegressionHarness {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    /// Optional human-readable note (documentation only; ignored by the model).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Local ref to the content/rule edit under test.
    #[serde(rename = "editRef")]
    pub edit_ref: String,
    /// Untracked local root for generated re-run artifacts.
    #[serde(rename = "generatedOutputRoot")]
    pub generated_output_root: String,
    /// Optional override for the per-level bounded search budget.
    #[serde(rename = "maxStates", default, skip_serializing_if = "Option::is_none")]
    pub max_states: Option<usize>,
    pub levels: Vec<DesignRegressionLevelInput>,
    pub guardrails: Vec<String>,
}

/// One affected level: its current (post-edit) spec, the designer's captured
/// intent, and the status recorded at baseline (before the edit).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DesignRegressionLevelInput {
    #[serde(rename = "levelId")]
    pub level_id: String,
    /// The current `ouroforge.grid-puzzle.v1` spec after the edit.
    pub spec: Value,
    /// The designer intent (`intendedSolution` + optional `taughtMechanic`).
    pub intent: Value,
    pub baseline: DesignBaselineStatus,
}

/// The design status recorded for a level at baseline, before the edit.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DesignBaselineStatus {
    pub solvable: bool,
    #[serde(rename = "oversolutionCount")]
    pub oversolution_count: usize,
    /// Optional witness that proved baseline solvability. When present the
    /// harness re-validates it against the current spec; a witness that no
    /// longer wins means the baseline reference is stale and the comparison
    /// fails closed (never a false improved/unchanged classification).
    #[serde(
        rename = "evidenceWitness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evidence_witness: Option<Vec<String>>,
}

impl DesignBaselineStatus {
    fn is_clean(&self) -> bool {
        self.solvable && self.oversolution_count == 0
    }
}

/// The classified per-level outcome of the diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RegressionOutcome {
    /// Design status is the same as baseline.
    Unchanged,
    /// A previously-broken level is now design-clean.
    Improved,
    /// A previously-clean level is now broken — a regression.
    NewlyBroken,
    /// The comparison could not be decided (stale/exhausted/malformed input).
    Inconclusive,
}

impl RegressionOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Improved => "improved",
            Self::NewlyBroken => "newly-broken",
            Self::Inconclusive => "inconclusive",
        }
    }
}

/// A compact, serializable design-status summary for one side of the diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DesignStatusSummary {
    pub solvable: bool,
    #[serde(rename = "oversolutionCount")]
    pub oversolution_count: usize,
    pub clean: bool,
}

/// Descriptive difficulty measurements echoed into the report (never a quality,
/// difficulty, or fun guarantee).
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct DifficultySummary {
    #[serde(rename = "solutionLength")]
    pub solution_length: usize,
    #[serde(rename = "branchingFactor")]
    pub branching_factor: f64,
    #[serde(rename = "deadEndDensity")]
    pub dead_end_density: f64,
    #[serde(rename = "reachableStates")]
    pub reachable_states: usize,
}

/// The diff result for one affected level.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DesignRegressionLevelResult {
    #[serde(rename = "levelId")]
    pub level_id: String,
    pub outcome: RegressionOutcome,
    pub baseline: DesignStatusSummary,
    pub current: DesignStatusSummary,
    /// A replayable trace explaining a regression (present iff `newly-broken`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Vec<String>>,
    /// What the trace demonstrates: `shorter-than-intended` (an over-solution
    /// bypass) or `intended-solution-broken` (the intended path no longer wins).
    #[serde(rename = "traceKind", skip_serializing_if = "Option::is_none")]
    pub trace_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<DifficultySummary>,
    pub detail: String,
}

/// The overall verdict of a harness run.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DesignRegressionReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "editRef")]
    pub edit_ref: String,
    /// `clean`, `regressed`, or `inconclusive`.
    #[serde(rename = "overallVerdict")]
    pub overall_verdict: String,
    #[serde(rename = "regressionCount")]
    pub regression_count: usize,
    pub levels: Vec<DesignRegressionLevelResult>,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl DesignRegressionReport {
    /// True when at least one level regressed.
    pub fn has_regression(&self) -> bool {
        self.regression_count > 0
    }

    /// True when promotion/auto-apply is blocked (a regression or an
    /// inconclusive comparison requires human review and more evidence).
    pub fn promotion_blocked(&self) -> bool {
        !self.blocked_reasons.is_empty()
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize design regression report")
    }
}

impl DesignRegressionHarness {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let harness: Self = serde_json::from_str(input)
            .context("failed to parse design regression harness JSON")?;
        harness.validate()?;
        Ok(harness)
    }

    /// Validate the artifact shape and local-ref safety. Per-level computation
    /// failures are *not* validation errors — they fail closed into an
    /// `inconclusive` level result so the report is still produced.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != DESIGN_REGRESSION_HARNESS_SCHEMA_VERSION {
            return Err(anyhow!(
                "design regression harness schemaVersion must be {DESIGN_REGRESSION_HARNESS_SCHEMA_VERSION}"
            ));
        }
        require_local_ref("design regression harness editRef", &self.edit_ref)?;
        require_local_ref(
            "design regression harness generatedOutputRoot",
            &self.generated_output_root,
        )?;
        if let Some(max_states) = self.max_states {
            if max_states == 0 {
                return Err(anyhow!(
                    "design regression harness maxStates must be a positive bound"
                ));
            }
        }
        if self.levels.is_empty() {
            return Err(anyhow!(
                "design regression harness must record at least one affected level"
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for level in &self.levels {
            require_local_id("design regression harness levelId", &level.level_id)?;
            if !seen.insert(level.level_id.clone()) {
                return Err(anyhow!(
                    "design regression harness levelId \"{}\" is duplicated",
                    level.level_id
                ));
            }
        }
        if self.guardrails.is_empty() {
            return Err(anyhow!(
                "design regression harness must record at least one guardrail"
            ));
        }
        for guardrail in &self.guardrails {
            if guardrail.trim().is_empty() {
                return Err(anyhow!(
                    "design regression harness guardrails must not be empty"
                ));
            }
        }
        Ok(())
    }

    fn budget(&self) -> SolveBudget {
        SolveBudget {
            max_states: self.max_states.unwrap_or(DEFAULT_MAX_STATES),
        }
    }

    /// Re-run the solver + over-solution + difficulty suite over each affected
    /// level, diff against the recorded baseline, and classify the outcome.
    pub fn run(&self) -> Result<DesignRegressionReport> {
        self.validate()?;
        let budget = self.budget();
        let levels: Vec<DesignRegressionLevelResult> = self
            .levels
            .iter()
            .map(|level| compute_level(level, budget))
            .collect();

        let regression_count = levels
            .iter()
            .filter(|l| l.outcome == RegressionOutcome::NewlyBroken)
            .count();
        let improved_count = levels
            .iter()
            .filter(|l| l.outcome == RegressionOutcome::Improved)
            .count();
        let unchanged_count = levels
            .iter()
            .filter(|l| l.outcome == RegressionOutcome::Unchanged)
            .count();
        let inconclusive_count = levels
            .iter()
            .filter(|l| l.outcome == RegressionOutcome::Inconclusive)
            .count();

        let mut blocked = Vec::new();
        if inconclusive_count > 0 {
            blocked.push(format!(
                "design regression comparison is inconclusive for {inconclusive_count} level(s); promotion needs more evidence"
            ));
        }
        if regression_count > 0 {
            blocked.push(format!(
                "design regression detected in {regression_count} level(s); promotion blocked pending human review"
            ));
        }

        let overall_verdict = if inconclusive_count > 0 {
            "inconclusive"
        } else if regression_count > 0 {
            "regressed"
        } else {
            "clean"
        };

        let evidence_summary = vec![
            format!("edit:{}", self.edit_ref),
            format!("levels:{}", levels.len()),
            format!("newly-broken:{regression_count}"),
            format!("improved:{improved_count}"),
            format!("unchanged:{unchanged_count}"),
            format!("inconclusive:{inconclusive_count}"),
            format!("overall:{overall_verdict}"),
        ];

        Ok(DesignRegressionReport {
            schema_version: DESIGN_REGRESSION_HARNESS_SCHEMA_VERSION.to_string(),
            edit_ref: self.edit_ref.clone(),
            overall_verdict: overall_verdict.to_string(),
            regression_count,
            levels,
            evidence_summary,
            blocked_reasons: blocked,
            allowed_actions: vec![
                "inspect_regression_report".to_string(),
                "inspect_generated_runs".to_string(),
                "replay_trace".to_string(),
            ],
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "auto_apply_fix".to_string(),
                "merge_branch".to_string(),
                "execute_command".to_string(),
                "promote_without_evidence".to_string(),
                "self_approve".to_string(),
            ],
        })
    }
}

/// The recomputed current status of a level, plus the evidence needed to attach
/// a replayable trace to a regression.
struct CurrentStatus {
    solvable: bool,
    intended_wins: bool,
    oversolution_count: usize,
    shortest_oversolution: Option<(Vec<String>, String)>,
    intended_solution: Vec<String>,
    difficulty: Option<DifficultySummary>,
}

impl CurrentStatus {
    fn clean(&self) -> bool {
        self.solvable && self.intended_wins && self.oversolution_count == 0
    }

    fn summary(&self) -> DesignStatusSummary {
        DesignStatusSummary {
            solvable: self.solvable,
            oversolution_count: self.oversolution_count,
            clean: self.clean(),
        }
    }
}

fn baseline_summary(baseline: &DesignBaselineStatus) -> DesignStatusSummary {
    DesignStatusSummary {
        solvable: baseline.solvable,
        oversolution_count: baseline.oversolution_count,
        clean: baseline.is_clean(),
    }
}

/// Extract a non-empty intended-solution path of known directions, or `None`
/// when the intent is structurally malformed.
fn parse_intended_solution(intent: &Value) -> Option<Vec<String>> {
    let steps = intent.get("intendedSolution")?.as_array()?;
    if steps.is_empty() {
        return None;
    }
    let mut out = Vec::with_capacity(steps.len());
    for step in steps {
        let action = step.as_str()?;
        if !DIRECTIONS.contains(&action) {
            return None;
        }
        out.push(action.to_string());
    }
    Some(out)
}

fn compute_level(
    level: &DesignRegressionLevelInput,
    budget: SolveBudget,
) -> DesignRegressionLevelResult {
    let baseline = baseline_summary(&level.baseline);

    // A malformed current spec cannot be compared; fail closed.
    let initial = match puzzle_solver::validate_spec(&level.spec) {
        Ok(state) => state,
        Err(error) => {
            return inconclusive(level, baseline, format!("current spec is invalid: {error}"))
        }
    };

    // A stale baseline witness (one that no longer wins on the current spec)
    // means the recorded baseline can no longer be trusted; fail closed instead
    // of reporting a false improved/unchanged.
    if let Some(witness) = &level.baseline.evidence_witness {
        match puzzle_solver::replay(&level.spec, witness) {
            Ok(state) if state.is_won() => {}
            Ok(_) => {
                return inconclusive(
                    level,
                    baseline,
                    "baseline evidence witness is stale: it no longer solves the current level"
                        .to_string(),
                );
            }
            Err(error) => {
                return inconclusive(
                    level,
                    baseline,
                    format!("baseline evidence witness is invalid: {error}"),
                );
            }
        }
    }

    // The captured intent must be structurally well-formed before we can decide
    // whether it still wins.
    let intended_solution = match parse_intended_solution(&level.intent) {
        Some(path) => path,
        None => {
            return inconclusive(
                level,
                baseline,
                "captured intent is malformed: intendedSolution must be a non-empty array of grid directions"
                    .to_string(),
            );
        }
    };

    // Re-run the solver. An exhausted bound is reported explicitly and is never
    // treated as solvable/unsolvable.
    let (solvable, witness) = match puzzle_solver::search(initial, budget) {
        SolveOutcome::Solvable { witness, .. } => (true, Some(witness)),
        SolveOutcome::Unsolvable { .. } => (false, None),
        SolveOutcome::Exhausted { budget: bound, .. } => {
            return inconclusive(
                level,
                baseline,
                format!("solver exhausted its {bound}-state budget before deciding the level"),
            );
        }
    };

    // Does the captured intended solution still win on the current spec?
    let intended_wins = puzzle_solver::replay(&level.spec, &intended_solution)
        .map(|state| state.is_won())
        .unwrap_or(false);

    // Over-solutions are only meaningful when the intended solution still wins
    // (the detector measures bypasses of a valid intent).
    let mut oversolution_count = 0usize;
    let mut shortest_oversolution: Option<(Vec<String>, String)> = None;
    if intended_wins {
        match puzzle_oversolution::detect_oversolutions(&level.spec, &level.intent, budget) {
            Ok(report) => {
                if report.exhausted {
                    return inconclusive(
                        level,
                        baseline,
                        "over-solution search exhausted its budget before deciding the level"
                            .to_string(),
                    );
                }
                oversolution_count = report.counterexamples.len();
                shortest_oversolution = report
                    .counterexamples
                    .first()
                    .map(|ce| (ce.trace.clone(), ce.kind.clone()));
            }
            Err(error) => {
                return inconclusive(
                    level,
                    baseline,
                    format!("over-solution detection failed: {error}"),
                );
            }
        }
    }

    // Descriptive difficulty, computed from the solver's own witness. Best
    // effort: difficulty is descriptive only and never gates the verdict.
    let difficulty = match (&witness, intended_wins) {
        (Some(witness), true) if !witness.is_empty() => {
            let evidence = json!({
                "schemaVersion": EVIDENCE_SCHEMA,
                "witness": witness,
                "intendedSolution": intended_solution,
            });
            puzzle_difficulty_metric::compute_difficulty(&level.spec, &evidence, budget)
                .ok()
                .map(|metric| DifficultySummary {
                    solution_length: metric.solution_length,
                    branching_factor: metric.branching_factor,
                    dead_end_density: metric.dead_end_density,
                    reachable_states: metric.reachable_states,
                })
        }
        _ => None,
    };

    let current = CurrentStatus {
        solvable,
        intended_wins,
        oversolution_count,
        shortest_oversolution,
        intended_solution,
        difficulty,
    };

    classify(level, baseline, current)
}

fn classify(
    level: &DesignRegressionLevelInput,
    baseline: DesignStatusSummary,
    current: CurrentStatus,
) -> DesignRegressionLevelResult {
    let current_clean = current.clean();
    let baseline_clean = baseline.clean;
    let current_summary = current.summary();

    let (outcome, trace, trace_kind, detail) = match (baseline_clean, current_clean) {
        (true, true) => (
            RegressionOutcome::Unchanged,
            None,
            None,
            "level remains design-clean after the edit".to_string(),
        ),
        (false, true) => (
            RegressionOutcome::Improved,
            None,
            None,
            "previously-broken level is now design-clean after the edit".to_string(),
        ),
        (true, false) => {
            // Regression: attach a replayable trace. Prefer an over-solution
            // bypass; otherwise the intended path that no longer wins.
            if let Some((trace, kind)) = current.shortest_oversolution.clone() {
                (
                    RegressionOutcome::NewlyBroken,
                    Some(trace),
                    Some(kind),
                    format!(
                        "edit opened {} unintended over-solution(s); shortest bypass is replayable",
                        current.oversolution_count
                    ),
                )
            } else {
                (
                    RegressionOutcome::NewlyBroken,
                    Some(current.intended_solution.clone()),
                    Some("intended-solution-broken".to_string()),
                    "edit broke the intended solution: it no longer reaches the win state (replay it to watch it break)"
                        .to_string(),
                )
            }
        }
        (false, false) => (
            RegressionOutcome::Unchanged,
            None,
            None,
            "level was broken at baseline and remains broken after the edit".to_string(),
        ),
    };

    DesignRegressionLevelResult {
        level_id: level.level_id.clone(),
        outcome,
        baseline,
        current: current_summary,
        trace,
        trace_kind,
        difficulty: current.difficulty,
        detail,
    }
}

fn inconclusive(
    level: &DesignRegressionLevelInput,
    baseline: DesignStatusSummary,
    detail: String,
) -> DesignRegressionLevelResult {
    DesignRegressionLevelResult {
        level_id: level.level_id.clone(),
        outcome: RegressionOutcome::Inconclusive,
        baseline,
        current: DesignStatusSummary {
            solvable: false,
            oversolution_count: 0,
            clean: false,
        },
        trace: None,
        trace_kind: None,
        difficulty: None,
        detail,
    }
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 128
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, or dot"
        ));
    }
    Ok(())
}

fn require_local_ref(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} must stay inside the local trusted worktree"
        ));
    }
    Ok(())
}
