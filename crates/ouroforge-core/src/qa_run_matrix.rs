use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const QA_RUN_MATRIX_SCHEMA_VERSION: &str = "qa-swarm-run-matrix-v1";

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
/// Verdicts that resolve a row without further triage.
const RESOLVED_VERDICTS: &[&str] = &["passed", "failed"];
/// Verdicts that legitimately carry no evidence refs.
const NO_EVIDENCE_VERDICTS: &[&str] = &["skipped", "unsupported", "missing_evidence"];
const FLAKE_STATUSES: &[&str] = &["stable", "flaky", "inconclusive", "not-evaluated"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaRunMatrixArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "matrixId")]
    pub matrix_id: String,
    pub status: String,
    #[serde(rename = "runRef")]
    pub run_ref: String,
    #[serde(rename = "staleEvidenceRefs", default)]
    pub stale_evidence_refs: Vec<String>,
    pub rows: Vec<QaRunMatrixRow>,
    #[serde(rename = "dashboardCompat")]
    pub dashboard_compat: QaRunMatrixDashboardCompat,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaRunMatrixRow {
    #[serde(rename = "rowId")]
    pub row_id: String,
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "candidateId")]
    pub candidate_id: String,
    #[serde(rename = "fuzzSeed", default, skip_serializing_if = "Option::is_none")]
    pub fuzz_seed: Option<String>,
    #[serde(rename = "workerId")]
    pub worker_id: String,
    pub attempt: u32,
    #[serde(rename = "rerunGroup")]
    pub rerun_group: String,
    pub verdict: String,
    #[serde(
        rename = "failureClass",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_class: Option<String>,
    #[serde(rename = "flakeStatus")]
    pub flake_status: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "budgetUsed")]
    pub budget_used: QaRunBudgetUsed,
    #[serde(
        rename = "startedAtMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub started_at_ms: Option<u64>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaRunBudgetUsed {
    #[serde(rename = "maxActions")]
    pub max_actions: u32,
    #[serde(rename = "actionsUsed")]
    pub actions_used: u32,
    #[serde(rename = "maxDurationMs")]
    pub max_duration_ms: u64,
    #[serde(rename = "durationMs")]
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaRunMatrixDashboardCompat {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub surface: String,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaRunMatrixReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "matrixId")]
    pub matrix_id: String,
    pub status: String,
    #[serde(rename = "rowCount")]
    pub row_count: usize,
    #[serde(rename = "verdictCounts")]
    pub verdict_counts: BTreeMap<String, usize>,
    #[serde(rename = "rerunGroupCount")]
    pub rerun_group_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl QaRunMatrixArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse QA Run Matrix JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> QaRunMatrixReadModel {
        let mut verdict_counts = BTreeMap::new();
        let mut groups = BTreeSet::new();
        for row in &self.rows {
            *verdict_counts.entry(row.verdict.clone()).or_insert(0) += 1;
            groups.insert(row.rerun_group.clone());
        }
        QaRunMatrixReadModel {
            schema_version: self.schema_version.clone(),
            matrix_id: self.matrix_id.clone(),
            status: self.computed_status(),
            row_count: self.rows.len(),
            verdict_counts,
            rerun_group_count: groups.len(),
            blocked_count: self.blocked_count(),
            validation_summary: vec![
                "each row records scenario, candidate, fuzz seed, worker, attempt, rerun group, verdict, failure class, flake status, evidence refs, and bounded budget used".to_string(),
                "verdicts cover passed, failed, flaky, inconclusive, skipped, unsupported, timed_out, crashed, and missing_evidence as evidence inputs".to_string(),
                "duplicate rows, stale evidence refs, invalid worker/candidate ids, missing run refs, malformed verdicts, missing budgets, and inconsistent rerun groups fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "dashboard/Studio surfaces remain read-only or draft-only; existing run, evidence, and bundle read models remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize QA run matrix read model JSON")
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .rows
                .iter()
                .filter(|row| !row.blocked_reasons.is_empty())
                .count()
    }

    pub fn computed_status(&self) -> String {
        if !self.stale_evidence_refs.is_empty() {
            return "stale".to_string();
        }
        if !self.blocked_reasons.is_empty() {
            return "blocked".to_string();
        }
        let all_resolved = self.rows.iter().all(|row| {
            RESOLVED_VERDICTS.contains(&row.verdict.as_str()) && row.blocked_reasons.is_empty()
        });
        if all_resolved {
            "complete".to_string()
        } else {
            "partial".to_string()
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != QA_RUN_MATRIX_SCHEMA_VERSION {
            return Err(anyhow!(
                "QA run matrix schemaVersion must be {QA_RUN_MATRIX_SCHEMA_VERSION}"
            ));
        }
        require_id("QA run matrix matrixId", &self.matrix_id)?;
        if self.run_ref.trim().is_empty() {
            return Err(anyhow!("QA run matrix is missing run ref"));
        }
        require_ref("QA run matrix runRef", &self.run_ref)?;
        validate_ref_list(
            "QA run matrix staleEvidenceRefs",
            &self.stale_evidence_refs,
            false,
        )?;
        require_nonempty("QA run matrix rows", self.rows.len())?;
        if self.rows.len() > 512 {
            return Err(anyhow!("QA run matrix is overbroad for v1"));
        }

        self.dashboard_compat.validate()?;

        let mut row_ids = BTreeSet::new();
        let mut row_keys = BTreeSet::new();
        let mut group_identity: BTreeMap<&str, (&str, &str)> = BTreeMap::new();
        let mut group_attempts: BTreeMap<&str, BTreeSet<u32>> = BTreeMap::new();
        for row in &self.rows {
            row.validate()?;
            if !row_ids.insert(row.row_id.as_str()) {
                return Err(anyhow!("QA run matrix duplicate row id `{}`", row.row_id));
            }
            let key = format!(
                "{}|{}|{}|{}|{}",
                row.scenario_id,
                row.candidate_id,
                row.fuzz_seed.as_deref().unwrap_or("-"),
                row.worker_id,
                row.attempt
            );
            if !row_keys.insert(key) {
                return Err(anyhow!(
                    "QA run matrix duplicate row for scenario/candidate/seed/worker/attempt"
                ));
            }
            // Rerun group identity must be consistent.
            match group_identity.get(row.rerun_group.as_str()) {
                Some((scenario, candidate)) => {
                    if *scenario != row.scenario_id || *candidate != row.candidate_id {
                        return Err(anyhow!(
                            "QA run matrix inconsistent rerun group `{}`",
                            row.rerun_group
                        ));
                    }
                }
                None => {
                    group_identity.insert(
                        row.rerun_group.as_str(),
                        (row.scenario_id.as_str(), row.candidate_id.as_str()),
                    );
                }
            }
            if !group_attempts
                .entry(row.rerun_group.as_str())
                .or_default()
                .insert(row.attempt)
            {
                return Err(anyhow!(
                    "QA run matrix inconsistent rerun group `{}`: duplicate attempt {}",
                    row.rerun_group,
                    row.attempt
                ));
            }
        }

        let computed = self.computed_status();
        if self.status != computed {
            return Err(anyhow!(
                "QA run matrix status `{}` does not match computed classification `{computed}`",
                self.status
            ));
        }
        if (computed == "stale" || computed == "blocked") && self.blocked_count() == 0 {
            return Err(anyhow!(
                "QA run matrix {computed} status requires visible blockedReasons"
            ));
        }

        validate_text_list("QA run matrix blockedReasons", &self.blocked_reasons, false)?;
        require_text("QA run matrix boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "not trusted truth",
            "no auto-fix",
            "review-gated",
            "read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!("QA run matrix boundary must state `{required}`"));
            }
        }
        Ok(())
    }
}

impl QaRunMatrixRow {
    fn validate(&self) -> Result<()> {
        require_id("QA run matrix rowId", &self.row_id)?;
        require_id("QA run matrix scenarioId", &self.scenario_id)?;
        require_id("QA run matrix candidateId", &self.candidate_id)?;
        if let Some(seed) = &self.fuzz_seed {
            require_id("QA run matrix fuzzSeed", seed)?;
        }
        require_id("QA run matrix workerId", &self.worker_id)?;
        require_id("QA run matrix rerunGroup", &self.rerun_group)?;
        if self.attempt == 0 {
            return Err(anyhow!("QA run matrix attempt must be >= 1"));
        }
        if !VERDICTS.contains(&self.verdict.as_str()) {
            return Err(anyhow!(
                "QA run matrix malformed verdict `{}`",
                self.verdict
            ));
        }
        if let Some(failure_class) = &self.failure_class {
            require_text("QA run matrix failureClass", failure_class)?;
        }
        if !FLAKE_STATUSES.contains(&self.flake_status.as_str()) {
            return Err(anyhow!(
                "QA run matrix unsupported flake status `{}`",
                self.flake_status
            ));
        }
        validate_ref_list("QA run matrix evidenceRefs", &self.evidence_refs, false)?;
        if NO_EVIDENCE_VERDICTS.contains(&self.verdict.as_str()) {
            if self.verdict == "missing_evidence" && !self.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "QA run matrix row `{}` is missing_evidence but carries evidence refs",
                    self.row_id
                ));
            }
        } else if self.evidence_refs.is_empty() {
            return Err(anyhow!(
                "QA run matrix row `{}` is missing evidence for verdict `{}`",
                self.row_id,
                self.verdict
            ));
        }
        self.budget_used.validate(&self.row_id)?;
        validate_text_list(
            "QA run matrix row blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        Ok(())
    }
}

impl QaRunBudgetUsed {
    fn validate(&self, row_id: &str) -> Result<()> {
        if self.max_actions == 0 || self.max_duration_ms == 0 {
            return Err(anyhow!(
                "QA run matrix row `{row_id}` budget must be bounded (missing budget)"
            ));
        }
        if self.actions_used > self.max_actions || self.duration_ms > self.max_duration_ms {
            return Err(anyhow!(
                "QA run matrix row `{row_id}` budget used exceeds the declared budget"
            ));
        }
        Ok(())
    }
}

impl QaRunMatrixDashboardCompat {
    fn validate(&self) -> Result<()> {
        if !self.read_only {
            return Err(anyhow!(
                "QA run matrix dashboard surface must remain read-only or draft-only"
            ));
        }
        require_text("QA run matrix dashboardCompat.surface", &self.surface)?;
        validate_text_list("QA run matrix dashboardCompat.columns", &self.columns, true)
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
        "hidden worker",
        "remote swarm",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden QA run matrix authority text `{forbidden}`"
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
    let mut search_start = 0;
    while let Some(rel) = hay[search_start..].find(phrase) {
        let idx = search_start + rel;
        let clause_start = hay[..idx]
            .rfind(['.', ';', '!', '\n', '\r'])
            .map(|p| p + 1)
            .unwrap_or(0);
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
mod negation_scope_tests {
    use super::contains_positive_phrase;

    #[test]
    fn negated_then_positive_in_separate_sentences_is_flagged() {
        // A negated mention in one sentence must not whitelist a positive mention
        // in another (the prior fail-open bypass for issue #693).
        let value = "no auto-fix is authorized. auto-fix enabled for passing rows.";
        assert!(contains_positive_phrase(value, "auto-fix"));
    }

    #[test]
    fn list_style_single_negation_is_preserved() {
        // A single leading negation still covers a comma/or list of phrases.
        let value = "no auto-fix, auto-merge, or production-ready claim is made.";
        assert!(!contains_positive_phrase(value, "auto-fix"));
        assert!(!contains_positive_phrase(value, "auto-merge"));
        assert!(!contains_positive_phrase(value, "production-ready"));
    }

    #[test]
    fn plain_positive_is_flagged_and_absent_is_not() {
        assert!(contains_positive_phrase("auto-merge happens here", "auto-merge"));
        assert!(!contains_positive_phrase("read-only evidence only", "auto-merge"));
    }
}
