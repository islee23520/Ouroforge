use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const QA_FAILURE_BACKLOG_SCHEMA_VERSION: &str = "qa-failure-backlog-v1";

const FAILURE_CLASSES: &[&str] = &[
    "gameplay-logic",
    "level-design",
    "asset",
    "physics-collision",
    "input",
    "performance",
    "visual",
    "runtime-crash",
    "console-error",
    "probe-failure",
    "flaky",
    "unsupported",
    "unknown",
];
const OWNER_LANES: &[&str] = &[
    "gameplay",
    "level",
    "art",
    "audio",
    "engine",
    "qa",
    "unassigned",
];
/// Review statuses are deliberately triage-only: there is no applied/fixed state.
const REVIEW_STATUSES: &[&str] = &["pending-review", "triaged", "deferred", "rejected"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaFailureBacklogArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "backlogId")]
    pub backlog_id: String,
    pub status: String,
    #[serde(rename = "runMatrixRefs")]
    pub run_matrix_refs: Vec<String>,
    #[serde(rename = "staleRunRefs", default)]
    pub stale_run_refs: Vec<String>,
    pub items: Vec<QaBacklogItem>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaBacklogItem {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "failureClass")]
    pub failure_class: String,
    pub severity: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "ownerLane")]
    pub owner_lane: String,
    #[serde(rename = "suggestedInvestigation")]
    pub suggested_investigation: String,
    #[serde(rename = "reproductionContext")]
    pub reproduction_context: QaReproductionContext,
    #[serde(
        rename = "relatedScenarioId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_scenario_id: Option<String>,
    #[serde(
        rename = "relatedFuzzSeed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub related_fuzz_seed: Option<String>,
    #[serde(rename = "reviewStatus")]
    pub review_status: String,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaReproductionContext {
    pub summary: String,
    #[serde(
        rename = "worldStateRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub world_state_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaFailureBacklogReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "backlogId")]
    pub backlog_id: String,
    pub status: String,
    #[serde(rename = "itemCount")]
    pub item_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "failureClassCounts")]
    pub failure_class_counts: BTreeMap<String, usize>,
    #[serde(rename = "ownerLaneCounts")]
    pub owner_lane_counts: BTreeMap<String, usize>,
    #[serde(rename = "reviewStatusCounts")]
    pub review_status_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl QaFailureBacklogArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse QA Failure Backlog JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> QaFailureBacklogReadModel {
        let mut failure_class_counts = BTreeMap::new();
        let mut owner_lane_counts = BTreeMap::new();
        let mut review_status_counts = BTreeMap::new();
        for item in &self.items {
            *failure_class_counts
                .entry(item.failure_class.clone())
                .or_insert(0) += 1;
            *owner_lane_counts
                .entry(item.owner_lane.clone())
                .or_insert(0) += 1;
            *review_status_counts
                .entry(item.review_status.clone())
                .or_insert(0) += 1;
        }
        QaFailureBacklogReadModel {
            schema_version: self.schema_version.clone(),
            backlog_id: self.backlog_id.clone(),
            status: self.computed_status(),
            item_count: self.items.len(),
            blocked_count: self.blocked_count(),
            failure_class_counts,
            owner_lane_counts,
            review_status_counts,
            validation_summary: vec![
                "each backlog item links evidence, a failure class, an owner lane, a reproduction context, and a triage-only review status".to_string(),
                "failures become evidence-linked backlog items, not automatic fixes; there is no applied or fixed review status".to_string(),
                "missing evidence, invalid owner lane, unsupported class, missing reproduction context, stale refs, duplicate ids, and auto-fix/apply attempts fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "existing run matrix, multi-agent task boards, evidence bundle, dashboard, and Studio read models remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize QA failure backlog read model JSON")
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .items
                .iter()
                .filter(|item| !item.blocked_reasons.is_empty())
                .count()
    }

    fn has_blockers(&self) -> bool {
        !self.blocked_reasons.is_empty()
            || self
                .items
                .iter()
                .any(|item| !item.blocked_reasons.is_empty())
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
        if self.schema_version != QA_FAILURE_BACKLOG_SCHEMA_VERSION {
            return Err(anyhow!(
                "QA failure backlog schemaVersion must be {QA_FAILURE_BACKLOG_SCHEMA_VERSION}"
            ));
        }
        require_id("QA failure backlog backlogId", &self.backlog_id)?;
        validate_ref_list(
            "QA failure backlog runMatrixRefs",
            &self.run_matrix_refs,
            true,
        )?;
        validate_ref_list(
            "QA failure backlog staleRunRefs",
            &self.stale_run_refs,
            false,
        )?;
        let matrix: BTreeSet<&str> = self.run_matrix_refs.iter().map(String::as_str).collect();
        for stale in &self.stale_run_refs {
            if !matrix.contains(stale.as_str()) {
                return Err(anyhow!(
                    "QA failure backlog staleRunRefs must reference declared runMatrixRefs"
                ));
            }
        }
        require_nonempty("QA failure backlog items", self.items.len())?;
        if self.items.len() > 256 {
            return Err(anyhow!("QA failure backlog is overbroad for v1"));
        }
        let mut ids = BTreeSet::new();
        for item in &self.items {
            item.validate()?;
            if !ids.insert(item.item_id.as_str()) {
                return Err(anyhow!(
                    "QA failure backlog duplicate backlog id `{}`",
                    item.item_id
                ));
            }
        }
        validate_text_list(
            "QA failure backlog blockedReasons",
            &self.blocked_reasons,
            false,
        )?;

        let computed = self.computed_status();
        if self.status != computed {
            return Err(anyhow!(
                "QA failure backlog status `{}` does not match computed classification `{computed}`",
                self.status
            ));
        }
        if (computed == "stale" || computed == "blocked") && !self.has_blockers() {
            return Err(anyhow!(
                "QA failure backlog {computed} status requires visible blockedReasons"
            ));
        }

        require_text("QA failure backlog boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "evidence and backlog inputs",
            "not automatic fixes",
            "no auto-fix",
            "review-gated",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "QA failure backlog boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl QaBacklogItem {
    fn validate(&self) -> Result<()> {
        require_id("QA failure backlog itemId", &self.item_id)?;
        if !FAILURE_CLASSES.contains(&self.failure_class.as_str()) {
            return Err(anyhow!(
                "QA failure backlog unsupported failure class `{}`",
                self.failure_class
            ));
        }
        require_text("QA failure backlog severity", &self.severity)?;
        if !["low", "medium", "high", "critical"].contains(&self.severity.as_str()) {
            return Err(anyhow!(
                "QA failure backlog unsupported severity `{}`",
                self.severity
            ));
        }
        if self.evidence_refs.is_empty() {
            return Err(anyhow!(
                "QA failure backlog item `{}` is missing evidence",
                self.item_id
            ));
        }
        validate_ref_list("QA failure backlog evidenceRefs", &self.evidence_refs, true)?;
        if !OWNER_LANES.contains(&self.owner_lane.as_str()) {
            return Err(anyhow!(
                "QA failure backlog invalid owner lane `{}`",
                self.owner_lane
            ));
        }
        require_text(
            "QA failure backlog suggestedInvestigation",
            &self.suggested_investigation,
        )?;
        if self.reproduction_context.summary.trim().is_empty() {
            return Err(anyhow!(
                "QA failure backlog item `{}` is missing reproduction context",
                self.item_id
            ));
        }
        require_text(
            "QA failure backlog reproductionContext.summary",
            &self.reproduction_context.summary,
        )?;
        if let Some(world_state_ref) = &self.reproduction_context.world_state_ref {
            require_ref(
                "QA failure backlog reproductionContext.worldStateRef",
                world_state_ref,
            )?;
        }
        if let Some(scenario_id) = &self.related_scenario_id {
            require_id("QA failure backlog relatedScenarioId", scenario_id)?;
        }
        if let Some(fuzz_seed) = &self.related_fuzz_seed {
            require_id("QA failure backlog relatedFuzzSeed", fuzz_seed)?;
        }
        if !REVIEW_STATUSES.contains(&self.review_status.as_str()) {
            return Err(anyhow!(
                "QA failure backlog unsupported review status `{}`: backlog items are review-gated and never auto-applied",
                self.review_status
            ));
        }
        validate_text_list(
            "QA failure backlog item blockedReasons",
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
        "shipped-game",
        "hidden worker",
        "remote swarm",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden QA failure backlog authority text `{forbidden}`"
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
