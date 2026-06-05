//! Safe Source Apply Demo v1 (#713, #1 Milestone 15).
//!
//! Composes the safe-source-apply chain into one deterministic, low-risk demo
//! plan. The plan references — but never executes — the existing preview,
//! file-class, diff-integrity, worktree-context, stale-guard, high-risk-blocker,
//! sandbox dry-run, sandbox promotion, independent-review, apply-transaction,
//! rollback-snapshot, verification, rerun-comparison, audit-ledger, and
//! evidence-bundle stages. It proves the safety chain composes, not that broad
//! source mutation is enabled: apply scope stays limited to an explicitly allowed
//! low-risk source-like fixture, every gate fails closed, and no patch is ever
//! applied, no command ever executed, and no trusted file ever written by this
//! artifact. Generated demo outputs stay untracked; only fixture-scoped JSON is
//! committed.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const SAFE_SOURCE_APPLY_DEMO_SCHEMA_VERSION: &str = "safe-source-apply-demo-v1";

/// File classes the v1 demo may target. These are source-like author files, not
/// the high-risk classes that stay blocked for v1 (dependency manifests,
/// lockfiles, CI/workflows, build/shell/install scripts, credential/network/
/// cloud code, and release/publish/export files).
const ALLOWED_SOURCE_LIKE_CLASSES: &[&str] =
    &["rust-source", "rust-test", "json-fixture", "markdown-doc"];

/// File classes that must always fail closed for v1. Mirrors the high-risk
/// blocker contract so the demo cannot silently target a forbidden class.
const FORBIDDEN_FILE_CLASSES: &[&str] = &[
    "dependency-manifest",
    "lockfile",
    "ci-workflow",
    "build-script",
    "shell-script",
    "install-script",
    "credential",
    "network-config",
    "cloud-config",
    "release-config",
    "publish-config",
    "export-config",
];

/// The ordered safe-apply chain stages the demo must walk. Each entry is the
/// canonical `stageKind` plus whether it is a pre-apply gate (`true`) or
/// post-apply evidence (`false`). The order is significant: pre-apply gates must
/// precede the apply transaction, which must precede post-apply evidence.
const CHAIN_STAGES: &[(&str, bool)] = &[
    ("preview", true),
    ("file-class", true),
    ("diff-integrity", true),
    ("worktree-context", true),
    ("stale-guard", true),
    ("high-risk-blocker", true),
    ("sandbox-dry-run", true),
    ("sandbox-promotion", true),
    ("independent-review", true),
    ("apply-transaction", false),
    ("rollback-snapshot", false),
    ("verification", false),
    ("rerun-comparison", false),
    ("audit-ledger", false),
    ("evidence-bundle", false),
];

/// Statuses a stage may report. `validated` and `recorded` are healthy; the
/// remaining states are visible failures that force the plan off `ready`.
const SUPPORTED_STAGE_STATUSES: &[&str] = &["validated", "recorded", "blocked", "stale", "skipped"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SafeSourceApplyDemoArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "demoId")]
    pub demo_id: String,
    pub status: String,
    pub fixture: Value,
    pub stages: Vec<Value>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<Value>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "cleanupPolicy")]
    pub cleanup_policy: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SafeSourceApplyDemoReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "demoId")]
    pub demo_id: String,
    pub status: String,
    #[serde(rename = "fixtureFileClass")]
    pub fixture_file_class: String,
    #[serde(rename = "fixtureRiskTier")]
    pub fixture_risk_tier: String,
    #[serde(rename = "stageCount")]
    pub stage_count: usize,
    #[serde(rename = "preApplyGateCount")]
    pub pre_apply_gate_count: usize,
    #[serde(rename = "postApplyEvidenceCount")]
    pub post_apply_evidence_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "stageStatusCounts")]
    pub stage_status_counts: BTreeMap<String, usize>,
    #[serde(rename = "expectedEvidenceCount")]
    pub expected_evidence_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl SafeSourceApplyDemoArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse Safe Source Apply Demo JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> SafeSourceApplyDemoReadModel {
        let mut stage_status_counts = BTreeMap::new();
        let mut pre = 0;
        let mut post = 0;
        for stage in &self.stages {
            let kind = str_field(stage, "stageKind").unwrap_or("");
            if CHAIN_STAGES
                .iter()
                .any(|(name, is_gate)| *name == kind && *is_gate)
            {
                pre += 1;
            } else if CHAIN_STAGES.iter().any(|(name, _)| *name == kind) {
                post += 1;
            }
            *stage_status_counts
                .entry(str_field(stage, "status").unwrap_or("unknown").to_string())
                .or_insert(0) += 1;
        }
        SafeSourceApplyDemoReadModel {
            schema_version: self.schema_version.clone(),
            demo_id: self.demo_id.clone(),
            status: self.status.clone(),
            fixture_file_class: str_field(&self.fixture, "fileClass")
                .unwrap_or_default()
                .to_string(),
            fixture_risk_tier: str_field(&self.fixture, "riskTier")
                .unwrap_or_default()
                .to_string(),
            stage_count: self.stages.len(),
            pre_apply_gate_count: pre,
            post_apply_evidence_count: post,
            blocked_count: self.blocked_count(),
            stage_status_counts,
            expected_evidence_count: self.expected_evidence.len(),
            validation_summary: vec![
                "demo walks the full safe-source-apply chain over one allowed low-risk source-like fixture".to_string(),
                "pre-apply gates (preview, file-class, diff, worktree, stale, high-risk, sandbox, promotion, independent review) precede the apply transaction".to_string(),
                "post-apply evidence (rollback, verification, rerun, audit, bundle) is required before the demo can report ready".to_string(),
                "forbidden file classes, high-risk targets, missing/self review, stale targets, missing rollback, and failed verification all fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "demo references existing source apply artifacts and never executes commands or applies patches".to_string(),
                "preview, file-class, diff, sandbox, review, apply, rollback, verification, rerun, audit, and bundle artifacts remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize safe source apply demo read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SAFE_SOURCE_APPLY_DEMO_SCHEMA_VERSION {
            return Err(anyhow!(
                "safe source apply demo schemaVersion must be {SAFE_SOURCE_APPLY_DEMO_SCHEMA_VERSION}"
            ));
        }
        require_id("safe source apply demo demoId", &self.demo_id)?;
        self.validate_fixture()?;
        self.validate_stages()?;
        validate_plan_evidence(&self.expected_evidence)?;
        validate_text_list(
            "safe source apply demo blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        require_text("safe source apply demo cleanupPolicy", &self.cleanup_policy)?;
        let cleanup = self.cleanup_policy.to_ascii_lowercase();
        if !(cleanup.contains("untracked") || cleanup.contains("ignored")) {
            return Err(anyhow!(
                "safe source apply demo cleanupPolicy must state generated outputs stay untracked/ignored"
            ));
        }
        self.validate_status()?;
        self.validate_boundary()?;
        Ok(())
    }

    fn validate_fixture(&self) -> Result<()> {
        let fixture_id = required_str(&self.fixture, "fixtureId")?;
        require_id("safe source apply demo fixture.fixtureId", fixture_id)?;
        let file_class = required_str(&self.fixture, "fileClass")?;
        if FORBIDDEN_FILE_CLASSES.contains(&file_class) {
            return Err(anyhow!(
                "safe source apply demo fixture.fileClass `{file_class}` is a forbidden high-risk class for v1"
            ));
        }
        if !ALLOWED_SOURCE_LIKE_CLASSES.contains(&file_class) {
            return Err(anyhow!(
                "safe source apply demo fixture.fileClass `{file_class}` is not an allowed source-like class for v1"
            ));
        }
        let risk_tier = required_str(&self.fixture, "riskTier")?;
        if risk_tier != "low" {
            return Err(anyhow!(
                "safe source apply demo fixture.riskTier must be `low` for v1"
            ));
        }
        require_ref(
            "safe source apply demo fixture.path",
            required_str(&self.fixture, "path")?,
        )?;
        require_text(
            "safe source apply demo fixture.description",
            required_str(&self.fixture, "description")?,
        )?;
        Ok(())
    }

    fn validate_stages(&self) -> Result<()> {
        require_nonempty("safe source apply demo stages", self.stages.len())?;
        if self.stages.len() > CHAIN_STAGES.len() {
            return Err(anyhow!(
                "safe source apply demo stages are overbroad for v1"
            ));
        }
        let mut stage_ids = BTreeSet::new();
        let mut seen_kinds: Vec<&str> = Vec::new();
        for stage in &self.stages {
            let stage_id = required_str(stage, "stageId")?;
            require_id("safe source apply demo stages.stageId", stage_id)?;
            if !stage_ids.insert(stage_id.to_string()) {
                return Err(anyhow!(
                    "safe source apply demo stageId `{stage_id}` is duplicated"
                ));
            }
            let kind = required_str(stage, "stageKind")?;
            if !CHAIN_STAGES.iter().any(|(name, _)| *name == kind) {
                return Err(anyhow!(
                    "safe source apply demo stages.stageKind `{kind}` is unsupported for v1"
                ));
            }
            if seen_kinds.contains(&kind) {
                return Err(anyhow!(
                    "safe source apply demo stageKind `{kind}` is duplicated"
                ));
            }
            seen_kinds.push(kind);
            require_allowed(
                "safe source apply demo stages.status",
                required_str(stage, "status")?,
                SUPPORTED_STAGE_STATUSES,
            )?;
            require_ref(
                "safe source apply demo stages.artifactRef",
                required_str(stage, "artifactRef")?,
            )?;
            require_ref(
                "safe source apply demo stages.evidencePath",
                required_str(stage, "evidencePath")?,
            )?;
            require_text(
                "safe source apply demo stages.summary",
                required_str(stage, "summary")?,
            )?;
            if str_field(stage, "status") == Some("blocked")
                && string_array_field(stage, "blockedReasons").is_empty()
            {
                return Err(anyhow!(
                    "safe source apply demo blocked stage `{kind}` requires blockedReasons"
                ));
            }
            validate_string_array(
                stage,
                "blockedReasons",
                "safe source apply demo stages.blockedReasons",
                false,
            )?;
            if kind == "independent-review" {
                self.validate_review_stage(stage)?;
            }
        }
        self.validate_chain_order(&seen_kinds)?;
        Ok(())
    }

    fn validate_review_stage(&self, stage: &Value) -> Result<()> {
        let author = required_str(stage, "authorId")?;
        require_id("safe source apply demo independent-review authorId", author)?;
        let reviewer = required_str(stage, "reviewerId")?;
        require_id(
            "safe source apply demo independent-review reviewerId",
            reviewer,
        )?;
        if author == reviewer {
            return Err(anyhow!(
                "safe source apply demo independent-review requires reviewerId != authorId (no self-approval)"
            ));
        }
        let decision = required_str(stage, "decision")?;
        if !matches!(decision, "accepted" | "rejected") {
            return Err(anyhow!(
                "safe source apply demo independent-review decision must be accepted or rejected"
            ));
        }
        Ok(())
    }

    /// Enforce the safe-apply ordering invariant: every present pre-apply gate
    /// precedes the apply transaction, which precedes every present post-apply
    /// evidence stage. A `ready` demo additionally requires the full chain.
    fn validate_chain_order(&self, seen_kinds: &[&str]) -> Result<()> {
        if let Some(apply_idx) = seen_kinds.iter().position(|k| *k == "apply-transaction") {
            for (idx, kind) in seen_kinds.iter().enumerate() {
                let is_gate = CHAIN_STAGES
                    .iter()
                    .any(|(name, gate)| name == kind && *gate);
                let is_post = CHAIN_STAGES
                    .iter()
                    .any(|(name, gate)| name == kind && !*gate && *name != "apply-transaction");
                if is_gate && idx > apply_idx {
                    return Err(anyhow!(
                        "safe source apply demo pre-apply gate `{kind}` must precede apply-transaction"
                    ));
                }
                if is_post && idx < apply_idx {
                    return Err(anyhow!(
                        "safe source apply demo post-apply stage `{kind}` must follow apply-transaction"
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_status(&self) -> Result<()> {
        let blocked = self.blocked_count() > 0;
        let stale = self
            .stages
            .iter()
            .any(|stage| str_field(stage, "status") == Some("stale"));
        let present: BTreeSet<&str> = self
            .stages
            .iter()
            .filter_map(|stage| str_field(stage, "stageKind"))
            .collect();
        let complete = CHAIN_STAGES.iter().all(|(name, _)| present.contains(name));
        let all_healthy = self.stages.iter().all(|stage| {
            matches!(
                str_field(stage, "status"),
                Some("validated") | Some("recorded")
            )
        });
        match self.status.as_str() {
            "ready" if blocked || stale || !complete || !all_healthy => Err(anyhow!(
                "ready safe source apply demo requires the full chain with no blockers, stale targets, or failed stages"
            ))?,
            "partial" if !blocked => Err(anyhow!(
                "partial safe source apply demo requires visible missing coverage or blocked reasons"
            ))?,
            "blocked" if !blocked => Err(anyhow!(
                "blocked safe source apply demo requires visible blocked reasons"
            ))?,
            "stale" if !stale => Err(anyhow!(
                "stale safe source apply demo requires at least one stale stage"
            ))?,
            "ready" | "partial" | "blocked" | "stale" => {}
            _ => {
                return Err(anyhow!(
                    "safe source apply demo status must be ready, partial, blocked, or stale"
                ))
            }
        }
        Ok(())
    }

    fn validate_boundary(&self) -> Result<()> {
        require_text("safe source apply demo boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "review-gated trusted source apply",
            "not unrestricted source mutation",
            "no auto-apply",
            "independent review required",
            "rollback and evidence required",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "safe source apply demo boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .stages
                .iter()
                .filter(|stage| {
                    matches!(str_field(stage, "status"), Some("blocked") | Some("stale"))
                        || !string_array_field(stage, "blockedReasons").is_empty()
                })
                .count()
    }
}

fn validate_plan_evidence(items: &[Value]) -> Result<()> {
    require_nonempty("safe source apply demo expectedEvidence", items.len())?;
    let mut ids = BTreeSet::new();
    for item in items {
        let evidence_id = required_str(item, "evidenceId")?;
        require_id(
            "safe source apply demo expectedEvidence.evidenceId",
            evidence_id,
        )?;
        if !ids.insert(evidence_id.to_string()) {
            return Err(anyhow!(
                "safe source apply demo expectedEvidence evidenceId `{evidence_id}` is duplicated"
            ));
        }
        let path_hint = required_str(item, "pathHint")?;
        require_ref(
            "safe source apply demo expectedEvidence.pathHint",
            path_hint,
        )?;
        if !path_hint.contains("evidence") && !path_hint.contains("source-apply") {
            return Err(anyhow!(
                "safe source apply demo expectedEvidence requires evidence/source-apply path hints"
            ));
        }
        require_text(
            "safe source apply demo expectedEvidence.description",
            required_str(item, "description")?,
        )?;
    }
    Ok(())
}

fn validate_string_array<'a>(
    value: &'a Value,
    key: &str,
    field: &str,
    required: bool,
) -> Result<Vec<&'a str>> {
    let Some(items) = array_field(value, key) else {
        if required {
            return Err(anyhow!("{field} must not be empty"));
        }
        return Ok(Vec::new());
    };
    if required {
        require_nonempty(field, items.len())?;
    }
    let mut result = Vec::new();
    for item in items {
        let Some(text) = item.as_str() else {
            return Err(anyhow!("{field} entries must be strings"));
        };
        require_text(field, text)?;
        result.push(text);
    }
    Ok(result)
}

fn string_array_field<'a>(value: &'a Value, key: &str) -> Vec<&'a str> {
    array_field(value, key)
        .map(|items| items.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
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

fn require_allowed(field: &str, value: &str, allowed: &[&str]) -> Result<()> {
    require_id(field, value)?;
    if !allowed.contains(&value) {
        return Err(anyhow!("{field} `{value}` is unsupported for v1"));
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

fn required_str<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("safe source apply demo missing required string field `{key}`"))
}

fn str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

fn array_field<'a>(value: &'a Value, key: &str) -> Option<&'a Vec<Value>> {
    value.get(key).and_then(Value::as_array)
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
        "dynamic import",
        "command bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "self-approval",
        "reviewer bypass",
        "godot replacement",
        "production-ready",
        "http://",
        "https://",
        "secure sandbox guarantee",
        "autonomous source repair",
        "unrestricted source mutation",
        "native export",
        "dependency mutation",
        "arbitrary source mutation",
        "arbitrary script execution",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden safe source apply authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    const NEGATIONS: [&str; 8] = [
        "no ",
        "not ",
        "without ",
        "avoid ",
        "forbid ",
        "forbidden ",
        "untrusted ",
        "never ",
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
