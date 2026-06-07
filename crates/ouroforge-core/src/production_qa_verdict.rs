//! Crash/Accessibility QA and Consolidated Production-QA Verdict v1 (#1669).
//!
//! This module does not introduce a new evaluator. It composes the existing
//! per-check QA results — the regression matrix, visual-regression-at-scale,
//! performance/soak, crash/flaky classification, accessibility, and asset-UX
//! checks — into a single descriptive production-QA verdict per game build,
//! reusing the evaluator's `declared-gate-and` aggregation with a neutral
//! undeclared-gate policy. Each check references the existing per-check
//! artifact that produced it; the consolidated verdict is `pass` only when
//! every declared check passes and fails closed when any declared check fails.
//!
//! The verdict is descriptive evidence only — never a quality/fun guarantee,
//! and never a trusted mutation, auto-fix, auto-apply, or release authority.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const PRODUCTION_QA_VERDICT_SCHEMA_VERSION: &str = "production-qa-verdict-v1";

/// The QA functions composed into the consolidated verdict. Each is an existing
/// per-check capability, not a new engine.
const CHECK_KINDS: &[&str] = &[
    "regressionMatrix",
    "visualRegression",
    "performanceSoak",
    "crash",
    "flaky",
    "accessibility",
    "assetUx",
];

/// Per-check statuses. `skipped` marks an undeclared (neutral) check.
const CHECK_STATUSES: &[&str] = &["pass", "fail", "inconclusive", "skipped", "blocked"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaVerdictArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "verdictId")]
    pub verdict_id: String,
    /// Run-completion classification: `complete`, `blocked`, or `stale`.
    pub status: String,
    /// Consolidated verdict: `pass`, `fail`, `inconclusive`, or `blocked`.
    pub verdict: String,
    /// The whole-game build this verdict describes.
    #[serde(rename = "gameBuildRef")]
    pub game_build_ref: String,
    pub checks: Vec<ProductionQaCheck>,
    #[serde(rename = "staleEvidenceRefs", default)]
    pub stale_evidence_refs: Vec<String>,
    #[serde(rename = "dashboardCompat")]
    pub dashboard_compat: ProductionQaVerdictDashboardCompat,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaCheck {
    #[serde(rename = "checkId")]
    pub check_id: String,
    /// One of the composed QA functions (regressionMatrix, visualRegression,
    /// performanceSoak, crash, flaky, accessibility, assetUx).
    pub kind: String,
    /// Whether this check was declared/run for this build. Undeclared checks are
    /// neutral and do not affect the consolidated verdict (declared-gate-and).
    pub declared: bool,
    /// Per-check status (`pass`/`fail`/`inconclusive`/`blocked` when declared;
    /// `skipped` when undeclared).
    pub status: String,
    #[serde(rename = "failureCount", default)]
    pub failure_count: u32,
    /// Reference to the existing per-check artifact that produced this result
    /// (the reuse anchor). Required for declared checks.
    #[serde(rename = "evidenceRef", default)]
    pub evidence_ref: String,
    #[serde(rename = "evidenceRefs", default)]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaVerdictDashboardCompat {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub surface: String,
    pub columns: Vec<String>,
}

/// A declared check that did not pass, propagated into the consolidated verdict.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FailingCheck {
    #[serde(rename = "checkId")]
    pub check_id: String,
    pub kind: String,
    pub status: String,
    #[serde(rename = "failureCount")]
    pub failure_count: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionQaVerdictReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "verdictId")]
    pub verdict_id: String,
    pub status: String,
    pub verdict: String,
    #[serde(rename = "checkCount")]
    pub check_count: usize,
    #[serde(rename = "declaredCount")]
    pub declared_count: usize,
    #[serde(rename = "passedCount")]
    pub passed_count: usize,
    #[serde(rename = "statusCounts")]
    pub status_counts: BTreeMap<String, usize>,
    #[serde(rename = "failingChecks")]
    pub failing_checks: Vec<FailingCheck>,
    #[serde(rename = "aggregationOperator")]
    pub aggregation_operator: String,
    #[serde(rename = "undeclaredGatePolicy")]
    pub undeclared_gate_policy: String,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl ProductionQaVerdictArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse Production QA Verdict JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    /// Deterministically aggregates the checks, independent of input order.
    pub fn read_model(&self) -> ProductionQaVerdictReadModel {
        let mut status_counts = BTreeMap::new();
        for check in &self.checks {
            *status_counts.entry(check.status.clone()).or_insert(0) += 1;
        }
        let failing_checks = self.failing_checks();
        ProductionQaVerdictReadModel {
            schema_version: self.schema_version.clone(),
            verdict_id: self.verdict_id.clone(),
            status: self.computed_status(),
            verdict: self.computed_verdict(),
            check_count: self.checks.len(),
            declared_count: self.checks.iter().filter(|c| c.declared).count(),
            passed_count: self
                .checks
                .iter()
                .filter(|c| c.declared && c.status == "pass")
                .count(),
            status_counts,
            failing_checks,
            aggregation_operator: "declared-gate-and".to_string(),
            undeclared_gate_policy: "neutral".to_string(),
            blocked_count: self.blocked_count(),
            validation_summary: vec![
                "each check composes an existing per-check QA result (regression matrix, visual, performance/soak, crash, flaky, accessibility, asset-UX) and references the artifact that produced it".to_string(),
                "the consolidated verdict is declared-gate-and: pass only when every declared check passes, and fail-closed when any declared check fails; undeclared checks are neutral".to_string(),
                "unknown kinds, malformed statuses, declared checks missing evidence, declared/skipped mismatches, and a status/verdict mismatch fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "composes existing per-check results via the evaluator declared-gate-and aggregation; no new evaluator".to_string(),
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "the consolidated verdict is descriptive, not a quality/fun guarantee or a release authority".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize production QA verdict read model JSON")
    }

    /// Declared checks that did not pass, deterministically sorted.
    pub fn failing_checks(&self) -> Vec<FailingCheck> {
        let mut failing: Vec<FailingCheck> = self
            .checks
            .iter()
            .filter(|c| c.declared && c.status == "fail")
            .map(|c| FailingCheck {
                check_id: c.check_id.clone(),
                kind: c.kind.clone(),
                status: c.status.clone(),
                failure_count: c.failure_count,
            })
            .collect();
        failing.sort();
        failing
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .checks
                .iter()
                .filter(|c| !c.blocked_reasons.is_empty() || (c.declared && c.status == "blocked"))
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

    /// Composes the declared checks with `declared-gate-and` semantics.
    pub fn computed_verdict(&self) -> String {
        if self.computed_status() != "complete" {
            return "blocked".to_string();
        }
        let declared: Vec<&ProductionQaCheck> = self.checks.iter().filter(|c| c.declared).collect();
        if declared.is_empty() {
            return "inconclusive".to_string();
        }
        if declared.iter().any(|c| c.status == "fail") {
            return "fail".to_string();
        }
        if declared.iter().any(|c| c.status != "pass") {
            return "inconclusive".to_string();
        }
        "pass".to_string()
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCTION_QA_VERDICT_SCHEMA_VERSION {
            return Err(anyhow!(
                "production QA verdict schemaVersion must be {PRODUCTION_QA_VERDICT_SCHEMA_VERSION}"
            ));
        }
        require_id("production QA verdict verdictId", &self.verdict_id)?;
        require_ref("production QA verdict gameBuildRef", &self.game_build_ref)?;
        validate_ref_list(
            "production QA verdict staleEvidenceRefs",
            &self.stale_evidence_refs,
            false,
        )?;
        require_nonempty("production QA verdict checks", self.checks.len())?;
        if self.checks.len() > 256 {
            return Err(anyhow!("production QA verdict is overbroad for v1"));
        }

        self.dashboard_compat.validate()?;

        let mut check_ids = BTreeSet::new();
        for check in &self.checks {
            check.validate()?;
            if !check_ids.insert(check.check_id.as_str()) {
                return Err(anyhow!(
                    "production QA verdict duplicate check id `{}`",
                    check.check_id
                ));
            }
        }

        let computed_status = self.computed_status();
        if self.status != computed_status {
            return Err(anyhow!(
                "production QA verdict status `{}` does not match computed classification `{computed_status}`",
                self.status
            ));
        }
        if (computed_status == "stale" || computed_status == "blocked") && self.blocked_count() == 0
        {
            return Err(anyhow!(
                "production QA verdict {computed_status} status requires visible blockedReasons"
            ));
        }
        let computed_verdict = self.computed_verdict();
        if self.verdict != computed_verdict {
            return Err(anyhow!(
                "production QA verdict `{}` does not match computed verdict `{computed_verdict}`",
                self.verdict
            ));
        }

        validate_text_list(
            "production QA verdict blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        require_text("production QA verdict boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "not trusted truth",
            "no auto-fix",
            "review-gated",
            "read-only or draft-only",
            "composes existing checks",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "production QA verdict boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl ProductionQaCheck {
    fn validate(&self) -> Result<()> {
        require_id("production QA verdict checkId", &self.check_id)?;
        if !CHECK_KINDS.contains(&self.kind.as_str()) {
            return Err(anyhow!(
                "production QA verdict unknown check kind `{}`",
                self.kind
            ));
        }
        if !CHECK_STATUSES.contains(&self.status.as_str()) {
            return Err(anyhow!(
                "production QA verdict malformed check status `{}`",
                self.status
            ));
        }
        // An undeclared check is neutral and must be marked `skipped`; a declared
        // check must carry a real status and reference the per-check artifact.
        if self.declared {
            if self.status == "skipped" {
                return Err(anyhow!(
                    "production QA verdict declared check `{}` must not be skipped",
                    self.check_id
                ));
            }
            require_ref(
                "production QA verdict check evidenceRef",
                &self.evidence_ref,
            )?;
        } else if self.status != "skipped" {
            return Err(anyhow!(
                "production QA verdict undeclared check `{}` must be skipped (neutral)",
                self.check_id
            ));
        }
        validate_ref_list(
            "production QA verdict check evidenceRefs",
            &self.evidence_refs,
            false,
        )?;
        validate_text_list(
            "production QA verdict check blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        Ok(())
    }
}

impl ProductionQaVerdictDashboardCompat {
    fn validate(&self) -> Result<()> {
        if !self.read_only {
            return Err(anyhow!(
                "production QA verdict dashboard surface must remain read-only or draft-only"
            ));
        }
        require_text(
            "production QA verdict dashboardCompat.surface",
            &self.surface,
        )?;
        validate_text_list(
            "production QA verdict dashboardCompat.columns",
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
                "{field} contains forbidden production QA verdict authority text `{forbidden}`"
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

    fn check(id: &str, kind: &str, declared: bool, status: &str) -> ProductionQaCheck {
        ProductionQaCheck {
            check_id: id.to_string(),
            kind: kind.to_string(),
            declared,
            status: status.to_string(),
            failure_count: if status == "fail" { 1 } else { 0 },
            evidence_ref: if declared {
                "evidence/qa/check.json".to_string()
            } else {
                String::new()
            },
            evidence_refs: Vec::new(),
            blocked_reasons: Vec::new(),
        }
    }

    fn artifact(checks: Vec<ProductionQaCheck>) -> ProductionQaVerdictArtifact {
        let mut artifact = ProductionQaVerdictArtifact {
            schema_version: PRODUCTION_QA_VERDICT_SCHEMA_VERSION.to_string(),
            verdict_id: "verdict-001".to_string(),
            status: "complete".to_string(),
            verdict: "pass".to_string(),
            game_build_ref: "runs/build-001".to_string(),
            checks,
            stale_evidence_refs: Vec::new(),
            dashboard_compat: ProductionQaVerdictDashboardCompat {
                read_only: true,
                surface: "evidence-dashboard".to_string(),
                columns: vec!["kind".to_string(), "status".to_string()],
            },
            blocked_reasons: Vec::new(),
            boundary:
                "Consolidated checks are evidence inputs, not trusted truth; no auto-fix; review-gated; dashboards stay read-only or draft-only; composes existing checks via declared-gate-and"
                    .to_string(),
        };
        artifact.verdict = artifact.computed_verdict();
        artifact
    }

    #[test]
    fn all_declared_pass_is_consolidated_pass() {
        let checks = vec![
            check("c1", "regressionMatrix", true, "pass"),
            check("c2", "visualRegression", true, "pass"),
            check("c3", "performanceSoak", true, "pass"),
            check("c4", "crash", true, "pass"),
            check("c5", "accessibility", true, "pass"),
            check("c6", "assetUx", false, "skipped"),
        ];
        let artifact = artifact(checks);
        artifact.validate().expect("valid");
        assert_eq!(artifact.computed_verdict(), "pass");
        // Undeclared assetUx check is neutral.
        assert_eq!(artifact.read_model().declared_count, 5);
    }

    #[test]
    fn any_declared_failure_propagates() {
        let checks = vec![
            check("c1", "regressionMatrix", true, "pass"),
            check("c2", "crash", true, "fail"),
        ];
        let artifact = artifact(checks);
        artifact.validate().expect("valid");
        assert_eq!(artifact.computed_verdict(), "fail");
        assert_eq!(artifact.failing_checks().len(), 1);
        assert_eq!(artifact.failing_checks()[0].kind, "crash");
    }

    #[test]
    fn declared_inconclusive_blocks_pass() {
        let checks = vec![
            check("c1", "regressionMatrix", true, "pass"),
            check("c2", "flaky", true, "inconclusive"),
        ];
        let artifact = artifact(checks);
        artifact.validate().expect("valid");
        assert_eq!(artifact.computed_verdict(), "inconclusive");
    }

    #[test]
    fn read_model_is_order_independent() {
        let checks = vec![
            check("c1", "regressionMatrix", true, "pass"),
            check("c2", "crash", true, "fail"),
            check("c3", "assetUx", false, "skipped"),
        ];
        let forward = artifact(checks.clone());
        let mut reversed = checks;
        reversed.reverse();
        let reversed = artifact(reversed);
        assert_eq!(
            forward.read_model_json().unwrap(),
            reversed.read_model_json().unwrap()
        );
    }
}
