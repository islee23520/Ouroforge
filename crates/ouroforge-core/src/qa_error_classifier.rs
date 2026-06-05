use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const QA_ERROR_CLASSIFIER_SCHEMA_VERSION: &str = "qa-error-classifier-v1";

const SUPPORTED_ERROR_KINDS: &[&str] = &[
    "console",
    "exception",
    "crash",
    "probe-unavailable",
    "asset-load-failure",
    "scenario-timeout",
];
const FAILURE_CLASSES: &[&str] = &[
    "warning",
    "error",
    "crash",
    "timeout",
    "probe-failure",
    "asset-failure",
    "unknown",
    "inconclusive",
];
const SEVERITIES: &[&str] = &["info", "warning", "error", "critical"];
const CONSOLE_LEVELS: &[&str] = &["log", "info", "warning", "error", "debug"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaErrorClassifierArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "classifierId")]
    pub classifier_id: String,
    pub status: String,
    #[serde(rename = "runMatrixRefs")]
    pub run_matrix_refs: Vec<String>,
    #[serde(rename = "staleRunRefs", default)]
    pub stale_run_refs: Vec<String>,
    pub entries: Vec<QaErrorEntry>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaErrorEntry {
    #[serde(rename = "errorId")]
    pub error_id: String,
    pub kind: String,
    #[serde(rename = "failureClass")]
    pub failure_class: String,
    pub severity: String,
    pub message: String,
    #[serde(
        rename = "consoleLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub console_level: Option<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(
        rename = "affectedWorker",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub affected_worker: Option<String>,
    #[serde(
        rename = "affectedRun",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub affected_run: Option<String>,
    #[serde(
        rename = "affectedScenario",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub affected_scenario: Option<String>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaErrorClassifierReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "classifierId")]
    pub classifier_id: String,
    pub status: String,
    #[serde(rename = "entryCount")]
    pub entry_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "failureClassCounts")]
    pub failure_class_counts: BTreeMap<String, usize>,
    #[serde(rename = "kindCounts")]
    pub kind_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

fn allowed_classes_for_kind(kind: &str) -> &'static [&'static str] {
    match kind {
        "console" => &["warning", "error", "unknown", "inconclusive"],
        "exception" => &["error", "crash", "unknown", "inconclusive"],
        "crash" => &["crash", "unknown", "inconclusive"],
        "probe-unavailable" => &["probe-failure", "inconclusive", "unknown"],
        "asset-load-failure" => &["asset-failure", "error", "unknown", "inconclusive"],
        "scenario-timeout" => &["timeout", "inconclusive", "unknown"],
        _ => &[],
    }
}

impl QaErrorClassifierArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse QA Error Classifier JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> QaErrorClassifierReadModel {
        let mut failure_class_counts = BTreeMap::new();
        let mut kind_counts = BTreeMap::new();
        for entry in &self.entries {
            *failure_class_counts
                .entry(entry.failure_class.clone())
                .or_insert(0) += 1;
            *kind_counts.entry(entry.kind.clone()).or_insert(0) += 1;
        }
        QaErrorClassifierReadModel {
            schema_version: self.schema_version.clone(),
            classifier_id: self.classifier_id.clone(),
            status: self.computed_status(),
            entry_count: self.entries.len(),
            blocked_count: self.blocked_count(),
            failure_class_counts,
            kind_counts,
            validation_summary: vec![
                "entries classify console levels, exceptions, crash-like signals, probe-unavailable, asset load failures, and scenario timeouts as evidence inputs".to_string(),
                "each entry carries a supported error kind, failure class, severity, and consistent kind/class pairing".to_string(),
                "missing console/probe evidence, malformed payloads, unknown severity, unsupported kinds, missing classification, and stale run refs fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "existing run matrix, failure classification, evidence bundle, journal, dashboard, and Studio read models remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize QA error classifier read model JSON")
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .entries
                .iter()
                .filter(|entry| !entry.blocked_reasons.is_empty())
                .count()
    }

    fn has_blockers(&self) -> bool {
        !self.blocked_reasons.is_empty()
            || self
                .entries
                .iter()
                .any(|entry| !entry.blocked_reasons.is_empty())
    }

    pub fn computed_status(&self) -> String {
        if !self.stale_run_refs.is_empty() {
            "stale".to_string()
        } else if self.has_blockers() {
            "blocked".to_string()
        } else {
            "classified".to_string()
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != QA_ERROR_CLASSIFIER_SCHEMA_VERSION {
            return Err(anyhow!(
                "QA error classifier schemaVersion must be {QA_ERROR_CLASSIFIER_SCHEMA_VERSION}"
            ));
        }
        require_id("QA error classifier classifierId", &self.classifier_id)?;
        validate_ref_list(
            "QA error classifier runMatrixRefs",
            &self.run_matrix_refs,
            true,
        )?;
        validate_ref_list(
            "QA error classifier staleRunRefs",
            &self.stale_run_refs,
            false,
        )?;
        let matrix: BTreeSet<&str> = self.run_matrix_refs.iter().map(String::as_str).collect();
        for stale in &self.stale_run_refs {
            if !matrix.contains(stale.as_str()) {
                return Err(anyhow!(
                    "QA error classifier staleRunRefs must reference declared runMatrixRefs"
                ));
            }
        }
        require_nonempty("QA error classifier entries", self.entries.len())?;
        if self.entries.len() > 128 {
            return Err(anyhow!("QA error classifier is overbroad for v1"));
        }
        let mut ids = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !ids.insert(entry.error_id.as_str()) {
                return Err(anyhow!(
                    "QA error classifier errorId `{}` is duplicated",
                    entry.error_id
                ));
            }
        }
        validate_text_list(
            "QA error classifier blockedReasons",
            &self.blocked_reasons,
            false,
        )?;

        let computed = self.computed_status();
        if self.status != computed {
            return Err(anyhow!(
                "QA error classifier status `{}` does not match computed classification `{computed}`",
                self.status
            ));
        }
        if (computed == "stale" || computed == "blocked") && !self.has_blockers() {
            return Err(anyhow!(
                "QA error classifier {computed} status requires visible blockedReasons"
            ));
        }

        require_text("QA error classifier boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "not trusted truth",
            "evidence and backlog inputs",
            "no auto-fix",
            "review-gated",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "QA error classifier boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl QaErrorEntry {
    fn validate(&self) -> Result<()> {
        require_id("QA error classifier errorId", &self.error_id)?;
        if !SUPPORTED_ERROR_KINDS.contains(&self.kind.as_str()) {
            return Err(anyhow!(
                "QA error classifier unsupported error kind `{}`",
                self.kind
            ));
        }
        if self.failure_class.trim().is_empty() {
            return Err(anyhow!(
                "QA error classifier entry `{}` is missing classification",
                self.error_id
            ));
        }
        if !FAILURE_CLASSES.contains(&self.failure_class.as_str()) {
            return Err(anyhow!(
                "QA error classifier unsupported failure class `{}`",
                self.failure_class
            ));
        }
        if !allowed_classes_for_kind(&self.kind).contains(&self.failure_class.as_str()) {
            return Err(anyhow!(
                "QA error classifier failure class `{}` is inconsistent with kind `{}`",
                self.failure_class,
                self.kind
            ));
        }
        if !SEVERITIES.contains(&self.severity.as_str()) {
            return Err(anyhow!(
                "QA error classifier unknown severity `{}`",
                self.severity
            ));
        }
        if self.message.trim().is_empty() {
            return Err(anyhow!(
                "QA error classifier entry `{}` has a malformed error payload: empty message",
                self.error_id
            ));
        }
        require_text("QA error classifier message", &self.message)?;

        if self.kind == "console" {
            match &self.console_level {
                None => {
                    return Err(anyhow!(
                        "QA error classifier console entry `{}` requires a consoleLevel",
                        self.error_id
                    ));
                }
                Some(level) if !CONSOLE_LEVELS.contains(&level.as_str()) => {
                    return Err(anyhow!(
                        "QA error classifier unsupported console level `{level}`"
                    ));
                }
                Some(_) => {}
            }
        } else if self.console_level.is_some() {
            return Err(anyhow!(
                "QA error classifier consoleLevel is only valid for console entries"
            ));
        }

        // Console and probe entries must carry their supporting evidence.
        if (self.kind == "console" || self.kind == "probe-unavailable")
            && self.evidence_refs.is_empty()
        {
            return Err(anyhow!(
                "QA error classifier {} entry `{}` is missing console/probe evidence",
                self.kind,
                self.error_id
            ));
        }
        validate_ref_list(
            "QA error classifier evidenceRefs",
            &self.evidence_refs,
            false,
        )?;

        for (field, value) in [
            ("affectedWorker", &self.affected_worker),
            ("affectedRun", &self.affected_run),
            ("affectedScenario", &self.affected_scenario),
        ] {
            if let Some(id) = value {
                require_id(&format!("QA error classifier {field}"), id)?;
            }
        }
        validate_text_list(
            "QA error classifier entry blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
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
        "production safety",
        "shipped-game",
        "hidden worker",
        "remote swarm",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden QA error classifier authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    value.contains(phrase)
        && ![
            "no ",
            "not ",
            "without ",
            "avoid ",
            "forbid ",
            "forbidden ",
            "not yet ",
        ]
        .iter()
        .any(|prefix| value.contains(&format!("{prefix}{phrase}")))
}
