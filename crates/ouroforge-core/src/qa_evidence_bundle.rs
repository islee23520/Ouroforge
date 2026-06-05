use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const QA_EVIDENCE_BUNDLE_SCHEMA_VERSION: &str = "qa-swarm-evidence-bundle-v1";

/// Every clearly-separated QA/playtest artifact the bundle must reference.
const COMPONENT_TYPES: &[&str] = &[
    "scenario-candidates",
    "fuzz-plans",
    "worker-assignments",
    "invariant-checks",
    "route-attempts",
    "visual-comparisons",
    "performance-budgets",
    "error-classifications",
    "flake-policy",
    "failure-classifications",
    "mutation-backlog",
    "run-matrix",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaEvidenceBundleArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    pub status: String,
    #[serde(rename = "runRef")]
    pub run_ref: String,
    pub components: Vec<QaBundleComponent>,
    #[serde(rename = "outputRoots")]
    pub output_roots: Vec<String>,
    #[serde(rename = "cleanupConfirmed")]
    pub cleanup_confirmed: bool,
    #[serde(rename = "budgetsConfirmed")]
    pub budgets_confirmed: bool,
    #[serde(rename = "matrixRowsConsistent")]
    pub matrix_rows_consistent: bool,
    #[serde(rename = "flakyResolved")]
    pub flaky_resolved: bool,
    #[serde(rename = "finalSummary")]
    pub final_summary: String,
    #[serde(rename = "dashboardExport")]
    pub dashboard_export: QaBundleDashboardExport,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaBundleComponent {
    #[serde(rename = "componentType")]
    pub component_type: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub present: bool,
    #[serde(default)]
    pub stale: bool,
    pub resolved: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaBundleDashboardExport {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub surface: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaEvidenceBundleReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    pub status: String,
    #[serde(rename = "componentCount")]
    pub component_count: usize,
    #[serde(rename = "presentComponentCount")]
    pub present_component_count: usize,
    #[serde(rename = "resolvedComponentCount")]
    pub resolved_component_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "componentStatusByType")]
    pub component_status_by_type: BTreeMap<String, String>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl QaEvidenceBundleArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse QA Evidence Bundle JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> QaEvidenceBundleReadModel {
        let mut component_status_by_type = BTreeMap::new();
        for component in &self.components {
            let status = if component.stale {
                "stale"
            } else if !component.present {
                "missing"
            } else if !component.resolved {
                "unresolved"
            } else {
                "resolved"
            };
            component_status_by_type.insert(component.component_type.clone(), status.to_string());
        }
        QaEvidenceBundleReadModel {
            schema_version: self.schema_version.clone(),
            bundle_id: self.bundle_id.clone(),
            status: self.computed_status(),
            component_count: self.components.len(),
            present_component_count: self.components.iter().filter(|c| c.present).count(),
            resolved_component_count: self
                .components
                .iter()
                .filter(|c| c.resolved && c.present && !c.stale)
                .count(),
            blocked_count: self.blocked_reasons.len(),
            component_status_by_type,
            validation_summary: vec![
                "the bundle references scenario candidates, fuzz plans, worker assignments, invariant checks, route attempts, visual/performance/error evidence, flake policy, failure classification, mutation backlog, and run matrix".to_string(),
                "status rolls up to complete, partial, blocked, or stale with explicit cleanup, budget, and matrix-consistency confirmation".to_string(),
                "missing refs, stale artifacts, unresolved flaky states, missing budgets, missing cleanup confirmation, unresolved output roots, and inconsistent matrix rows fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model with no auto-fix, auto-apply, or trusted mutation authority".to_string(),
                "dashboard export stays read-only or draft-only; existing QA artifact read models remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize QA evidence bundle read model JSON")
    }

    pub fn computed_status(&self) -> String {
        if self.components.iter().any(|c| c.stale) {
            return "stale".to_string();
        }
        let has_gap =
            self.components.iter().any(|c| !c.present || !c.resolved) || !self.flaky_resolved;
        if has_gap {
            return "partial".to_string();
        }
        if !self.blocked_reasons.is_empty() {
            return "blocked".to_string();
        }
        "complete".to_string()
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != QA_EVIDENCE_BUNDLE_SCHEMA_VERSION {
            return Err(anyhow!(
                "QA evidence bundle schemaVersion must be {QA_EVIDENCE_BUNDLE_SCHEMA_VERSION}"
            ));
        }
        require_id("QA evidence bundle bundleId", &self.bundle_id)?;
        if self.run_ref.trim().is_empty() {
            return Err(anyhow!("QA evidence bundle is missing run ref"));
        }
        require_ref("QA evidence bundle runRef", &self.run_ref)?;

        // Components must cover every clearly-separated artifact exactly once.
        let mut seen = BTreeSet::new();
        for component in &self.components {
            component.validate()?;
            if !seen.insert(component.component_type.as_str()) {
                return Err(anyhow!(
                    "QA evidence bundle duplicate component `{}`",
                    component.component_type
                ));
            }
        }
        for required in COMPONENT_TYPES {
            if !seen.contains(required) {
                return Err(anyhow!(
                    "QA evidence bundle is missing component `{required}`"
                ));
            }
        }

        require_text("QA evidence bundle finalSummary", &self.final_summary)?;
        self.dashboard_export.validate()?;

        // Output roots must resolve to disjoint local roots.
        validate_ref_list("QA evidence bundle outputRoots", &self.output_roots, true)?;
        validate_no_overlap(&self.output_roots)?;

        if !self.cleanup_confirmed {
            return Err(anyhow!(
                "QA evidence bundle is missing cleanup confirmation"
            ));
        }
        if !self.budgets_confirmed {
            return Err(anyhow!("QA evidence bundle is missing budgets"));
        }
        if !self.matrix_rows_consistent {
            return Err(anyhow!("QA evidence bundle has inconsistent matrix rows"));
        }

        validate_text_list(
            "QA evidence bundle blockedReasons",
            &self.blocked_reasons,
            false,
        )?;

        let computed = self.computed_status();
        if self.status != computed {
            return Err(anyhow!(
                "QA evidence bundle status `{}` does not match computed classification `{computed}`",
                self.status
            ));
        }
        if computed != "complete" && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "QA evidence bundle {computed} status requires visible blockedReasons"
            ));
        }

        require_text("QA evidence bundle boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence inputs",
            "not trusted truth",
            "no auto-fix",
            "review-gated",
            "read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "QA evidence bundle boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl QaBundleComponent {
    fn validate(&self) -> Result<()> {
        if !COMPONENT_TYPES.contains(&self.component_type.as_str()) {
            return Err(anyhow!(
                "QA evidence bundle malformed artifact: unsupported component type `{}`",
                self.component_type
            ));
        }
        // A present component must carry a usable ref.
        if self.present {
            if self.reference.trim().is_empty() {
                return Err(anyhow!(
                    "QA evidence bundle component `{}` is missing its ref",
                    self.component_type
                ));
            }
            require_ref("QA evidence bundle component ref", &self.reference)?;
        } else if !self.reference.trim().is_empty() {
            // Absent components must not claim a ref.
            require_ref("QA evidence bundle component ref", &self.reference)?;
        }
        Ok(())
    }
}

impl QaBundleDashboardExport {
    fn validate(&self) -> Result<()> {
        if !self.read_only {
            return Err(anyhow!(
                "QA evidence bundle dashboard export must remain read-only or draft-only"
            ));
        }
        require_text("QA evidence bundle dashboardExport.surface", &self.surface)?;
        validate_text_list(
            "QA evidence bundle dashboardExport.fields",
            &self.fields,
            true,
        )
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
                    "QA evidence bundle has unresolved output roots `{a}` and `{b}`"
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
        "shipped-game",
        "hidden worker",
        "remote swarm",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden QA evidence bundle authority text `{forbidden}`"
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
