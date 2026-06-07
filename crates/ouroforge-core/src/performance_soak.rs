//! Performance and Soak Testing v1 (#1668) — soak/performance runs over long
//! sessions and large content, built on the existing frame-budget surface.
//!
//! This module does not introduce a new profiler. Each soak segment records a
//! single metric sample (reusing the frame-budget metric kinds and the
//! `lte`/`lt` comparators) and references the existing frame-budget / QA
//! performance-budget evidence that produced it. The aggregation rolls the
//! ordered segments up into a descriptive verdict: `pass` (every sample within
//! budget and the session stays within the stability tolerance), `regressed`
//! (a planted budget breach), or `unstable` (soak drift exceeds the declared
//! tolerance). It performs no trusted mutation, auto-fix, auto-apply, or quality
//! judgement; outputs are evidence inputs only and remain review-gated.
//!
//! Samples are recorded fixture values (integer milli-units, no live wall-clock
//! timing), so evaluation is deterministic and never flaky.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PERFORMANCE_SOAK_SCHEMA_VERSION: &str = "performance-soak-v1";

/// Metric kinds reused from the QA performance budget surface. Timings are
/// milliseconds, counts are integers; both are recorded as integer milli-units
/// (`valueX1000`) for deterministic comparison.
const METRIC_KINDS: &[&str] = &[
    "frameTimeMs",
    "updateMs",
    "renderMs",
    "probeMs",
    "evidenceMs",
    "entityCount",
    "drawCallCount",
    "collisionPairCount",
];

/// Comparators reused from the QA performance budget surface.
const SUPPORTED_COMPARATORS: &[&str] = &["lte", "lt"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PerformanceSoakArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "suiteId")]
    pub suite_id: String,
    /// Run-completion classification: `complete`, `blocked`, or `stale`.
    pub status: String,
    /// Evaluation outcome: `pass`, `regressed`, `unstable`, or `blocked`.
    pub verdict: String,
    /// The whole-game build under soak/performance QA.
    #[serde(rename = "gameBuildRef")]
    pub game_build_ref: String,
    /// The reused frame-budget / performance-budget threshold artifact.
    #[serde(rename = "budgetRef")]
    pub budget_ref: String,
    /// The single metric this soak suite tracks across the session.
    #[serde(rename = "metricKind")]
    pub metric_kind: String,
    /// Comparator applied against the budget (`lte` or `lt`).
    pub comparator: String,
    /// Budget threshold in integer milli-units (metric value x1000).
    #[serde(rename = "budgetX1000")]
    pub budget_x1000: u64,
    /// Maximum allowed drift (worst sample minus first sample, in milli-units)
    /// for the session to count as stable.
    #[serde(rename = "stabilityToleranceX1000")]
    pub stability_tolerance_x1000: u64,
    pub segments: Vec<PerformanceSoakSegment>,
    #[serde(rename = "staleEvidenceRefs", default)]
    pub stale_evidence_refs: Vec<String>,
    #[serde(rename = "dashboardCompat")]
    pub dashboard_compat: PerformanceSoakDashboardCompat,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PerformanceSoakSegment {
    #[serde(rename = "segmentId")]
    pub segment_id: String,
    /// 1-based ordinal position of this sample in the session.
    pub sequence: u32,
    /// Elapsed session time at this sample, in milliseconds (descriptive only).
    #[serde(rename = "sessionElapsedMs")]
    pub session_elapsed_ms: u64,
    /// Measured metric value in integer milli-units (metric value x1000).
    #[serde(rename = "valueX1000")]
    pub value_x1000: u64,
    /// Reference to the existing frame-budget evidence that produced this
    /// sample. This is the reuse anchor: soak reuses the frame-budget surface,
    /// never a new profiler.
    #[serde(rename = "frameBudgetRef")]
    pub frame_budget_ref: String,
    #[serde(rename = "evidenceRefs", default)]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PerformanceSoakDashboardCompat {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub surface: String,
    pub columns: Vec<String>,
}

/// A segment whose sample breached the budget. Descriptive evidence only.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BudgetBreach {
    pub sequence: u32,
    #[serde(rename = "segmentId")]
    pub segment_id: String,
    #[serde(rename = "valueX1000")]
    pub value_x1000: u64,
    #[serde(rename = "budgetX1000")]
    pub budget_x1000: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PerformanceSoakReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "suiteId")]
    pub suite_id: String,
    pub status: String,
    pub verdict: String,
    #[serde(rename = "metricKind")]
    pub metric_kind: String,
    #[serde(rename = "segmentCount")]
    pub segment_count: usize,
    #[serde(rename = "withinBudgetCount")]
    pub within_budget_count: usize,
    #[serde(rename = "budgetBreaches")]
    pub budget_breaches: Vec<BudgetBreach>,
    #[serde(rename = "breachCount")]
    pub breach_count: usize,
    #[serde(rename = "baselineValueX1000")]
    pub baseline_value_x1000: u64,
    #[serde(rename = "worstValueX1000")]
    pub worst_value_x1000: u64,
    #[serde(rename = "driftX1000")]
    pub drift_x1000: u64,
    #[serde(rename = "soakStable")]
    pub soak_stable: bool,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

fn comparator_holds(value: u64, budget: u64, comparator: &str) -> bool {
    match comparator {
        "lte" => value <= budget,
        "lt" => value < budget,
        _ => false,
    }
}

impl PerformanceSoakArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse Performance Soak JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    /// Deterministically aggregates the segments, independent of input order.
    pub fn read_model(&self) -> PerformanceSoakReadModel {
        let breaches = self.budget_breaches();
        let (baseline, worst, drift, stable) = self.stability();
        PerformanceSoakReadModel {
            schema_version: self.schema_version.clone(),
            suite_id: self.suite_id.clone(),
            status: self.computed_status(),
            verdict: self.computed_verdict(),
            metric_kind: self.metric_kind.clone(),
            segment_count: self.segments.len(),
            within_budget_count: self
                .segments
                .iter()
                .filter(|s| comparator_holds(s.value_x1000, self.budget_x1000, &self.comparator))
                .count(),
            breach_count: breaches.len(),
            budget_breaches: breaches,
            baseline_value_x1000: baseline,
            worst_value_x1000: worst,
            drift_x1000: drift,
            soak_stable: stable,
            blocked_count: self.blocked_count(),
            validation_summary: vec![
                "each segment records one frame-budget metric sample (milli-units), references the frame-budget evidence that produced it, and carries replayable evidence refs".to_string(),
                "a sample outside the budget is a detected performance regression; session drift beyond the stability tolerance is reported as unstable".to_string(),
                "duplicate segment ids/sequences, unknown metric kinds, unsupported comparators, and missing budget/evidence refs fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "soak/performance testing over the existing frame-budget surface; no new profiler".to_string(),
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "samples are recorded integer milli-units, not live wall-clock timing, so the verdict is deterministic and never flaky".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize performance soak read model JSON")
    }

    /// Segments whose sample breached the budget, deterministically sorted.
    pub fn budget_breaches(&self) -> Vec<BudgetBreach> {
        let mut breaches: Vec<BudgetBreach> = self
            .segments
            .iter()
            .filter(|s| !comparator_holds(s.value_x1000, self.budget_x1000, &self.comparator))
            .map(|s| BudgetBreach {
                sequence: s.sequence,
                segment_id: s.segment_id.clone(),
                value_x1000: s.value_x1000,
                budget_x1000: self.budget_x1000,
            })
            .collect();
        breaches.sort();
        breaches
    }

    /// Returns `(baselineValue, worstValue, drift, stable)` where the baseline
    /// is the sample at the lowest sequence and drift is the non-negative growth
    /// from baseline to the worst sample.
    fn stability(&self) -> (u64, u64, u64, bool) {
        let baseline = self
            .segments
            .iter()
            .min_by_key(|s| s.sequence)
            .map(|s| s.value_x1000)
            .unwrap_or(0);
        let worst = self
            .segments
            .iter()
            .map(|s| s.value_x1000)
            .max()
            .unwrap_or(0);
        let drift = worst.saturating_sub(baseline);
        let stable = drift <= self.stability_tolerance_x1000;
        (baseline, worst, drift, stable)
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .segments
                .iter()
                .filter(|s| !s.blocked_reasons.is_empty())
                .count()
    }

    pub fn computed_status(&self) -> String {
        if !self.stale_evidence_refs.is_empty() {
            return "stale".to_string();
        }
        if self.blocked_count() > 0 {
            return "blocked".to_string();
        }
        "complete".to_string()
    }

    pub fn computed_verdict(&self) -> String {
        if self.computed_status() != "complete" {
            return "blocked".to_string();
        }
        if !self.budget_breaches().is_empty() {
            return "regressed".to_string();
        }
        let (_, _, _, stable) = self.stability();
        if !stable {
            return "unstable".to_string();
        }
        "pass".to_string()
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PERFORMANCE_SOAK_SCHEMA_VERSION {
            return Err(anyhow!(
                "performance soak schemaVersion must be {PERFORMANCE_SOAK_SCHEMA_VERSION}"
            ));
        }
        require_id("performance soak suiteId", &self.suite_id)?;
        require_ref("performance soak gameBuildRef", &self.game_build_ref)?;
        require_ref("performance soak budgetRef", &self.budget_ref)?;
        if !METRIC_KINDS.contains(&self.metric_kind.as_str()) {
            return Err(anyhow!(
                "performance soak unknown metric kind `{}`",
                self.metric_kind
            ));
        }
        if !SUPPORTED_COMPARATORS.contains(&self.comparator.as_str()) {
            return Err(anyhow!(
                "performance soak unsupported comparator `{}`",
                self.comparator
            ));
        }
        if self.budget_x1000 == 0 {
            return Err(anyhow!(
                "performance soak budgetX1000 must be greater than 0"
            ));
        }
        validate_ref_list(
            "performance soak staleEvidenceRefs",
            &self.stale_evidence_refs,
            false,
        )?;
        require_nonempty("performance soak segments", self.segments.len())?;
        if self.segments.len() > 4096 {
            return Err(anyhow!("performance soak suite is overbroad for v1"));
        }

        self.dashboard_compat.validate()?;

        let mut segment_ids = BTreeSet::new();
        let mut sequences = BTreeSet::new();
        for segment in &self.segments {
            segment.validate()?;
            if !segment_ids.insert(segment.segment_id.as_str()) {
                return Err(anyhow!(
                    "performance soak duplicate segment id `{}`",
                    segment.segment_id
                ));
            }
            if !sequences.insert(segment.sequence) {
                return Err(anyhow!(
                    "performance soak duplicate segment sequence {}",
                    segment.sequence
                ));
            }
        }

        let computed_status = self.computed_status();
        if self.status != computed_status {
            return Err(anyhow!(
                "performance soak status `{}` does not match computed classification `{computed_status}`",
                self.status
            ));
        }
        if (computed_status == "stale" || computed_status == "blocked") && self.blocked_count() == 0
        {
            return Err(anyhow!(
                "performance soak {computed_status} status requires visible blockedReasons"
            ));
        }
        let computed_verdict = self.computed_verdict();
        if self.verdict != computed_verdict {
            return Err(anyhow!(
                "performance soak verdict `{}` does not match computed verdict `{computed_verdict}`",
                self.verdict
            ));
        }

        validate_text_list(
            "performance soak blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        require_text("performance soak boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "not trusted truth",
            "no auto-fix",
            "review-gated",
            "read-only or draft-only",
            "reuses the frame-budget",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!("performance soak boundary must state `{required}`"));
            }
        }
        Ok(())
    }
}

impl PerformanceSoakSegment {
    fn validate(&self) -> Result<()> {
        require_id("performance soak segmentId", &self.segment_id)?;
        if self.sequence == 0 {
            return Err(anyhow!("performance soak segment sequence must be >= 1"));
        }
        require_ref("performance soak frameBudgetRef", &self.frame_budget_ref)?;
        validate_ref_list("performance soak evidenceRefs", &self.evidence_refs, false)?;
        validate_text_list(
            "performance soak segment blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        Ok(())
    }
}

impl PerformanceSoakDashboardCompat {
    fn validate(&self) -> Result<()> {
        if !self.read_only {
            return Err(anyhow!(
                "performance soak dashboard surface must remain read-only or draft-only"
            ));
        }
        require_text("performance soak dashboardCompat.surface", &self.surface)?;
        validate_text_list(
            "performance soak dashboardCompat.columns",
            &self.columns,
            true,
        )
    }
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
        "quality guarantee",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden performance soak authority text `{forbidden}`"
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

    fn segment(id: &str, sequence: u32, value: u64) -> PerformanceSoakSegment {
        PerformanceSoakSegment {
            segment_id: id.to_string(),
            sequence,
            session_elapsed_ms: sequence as u64 * 60_000,
            value_x1000: value,
            frame_budget_ref: "evidence/perf/frame-budget.json".to_string(),
            evidence_refs: vec!["runs/soak/sample.json".to_string()],
            blocked_reasons: Vec::new(),
        }
    }

    fn artifact(segments: Vec<PerformanceSoakSegment>) -> PerformanceSoakArtifact {
        let mut artifact = PerformanceSoakArtifact {
            schema_version: PERFORMANCE_SOAK_SCHEMA_VERSION.to_string(),
            suite_id: "soak-001".to_string(),
            status: "complete".to_string(),
            verdict: "pass".to_string(),
            game_build_ref: "runs/build-001".to_string(),
            budget_ref: "examples/qa-performance-budget-v1/budget.pass.fixture.json".to_string(),
            metric_kind: "frameTimeMs".to_string(),
            comparator: "lte".to_string(),
            budget_x1000: 16_000,
            stability_tolerance_x1000: 4_000,
            segments,
            stale_evidence_refs: Vec::new(),
            dashboard_compat: PerformanceSoakDashboardCompat {
                read_only: true,
                surface: "evidence-dashboard".to_string(),
                columns: vec!["sequence".to_string(), "value".to_string()],
            },
            blocked_reasons: Vec::new(),
            boundary:
                "Soak samples are evidence inputs, not trusted truth; no auto-fix; review-gated; dashboards stay read-only or draft-only; reuses the frame-budget surface"
                    .to_string(),
        };
        artifact.verdict = artifact.computed_verdict();
        artifact
    }

    #[test]
    fn budget_pass_and_stable_session() {
        let segments = vec![
            segment("s1", 1, 12_000),
            segment("s2", 2, 12_500),
            segment("s3", 3, 13_000),
        ];
        let artifact = artifact(segments);
        artifact.validate().expect("valid");
        assert_eq!(artifact.computed_verdict(), "pass");
        assert!(artifact.budget_breaches().is_empty());
    }

    #[test]
    fn planted_budget_regression_is_detected() {
        let segments = vec![
            segment("s1", 1, 12_000),
            segment("s2", 2, 18_000), // over the 16ms budget
        ];
        let artifact = artifact(segments);
        artifact.validate().expect("valid");
        assert_eq!(artifact.computed_verdict(), "regressed");
        let breaches = artifact.budget_breaches();
        assert_eq!(breaches.len(), 1);
        assert_eq!(breaches[0].segment_id, "s2");
    }

    #[test]
    fn soak_drift_beyond_tolerance_is_unstable() {
        // All under the 16ms budget, but drift 1000->15000 exceeds tolerance 4000.
        let segments = vec![
            segment("s1", 1, 10_000),
            segment("s2", 2, 12_000),
            segment("s3", 3, 15_000),
        ];
        let artifact = artifact(segments);
        artifact.validate().expect("valid");
        assert_eq!(artifact.computed_verdict(), "unstable");
        assert!(artifact.budget_breaches().is_empty());
    }

    #[test]
    fn read_model_is_order_independent() {
        let segments = vec![
            segment("s1", 1, 12_000),
            segment("s2", 2, 18_000),
            segment("s3", 3, 13_000),
        ];
        let forward = artifact(segments.clone());
        let mut reversed_segments = segments;
        reversed_segments.reverse();
        let reversed = artifact(reversed_segments);
        assert_eq!(
            forward.read_model_json().unwrap(),
            reversed.read_model_json().unwrap()
        );
    }
}
