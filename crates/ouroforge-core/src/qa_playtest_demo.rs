use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const QA_PLAYTEST_DEMO_SCHEMA_VERSION: &str = "qa-playtest-demo-v1";

/// Bounded caps so the demo never describes unbounded exploration.
const MAX_FUZZ_INPUTS: u32 = 1024;
const MAX_WORKER_ACTIONS: u32 = 4096;
const MAX_WORKER_DURATION_MS: u64 = 600_000;

/// Stages the demo wires together end-to-end.
const STAGE_KINDS: &[&str] = &[
    "scenario-candidates",
    "fuzz-plan",
    "worker-assignment",
    "invariant-checks",
    "route-attempts",
    "visual-evidence",
    "performance-evidence",
    "error-evidence",
    "flake-handling",
    "failure-classification",
    "backlog",
    "run-matrix",
    "evidence-bundle",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaPlaytestDemoManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "demoId")]
    pub demo_id: String,
    #[serde(rename = "fixtureProjectRef")]
    pub fixture_project_ref: String,
    #[serde(rename = "fuzzPlan")]
    pub fuzz_plan: QaPlaytestFuzzPlan,
    #[serde(rename = "workerAssignments")]
    pub worker_assignments: Vec<QaPlaytestWorker>,
    pub stages: Vec<QaPlaytestStage>,
    pub commands: Vec<String>,
    #[serde(rename = "expectedEvidenceRefs")]
    pub expected_evidence_refs: Vec<String>,
    #[serde(rename = "knownGaps")]
    pub known_gaps: Vec<String>,
    #[serde(rename = "cleanupPolicy")]
    pub cleanup_policy: String,
    #[serde(rename = "generatedOutputRoots")]
    pub generated_output_roots: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaPlaytestFuzzPlan {
    pub seeds: Vec<String>,
    #[serde(rename = "maxInputs")]
    pub max_inputs: u32,
    #[serde(rename = "outputRoots")]
    pub output_roots: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaPlaytestWorker {
    #[serde(rename = "workerId")]
    pub worker_id: String,
    #[serde(rename = "maxActions")]
    pub max_actions: u32,
    #[serde(rename = "maxDurationMs")]
    pub max_duration_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaPlaytestStage {
    #[serde(rename = "stageId")]
    pub stage_id: String,
    pub kind: String,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    pub present: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaPlaytestDemoReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "demoId")]
    pub demo_id: String,
    #[serde(rename = "stageCount")]
    pub stage_count: usize,
    #[serde(rename = "presentStageCount")]
    pub present_stage_count: usize,
    #[serde(rename = "workerCount")]
    pub worker_count: usize,
    #[serde(rename = "knownGapCount")]
    pub known_gap_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl QaPlaytestDemoManifest {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let manifest: Self =
            serde_json::from_str(input).context("failed to parse QA Playtest Demo JSON")?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn read_model(&self) -> QaPlaytestDemoReadModel {
        QaPlaytestDemoReadModel {
            schema_version: self.schema_version.clone(),
            demo_id: self.demo_id.clone(),
            stage_count: self.stages.len(),
            present_stage_count: self.stages.iter().filter(|s| s.present).count(),
            worker_count: self.worker_assignments.len(),
            known_gap_count: self.known_gaps.len(),
            validation_summary: vec![
                "the demo wires scenario candidates, fuzz plans, worker assignments, invariant/route/visual/performance/error evidence, flake handling, failure classification, backlog, run matrix, and evidence bundle on a bounded fixture project".to_string(),
                "fuzz inputs, worker actions, and durations are bounded, output roots are disjoint, and a cleanup policy plus known gaps are explicit".to_string(),
                "unbounded budgets, overlapping output roots, missing cleanup, missing known gaps, missing stages, and unsafe refs fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating demo manifest with no auto-fix, auto-apply, hidden workers, or trusted mutation authority".to_string(),
                "QA/playtest outputs remain evidence and backlog inputs only until reviewed".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize QA playtest demo read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != QA_PLAYTEST_DEMO_SCHEMA_VERSION {
            return Err(anyhow!(
                "QA playtest demo schemaVersion must be {QA_PLAYTEST_DEMO_SCHEMA_VERSION}"
            ));
        }
        require_id("QA playtest demo demoId", &self.demo_id)?;
        require_ref(
            "QA playtest demo fixtureProjectRef",
            &self.fixture_project_ref,
        )?;

        // Bounded fuzz plan.
        require_nonempty(
            "QA playtest demo fuzzPlan.seeds",
            self.fuzz_plan.seeds.len(),
        )?;
        let mut seeds = BTreeSet::new();
        for seed in &self.fuzz_plan.seeds {
            require_id("QA playtest demo fuzzPlan.seed", seed)?;
            if !seeds.insert(seed.as_str()) {
                return Err(anyhow!("QA playtest demo duplicate fuzz seed `{seed}`"));
            }
        }
        if self.fuzz_plan.max_inputs == 0 || self.fuzz_plan.max_inputs > MAX_FUZZ_INPUTS {
            return Err(anyhow!(
                "QA playtest demo fuzz maxInputs must be bounded between 1 and {MAX_FUZZ_INPUTS}; unbounded fuzzing is not allowed"
            ));
        }
        validate_ref_list(
            "QA playtest demo fuzzPlan.outputRoots",
            &self.fuzz_plan.output_roots,
            true,
        )?;

        // Bounded local workers.
        require_nonempty(
            "QA playtest demo workerAssignments",
            self.worker_assignments.len(),
        )?;
        let mut worker_ids = BTreeSet::new();
        for worker in &self.worker_assignments {
            require_id("QA playtest demo workerId", &worker.worker_id)?;
            if !worker_ids.insert(worker.worker_id.as_str()) {
                return Err(anyhow!(
                    "QA playtest demo duplicate worker `{}`",
                    worker.worker_id
                ));
            }
            if worker.max_actions == 0 || worker.max_actions > MAX_WORKER_ACTIONS {
                return Err(anyhow!(
                    "QA playtest demo worker `{}` maxActions must be bounded between 1 and {MAX_WORKER_ACTIONS}; unbounded workers are not allowed",
                    worker.worker_id
                ));
            }
            if worker.max_duration_ms == 0 || worker.max_duration_ms > MAX_WORKER_DURATION_MS {
                return Err(anyhow!(
                    "QA playtest demo worker `{}` maxDurationMs must be bounded between 1 and {MAX_WORKER_DURATION_MS}",
                    worker.worker_id
                ));
            }
        }

        // Stages must cover every kind exactly once.
        require_nonempty("QA playtest demo stages", self.stages.len())?;
        let mut stage_ids = BTreeSet::new();
        let mut stage_kinds = BTreeSet::new();
        for stage in &self.stages {
            require_id("QA playtest demo stageId", &stage.stage_id)?;
            if !stage_ids.insert(stage.stage_id.as_str()) {
                return Err(anyhow!(
                    "QA playtest demo duplicate stage id `{}`",
                    stage.stage_id
                ));
            }
            if !STAGE_KINDS.contains(&stage.kind.as_str()) {
                return Err(anyhow!(
                    "QA playtest demo unsupported stage kind `{}`",
                    stage.kind
                ));
            }
            if !stage_kinds.insert(stage.kind.as_str()) {
                return Err(anyhow!(
                    "QA playtest demo duplicate stage kind `{}`",
                    stage.kind
                ));
            }
            require_ref("QA playtest demo stage.evidenceRef", &stage.evidence_ref)?;
        }
        for kind in STAGE_KINDS {
            if !stage_kinds.contains(kind) {
                return Err(anyhow!("QA playtest demo is missing stage `{kind}`"));
            }
        }

        validate_text_list("QA playtest demo commands", &self.commands, true)?;
        validate_ref_list(
            "QA playtest demo expectedEvidenceRefs",
            &self.expected_evidence_refs,
            true,
        )?;

        // A demo must state its known gaps honestly.
        validate_text_list("QA playtest demo knownGaps", &self.known_gaps, true)?;

        if self.cleanup_policy.trim().is_empty() {
            return Err(anyhow!("QA playtest demo cleanup policy must not be empty"));
        }
        require_text("QA playtest demo cleanupPolicy", &self.cleanup_policy)?;

        validate_ref_list(
            "QA playtest demo generatedOutputRoots",
            &self.generated_output_roots,
            true,
        )?;
        // All output roots (fuzz + generated) must be disjoint.
        let mut all_roots = self.fuzz_plan.output_roots.clone();
        all_roots.extend(self.generated_output_roots.iter().cloned());
        validate_no_overlap(&all_roots)?;

        require_text("QA playtest demo boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence and backlog inputs",
            "bounded",
            "no auto-fix",
            "no hidden workers",
            "review-gated",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!("QA playtest demo boundary must state `{required}`"));
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
                    "QA playtest demo has overlapping output roots `{a}` and `{b}`"
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
        "remote swarm",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden QA playtest demo authority text `{forbidden}`"
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
