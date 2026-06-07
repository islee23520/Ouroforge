//! Visual-Regression at Scale v1 (#1667) — scales the existing visual gate
//! across many screens and content variants.
//!
//! This module does not introduce a new image-comparison engine. Each screen
//! cell reuses the evaluator's [`VisualComparisonOutcome`] (the visual gate's
//! per-screenshot verdict) and references the existing
//! `visual-comparison-evidence-v1` artifact that produced it. The aggregation
//! rolls the per-screen outcomes up into a descriptive read model: matched
//! baselines, detected visual regressions (a screen whose comparison `changed`
//! against its baseline), and explicitly-surfaced missing baselines. It
//! performs no trusted mutation, auto-fix, auto-apply, or quality judgement;
//! outputs are evidence inputs only and remain review-gated. "Looks good" stays
//! a human decision.

use anyhow::{anyhow, Context, Result};
use ouroforge_evaluator::VisualComparisonOutcome;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const VISUAL_REGRESSION_SCALE_SCHEMA_VERSION: &str = "visual-regression-scale-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualRegressionScaleArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "suiteId")]
    pub suite_id: String,
    pub status: String,
    /// The whole-game build whose screens are under visual QA.
    #[serde(rename = "gameBuildRef")]
    pub game_build_ref: String,
    /// The reused baseline screenshot set (existing visual gate baselines).
    #[serde(rename = "baselineSetRef")]
    pub baseline_set_ref: String,
    pub screens: Vec<VisualRegressionScreen>,
    #[serde(rename = "staleEvidenceRefs", default)]
    pub stale_evidence_refs: Vec<String>,
    #[serde(rename = "dashboardCompat")]
    pub dashboard_compat: VisualRegressionDashboardCompat,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualRegressionScreen {
    #[serde(rename = "screenId")]
    pub screen_id: String,
    #[serde(rename = "contentVariant")]
    pub content_variant: String,
    /// The visual gate's per-screenshot outcome, reused verbatim from the
    /// evaluator.
    pub outcome: VisualComparisonOutcome,
    /// Reference to the existing `visual-comparison-evidence-v1` artifact that
    /// produced this outcome. This is the reuse anchor: the suite scales the
    /// existing visual gate, never a new comparison engine.
    #[serde(rename = "visualEvidenceRef")]
    pub visual_evidence_ref: String,
    #[serde(rename = "evidenceRefs", default)]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualRegressionDashboardCompat {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub surface: String,
    pub columns: Vec<String>,
}

/// A detected visual regression: a screen whose comparison `changed` against
/// its baseline. Descriptive evidence only.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisualRegressionHit {
    #[serde(rename = "screenId")]
    pub screen_id: String,
    #[serde(rename = "contentVariant")]
    pub content_variant: String,
    #[serde(rename = "visualEvidenceRef")]
    pub visual_evidence_ref: String,
}

/// A screen whose baseline (or current screenshot) was missing, surfaced
/// explicitly rather than silently treated as a pass.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MissingBaselineScreen {
    #[serde(rename = "screenId")]
    pub screen_id: String,
    #[serde(rename = "contentVariant")]
    pub content_variant: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualRegressionScaleReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "suiteId")]
    pub suite_id: String,
    pub status: String,
    #[serde(rename = "screenCount")]
    pub screen_count: usize,
    #[serde(rename = "matchedCount")]
    pub matched_count: usize,
    #[serde(rename = "outcomeCounts")]
    pub outcome_counts: BTreeMap<String, usize>,
    #[serde(rename = "regressions")]
    pub regressions: Vec<VisualRegressionHit>,
    #[serde(rename = "regressionCount")]
    pub regression_count: usize,
    #[serde(rename = "missingBaselines")]
    pub missing_baselines: Vec<MissingBaselineScreen>,
    #[serde(rename = "missingBaselineCount")]
    pub missing_baseline_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

/// JSON token for a [`VisualComparisonOutcome`] (snake_case, matching the
/// evaluator's serde representation).
fn outcome_token(outcome: VisualComparisonOutcome) -> &'static str {
    match outcome {
        VisualComparisonOutcome::Unchanged => "unchanged",
        VisualComparisonOutcome::Changed => "changed",
        VisualComparisonOutcome::MissingScreenshot => "missing_screenshot",
        VisualComparisonOutcome::MalformedScreenshot => "malformed_screenshot",
        VisualComparisonOutcome::MismatchedDimensions => "mismatched_dimensions",
        VisualComparisonOutcome::Unsupported => "unsupported",
        VisualComparisonOutcome::Blocked => "blocked",
    }
}

impl VisualRegressionScaleArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse Visual Regression Scale JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    /// Deterministically aggregates the screens, independent of their order.
    pub fn read_model(&self) -> VisualRegressionScaleReadModel {
        let mut outcome_counts = BTreeMap::new();
        for screen in &self.screens {
            *outcome_counts
                .entry(outcome_token(screen.outcome).to_string())
                .or_insert(0) += 1;
        }
        let regressions = self.detect_regressions();
        let missing_baselines = self.missing_baselines();
        VisualRegressionScaleReadModel {
            schema_version: self.schema_version.clone(),
            suite_id: self.suite_id.clone(),
            status: self.computed_status(),
            screen_count: self.screens.len(),
            matched_count: self
                .screens
                .iter()
                .filter(|s| s.outcome == VisualComparisonOutcome::Unchanged)
                .count(),
            outcome_counts,
            regression_count: regressions.len(),
            regressions,
            missing_baseline_count: missing_baselines.len(),
            missing_baselines,
            blocked_count: self.blocked_count(),
            validation_summary: vec![
                "each screen reuses the evaluator visual comparison outcome, references the visual-comparison-evidence artifact that produced it, and carries replayable evidence refs".to_string(),
                "a changed comparison is a detected visual regression; a missing screenshot/baseline is surfaced explicitly, never treated as a silent pass".to_string(),
                "duplicate screen ids, duplicate screen/variant coordinates, missing visual evidence refs, and changed/unchanged screens without evidence fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "scales the existing visual gate across screens and content variants; no new comparison engine".to_string(),
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "dashboard/Studio surfaces remain read-only or draft-only; the verdict is descriptive — \"looks good\" stays a human decision".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize visual regression scale read model JSON")
    }

    /// Detected visual regressions: screens whose comparison `changed` against
    /// the baseline. Deterministically sorted.
    pub fn detect_regressions(&self) -> Vec<VisualRegressionHit> {
        let mut hits: Vec<VisualRegressionHit> = self
            .screens
            .iter()
            .filter(|s| s.outcome == VisualComparisonOutcome::Changed)
            .map(|s| VisualRegressionHit {
                screen_id: s.screen_id.clone(),
                content_variant: s.content_variant.clone(),
                visual_evidence_ref: s.visual_evidence_ref.clone(),
            })
            .collect();
        hits.sort();
        hits
    }

    /// Screens with a missing baseline/screenshot, surfaced explicitly.
    pub fn missing_baselines(&self) -> Vec<MissingBaselineScreen> {
        let mut missing: Vec<MissingBaselineScreen> = self
            .screens
            .iter()
            .filter(|s| s.outcome == VisualComparisonOutcome::MissingScreenshot)
            .map(|s| MissingBaselineScreen {
                screen_id: s.screen_id.clone(),
                content_variant: s.content_variant.clone(),
            })
            .collect();
        missing.sort();
        missing
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .screens
                .iter()
                .filter(|s| {
                    !s.blocked_reasons.is_empty() || s.outcome == VisualComparisonOutcome::Blocked
                })
                .count()
    }

    pub fn computed_status(&self) -> String {
        if !self.stale_evidence_refs.is_empty() {
            return "stale".to_string();
        }
        if !self.blocked_reasons.is_empty()
            || self.screens.iter().any(|s| {
                !s.blocked_reasons.is_empty() || s.outcome == VisualComparisonOutcome::Blocked
            })
        {
            return "blocked".to_string();
        }
        let all_resolved = self.screens.iter().all(|s| {
            matches!(
                s.outcome,
                VisualComparisonOutcome::Unchanged | VisualComparisonOutcome::Changed
            )
        });
        if all_resolved {
            "complete".to_string()
        } else {
            "partial".to_string()
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != VISUAL_REGRESSION_SCALE_SCHEMA_VERSION {
            return Err(anyhow!(
                "visual regression scale schemaVersion must be {VISUAL_REGRESSION_SCALE_SCHEMA_VERSION}"
            ));
        }
        require_id("visual regression scale suiteId", &self.suite_id)?;
        require_ref("visual regression scale gameBuildRef", &self.game_build_ref)?;
        require_ref(
            "visual regression scale baselineSetRef",
            &self.baseline_set_ref,
        )?;
        validate_ref_list(
            "visual regression scale staleEvidenceRefs",
            &self.stale_evidence_refs,
            false,
        )?;
        require_nonempty("visual regression scale screens", self.screens.len())?;
        if self.screens.len() > 1024 {
            return Err(anyhow!("visual regression scale suite is overbroad for v1"));
        }

        self.dashboard_compat.validate()?;

        // The same screen is legitimately compared across content variants, so
        // the unique key is the (screenId, contentVariant) coordinate, not the
        // screen id alone.
        let mut coords = BTreeSet::new();
        for screen in &self.screens {
            screen.validate()?;
            let coord = (screen.screen_id.as_str(), screen.content_variant.as_str());
            if !coords.insert(coord) {
                return Err(anyhow!(
                    "visual regression scale duplicate screen/variant coordinate"
                ));
            }
        }

        let computed = self.computed_status();
        if self.status != computed {
            return Err(anyhow!(
                "visual regression scale status `{}` does not match computed classification `{computed}`",
                self.status
            ));
        }
        if (computed == "stale" || computed == "blocked") && self.blocked_count() == 0 {
            return Err(anyhow!(
                "visual regression scale {computed} status requires visible blockedReasons"
            ));
        }

        validate_text_list(
            "visual regression scale blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        require_text("visual regression scale boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "not trusted truth",
            "no auto-fix",
            "review-gated",
            "read-only or draft-only",
            "reuses the visual gate",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "visual regression scale boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl VisualRegressionScreen {
    fn validate(&self) -> Result<()> {
        require_id("visual regression scale screenId", &self.screen_id)?;
        require_id(
            "visual regression scale contentVariant",
            &self.content_variant,
        )?;
        require_ref(
            "visual regression scale visualEvidenceRef",
            &self.visual_evidence_ref,
        )?;
        validate_ref_list(
            "visual regression scale evidenceRefs",
            &self.evidence_refs,
            false,
        )?;
        // A matched or regressed screen must carry replayable evidence; missing,
        // unsupported, malformed, mismatched, and blocked screens may not.
        match self.outcome {
            VisualComparisonOutcome::Unchanged | VisualComparisonOutcome::Changed
                if self.evidence_refs.is_empty() =>
            {
                return Err(anyhow!(
                    "visual regression scale screen `{}` is missing evidence for outcome `{}`",
                    self.screen_id,
                    outcome_token(self.outcome)
                ));
            }
            _ => {}
        }
        validate_text_list(
            "visual regression scale screen blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        Ok(())
    }
}

impl VisualRegressionDashboardCompat {
    fn validate(&self) -> Result<()> {
        if !self.read_only {
            return Err(anyhow!(
                "visual regression scale dashboard surface must remain read-only or draft-only"
            ));
        }
        require_text(
            "visual regression scale dashboardCompat.surface",
            &self.surface,
        )?;
        validate_text_list(
            "visual regression scale dashboardCompat.columns",
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
        "looks good",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden visual regression scale authority text `{forbidden}`"
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

    fn screen(
        id: &str,
        variant: &str,
        outcome: VisualComparisonOutcome,
        evidence: &[&str],
    ) -> VisualRegressionScreen {
        VisualRegressionScreen {
            screen_id: id.to_string(),
            content_variant: variant.to_string(),
            outcome,
            visual_evidence_ref: "evidence/visual/cmp.json".to_string(),
            evidence_refs: evidence.iter().map(|s| s.to_string()).collect(),
            blocked_reasons: Vec::new(),
        }
    }

    fn artifact(
        screens: Vec<VisualRegressionScreen>,
        status: &str,
    ) -> VisualRegressionScaleArtifact {
        VisualRegressionScaleArtifact {
            schema_version: VISUAL_REGRESSION_SCALE_SCHEMA_VERSION.to_string(),
            suite_id: "suite-001".to_string(),
            status: status.to_string(),
            game_build_ref: "runs/build-001".to_string(),
            baseline_set_ref: "examples/visual-comparison-evidence-v1".to_string(),
            screens,
            stale_evidence_refs: Vec::new(),
            dashboard_compat: VisualRegressionDashboardCompat {
                read_only: true,
                surface: "evidence-dashboard".to_string(),
                columns: vec!["screen".to_string(), "outcome".to_string()],
            },
            blocked_reasons: Vec::new(),
            boundary:
                "Visual regression cells are evidence inputs, not trusted truth; no auto-fix; review-gated; dashboards stay read-only or draft-only; reuses the visual gate"
                    .to_string(),
        }
    }

    #[test]
    fn baseline_match_has_no_regression() {
        let screens = vec![
            screen(
                "title",
                "base",
                VisualComparisonOutcome::Unchanged,
                &["runs/a"],
            ),
            screen(
                "hud",
                "base",
                VisualComparisonOutcome::Unchanged,
                &["runs/b"],
            ),
        ];
        let artifact = artifact(screens, "complete");
        artifact.validate().expect("valid");
        assert_eq!(artifact.detect_regressions().len(), 0);
        assert_eq!(artifact.read_model().matched_count, 2);
    }

    #[test]
    fn planted_visual_diff_is_detected() {
        let screens = vec![
            screen(
                "title",
                "base",
                VisualComparisonOutcome::Unchanged,
                &["runs/a"],
            ),
            screen(
                "hud",
                "variant-a",
                VisualComparisonOutcome::Changed,
                &["runs/b"],
            ),
        ];
        let artifact = artifact(screens, "complete");
        artifact.validate().expect("valid");
        let hits = artifact.detect_regressions();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].screen_id, "hud");
        assert_eq!(hits[0].content_variant, "variant-a");
    }

    #[test]
    fn missing_baseline_is_surfaced_not_passed() {
        let screens = vec![
            screen(
                "title",
                "base",
                VisualComparisonOutcome::Unchanged,
                &["runs/a"],
            ),
            screen(
                "new",
                "base",
                VisualComparisonOutcome::MissingScreenshot,
                &[],
            ),
        ];
        let artifact = artifact(screens, "partial");
        artifact.validate().expect("valid");
        let read_model = artifact.read_model();
        assert_eq!(read_model.status, "partial");
        assert_eq!(read_model.missing_baseline_count, 1);
        assert_eq!(read_model.missing_baselines[0].screen_id, "new");
        // A missing baseline is not counted as a match or a regression.
        assert_eq!(read_model.matched_count, 1);
        assert_eq!(read_model.regression_count, 0);
    }

    #[test]
    fn read_model_is_order_independent() {
        let screens = vec![
            screen(
                "title",
                "base",
                VisualComparisonOutcome::Unchanged,
                &["runs/a"],
            ),
            screen(
                "hud",
                "variant-a",
                VisualComparisonOutcome::Changed,
                &["runs/b"],
            ),
            screen(
                "menu",
                "base",
                VisualComparisonOutcome::MissingScreenshot,
                &[],
            ),
        ];
        let forward = artifact(screens.clone(), "partial");
        let mut reversed = screens;
        reversed.reverse();
        let reversed = artifact(reversed, "partial");
        assert_eq!(
            forward.read_model_json().unwrap(),
            reversed.read_model_json().unwrap()
        );
    }
}
