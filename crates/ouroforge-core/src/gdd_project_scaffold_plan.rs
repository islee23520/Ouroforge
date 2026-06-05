use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_PROJECT_SCAFFOLD_PLAN_SCHEMA_VERSION: &str = "gdd-project-scaffold-plan-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddProjectScaffoldPlanStatus {
    Ready,
    Deferred,
    Blocked,
    Stale,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddScaffoldFileKind {
    ProjectManifest,
    Seed,
    Scene,
    AssetManifest,
    ScenarioPack,
    EvidenceNote,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddScaffoldTemplateSupport {
    Supported,
    Unsupported,
    Deferred,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GddProjectScaffoldPlanArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: GddProjectScaffoldPlanStatus,
    #[serde(rename = "feasibilityGateRef")]
    pub feasibility_gate_ref: String,
    #[serde(rename = "feasibilityState")]
    pub feasibility_state: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "generatedStateRoots")]
    pub generated_state_roots: Vec<String>,
    pub files: Vec<GddScaffoldPlannedFile>,
    #[serde(rename = "sourceLikeFixtureRefs")]
    pub source_like_fixture_refs: Vec<String>,
    #[serde(rename = "expectedCommands")]
    pub expected_commands: Vec<GddScaffoldExpectedCommand>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddScaffoldPlannedFile {
    pub path: String,
    pub kind: GddScaffoldFileKind,
    #[serde(rename = "templateSupport")]
    pub template_support: GddScaffoldTemplateSupport,
    #[serde(rename = "targetExists")]
    pub target_exists: bool,
    #[serde(rename = "staleTarget")]
    pub stale_target: bool,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddScaffoldExpectedCommand {
    pub argv: Vec<String>,
    #[serde(rename = "previewOnly")]
    pub preview_only: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddProjectScaffoldPlanReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: String,
    #[serde(rename = "plannedFileCount")]
    pub planned_file_count: usize,
    #[serde(rename = "blockedFileCount")]
    pub blocked_file_count: usize,
    #[serde(rename = "commandCount")]
    pub command_count: usize,
    #[serde(rename = "fileKindCounts")]
    pub file_kind_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddProjectScaffoldPlanArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse GDD Project Scaffold Plan JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddProjectScaffoldPlanReadModel {
        let mut file_kind_counts = BTreeMap::new();
        let mut blocked_file_count = 0;
        for file in &self.files {
            *file_kind_counts
                .entry(file_kind_label(&file.kind).to_string())
                .or_insert(0) += 1;
            if !file.blocked_reasons.is_empty() || file.stale_target {
                blocked_file_count += 1;
            }
        }
        GddProjectScaffoldPlanReadModel {
            schema_version: self.schema_version.clone(),
            plan_id: self.plan_id.clone(),
            status: status_label(&self.status).to_string(),
            planned_file_count: self.files.len(),
            blocked_file_count,
            command_count: self.expected_commands.len(),
            file_kind_counts,
            validation_summary: vec![
                "scaffold plan is preview-only and linked to a feasibility gate".to_string(),
                "planned files remain untrusted until later review-gated prototype apply".to_string(),
                "unsafe paths, stale targets, unsupported templates, and overbroad plans fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "reuses existing project manifest, scaffold, scene, asset manifest, and scenario pack contracts by reference".to_string(),
                "display-only read model; no browser trusted write or direct file write authority".to_string(),
                "generated-state roots remain explicit and ignored unless fixture-scoped".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD project scaffold plan read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_PROJECT_SCAFFOLD_PLAN_SCHEMA_VERSION {
            return Err(anyhow!(
                "GDD project scaffold plan schemaVersion must be {GDD_PROJECT_SCAFFOLD_PLAN_SCHEMA_VERSION}"
            ));
        }
        require_local_id("GDD project scaffold plan planId", &self.plan_id)?;
        require_local_ref(
            "GDD project scaffold plan feasibilityGateRef",
            &self.feasibility_gate_ref,
        )?;
        require_text(
            "GDD project scaffold plan feasibilityState",
            &self.feasibility_state,
        )?;
        require_local_id("GDD project scaffold plan projectId", &self.project_id)?;
        require_text("GDD project scaffold plan projectName", &self.project_name)?;
        validate_local_ref_list(
            "GDD project scaffold plan generatedStateRoots",
            &self.generated_state_roots,
            true,
        )?;
        validate_local_ref_list(
            "GDD project scaffold plan sourceLikeFixtureRefs",
            &self.source_like_fixture_refs,
            false,
        )?;
        require_nonempty("GDD project scaffold plan files", self.files.len())?;
        if self.files.len() > 12 {
            return Err(anyhow!(
                "GDD project scaffold plan files are overbroad for v1"
            ));
        }
        let mut paths = BTreeSet::new();
        for file in &self.files {
            file.validate(&self.generated_state_roots)?;
            if !paths.insert(file.path.clone()) {
                return Err(anyhow!(
                    "GDD project scaffold plan file path `{}` is duplicated",
                    file.path
                ));
            }
        }
        for command in &self.expected_commands {
            command.validate()?;
        }
        validate_string_list(
            "GDD project scaffold plan blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if self.feasibility_state != "pass" && self.status == GddProjectScaffoldPlanStatus::Ready {
            return Err(anyhow!(
                "ready GDD project scaffold plan requires feasibilityState pass"
            ));
        }
        let has_blocked_file = self
            .files
            .iter()
            .any(|file| !file.blocked_reasons.is_empty() || file.stale_target);
        let has_unsupported_template = self
            .files
            .iter()
            .any(|file| file.template_support != GddScaffoldTemplateSupport::Supported);
        match self.status {
            GddProjectScaffoldPlanStatus::Ready => {
                if !self.blocked_reasons.is_empty() || has_blocked_file || has_unsupported_template
                {
                    return Err(anyhow!("ready GDD project scaffold plan must not include blocked, stale, or unsupported file plans"));
                }
            }
            GddProjectScaffoldPlanStatus::Deferred | GddProjectScaffoldPlanStatus::Blocked => {
                if self.blocked_reasons.is_empty() && !has_blocked_file && !has_unsupported_template
                {
                    return Err(anyhow!("deferred/blocked GDD project scaffold plan requires visible blocked reasons or unsupported templates"));
                }
            }
            GddProjectScaffoldPlanStatus::Stale => {
                if !self.files.iter().any(|file| file.stale_target) {
                    return Err(anyhow!(
                        "stale GDD project scaffold plan requires at least one staleTarget"
                    ));
                }
            }
        }
        require_text("GDD project scaffold plan boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "preview first",
            "untrusted until rust/local validation",
            "review-gated prototype apply",
            "no direct trusted writes",
            "no autonomous unrestricted game creation",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD project scaffold plan boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl GddScaffoldPlannedFile {
    fn validate(&self, generated_roots: &[String]) -> Result<()> {
        require_local_ref("GDD project scaffold plan files.path", &self.path)?;
        validate_string_list(
            "GDD project scaffold plan files.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if self.template_support != GddScaffoldTemplateSupport::Supported
            && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!("GDD project scaffold plan unsupported/deferred template `{}` must include blockedReasons", self.path));
        }
        if self.stale_target && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "GDD project scaffold plan stale target `{}` must include blockedReasons",
                self.path
            ));
        }
        if generated_roots.contains(&self.path) {
            return Err(anyhow!(
                "GDD project scaffold plan file `{}` collides with generatedStateRoots",
                self.path
            ));
        }
        Ok(())
    }
}

impl GddScaffoldExpectedCommand {
    fn validate(&self) -> Result<()> {
        require_nonempty(
            "GDD project scaffold plan expectedCommands.argv",
            self.argv.len(),
        )?;
        for arg in &self.argv {
            require_text("GDD project scaffold plan expectedCommands.argv", arg)?;
        }
        if !self.preview_only
            || self
                .argv
                .iter()
                .any(|arg| arg == "--apply" || arg == "write")
        {
            return Err(anyhow!(
                "GDD project scaffold plan expectedCommands must remain preview-only"
            ));
        }
        Ok(())
    }
}

fn status_label(status: &GddProjectScaffoldPlanStatus) -> &'static str {
    match status {
        GddProjectScaffoldPlanStatus::Ready => "ready",
        GddProjectScaffoldPlanStatus::Deferred => "deferred",
        GddProjectScaffoldPlanStatus::Blocked => "blocked",
        GddProjectScaffoldPlanStatus::Stale => "stale",
    }
}
fn file_kind_label(kind: &GddScaffoldFileKind) -> &'static str {
    match kind {
        GddScaffoldFileKind::ProjectManifest => "project-manifest",
        GddScaffoldFileKind::Seed => "seed",
        GddScaffoldFileKind::Scene => "scene",
        GddScaffoldFileKind::AssetManifest => "asset-manifest",
        GddScaffoldFileKind::ScenarioPack => "scenario-pack",
        GddScaffoldFileKind::EvidenceNote => "evidence-note",
    }
}
fn validate_string_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}
fn validate_local_ref_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_local_ref(field, value)?;
    }
    Ok(())
}
fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}
fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, or dot"
        ));
    }
    Ok(())
}
fn require_local_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} contains forbidden traversal and must stay inside local fixture/reference roots"
        ));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/")
        || value.starts_with("runs/"))
    {
        return Err(anyhow!(
            "{field} must use examples/, docs/, seeds/, or runs/ refs"
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
        "dynamic import",
        "command bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "godot replacement",
        "production-ready",
        "http://",
        "https://",
        "autonomous unrestricted game creation",
        "native export",
        "plugin runtime",
        "asset generation",
        "commercial readiness",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD/scaffold authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}
fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    const NEGATIONS: [&str; 6] = ["no ", "not ", "without ", "avoid ", "forbid ", "forbidden "];
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
