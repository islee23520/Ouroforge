use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const QA_FLAKE_RERUN_POLICY_SCHEMA_VERSION: &str = "qa-flake-rerun-policy-v1";

/// Hard cap so rerun budgets stay bounded for v1.
const MAX_RERUN_CAP: u32 = 10;
const OUTCOMES: &[&str] = &["passed", "failed", "inconclusive", "divergent"];
const CLASSIFICATIONS: &[&str] = &[
    "stable-pass",
    "stable-fail",
    "flaky",
    "inconclusive",
    "exhausted",
    "unsupported",
    "stale",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FlakeRerunPolicyArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "policyId")]
    pub policy_id: String,
    pub classification: String,
    #[serde(rename = "runMatrixRefs")]
    pub run_matrix_refs: Vec<String>,
    #[serde(rename = "staleRunRefs", default)]
    pub stale_run_refs: Vec<String>,
    #[serde(rename = "maxReruns")]
    pub max_reruns: u32,
    #[serde(rename = "rerunsUsed")]
    pub reruns_used: u32,
    #[serde(rename = "consistencyThreshold")]
    pub consistency_threshold: f64,
    #[serde(rename = "rerunSupported")]
    pub rerun_supported: bool,
    #[serde(rename = "cleanupPolicy")]
    pub cleanup_policy: String,
    #[serde(rename = "outputRoots")]
    pub output_roots: Vec<String>,
    #[serde(rename = "originalEvidenceRef")]
    pub original_evidence_ref: String,
    #[serde(rename = "rerunEvidenceRefs", default)]
    pub rerun_evidence_refs: Vec<String>,
    #[serde(rename = "observedOutcomes")]
    pub observed_outcomes: Vec<String>,
    #[serde(rename = "divergentFields", default)]
    pub divergent_fields: Vec<String>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FlakeRerunPolicyReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "policyId")]
    pub policy_id: String,
    pub classification: String,
    #[serde(rename = "maxReruns")]
    pub max_reruns: u32,
    #[serde(rename = "rerunsUsed")]
    pub reruns_used: u32,
    #[serde(rename = "outcomeCounts")]
    pub outcome_counts: BTreeMap<String, usize>,
    #[serde(rename = "divergentFieldCount")]
    pub divergent_field_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl FlakeRerunPolicyArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse QA Flake/Rerun Policy JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> FlakeRerunPolicyReadModel {
        let mut outcome_counts = BTreeMap::new();
        for outcome in &self.observed_outcomes {
            *outcome_counts.entry(outcome.clone()).or_insert(0) += 1;
        }
        FlakeRerunPolicyReadModel {
            schema_version: self.schema_version.clone(),
            policy_id: self.policy_id.clone(),
            classification: self.computed_classification(),
            max_reruns: self.max_reruns,
            reruns_used: self.reruns_used,
            outcome_counts,
            divergent_field_count: self.divergent_fields.len(),
            blocked_count: self.blocked_reasons.len(),
            validation_summary: vec![
                "the policy bounds reruns, records a consistency threshold, cleanup policy, and output roots as evidence inputs".to_string(),
                "classification covers stable-pass, stable-fail, flaky, inconclusive, exhausted, unsupported, and stale states".to_string(),
                "unbounded reruns, missing thresholds, overlapping outputs, missing cleanup, stale refs, malformed comparisons, and missing original evidence fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "existing run matrix, failure classification, and evidence bundle read models remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize QA flake/rerun policy read model JSON")
    }

    fn outcome_counts(&self) -> BTreeMap<&str, usize> {
        let mut counts = BTreeMap::new();
        for outcome in &self.observed_outcomes {
            *counts.entry(outcome.as_str()).or_insert(0) += 1;
        }
        counts
    }

    /// Consistent when the majority pass/fail outcome meets the consistency threshold.
    fn consistent_outcome(&self) -> Option<&'static str> {
        if self.observed_outcomes.is_empty() {
            return None;
        }
        let counts = self.outcome_counts();
        // Opposing pass/fail evidence is never stable, no matter how permissive
        // the consistency threshold is; such runs fall through to flaky/exhausted
        // classification (issue #691: pass-then-fail / fail-then-pass are flaky).
        let has_pass = counts.get("passed").copied().unwrap_or(0) > 0;
        let has_fail = counts.get("failed").copied().unwrap_or(0) > 0;
        if has_pass && has_fail {
            return None;
        }
        let total = self.observed_outcomes.len() as f64;
        for candidate in ["passed", "failed"] {
            let count = counts.get(candidate).copied().unwrap_or(0) as f64;
            if count / total >= self.consistency_threshold && count > 0.0 {
                return Some(if candidate == "passed" {
                    "passed"
                } else {
                    "failed"
                });
            }
        }
        None
    }

    pub fn computed_classification(&self) -> String {
        if !self.stale_run_refs.is_empty() {
            return "stale".to_string();
        }
        if !self.rerun_supported {
            return "unsupported".to_string();
        }
        if let Some(outcome) = self.consistent_outcome() {
            return if outcome == "passed" {
                "stable-pass".to_string()
            } else {
                "stable-fail".to_string()
            };
        }
        if self.reruns_used >= self.max_reruns {
            return "exhausted".to_string();
        }
        let counts = self.outcome_counts();
        let has_pass = counts.get("passed").copied().unwrap_or(0) > 0;
        let has_fail = counts.get("failed").copied().unwrap_or(0) > 0;
        let has_divergent = counts.get("divergent").copied().unwrap_or(0) > 0;
        if (has_pass && has_fail) || has_divergent || !self.divergent_fields.is_empty() {
            "flaky".to_string()
        } else {
            "inconclusive".to_string()
        }
    }

    fn requires_blockers(classification: &str) -> bool {
        matches!(
            classification,
            "stale" | "unsupported" | "exhausted" | "inconclusive"
        )
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != QA_FLAKE_RERUN_POLICY_SCHEMA_VERSION {
            return Err(anyhow!(
                "QA flake/rerun policy schemaVersion must be {QA_FLAKE_RERUN_POLICY_SCHEMA_VERSION}"
            ));
        }
        require_id("QA flake/rerun policy policyId", &self.policy_id)?;
        if !CLASSIFICATIONS.contains(&self.classification.as_str()) {
            return Err(anyhow!(
                "QA flake/rerun policy unsupported classification `{}`",
                self.classification
            ));
        }
        validate_ref_list(
            "QA flake/rerun policy runMatrixRefs",
            &self.run_matrix_refs,
            true,
        )?;
        validate_ref_list(
            "QA flake/rerun policy staleRunRefs",
            &self.stale_run_refs,
            false,
        )?;
        let matrix: BTreeSet<&str> = self.run_matrix_refs.iter().map(String::as_str).collect();
        for stale in &self.stale_run_refs {
            if !matrix.contains(stale.as_str()) {
                return Err(anyhow!(
                    "QA flake/rerun policy staleRunRefs must reference declared runMatrixRefs"
                ));
            }
        }

        // Bounded rerun budget.
        if self.max_reruns == 0 || self.max_reruns > MAX_RERUN_CAP {
            return Err(anyhow!(
                "QA flake/rerun policy maxReruns must be bounded between 1 and {MAX_RERUN_CAP}; unbounded reruns are not allowed"
            ));
        }
        if self.reruns_used > self.max_reruns {
            return Err(anyhow!(
                "QA flake/rerun policy rerunsUsed must not exceed maxReruns"
            ));
        }
        if !self.consistency_threshold.is_finite()
            || self.consistency_threshold <= 0.0
            || self.consistency_threshold > 1.0
        {
            return Err(anyhow!(
                "QA flake/rerun policy consistency threshold must be between 0 and 1"
            ));
        }

        if self.cleanup_policy.trim().is_empty() {
            return Err(anyhow!(
                "QA flake/rerun policy cleanup policy must not be empty"
            ));
        }
        require_text("QA flake/rerun policy cleanupPolicy", &self.cleanup_policy)?;

        validate_ref_list(
            "QA flake/rerun policy outputRoots",
            &self.output_roots,
            true,
        )?;
        validate_no_overlap(&self.output_roots)?;

        if self.original_evidence_ref.trim().is_empty() {
            return Err(anyhow!(
                "QA flake/rerun policy is missing original evidence"
            ));
        }
        require_ref(
            "QA flake/rerun policy originalEvidenceRef",
            &self.original_evidence_ref,
        )?;
        validate_ref_list(
            "QA flake/rerun policy rerunEvidenceRefs",
            &self.rerun_evidence_refs,
            false,
        )?;

        // Comparison must be well-formed: outcomes line up with reruns used.
        if self.observed_outcomes.is_empty() {
            return Err(anyhow!(
                "QA flake/rerun policy malformed comparison: observedOutcomes must not be empty"
            ));
        }
        for outcome in &self.observed_outcomes {
            if !OUTCOMES.contains(&outcome.as_str()) {
                return Err(anyhow!(
                    "QA flake/rerun policy malformed comparison: unsupported outcome `{outcome}`"
                ));
            }
        }
        if self.observed_outcomes.len() != self.reruns_used as usize + 1 {
            return Err(anyhow!(
                "QA flake/rerun policy malformed comparison: observedOutcomes must equal rerunsUsed + 1 (original)"
            ));
        }
        if self.rerun_evidence_refs.len() != self.reruns_used as usize {
            return Err(anyhow!(
                "QA flake/rerun policy malformed comparison: rerunEvidenceRefs must equal rerunsUsed"
            ));
        }
        validate_text_list(
            "QA flake/rerun policy divergentFields",
            &self.divergent_fields,
            false,
        )?;
        validate_text_list(
            "QA flake/rerun policy blockedReasons",
            &self.blocked_reasons,
            false,
        )?;

        let computed = self.computed_classification();
        if self.classification != computed {
            return Err(anyhow!(
                "QA flake/rerun policy classification `{}` does not match computed classification `{computed}`",
                self.classification
            ));
        }
        if Self::requires_blockers(&computed) && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "QA flake/rerun policy {computed} classification requires visible blockedReasons"
            ));
        }

        require_text("QA flake/rerun policy boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "bounded reruns",
            "cleanup policy",
            "no auto-fix",
            "review-gated",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "QA flake/rerun policy boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

/// Output roots must be disjoint: no root may contain another.
fn validate_no_overlap(roots: &[String]) -> Result<()> {
    for (i, a) in roots.iter().enumerate() {
        for (j, b) in roots.iter().enumerate() {
            if i == j {
                continue;
            }
            let a_prefix = format!("{}/", a.trim_end_matches('/'));
            if b == a || b.starts_with(&a_prefix) {
                return Err(anyhow!(
                    "QA flake/rerun policy has overlapping output roots `{a}` and `{b}`"
                ));
            }
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
        "quality guarantee",
        "shipped-game",
        "hidden worker",
        "remote swarm",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden QA flake/rerun policy authority text `{forbidden}`"
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
