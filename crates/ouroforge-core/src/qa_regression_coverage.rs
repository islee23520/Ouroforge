use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const QA_REGRESSION_COVERAGE_SCHEMA_VERSION: &str = "qa-swarm-regression-coverage-v1";

/// Every QA/playtest area the regression suite must account for.
const REQUIRED_AREAS: &[&str] = &[
    "scenario-generation",
    "fuzzing-plan",
    "worker-budget",
    "runtime-invariant",
    "route-attempt",
    "visual-comparison",
    "performance-budget",
    "error-classifier",
    "flake-rerun",
    "failure-backlog",
    "run-matrix",
    "evidence-bundle",
    "studio-dashboard-read-model",
    "malformed-missing-stale",
];
const COVERAGE_STATUSES: &[&str] = &["in-repo", "documented-gap"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaRegressionCoverageArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "coverageId")]
    pub coverage_id: String,
    pub entries: Vec<QaRegressionCoverageEntry>,
    #[serde(rename = "knownGaps")]
    pub known_gaps: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaRegressionCoverageEntry {
    pub area: String,
    #[serde(rename = "coverageStatus")]
    pub coverage_status: String,
    #[serde(rename = "testRef")]
    pub test_ref: String,
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaRegressionCoverageReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "coverageId")]
    pub coverage_id: String,
    #[serde(rename = "areaCount")]
    pub area_count: usize,
    #[serde(rename = "coverageStatusCounts")]
    pub coverage_status_counts: BTreeMap<String, usize>,
    #[serde(rename = "knownGapCount")]
    pub known_gap_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl QaRegressionCoverageArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse QA Regression Coverage JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> QaRegressionCoverageReadModel {
        let mut coverage_status_counts = BTreeMap::new();
        for entry in &self.entries {
            *coverage_status_counts
                .entry(entry.coverage_status.clone())
                .or_insert(0) += 1;
        }
        QaRegressionCoverageReadModel {
            schema_version: self.schema_version.clone(),
            coverage_id: self.coverage_id.clone(),
            area_count: self.entries.len(),
            coverage_status_counts,
            known_gap_count: self.known_gaps.len(),
            validation_summary: vec![
                "the matrix enumerates every QA/playtest area and records where its regression coverage lives".to_string(),
                "in-repo areas are validated by the regression suite or their focused contract tests; documented gaps are listed honestly".to_string(),
                "missing areas, unsupported coverage status, unsafe refs, and missing known gaps fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model that adds regression coverage, not auto-fix or trusted mutation".to_string(),
                "existing QA artifact contract tests and read models remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize QA regression coverage read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != QA_REGRESSION_COVERAGE_SCHEMA_VERSION {
            return Err(anyhow!(
                "QA regression coverage schemaVersion must be {QA_REGRESSION_COVERAGE_SCHEMA_VERSION}"
            ));
        }
        require_id("QA regression coverage coverageId", &self.coverage_id)?;
        require_nonempty("QA regression coverage entries", self.entries.len())?;

        let mut areas = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !areas.insert(entry.area.as_str()) {
                return Err(anyhow!(
                    "QA regression coverage duplicate area `{}`",
                    entry.area
                ));
            }
        }
        for required in REQUIRED_AREAS {
            if !areas.contains(required) {
                return Err(anyhow!(
                    "QA regression coverage is missing area `{required}`"
                ));
            }
        }
        for area in &areas {
            if !REQUIRED_AREAS.contains(area) {
                return Err(anyhow!("QA regression coverage unsupported area `{area}`"));
            }
        }

        // A regression suite must state its known gaps honestly.
        validate_text_list("QA regression coverage knownGaps", &self.known_gaps, true)?;

        require_text("QA regression coverage boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "regression coverage",
            "no auto-fix",
            "review-gated",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "QA regression coverage boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl QaRegressionCoverageEntry {
    fn validate(&self) -> Result<()> {
        if !REQUIRED_AREAS.contains(&self.area.as_str()) {
            return Err(anyhow!(
                "QA regression coverage unsupported area `{}`",
                self.area
            ));
        }
        if !COVERAGE_STATUSES.contains(&self.coverage_status.as_str()) {
            return Err(anyhow!(
                "QA regression coverage unsupported coverage status `{}`",
                self.coverage_status
            ));
        }
        require_ref("QA regression coverage testRef", &self.test_ref)?;
        require_text("QA regression coverage notes", &self.notes)?;
        Ok(())
    }
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
                "{field} contains forbidden QA regression coverage authority text `{forbidden}`"
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
