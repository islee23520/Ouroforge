//! Before/After Comparison Artifact v1 (#2380, #1 M126).
//!
//! Deterministically compares two evidence bundle summaries and emits a verdict
//! over semantic/gameplay dimensions. This module is data-only: it does not run
//! browsers, compute pixel diffs, mutate files, apply patches, or execute
//! commands. Screenshot differences are referenced as artifact links even when
//! pixel-diff is unavailable or unstable.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const BEFORE_AFTER_COMPARISON_SCHEMA_VERSION: &str = "before-after-comparison-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BeforeAfterVerdict {
    Improvement,
    Regression,
    NoChange,
    Inconclusive,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonDimensionKind {
    Flags,
    Events,
    Screenshots,
    ConsoleDiagnostics,
    FrameStats,
    ReplayResult,
    KnownGaps,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BeforeAfterEvidenceRef {
    #[serde(rename = "runId")]
    pub run_id: String,
    pub path: String,
    pub digest: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceBundleSummary {
    #[serde(rename = "runId")]
    pub run_id: String,
    pub artifacts: Vec<BeforeAfterEvidenceRef>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub flags: BTreeMap<String, bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub screenshots: Vec<BeforeAfterEvidenceRef>,
    #[serde(
        rename = "consoleDiagnostics",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub console_diagnostics: Vec<String>,
    #[serde(
        rename = "frameStats",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_stats: Option<FrameStatsSummary>,
    #[serde(
        rename = "replayResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replay_result: Option<ReplayResultSummary>,
    #[serde(rename = "knownGaps", default, skip_serializing_if = "Vec::is_empty")]
    pub known_gaps: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FrameStatsSummary {
    #[serde(rename = "avgFrameMs")]
    pub avg_frame_ms: u32,
    #[serde(rename = "maxFrameMs")]
    pub max_frame_ms: u32,
    #[serde(rename = "droppedFrames")]
    pub dropped_frames: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReplayResultSummary {
    pub passed: bool,
    #[serde(rename = "finalStateDigest")]
    pub final_state_digest: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BeforeAfterComparisonInput {
    #[serde(rename = "comparisonId")]
    pub comparison_id: String,
    pub before: EvidenceBundleSummary,
    pub after: EvidenceBundleSummary,
    #[serde(rename = "reviewedChangeRef")]
    pub reviewed_change_ref: BeforeAfterEvidenceRef,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BeforeAfterComparisonArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "comparisonId")]
    pub comparison_id: String,
    pub verdict: BeforeAfterVerdict,
    #[serde(rename = "beforeRunId")]
    pub before_run_id: String,
    #[serde(rename = "afterRunId")]
    pub after_run_id: String,
    pub dimensions: Vec<ComparisonDimension>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<BeforeAfterEvidenceRef>,
    #[serde(rename = "knownGaps")]
    pub known_gaps: Vec<String>,
    #[serde(rename = "m127JournalReady")]
    pub m127_journal_ready: bool,
    #[serde(rename = "determinismKey")]
    pub determinism_key: String,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ComparisonDimension {
    pub kind: ComparisonDimensionKind,
    pub verdict: BeforeAfterVerdict,
    pub summary: String,
    #[serde(rename = "beforeRefs")]
    pub before_refs: Vec<BeforeAfterEvidenceRef>,
    #[serde(rename = "afterRefs")]
    pub after_refs: Vec<BeforeAfterEvidenceRef>,
}

impl BeforeAfterComparisonInput {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse before/after comparison input JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        require_local_id("before/after comparisonId", &self.comparison_id)?;
        self.before.validate("before bundle")?;
        self.after.validate("after bundle")?;
        if self.before.run_id == self.after.run_id {
            return Err(anyhow!(
                "before/after comparison requires distinct before and after run ids"
            ));
        }
        self.reviewed_change_ref.validate("reviewedChangeRef")?;
        require_nonempty("before/after comparison guardrails", self.guardrails.len())?;
        for guardrail in &self.guardrails {
            require_boundary_text("before/after comparison guardrail", guardrail)?;
        }
        Ok(())
    }

    pub fn compare(&self) -> Result<BeforeAfterComparisonArtifact> {
        self.validate()?;
        let dimensions = vec![
            compare_flags(&self.before, &self.after),
            compare_events(&self.before, &self.after),
            compare_screenshots(&self.before, &self.after),
            compare_console(&self.before, &self.after),
            compare_frame_stats(&self.before, &self.after),
            compare_replay(&self.before, &self.after),
            compare_known_gaps(&self.before, &self.after),
        ];
        let verdict = summarize_verdict(&dimensions);
        let mut evidence_refs = Vec::new();
        evidence_refs.push(self.reviewed_change_ref.clone());
        evidence_refs.extend(self.before.artifacts.iter().cloned());
        evidence_refs.extend(self.after.artifacts.iter().cloned());
        evidence_refs.extend(self.before.screenshots.iter().cloned());
        evidence_refs.extend(self.after.screenshots.iter().cloned());
        evidence_refs
            .sort_by(|a, b| (&a.run_id, &a.path, &a.digest).cmp(&(&b.run_id, &b.path, &b.digest)));
        evidence_refs.dedup();
        let known_gaps = sorted_union(&self.before.known_gaps, &self.after.known_gaps);
        let determinism_key = deterministic_key(&self.comparison_id, &self.before, &self.after);
        Ok(BeforeAfterComparisonArtifact {
            schema_version: BEFORE_AFTER_COMPARISON_SCHEMA_VERSION.to_string(),
            comparison_id: self.comparison_id.clone(),
            verdict,
            before_run_id: self.before.run_id.clone(),
            after_run_id: self.after.run_id.clone(),
            dimensions,
            evidence_refs,
            known_gaps,
            m127_journal_ready: true,
            determinism_key,
            forbidden_actions: forbidden_actions(),
        })
    }
}

fn compare_flags(
    before: &EvidenceBundleSummary,
    after: &EvidenceBundleSummary,
) -> ComparisonDimension {
    let before_pass = *before.flags.get("scenario_passed").unwrap_or(&false);
    let after_pass = *after.flags.get("scenario_passed").unwrap_or(&false);
    let verdict = match (before_pass, after_pass) {
        (false, true) => BeforeAfterVerdict::Improvement,
        (true, false) => BeforeAfterVerdict::Regression,
        _ if before.flags == after.flags => BeforeAfterVerdict::NoChange,
        _ => BeforeAfterVerdict::Inconclusive,
    };
    ComparisonDimension {
        kind: ComparisonDimensionKind::Flags,
        verdict,
        summary: format!(
            "flag changes before={} after={}",
            before.flags.len(),
            after.flags.len()
        ),
        before_refs: refs_for_kind(before, "flags"),
        after_refs: refs_for_kind(after, "flags"),
    }
}

fn compare_events(
    before: &EvidenceBundleSummary,
    after: &EvidenceBundleSummary,
) -> ComparisonDimension {
    let before_errors = count_prefixed(&before.events, "error.");
    let after_errors = count_prefixed(&after.events, "error.");
    let before_success = count_prefixed(&before.events, "success.");
    let after_success = count_prefixed(&after.events, "success.");
    let verdict = if after_errors < before_errors || after_success > before_success {
        BeforeAfterVerdict::Improvement
    } else if after_errors > before_errors || after_success < before_success {
        BeforeAfterVerdict::Regression
    } else if canonical_vec(&before.events) == canonical_vec(&after.events) {
        BeforeAfterVerdict::NoChange
    } else {
        BeforeAfterVerdict::Inconclusive
    };
    ComparisonDimension {
        kind: ComparisonDimensionKind::Events,
        verdict,
        summary: format!(
            "events before={} after={} error_delta={} success_delta={}",
            before.events.len(),
            after.events.len(),
            after_errors as i64 - before_errors as i64,
            after_success as i64 - before_success as i64
        ),
        before_refs: refs_for_kind(before, "events"),
        after_refs: refs_for_kind(after, "events"),
    }
}

fn compare_screenshots(
    before: &EvidenceBundleSummary,
    after: &EvidenceBundleSummary,
) -> ComparisonDimension {
    let verdict = if before.screenshots.is_empty() || after.screenshots.is_empty() {
        BeforeAfterVerdict::Inconclusive
    } else if before.screenshots == after.screenshots {
        BeforeAfterVerdict::NoChange
    } else {
        BeforeAfterVerdict::Inconclusive
    };
    ComparisonDimension {
        kind: ComparisonDimensionKind::Screenshots,
        verdict,
        summary: "screenshot refs recorded; pixel diff not required for this semantic comparison"
            .to_string(),
        before_refs: before.screenshots.clone(),
        after_refs: after.screenshots.clone(),
    }
}

fn compare_console(
    before: &EvidenceBundleSummary,
    after: &EvidenceBundleSummary,
) -> ComparisonDimension {
    let before_errors = count_prefixed(&before.console_diagnostics, "error");
    let after_errors = count_prefixed(&after.console_diagnostics, "error");
    let verdict = if after_errors < before_errors {
        BeforeAfterVerdict::Improvement
    } else if after_errors > before_errors {
        BeforeAfterVerdict::Regression
    } else if canonical_vec(&before.console_diagnostics)
        == canonical_vec(&after.console_diagnostics)
    {
        BeforeAfterVerdict::NoChange
    } else {
        BeforeAfterVerdict::Inconclusive
    };
    ComparisonDimension {
        kind: ComparisonDimensionKind::ConsoleDiagnostics,
        verdict,
        summary: format!("console error count before={before_errors} after={after_errors}"),
        before_refs: refs_for_kind(before, "console"),
        after_refs: refs_for_kind(after, "console"),
    }
}

fn compare_frame_stats(
    before: &EvidenceBundleSummary,
    after: &EvidenceBundleSummary,
) -> ComparisonDimension {
    let verdict = match (&before.frame_stats, &after.frame_stats) {
        (Some(before), Some(after)) => {
            if after.max_frame_ms < before.max_frame_ms
                && after.dropped_frames <= before.dropped_frames
            {
                BeforeAfterVerdict::Improvement
            } else if after.max_frame_ms > before.max_frame_ms
                || after.dropped_frames > before.dropped_frames
            {
                BeforeAfterVerdict::Regression
            } else if before == after {
                BeforeAfterVerdict::NoChange
            } else {
                BeforeAfterVerdict::Inconclusive
            }
        }
        _ => BeforeAfterVerdict::Inconclusive,
    };
    ComparisonDimension {
        kind: ComparisonDimensionKind::FrameStats,
        verdict,
        summary: format!(
            "frame stats before={:?} after={:?}",
            before.frame_stats, after.frame_stats
        ),
        before_refs: refs_for_kind(before, "frame"),
        after_refs: refs_for_kind(after, "frame"),
    }
}

fn compare_replay(
    before: &EvidenceBundleSummary,
    after: &EvidenceBundleSummary,
) -> ComparisonDimension {
    let verdict = match (&before.replay_result, &after.replay_result) {
        (Some(before), Some(after)) => match (before.passed, after.passed) {
            (false, true) => BeforeAfterVerdict::Improvement,
            (true, false) => BeforeAfterVerdict::Regression,
            _ if before == after => BeforeAfterVerdict::NoChange,
            _ => BeforeAfterVerdict::Inconclusive,
        },
        _ => BeforeAfterVerdict::Inconclusive,
    };
    ComparisonDimension {
        kind: ComparisonDimensionKind::ReplayResult,
        verdict,
        summary: format!(
            "replay before={:?} after={:?}",
            before.replay_result, after.replay_result
        ),
        before_refs: refs_for_kind(before, "replay"),
        after_refs: refs_for_kind(after, "replay"),
    }
}

fn compare_known_gaps(
    before: &EvidenceBundleSummary,
    after: &EvidenceBundleSummary,
) -> ComparisonDimension {
    let before_set: BTreeSet<_> = before.known_gaps.iter().collect();
    let after_set: BTreeSet<_> = after.known_gaps.iter().collect();
    let verdict = if after_set.len() < before_set.len() && after_set.is_subset(&before_set) {
        BeforeAfterVerdict::Improvement
    } else if after_set.len() > before_set.len() && before_set.is_subset(&after_set) {
        BeforeAfterVerdict::Regression
    } else if before_set == after_set {
        BeforeAfterVerdict::NoChange
    } else {
        BeforeAfterVerdict::Inconclusive
    };
    ComparisonDimension {
        kind: ComparisonDimensionKind::KnownGaps,
        verdict,
        summary: format!(
            "known gaps before={} after={}",
            before.known_gaps.len(),
            after.known_gaps.len()
        ),
        before_refs: refs_for_kind(before, "gaps"),
        after_refs: refs_for_kind(after, "gaps"),
    }
}

fn summarize_verdict(dimensions: &[ComparisonDimension]) -> BeforeAfterVerdict {
    let regressions = dimensions
        .iter()
        .filter(|d| d.verdict == BeforeAfterVerdict::Regression)
        .count();
    let improvements = dimensions
        .iter()
        .filter(|d| d.verdict == BeforeAfterVerdict::Improvement)
        .count();
    let inconclusive = dimensions
        .iter()
        .filter(|d| d.verdict == BeforeAfterVerdict::Inconclusive)
        .count();
    if regressions > 0 {
        BeforeAfterVerdict::Regression
    } else if improvements > 0 {
        BeforeAfterVerdict::Improvement
    } else if inconclusive > 0 {
        BeforeAfterVerdict::Inconclusive
    } else {
        BeforeAfterVerdict::NoChange
    }
}

fn refs_for_kind(bundle: &EvidenceBundleSummary, needle: &str) -> Vec<BeforeAfterEvidenceRef> {
    bundle
        .artifacts
        .iter()
        .filter(|reference| reference.path.contains(needle))
        .cloned()
        .collect()
}

fn deterministic_key(
    id: &str,
    before: &EvidenceBundleSummary,
    after: &EvidenceBundleSummary,
) -> String {
    let material = format!(
        "{}|{}|{}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
        id,
        before.run_id,
        after.run_id,
        before.flags,
        after.flags,
        canonical_vec(&before.events),
        canonical_vec(&after.events),
        before.frame_stats,
        after.frame_stats
    );
    fnv1a64_utf8_digest(&material)
}

fn fnv1a64_utf8_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

fn sorted_union(left: &[String], right: &[String]) -> Vec<String> {
    left.iter()
        .chain(right.iter())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn canonical_vec(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort();
    values
}

fn count_prefixed(values: &[String], prefix: &str) -> usize {
    values
        .iter()
        .filter(|value| value.starts_with(prefix))
        .count()
}

fn forbidden_actions() -> Vec<String> {
    vec![
        "run_browser".to_string(),
        "execute_command".to_string(),
        "pixel_diff_claim_without_artifact".to_string(),
        "mutate_source".to_string(),
        "auto_apply".to_string(),
        "auto_merge".to_string(),
    ]
}

impl EvidenceBundleSummary {
    fn validate(&self, label: &str) -> Result<()> {
        require_local_id(&format!("{label} runId"), &self.run_id)?;
        require_nonempty(&format!("{label} artifacts"), self.artifacts.len())?;
        for artifact in &self.artifacts {
            artifact.validate(&format!("{label} artifact"))?;
        }
        for screenshot in &self.screenshots {
            screenshot.validate(&format!("{label} screenshot"))?;
        }
        for event in &self.events {
            require_local_id(&format!("{label} event"), event)?;
        }
        for diagnostic in &self.console_diagnostics {
            require_boundary_text(&format!("{label} console diagnostic"), diagnostic)?;
        }
        if let Some(replay) = &self.replay_result {
            replay.validate(label)?;
        }
        for gap in &self.known_gaps {
            require_boundary_text(&format!("{label} known gap"), gap)?;
        }
        Ok(())
    }
}

impl ReplayResultSummary {
    fn validate(&self, label: &str) -> Result<()> {
        require_digest(
            &format!("{label} replay finalStateDigest"),
            &self.final_state_digest,
        )?;
        for failure in &self.failures {
            require_boundary_text(&format!("{label} replay failure"), failure)?;
        }
        Ok(())
    }
}

impl BeforeAfterEvidenceRef {
    fn validate(&self, label: &str) -> Result<()> {
        require_local_id(&format!("{label} runId"), &self.run_id)?;
        require_relative_path(&format!("{label} path"), &self.path)?;
        require_digest(&format!("{label} digest"), &self.digest)
    }
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if value.len() > 160
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, dot, or colon"
        ));
    }
    Ok(())
}

fn require_relative_path(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} must stay inside the local artifact root"));
    }
    Ok(())
}

fn require_digest(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if !value.contains(':') || value.len() > 160 {
        return Err(anyhow!("{field} must include an algorithm prefix"));
    }
    Ok(())
}

fn require_boundary_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "command bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "self-approval",
        "production-ready",
        "godot replacement",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden before/after comparison authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison_distinguishes_improvement_and_is_deterministic() {
        let input = planted_input(true);
        let first = input.compare().unwrap();
        let second = input.compare().unwrap();
        assert_eq!(first, second, "same two bundles produce identical output");
        assert_eq!(first.verdict, BeforeAfterVerdict::Improvement);
        assert_eq!(first.before_run_id, "run-before");
        assert_eq!(first.after_run_id, "run-after");
        assert!(first.dimensions.iter().any(|dimension| dimension.kind
            == ComparisonDimensionKind::Screenshots
            && !dimension.before_refs.is_empty()
            && !dimension.after_refs.is_empty()));
    }

    #[test]
    fn regression_is_prioritized_over_improvement() {
        let mut input = planted_input(true);
        input.after.replay_result = Some(ReplayResultSummary {
            passed: false,
            final_state_digest: "sha256:afterbad".to_string(),
            failures: vec!["replay failed".to_string()],
        });
        input.after.frame_stats = Some(FrameStatsSummary {
            avg_frame_ms: 20,
            max_frame_ms: 99,
            dropped_frames: 9,
        });
        let artifact = input.compare().unwrap();
        assert_eq!(artifact.verdict, BeforeAfterVerdict::Regression);
    }

    #[test]
    fn missing_evidence_is_inconclusive() {
        let mut input = planted_input(false);
        input.after.screenshots.clear();
        input.after.frame_stats = None;
        input.after.replay_result = None;
        let artifact = input.compare().unwrap();
        assert_eq!(artifact.verdict, BeforeAfterVerdict::Inconclusive);
    }

    fn planted_input(improved: bool) -> BeforeAfterComparisonInput {
        let before_pass = false;
        let after_pass = improved;
        BeforeAfterComparisonInput {
            comparison_id: "cmp-126-3".to_string(),
            before: bundle("run-before", before_pass, 4, 2, "before"),
            after: bundle("run-after", after_pass, 2, 0, "after"),
            reviewed_change_ref: reference("run-review", "mutation/review.json", "sha256:review"),
            guardrails: vec!["comparison is data-only and report-only".to_string()],
        }
    }

    fn bundle(
        run_id: &str,
        scenario_passed: bool,
        max_frame_ms: u32,
        dropped_frames: u32,
        label: &str,
    ) -> EvidenceBundleSummary {
        let mut flags = BTreeMap::new();
        flags.insert("scenario_passed".to_string(), scenario_passed);
        EvidenceBundleSummary {
            run_id: run_id.to_string(),
            artifacts: vec![
                reference(
                    run_id,
                    &format!("evidence/{label}-flags.json"),
                    "sha256:flags",
                ),
                reference(
                    run_id,
                    &format!("evidence/{label}-events.json"),
                    "sha256:events",
                ),
                reference(
                    run_id,
                    &format!("evidence/{label}-console.json"),
                    "sha256:console",
                ),
                reference(
                    run_id,
                    &format!("evidence/{label}-frame.json"),
                    "sha256:frame",
                ),
                reference(
                    run_id,
                    &format!("evidence/{label}-replay.json"),
                    "sha256:replay",
                ),
            ],
            flags,
            events: if scenario_passed {
                vec!["success.exit".to_string()]
            } else {
                vec!["error.missing-exit".to_string()]
            },
            screenshots: vec![reference(
                run_id,
                &format!("screenshots/{label}.png"),
                &format!("sha256:{label}"),
            )],
            console_diagnostics: if scenario_passed {
                vec![]
            } else {
                vec!["error missing exit event".to_string()]
            },
            frame_stats: Some(FrameStatsSummary {
                avg_frame_ms: 1,
                max_frame_ms,
                dropped_frames,
            }),
            replay_result: Some(ReplayResultSummary {
                passed: scenario_passed,
                final_state_digest: format!("sha256:{label}state"),
                failures: if scenario_passed {
                    vec![]
                } else {
                    vec!["missing exit".to_string()]
                },
            }),
            known_gaps: if scenario_passed {
                vec![]
            } else {
                vec!["exit not reached".to_string()]
            },
        }
    }

    fn reference(run_id: &str, path: &str, digest: &str) -> BeforeAfterEvidenceRef {
        BeforeAfterEvidenceRef {
            run_id: run_id.to_string(),
            path: path.to_string(),
            digest: digest.to_string(),
        }
    }
}
