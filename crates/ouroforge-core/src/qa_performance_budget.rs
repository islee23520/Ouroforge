use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const QA_PERFORMANCE_BUDGET_SCHEMA_VERSION: &str = "qa-performance-budget-v1";

/// Metric kinds the budget understands. Timings are milliseconds, counts are integers.
const TIMING_METRIC_KINDS: &[&str] = &[
    "frameTimeMs",
    "updateMs",
    "renderMs",
    "probeMs",
    "evidenceMs",
];
const COUNT_METRIC_KINDS: &[&str] = &["entityCount", "drawCallCount", "collisionPairCount"];

/// Trusted Rust/local emitters. Browser sources are evidence inputs only.
const TRUSTED_METRIC_SOURCES: &[&str] = &["rust_runtime_profiler", "frame_budget_evidence"];
const LOW_TRUST_METRIC_SOURCES: &[&str] = &["browser_probe", "browser_studio_probe"];

const SUPPORTED_COMPARATORS: &[&str] = &["lte", "lt"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PerformanceBudgetArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "budgetId")]
    pub budget_id: String,
    /// Declared status; must equal the computed classification.
    pub status: String,
    #[serde(rename = "runMatrixRefs")]
    pub run_matrix_refs: Vec<String>,
    #[serde(rename = "staleRunRefs", default)]
    pub stale_run_refs: Vec<String>,
    #[serde(rename = "baselineRefs", default)]
    pub baseline_refs: Vec<String>,
    #[serde(rename = "profilingRefs")]
    pub profiling_refs: Vec<String>,
    pub metrics: Vec<PerformanceMetric>,
    pub thresholds: Vec<PerformanceThreshold>,
    pub verdict: String,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "trustWarnings", default)]
    pub trust_warnings: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PerformanceMetric {
    #[serde(rename = "metricId")]
    pub metric_id: String,
    pub kind: String,
    pub source: String,
    pub present: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    pub unit: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PerformanceThreshold {
    #[serde(rename = "thresholdId")]
    pub threshold_id: String,
    #[serde(rename = "metricKind")]
    pub metric_kind: String,
    pub comparator: String,
    pub limit: f64,
    pub supported: bool,
    #[serde(rename = "baselineRequired", default)]
    pub baseline_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceBudgetStatus {
    Pass,
    Fail,
    Inconclusive,
    Missing,
    Unsupported,
    Stale,
}

impl PerformanceBudgetStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Inconclusive => "inconclusive",
            Self::Missing => "missing",
            Self::Unsupported => "unsupported",
            Self::Stale => "stale",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PerformanceBudgetReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "budgetId")]
    pub budget_id: String,
    pub status: String,
    #[serde(rename = "runMatrixRefCount")]
    pub run_matrix_ref_count: usize,
    #[serde(rename = "baselineRefCount")]
    pub baseline_ref_count: usize,
    #[serde(rename = "metricCount")]
    pub metric_count: usize,
    #[serde(rename = "presentMetricCount")]
    pub present_metric_count: usize,
    #[serde(rename = "thresholdCount")]
    pub threshold_count: usize,
    #[serde(rename = "trustedThresholdCount")]
    pub trusted_threshold_count: usize,
    #[serde(rename = "violationCount")]
    pub violation_count: usize,
    #[serde(rename = "lowTrustMetricCount")]
    pub low_trust_metric_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "metricKindCounts")]
    pub metric_kind_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

/// A trusted-threshold violation: a present, trusted metric exceeded its limit.
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceBudgetViolation {
    pub threshold_id: String,
    pub metric_kind: String,
    pub actual: f64,
    pub limit: f64,
}

impl PerformanceBudgetArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse QA Performance Budget JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> PerformanceBudgetReadModel {
        let mut metric_kind_counts = BTreeMap::new();
        for metric in &self.metrics {
            *metric_kind_counts.entry(metric.kind.clone()).or_insert(0) += 1;
        }
        PerformanceBudgetReadModel {
            schema_version: self.schema_version.clone(),
            budget_id: self.budget_id.clone(),
            status: self.computed_status().as_str().to_string(),
            run_matrix_ref_count: self.run_matrix_refs.len(),
            baseline_ref_count: self.baseline_refs.len(),
            metric_count: self.metrics.len(),
            present_metric_count: self.metrics.iter().filter(|m| m.present).count(),
            threshold_count: self.thresholds.len(),
            trusted_threshold_count: self.trusted_thresholds().count(),
            violation_count: self.violations().len(),
            low_trust_metric_count: self
                .metrics
                .iter()
                .filter(|m| LOW_TRUST_METRIC_SOURCES.contains(&m.source.as_str()))
                .count(),
            blocked_count: self.blocked_reasons.len(),
            metric_kind_counts,
            validation_summary: vec![
                "metrics record frame/update/render/probe/evidence timings and entity/draw/collision counts as evidence inputs".to_string(),
                "thresholds are evaluated only against present, trusted-source metrics; browser-sourced metrics stay advisory".to_string(),
                "missing metrics, malformed metrics, stale run refs, unsupported thresholds, and missing required baselines fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "existing run matrix, runtime frame budget, profiling evidence, dashboard, and Studio read models remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize QA performance budget read model JSON")
    }

    /// Trusted thresholds: those whose target metric is present and emitted by a trusted source.
    fn trusted_thresholds(&self) -> impl Iterator<Item = &PerformanceThreshold> {
        self.thresholds.iter().filter(move |threshold| {
            self.metrics.iter().any(|metric| {
                metric.kind == threshold.metric_kind
                    && metric.present
                    && TRUSTED_METRIC_SOURCES.contains(&metric.source.as_str())
            })
        })
    }

    /// Present, trusted-source metrics that exceed a supported threshold limit.
    pub fn violations(&self) -> Vec<PerformanceBudgetViolation> {
        let mut violations = Vec::new();
        for threshold in &self.thresholds {
            if !threshold.supported {
                continue;
            }
            for metric in &self.metrics {
                if metric.kind != threshold.metric_kind
                    || !metric.present
                    || !TRUSTED_METRIC_SOURCES.contains(&metric.source.as_str())
                {
                    continue;
                }
                let Some(value) = metric.value else {
                    continue;
                };
                let exceeds = match threshold.comparator.as_str() {
                    "lt" => value >= threshold.limit,
                    _ => value > threshold.limit,
                };
                if exceeds {
                    violations.push(PerformanceBudgetViolation {
                        threshold_id: threshold.threshold_id.clone(),
                        metric_kind: threshold.metric_kind.clone(),
                        actual: value,
                        limit: threshold.limit,
                    });
                }
            }
        }
        violations
    }

    fn metric_for_kind(&self, kind: &str) -> Option<&PerformanceMetric> {
        self.metrics.iter().find(|metric| metric.kind == kind)
    }

    /// Classify the budget. Precedence: stale > unsupported > missing > fail/pass/inconclusive.
    pub fn computed_status(&self) -> PerformanceBudgetStatus {
        if !self.stale_run_refs.is_empty() {
            return PerformanceBudgetStatus::Stale;
        }
        if self.thresholds.iter().any(|threshold| !threshold.supported) {
            return PerformanceBudgetStatus::Unsupported;
        }
        let has_missing_metric = self.thresholds.iter().any(|threshold| {
            self.metric_for_kind(&threshold.metric_kind)
                .map(|metric| !metric.present)
                .unwrap_or(true)
        });
        if has_missing_metric {
            return PerformanceBudgetStatus::Missing;
        }
        let trusted_count = self.trusted_thresholds().count();
        if trusted_count == 0 {
            return PerformanceBudgetStatus::Inconclusive;
        }
        if self.violations().is_empty() {
            PerformanceBudgetStatus::Pass
        } else {
            PerformanceBudgetStatus::Fail
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != QA_PERFORMANCE_BUDGET_SCHEMA_VERSION {
            return Err(anyhow!(
                "QA performance budget schemaVersion must be {QA_PERFORMANCE_BUDGET_SCHEMA_VERSION}"
            ));
        }
        require_id("QA performance budget budgetId", &self.budget_id)?;
        validate_ref_list(
            "QA performance budget runMatrixRefs",
            &self.run_matrix_refs,
            true,
        )?;
        validate_ref_list(
            "QA performance budget staleRunRefs",
            &self.stale_run_refs,
            false,
        )?;
        // Stale run refs must be drawn from the declared run matrix.
        let matrix: BTreeSet<&str> = self.run_matrix_refs.iter().map(String::as_str).collect();
        for stale in &self.stale_run_refs {
            if !matrix.contains(stale.as_str()) {
                return Err(anyhow!(
                    "QA performance budget staleRunRefs must reference declared runMatrixRefs"
                ));
            }
        }
        validate_ref_list(
            "QA performance budget baselineRefs",
            &self.baseline_refs,
            false,
        )?;
        validate_ref_list(
            "QA performance budget profilingRefs",
            &self.profiling_refs,
            true,
        )?;
        require_nonempty("QA performance budget metrics", self.metrics.len())?;
        require_nonempty("QA performance budget thresholds", self.thresholds.len())?;
        if self.metrics.len() > 64 || self.thresholds.len() > 64 {
            return Err(anyhow!("QA performance budget is overbroad for v1"));
        }
        self.validate_metrics()?;
        self.validate_thresholds()?;
        validate_text_list(
            "QA performance budget blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        validate_text_list(
            "QA performance budget trustWarnings",
            &self.trust_warnings,
            false,
        )?;
        require_text("QA performance budget verdict", &self.verdict)?;

        // Browser/low-trust metrics are evidence inputs only; they require a trust warning.
        let has_low_trust = self
            .metrics
            .iter()
            .any(|metric| LOW_TRUST_METRIC_SOURCES.contains(&metric.source.as_str()));
        if has_low_trust
            && !self
                .trust_warnings
                .iter()
                .any(|warning| warning.to_ascii_lowercase().contains("browser"))
        {
            return Err(anyhow!(
                "QA performance budget browser-sourced metrics require a browser trust warning"
            ));
        }

        // Missing baseline where required fails closed.
        if self.baseline_refs.is_empty()
            && self
                .thresholds
                .iter()
                .any(|threshold| threshold.baseline_required)
        {
            return Err(anyhow!(
                "QA performance budget baseline required but no baselineRefs provided"
            ));
        }

        // Declared status must match the computed classification.
        let computed = self.computed_status();
        if self.status != computed.as_str() {
            return Err(anyhow!(
                "QA performance budget status `{}` does not match computed classification `{}`",
                self.status,
                computed.as_str()
            ));
        }
        if matches!(
            computed,
            PerformanceBudgetStatus::Missing
                | PerformanceBudgetStatus::Unsupported
                | PerformanceBudgetStatus::Stale
        ) && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!(
                "QA performance budget {} status requires visible blockedReasons",
                computed.as_str()
            ));
        }

        require_text("QA performance budget boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "performance metrics",
            "evidence inputs",
            "not trusted truth",
            "no auto-fix",
            "no production performance guarantee",
            "review-gated",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "QA performance budget boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    fn validate_metrics(&self) -> Result<()> {
        let mut ids = BTreeSet::new();
        for metric in &self.metrics {
            require_id("QA performance budget metricId", &metric.metric_id)?;
            if !ids.insert(metric.metric_id.as_str()) {
                return Err(anyhow!(
                    "QA performance budget metricId `{}` is duplicated",
                    metric.metric_id
                ));
            }
            let is_timing = TIMING_METRIC_KINDS.contains(&metric.kind.as_str());
            let is_count = COUNT_METRIC_KINDS.contains(&metric.kind.as_str());
            if !is_timing && !is_count {
                return Err(anyhow!(
                    "QA performance budget unsupported metric kind `{}`",
                    metric.kind
                ));
            }
            if !TRUSTED_METRIC_SOURCES.contains(&metric.source.as_str())
                && !LOW_TRUST_METRIC_SOURCES.contains(&metric.source.as_str())
            {
                return Err(anyhow!(
                    "QA performance budget unsupported metric source `{}`",
                    metric.source
                ));
            }
            let expected_unit = if is_timing { "ms" } else { "count" };
            if metric.unit != expected_unit {
                return Err(anyhow!(
                    "QA performance budget metric `{}` unit must be `{expected_unit}`",
                    metric.metric_id
                ));
            }
            match (metric.present, metric.value) {
                (true, None) => {
                    return Err(anyhow!(
                        "QA performance budget present metric `{}` is malformed: missing value",
                        metric.metric_id
                    ));
                }
                (true, Some(value)) => {
                    if !value.is_finite() || value < 0.0 {
                        return Err(anyhow!(
                            "QA performance budget present metric `{}` is malformed: value must be finite and non-negative",
                            metric.metric_id
                        ));
                    }
                    if is_count && value.fract() != 0.0 {
                        return Err(anyhow!(
                            "QA performance budget count metric `{}` is malformed: value must be a whole number",
                            metric.metric_id
                        ));
                    }
                }
                (false, Some(_)) => {
                    return Err(anyhow!(
                        "QA performance budget absent metric `{}` must not carry a value",
                        metric.metric_id
                    ));
                }
                (false, None) => {}
            }
        }
        Ok(())
    }

    fn validate_thresholds(&self) -> Result<()> {
        let mut ids = BTreeSet::new();
        let metric_kinds: BTreeSet<&str> = self.metrics.iter().map(|m| m.kind.as_str()).collect();
        for threshold in &self.thresholds {
            require_id("QA performance budget thresholdId", &threshold.threshold_id)?;
            if !ids.insert(threshold.threshold_id.as_str()) {
                return Err(anyhow!(
                    "QA performance budget thresholdId `{}` is duplicated",
                    threshold.threshold_id
                ));
            }
            let is_timing = TIMING_METRIC_KINDS.contains(&threshold.metric_kind.as_str());
            let is_count = COUNT_METRIC_KINDS.contains(&threshold.metric_kind.as_str());
            if !is_timing && !is_count {
                return Err(anyhow!(
                    "QA performance budget threshold references unsupported metric kind `{}`",
                    threshold.metric_kind
                ));
            }
            if !metric_kinds.contains(threshold.metric_kind.as_str()) {
                return Err(anyhow!(
                    "QA performance budget threshold metric kind `{}` has no declared metric",
                    threshold.metric_kind
                ));
            }
            // Comparator is validated only for supported thresholds; an unsupported
            // threshold is recorded verbatim and never evaluated.
            if threshold.supported
                && !SUPPORTED_COMPARATORS.contains(&threshold.comparator.as_str())
            {
                return Err(anyhow!(
                    "QA performance budget unsupported comparator `{}`",
                    threshold.comparator
                ));
            }
            if !threshold.limit.is_finite() || threshold.limit < 0.0 {
                return Err(anyhow!(
                    "QA performance budget threshold `{}` limit must be finite and non-negative",
                    threshold.threshold_id
                ));
            }
        }
        Ok(())
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
        "production performance guarantee",
        "shipped-game",
        "hidden worker",
        "remote swarm",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden QA performance authority text `{forbidden}`"
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
        // A contrastive conjunction ends the preceding negation's scope so a
        // negated mention cannot whitelist a later positive mention in the same
        // sentence (e.g. `no auto-fix, but auto-fix enabled` fails closed),
        // while simple comma/or lists such as `no auto-apply or self-approval`
        // stay negated.
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
