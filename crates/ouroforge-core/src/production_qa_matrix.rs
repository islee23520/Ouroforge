//! Production-Scale QA Matrix v1 (#1666) — a regression matrix over the
//! existing QA / playtest swarm and scenario-coverage runners.
//!
//! This module does not introduce a new test engine. Each matrix cell binds a
//! `content variant x seed x target` coordinate to a verdict produced by an
//! existing runner (referenced through `qaRunMatrixRef`) plus replayable
//! evidence refs. The aggregation rolls the cells up into a descriptive read
//! model and detects cross-variant regressions (a coordinate that passed for
//! the baseline variant but failed for another variant). It performs no trusted
//! mutation, auto-fix, auto-apply, or quality judgement; outputs are evidence
//! inputs only and remain review-gated.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const PRODUCTION_QA_MATRIX_SCHEMA_VERSION: &str = "production-qa-matrix-v1";

/// Verdict taxonomy reused from the QA swarm run matrix; cells are evidence
/// inputs, never trusted truth.
const VERDICTS: &[&str] = &[
    "passed",
    "failed",
    "flaky",
    "inconclusive",
    "skipped",
    "unsupported",
    "timed_out",
    "crashed",
    "missing_evidence",
];
/// Verdicts that resolve a cell without further triage.
const RESOLVED_VERDICTS: &[&str] = &["passed", "failed"];
/// Verdicts that legitimately carry no evidence refs.
const NO_EVIDENCE_VERDICTS: &[&str] = &["skipped", "unsupported", "missing_evidence"];
/// Verdicts that count as a failing outcome when comparing against the baseline
/// variant for cross-variant regression detection.
const FAILING_VERDICTS: &[&str] = &["failed", "timed_out", "crashed"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaMatrixArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "matrixId")]
    pub matrix_id: String,
    pub status: String,
    /// The whole-game build under QA; cells are bound to this build.
    #[serde(rename = "gameBuildRef")]
    pub game_build_ref: String,
    /// The content variant treated as the regression baseline. Must be declared
    /// in `contentVariants`.
    #[serde(rename = "baselineVariant")]
    pub baseline_variant: String,
    /// Declared content-variant dimension.
    #[serde(rename = "contentVariants")]
    pub content_variants: Vec<String>,
    /// Declared deterministic-seed dimension.
    pub seeds: Vec<String>,
    /// Declared supported-target dimension.
    pub targets: Vec<String>,
    pub cells: Vec<ProductionQaMatrixCell>,
    #[serde(rename = "staleEvidenceRefs", default)]
    pub stale_evidence_refs: Vec<String>,
    #[serde(rename = "dashboardCompat")]
    pub dashboard_compat: ProductionQaMatrixDashboardCompat,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaMatrixCell {
    #[serde(rename = "cellId")]
    pub cell_id: String,
    #[serde(rename = "contentVariant")]
    pub content_variant: String,
    pub seed: String,
    pub target: String,
    pub verdict: String,
    #[serde(
        rename = "failureClass",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_class: Option<String>,
    /// Reference to the existing QA swarm run matrix (or scenario-coverage run)
    /// that produced this cell. This is the reuse anchor: the matrix is built
    /// over existing runners, never a new engine.
    #[serde(rename = "qaRunMatrixRef")]
    pub qa_run_matrix_ref: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaMatrixDashboardCompat {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub surface: String,
    pub columns: Vec<String>,
}

/// A cross-variant regression: a `seed x target` coordinate that the baseline
/// variant passed but a non-baseline variant failed. Descriptive evidence only.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DetectedRegression {
    pub seed: String,
    pub target: String,
    #[serde(rename = "contentVariant")]
    pub content_variant: String,
    #[serde(rename = "baselineVerdict")]
    pub baseline_verdict: String,
    #[serde(rename = "variantVerdict")]
    pub variant_verdict: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

/// Deterministic, Rust-owned execution plan for the declared production QA
/// matrix. This is a driver model only: every item delegates to an existing QA
/// run matrix or scenario-coverage evidence ref and carries replayable evidence
/// refs. It does not start workers or write generated run artifacts.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaMatrixExecutionPlan {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "matrixId")]
    pub matrix_id: String,
    #[serde(rename = "gameBuildRef")]
    pub game_build_ref: String,
    #[serde(rename = "baselineVariant")]
    pub baseline_variant: String,
    #[serde(rename = "expectedWorkItemCount")]
    pub expected_work_item_count: usize,
    #[serde(rename = "workItems")]
    pub work_items: Vec<ProductionQaMatrixWorkItem>,
    #[serde(rename = "detectedRegressions")]
    pub detected_regressions: Vec<ProductionQaMatrixRegressionEvidence>,
    #[serde(rename = "reuseBoundary")]
    pub reuse_boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaMatrixWorkItem {
    #[serde(rename = "workItemId")]
    pub work_item_id: String,
    #[serde(rename = "cellId")]
    pub cell_id: String,
    #[serde(rename = "contentVariant")]
    pub content_variant: String,
    pub seed: String,
    pub target: String,
    pub verdict: String,
    #[serde(rename = "runnerRef")]
    pub runner_ref: String,
    #[serde(rename = "replayEvidenceRefs")]
    pub replay_evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaMatrixRegressionEvidence {
    pub seed: String,
    pub target: String,
    #[serde(rename = "contentVariant")]
    pub content_variant: String,
    #[serde(rename = "baselineWorkItemId")]
    pub baseline_work_item_id: String,
    #[serde(rename = "variantWorkItemId")]
    pub variant_work_item_id: String,
    #[serde(rename = "baselineEvidenceRefs")]
    pub baseline_evidence_refs: Vec<String>,
    #[serde(rename = "variantEvidenceRefs")]
    pub variant_evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaMatrixReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "matrixId")]
    pub matrix_id: String,
    pub status: String,
    #[serde(rename = "cellCount")]
    pub cell_count: usize,
    #[serde(rename = "expectedCellCount")]
    pub expected_cell_count: usize,
    #[serde(rename = "coverageComplete")]
    pub coverage_complete: bool,
    #[serde(rename = "variantCount")]
    pub variant_count: usize,
    #[serde(rename = "seedCount")]
    pub seed_count: usize,
    #[serde(rename = "targetCount")]
    pub target_count: usize,
    #[serde(rename = "verdictCounts")]
    pub verdict_counts: BTreeMap<String, usize>,
    #[serde(rename = "detectedRegressions")]
    pub detected_regressions: Vec<DetectedRegression>,
    #[serde(rename = "regressionCount")]
    pub regression_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl ProductionQaMatrixArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse Production QA Matrix JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    /// Deterministically aggregates the cells, independent of their input order.
    pub fn read_model(&self) -> ProductionQaMatrixReadModel {
        let mut verdict_counts = BTreeMap::new();
        for cell in &self.cells {
            *verdict_counts.entry(cell.verdict.clone()).or_insert(0) += 1;
        }
        let detected_regressions = self.detect_regressions();
        ProductionQaMatrixReadModel {
            schema_version: self.schema_version.clone(),
            matrix_id: self.matrix_id.clone(),
            status: self.computed_status(),
            cell_count: self.cells.len(),
            expected_cell_count: self.expected_cell_count(),
            coverage_complete: self.cells.len() == self.expected_cell_count(),
            variant_count: self.content_variants.len(),
            seed_count: self.seeds.len(),
            target_count: self.targets.len(),
            verdict_counts,
            regression_count: detected_regressions.len(),
            detected_regressions,
            blocked_count: self.blocked_count(),
            validation_summary: vec![
                "each cell binds a content variant x seed x target coordinate to a verdict, the reused QA run matrix ref, evidence refs, and bounded blocked reasons".to_string(),
                "cross-variant regressions are coordinates the baseline variant passed but another variant failed, reported with replayable evidence".to_string(),
                "duplicate coordinates, undeclared dimension members, malformed verdicts, missing evidence, missing run refs, and a baseline outside contentVariants fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "regression matrix over existing QA swarm / scenario-coverage runners; no new test engine".to_string(),
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "dashboard/Studio surfaces remain read-only or draft-only; the verdict is descriptive, not a quality guarantee".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize production QA matrix read model JSON")
    }

    /// Builds the executable driver model for the declared cartesian matrix.
    ///
    /// This deliberately fails closed unless every declared
    /// `contentVariant x seed x target` coordinate has exactly one validated
    /// cell, a reused runner ref, and replayable evidence refs. The returned
    /// work items are sorted by declared dimension order and only reference
    /// existing runner/evidence artifacts; no workers are spawned here.
    pub fn execution_plan(&self) -> Result<ProductionQaMatrixExecutionPlan> {
        self.validate()?;
        if !self.stale_evidence_refs.is_empty() {
            return Err(anyhow!(
                "production QA matrix cannot produce an execution plan with stale evidence refs"
            ));
        }
        if self.blocked_count() > 0 {
            return Err(anyhow!(
                "production QA matrix cannot produce an execution plan while blocked"
            ));
        }
        if self.cells.len() != self.expected_cell_count() {
            return Err(anyhow!(
                "production QA matrix cannot produce complete runner work items: expected {} coordinates but found {} cells",
                self.expected_cell_count(),
                self.cells.len()
            ));
        }

        let mut by_coord: BTreeMap<(&str, &str, &str), &ProductionQaMatrixCell> = BTreeMap::new();
        for cell in &self.cells {
            by_coord.insert(
                (
                    cell.content_variant.as_str(),
                    cell.seed.as_str(),
                    cell.target.as_str(),
                ),
                cell,
            );
        }

        let mut work_items = Vec::with_capacity(self.expected_cell_count());
        for variant in &self.content_variants {
            for seed in &self.seeds {
                for target in &self.targets {
                    let cell = by_coord
                        .get(&(variant.as_str(), seed.as_str(), target.as_str()))
                        .ok_or_else(|| {
                            anyhow!(
                                "production QA matrix missing complete runner work item for coordinate {variant}/{seed}/{target}"
                            )
                        })?;
                    if cell.evidence_refs.is_empty() {
                        return Err(anyhow!(
                            "production QA matrix cell `{}` cannot be executable without replayable evidence refs",
                            cell.cell_id
                        ));
                    }
                    work_items.push(ProductionQaMatrixWorkItem {
                        work_item_id: format!("{}:{}:{}:{}", self.matrix_id, variant, seed, target),
                        cell_id: cell.cell_id.clone(),
                        content_variant: variant.clone(),
                        seed: seed.clone(),
                        target: target.clone(),
                        verdict: cell.verdict.clone(),
                        runner_ref: cell.qa_run_matrix_ref.clone(),
                        replay_evidence_refs: cell.evidence_refs.clone(),
                    });
                }
            }
        }

        let work_item_by_coord: BTreeMap<(&str, &str, &str), &ProductionQaMatrixWorkItem> =
            work_items
                .iter()
                .map(|item| {
                    (
                        (
                            item.content_variant.as_str(),
                            item.seed.as_str(),
                            item.target.as_str(),
                        ),
                        item,
                    )
                })
                .collect();
        let mut detected_regressions = Vec::new();
        for regression in self.detect_regressions() {
            let baseline = work_item_by_coord
                .get(&(
                    self.baseline_variant.as_str(),
                    regression.seed.as_str(),
                    regression.target.as_str(),
                ))
                .ok_or_else(|| {
                    anyhow!(
                        "production QA matrix regression lacks baseline runner work item for {}/{}",
                        regression.seed,
                        regression.target
                    )
                })?;
            let variant = work_item_by_coord
                .get(&(
                    regression.content_variant.as_str(),
                    regression.seed.as_str(),
                    regression.target.as_str(),
                ))
                .ok_or_else(|| {
                    anyhow!(
                        "production QA matrix regression lacks variant runner work item for {}/{}/{}",
                        regression.content_variant,
                        regression.seed,
                        regression.target
                    )
                })?;
            if baseline.replay_evidence_refs.is_empty() || variant.replay_evidence_refs.is_empty() {
                return Err(anyhow!(
                    "production QA matrix regression for {}/{}/{} is missing replayable baseline or variant evidence",
                    regression.content_variant,
                    regression.seed,
                    regression.target
                ));
            }
            detected_regressions.push(ProductionQaMatrixRegressionEvidence {
                seed: regression.seed.clone(),
                target: regression.target.clone(),
                content_variant: regression.content_variant.clone(),
                baseline_work_item_id: baseline.work_item_id.clone(),
                variant_work_item_id: variant.work_item_id.clone(),
                baseline_evidence_refs: baseline.replay_evidence_refs.clone(),
                variant_evidence_refs: variant.replay_evidence_refs.clone(),
            });
        }
        detected_regressions.sort();

        Ok(ProductionQaMatrixExecutionPlan {
            schema_version: self.schema_version.clone(),
            matrix_id: self.matrix_id.clone(),
            game_build_ref: self.game_build_ref.clone(),
            baseline_variant: self.baseline_variant.clone(),
            expected_work_item_count: self.expected_cell_count(),
            work_items,
            detected_regressions,
            reuse_boundary: concat!(
                "driver model only; each work item reuses an existing qaRunMatrixRef ",
                "or scenario-coverage evidence ref; no new engine, hidden workers, ",
                "or trusted mutation"
            )
            .to_string(),
        })
    }

    /// Full cartesian size of the declared dimensions.
    fn expected_cell_count(&self) -> usize {
        self.content_variants
            .len()
            .saturating_mul(self.seeds.len())
            .saturating_mul(self.targets.len())
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .cells
                .iter()
                .filter(|cell| !cell.blocked_reasons.is_empty())
                .count()
    }

    /// Detects cross-variant regressions deterministically: for every
    /// `seed x target` coordinate where the baseline variant passed, any
    /// non-baseline variant whose verdict is a failing outcome is reported.
    pub fn detect_regressions(&self) -> Vec<DetectedRegression> {
        // Index cells by (variant, seed, target). Coordinates are unique
        // (enforced by validation), so the last write is the only write.
        let mut by_coord: BTreeMap<(&str, &str, &str), &ProductionQaMatrixCell> = BTreeMap::new();
        for cell in &self.cells {
            by_coord.insert(
                (
                    cell.content_variant.as_str(),
                    cell.seed.as_str(),
                    cell.target.as_str(),
                ),
                cell,
            );
        }
        let mut regressions = Vec::new();
        for seed in &self.seeds {
            for target in &self.targets {
                let baseline = match by_coord.get(&(
                    self.baseline_variant.as_str(),
                    seed.as_str(),
                    target.as_str(),
                )) {
                    Some(cell) if cell.verdict == "passed" => *cell,
                    _ => continue,
                };
                for variant in &self.content_variants {
                    if variant == &self.baseline_variant {
                        continue;
                    }
                    if let Some(cell) =
                        by_coord.get(&(variant.as_str(), seed.as_str(), target.as_str()))
                    {
                        if FAILING_VERDICTS.contains(&cell.verdict.as_str()) {
                            let mut evidence_refs = baseline.evidence_refs.clone();
                            evidence_refs.extend(cell.evidence_refs.iter().cloned());
                            regressions.push(DetectedRegression {
                                seed: seed.clone(),
                                target: target.clone(),
                                content_variant: variant.clone(),
                                baseline_verdict: baseline.verdict.clone(),
                                variant_verdict: cell.verdict.clone(),
                                evidence_refs,
                            });
                        }
                    }
                }
            }
        }
        regressions.sort();
        regressions
    }

    pub fn computed_status(&self) -> String {
        if !self.stale_evidence_refs.is_empty() {
            return "stale".to_string();
        }
        if !self.blocked_reasons.is_empty()
            || self
                .cells
                .iter()
                .any(|cell| !cell.blocked_reasons.is_empty())
        {
            return "blocked".to_string();
        }
        let all_resolved = self
            .cells
            .iter()
            .all(|cell| RESOLVED_VERDICTS.contains(&cell.verdict.as_str()));
        if all_resolved {
            "complete".to_string()
        } else {
            "partial".to_string()
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCTION_QA_MATRIX_SCHEMA_VERSION {
            return Err(anyhow!(
                "production QA matrix schemaVersion must be {PRODUCTION_QA_MATRIX_SCHEMA_VERSION}"
            ));
        }
        require_id("production QA matrix matrixId", &self.matrix_id)?;
        require_ref("production QA matrix gameBuildRef", &self.game_build_ref)?;

        validate_id_dimension(
            "production QA matrix contentVariants",
            &self.content_variants,
        )?;
        validate_id_dimension("production QA matrix seeds", &self.seeds)?;
        validate_id_dimension("production QA matrix targets", &self.targets)?;

        require_id(
            "production QA matrix baselineVariant",
            &self.baseline_variant,
        )?;
        if !self.content_variants.contains(&self.baseline_variant) {
            return Err(anyhow!(
                "production QA matrix baselineVariant `{}` must be declared in contentVariants",
                self.baseline_variant
            ));
        }

        validate_ref_list(
            "production QA matrix staleEvidenceRefs",
            &self.stale_evidence_refs,
            false,
        )?;
        require_nonempty("production QA matrix cells", self.cells.len())?;
        if self.cells.len() > 512 {
            return Err(anyhow!("production QA matrix is overbroad for v1"));
        }
        if self.cells.len() > self.expected_cell_count() {
            return Err(anyhow!(
                "production QA matrix has more cells than the declared dimensions allow"
            ));
        }

        self.dashboard_compat.validate()?;

        let variants: BTreeSet<&str> = self.content_variants.iter().map(String::as_str).collect();
        let seeds: BTreeSet<&str> = self.seeds.iter().map(String::as_str).collect();
        let targets: BTreeSet<&str> = self.targets.iter().map(String::as_str).collect();

        let mut cell_ids = BTreeSet::new();
        let mut coords = BTreeSet::new();
        for cell in &self.cells {
            cell.validate()?;
            if !cell_ids.insert(cell.cell_id.as_str()) {
                return Err(anyhow!(
                    "production QA matrix duplicate cell id `{}`",
                    cell.cell_id
                ));
            }
            if !variants.contains(cell.content_variant.as_str()) {
                return Err(anyhow!(
                    "production QA matrix cell `{}` uses undeclared contentVariant `{}`",
                    cell.cell_id,
                    cell.content_variant
                ));
            }
            if !seeds.contains(cell.seed.as_str()) {
                return Err(anyhow!(
                    "production QA matrix cell `{}` uses undeclared seed `{}`",
                    cell.cell_id,
                    cell.seed
                ));
            }
            if !targets.contains(cell.target.as_str()) {
                return Err(anyhow!(
                    "production QA matrix cell `{}` uses undeclared target `{}`",
                    cell.cell_id,
                    cell.target
                ));
            }
            let coord = (
                cell.content_variant.as_str(),
                cell.seed.as_str(),
                cell.target.as_str(),
            );
            if !coords.insert(coord) {
                return Err(anyhow!(
                    "production QA matrix duplicate coordinate for contentVariant/seed/target"
                ));
            }
        }

        let computed = self.computed_status();
        if self.status != computed {
            return Err(anyhow!(
                "production QA matrix status `{}` does not match computed classification `{computed}`",
                self.status
            ));
        }
        if (computed == "stale" || computed == "blocked") && self.blocked_count() == 0 {
            return Err(anyhow!(
                "production QA matrix {computed} status requires visible blockedReasons"
            ));
        }

        validate_text_list(
            "production QA matrix blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        require_text("production QA matrix boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "not trusted truth",
            "no auto-fix",
            "review-gated",
            "read-only or draft-only",
            "reuses existing runners",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "production QA matrix boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl ProductionQaMatrixCell {
    fn validate(&self) -> Result<()> {
        require_id("production QA matrix cellId", &self.cell_id)?;
        require_id("production QA matrix contentVariant", &self.content_variant)?;
        require_id("production QA matrix seed", &self.seed)?;
        require_id("production QA matrix target", &self.target)?;
        require_ref(
            "production QA matrix qaRunMatrixRef",
            &self.qa_run_matrix_ref,
        )?;
        if !VERDICTS.contains(&self.verdict.as_str()) {
            return Err(anyhow!(
                "production QA matrix malformed verdict `{}`",
                self.verdict
            ));
        }
        if let Some(failure_class) = &self.failure_class {
            require_text("production QA matrix failureClass", failure_class)?;
        }
        validate_ref_list(
            "production QA matrix evidenceRefs",
            &self.evidence_refs,
            false,
        )?;
        if NO_EVIDENCE_VERDICTS.contains(&self.verdict.as_str()) {
            if self.verdict == "missing_evidence" && !self.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "production QA matrix cell `{}` is missing_evidence but carries evidence refs",
                    self.cell_id
                ));
            }
        } else if self.evidence_refs.is_empty() {
            return Err(anyhow!(
                "production QA matrix cell `{}` is missing evidence for verdict `{}`",
                self.cell_id,
                self.verdict
            ));
        }
        validate_text_list(
            "production QA matrix cell blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        Ok(())
    }
}

impl ProductionQaMatrixDashboardCompat {
    fn validate(&self) -> Result<()> {
        if !self.read_only {
            return Err(anyhow!(
                "production QA matrix dashboard surface must remain read-only or draft-only"
            ));
        }
        require_text(
            "production QA matrix dashboardCompat.surface",
            &self.surface,
        )?;
        validate_text_list(
            "production QA matrix dashboardCompat.columns",
            &self.columns,
            true,
        )
    }
}

fn validate_id_dimension(field: &str, values: &[String]) -> Result<()> {
    require_nonempty(field, values.len())?;
    if values.len() > 64 {
        return Err(anyhow!("{field} is overbroad for v1"));
    }
    let mut seen = BTreeSet::new();
    for value in values {
        require_id(field, value)?;
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{field} contains duplicate member `{value}`"));
        }
    }
    Ok(())
}

fn validate_ref_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
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

fn validate_text_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':'))
    {
        return Err(anyhow!("{field} must be a bounded local id"));
    }
    Ok(())
}

fn require_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("data:") {
        return Err(anyhow!("{field} remote refs are not allowed"));
    }
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} contains forbidden traversal and must stay inside local fixture/reference roots"
        ));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/")
        || value.starts_with("runs/")
        || value.starts_with("evidence/"))
    {
        return Err(anyhow!(
            "{field} must use examples/, docs/, seeds/, runs/, or evidence/ refs"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "eval(",
        "dynamic code loading",
        "command bridge",
        "local server bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "auto-fix",
        "self-approval",
        "godot replacement",
        "production-ready",
        "shipped-game",
        "hidden worker",
        "remote swarm",
        "quality guarantee",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden production QA matrix authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    const NEGATIONS: [&str; 7] = [
        "no ",
        "not ",
        "without ",
        "avoid ",
        "forbid ",
        "forbidden ",
        "not yet ",
    ];
    let hay = value;
    // Scope negation to the clause/sentence containing each occurrence so a
    // negated mention in one sentence cannot whitelist a positive mention in
    // another (fail-closed), while a single leading negation still covers a
    // list such as `no auto-apply or self-approval`.
    const CONTRASTS: [&str; 6] = [
        " but ",
        " however ",
        " yet ",
        " whereas ",
        " nevertheless ",
        " though ",
    ];
    let mut search_start = 0;
    while let Some(rel) = hay[search_start..].find(phrase) {
        let idx = search_start + rel;
        let mut clause_start = hay[..idx]
            .rfind(['.', ';', '!', '\n', '\r'])
            .map(|p| p + 1)
            .unwrap_or(0);
        if let Some(reset) = CONTRASTS
            .iter()
            .filter_map(|c| {
                hay[clause_start..idx]
                    .rfind(c)
                    .map(|p| clause_start + p + c.len())
            })
            .max()
        {
            clause_start = reset;
        }
        let preceding = &hay[clause_start..idx];
        let negated = NEGATIONS.iter().any(|n| preceding.contains(n));
        if !negated {
            return true;
        }
        search_start = idx + phrase.len();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(
        id: &str,
        variant: &str,
        seed: &str,
        target: &str,
        verdict: &str,
        evidence: &[&str],
    ) -> ProductionQaMatrixCell {
        ProductionQaMatrixCell {
            cell_id: id.to_string(),
            content_variant: variant.to_string(),
            seed: seed.to_string(),
            target: target.to_string(),
            verdict: verdict.to_string(),
            failure_class: None,
            qa_run_matrix_ref: "evidence/qa/run-matrix.json".to_string(),
            evidence_refs: evidence.iter().map(|s| s.to_string()).collect(),
            blocked_reasons: Vec::new(),
        }
    }

    fn artifact(cells: Vec<ProductionQaMatrixCell>, status: &str) -> ProductionQaMatrixArtifact {
        ProductionQaMatrixArtifact {
            schema_version: PRODUCTION_QA_MATRIX_SCHEMA_VERSION.to_string(),
            matrix_id: "matrix-001".to_string(),
            status: status.to_string(),
            game_build_ref: "runs/build-001".to_string(),
            baseline_variant: "base".to_string(),
            content_variants: vec!["base".to_string(), "variant-a".to_string()],
            seeds: vec!["seed-1".to_string(), "seed-2".to_string()],
            targets: vec!["web".to_string()],
            cells,
            stale_evidence_refs: Vec::new(),
            dashboard_compat: ProductionQaMatrixDashboardCompat {
                read_only: true,
                surface: "evidence-dashboard".to_string(),
                columns: vec!["coordinate".to_string(), "verdict".to_string()],
            },
            blocked_reasons: Vec::new(),
            boundary:
                "QA matrix cells are evidence inputs, not trusted truth; no auto-fix; review-gated; dashboards stay read-only or draft-only; reuses existing runners"
                    .to_string(),
        }
    }

    #[test]
    fn detects_planted_cross_variant_regression() {
        let cells = vec![
            cell("c1", "base", "seed-1", "web", "passed", &["runs/a"]),
            cell("c2", "variant-a", "seed-1", "web", "failed", &["runs/b"]),
            cell("c3", "base", "seed-2", "web", "passed", &["runs/c"]),
            cell("c4", "variant-a", "seed-2", "web", "passed", &["runs/d"]),
        ];
        let artifact = artifact(cells, "complete");
        artifact.validate().expect("valid");
        let regressions = artifact.detect_regressions();
        assert_eq!(regressions.len(), 1);
        assert_eq!(regressions[0].content_variant, "variant-a");
        assert_eq!(regressions[0].seed, "seed-1");
        assert_eq!(regressions[0].variant_verdict, "failed");
        // Baseline pass + variant pass at seed-2 is not a regression.
    }

    #[test]
    fn read_model_is_order_independent() {
        let cells = vec![
            cell("c1", "base", "seed-1", "web", "passed", &["runs/a"]),
            cell("c2", "variant-a", "seed-1", "web", "failed", &["runs/b"]),
            cell("c3", "base", "seed-2", "web", "passed", &["runs/c"]),
            cell("c4", "variant-a", "seed-2", "web", "passed", &["runs/d"]),
        ];
        let forward = artifact(cells.clone(), "complete");
        let mut reversed_cells = cells;
        reversed_cells.reverse();
        let reversed = artifact(reversed_cells, "complete");
        assert_eq!(
            forward.read_model_json().unwrap(),
            reversed.read_model_json().unwrap(),
            "read model must be deterministic regardless of cell order"
        );
    }

    #[test]
    fn baseline_failure_is_not_a_cross_variant_regression() {
        // If the baseline itself failed, a failing variant is not a regression
        // relative to the baseline (nothing to regress from).
        let cells = vec![
            cell("c1", "base", "seed-1", "web", "failed", &["runs/a"]),
            cell("c2", "variant-a", "seed-1", "web", "failed", &["runs/b"]),
        ];
        let artifact = artifact(cells, "complete");
        assert!(artifact.detect_regressions().is_empty());
    }
}
