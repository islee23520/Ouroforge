use crate::{add_evidence_artifact, list_evidence_artifacts, EvidenceArtifact};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

pub const PLUGIN_REGISTRY_EVIDENCE_SCHEMA_VERSION: &str = "ouroforge.plugin-registry-evidence.v1";

const ALLOWED_CAPABILITIES: &[&str] = &[
    "dashboardPanel",
    "studioInspectorPanel",
    "scenarioTemplate",
    "assetMetadataProvider",
];

const ALLOWED_EXTENSION_POINTS: &[&str] = &[
    "dashboard.panels.readOnly",
    "studio.inspector.readOnly",
    "scenario.templates.readOnly",
    "assets.metadata.readOnly",
];

const ALLOWED_DASHBOARD_PANEL_DATA_SOURCES: &[&str] = &[
    "pluginRegistry.summary",
    "pluginRegistry.plugins",
    "pluginRegistry.blockedReasons",
];

const ALLOWED_DASHBOARD_PANEL_TEMPLATES: &[&str] = &[
    "pluginRegistrySummaryCard",
    "pluginDescriptorTable",
    "pluginBlockedReasonList",
];

const ALLOWED_DASHBOARD_PANEL_LAYOUTS: &[&str] = &["summary", "table", "list"];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginRegistryEvidenceArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "registryId")]
    pub registry_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "ledgerRef")]
    pub ledger_ref: String,
    #[serde(rename = "generatedState")]
    pub generated_state: PluginGeneratedStatePolicy,
    pub plugins: Vec<PluginDescriptorEvidence>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginGeneratedStatePolicy {
    pub root: String,
    #[serde(rename = "fixtureScoped")]
    pub fixture_scoped: bool,
    #[serde(rename = "trackedPolicy")]
    pub tracked_policy: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginDescriptorEvidence {
    #[serde(rename = "pluginId")]
    pub plugin_id: String,
    #[serde(rename = "manifestPath")]
    pub manifest_path: String,
    #[serde(rename = "manifestHash")]
    pub manifest_hash: String,
    #[serde(rename = "manifestVersion")]
    pub manifest_version: String,
    #[serde(rename = "validationStatus")]
    pub validation_status: PluginValidationStatus,
    #[serde(rename = "compatibilityStatus")]
    pub compatibility_status: PluginCompatibilityStatus,
    #[serde(rename = "declaredCapabilities", default)]
    pub declared_capabilities: Vec<String>,
    #[serde(rename = "extensionPoints", default)]
    pub extension_points: Vec<String>,
    #[serde(rename = "evidenceRefs", default)]
    pub evidence_refs: Vec<PluginEvidenceRef>,
    #[serde(rename = "dashboardPanels", default)]
    pub dashboard_panels: Vec<PluginDashboardPanelDescriptor>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginDashboardPanelDescriptor {
    #[serde(rename = "panelId")]
    pub panel_id: String,
    pub title: String,
    #[serde(rename = "dataSourceKey")]
    pub data_source_key: String,
    #[serde(rename = "templateRef")]
    pub template_ref: String,
    #[serde(rename = "layoutHint")]
    pub layout_hint: String,
    #[serde(rename = "displayHints", default)]
    pub display_hints: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginEvidenceRef {
    pub id: String,
    pub kind: String,
    pub path: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PluginValidationStatus {
    Valid,
    Blocked,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PluginCompatibilityStatus {
    Compatible,
    Incompatible,
    Unknown,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PluginRegistryEvidenceReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "registryId")]
    pub registry_id: String,
    pub status: String,
    #[serde(rename = "pluginCount")]
    pub plugin_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "capabilitySummary")]
    pub capability_summary: Vec<String>,
    #[serde(rename = "extensionPointSummary")]
    pub extension_point_summary: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "dashboardPanelSummary")]
    pub dashboard_panel_summary: Vec<String>,
    pub boundary: String,
}

impl PluginRegistryEvidenceArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse Plugin Registry Evidence JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PLUGIN_REGISTRY_EVIDENCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "plugin registry evidence schemaVersion must be {PLUGIN_REGISTRY_EVIDENCE_SCHEMA_VERSION}"
            ));
        }
        require_local_id("plugin registry evidence registryId", &self.registry_id)?;
        require_local_id("plugin registry evidence projectId", &self.project_id)?;
        require_local_id("plugin registry evidence runId", &self.run_id)?;
        require_generated_ref("plugin registry evidence ledgerRef", &self.ledger_ref)?;
        self.generated_state.validate()?;
        if self.plugins.is_empty() {
            return Err(anyhow!(
                "plugin registry evidence plugins must not be empty"
            ));
        }
        require_unique_ids(
            "plugin registry evidence plugins.pluginId",
            self.plugins.iter().map(|plugin| plugin.plugin_id.as_str()),
        )?;
        for plugin in &self.plugins {
            plugin.validate()?;
        }
        for reason in &self.blocked_reasons {
            require_local_text("plugin registry evidence blockedReasons", reason)?;
        }
        require_local_text("plugin registry evidence boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "declarative",
            "no executable plugin",
            "no command execution",
            "no network install",
            "read-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "plugin registry evidence boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    pub fn read_model(&self) -> PluginRegistryEvidenceReadModel {
        let mut capability_summary = Vec::new();
        let mut extension_point_summary = Vec::new();
        let mut blocked_reasons = self.blocked_reasons.clone();
        let mut dashboard_panel_summary = Vec::new();
        let mut blocked_count = 0;
        for plugin in &self.plugins {
            capability_summary.extend(
                plugin
                    .declared_capabilities
                    .iter()
                    .map(|capability| format!("{}:{capability}", plugin.plugin_id)),
            );
            extension_point_summary.extend(
                plugin
                    .extension_points
                    .iter()
                    .map(|extension_point| format!("{}:{extension_point}", plugin.plugin_id)),
            );
            dashboard_panel_summary.extend(plugin.dashboard_panels.iter().map(|panel| {
                format!(
                    "{}:{}:{}",
                    plugin.plugin_id, panel.panel_id, panel.template_ref
                )
            }));
            if plugin.validation_status == PluginValidationStatus::Blocked
                || plugin.compatibility_status == PluginCompatibilityStatus::Incompatible
                || !plugin.blocked_reasons.is_empty()
            {
                blocked_count += 1;
            }
            blocked_reasons.extend(
                plugin
                    .blocked_reasons
                    .iter()
                    .map(|reason| format!("{}:{reason}", plugin.plugin_id)),
            );
        }
        capability_summary.sort();
        capability_summary.dedup();
        extension_point_summary.sort();
        extension_point_summary.dedup();
        blocked_reasons.sort();
        blocked_reasons.dedup();
        dashboard_panel_summary.sort();
        dashboard_panel_summary.dedup();
        PluginRegistryEvidenceReadModel {
            schema_version: "ouroforge.plugin-registry-evidence-read-model.v1".to_string(),
            registry_id: self.registry_id.clone(),
            status: if blocked_count == 0 && blocked_reasons.is_empty() {
                "valid".to_string()
            } else {
                "blocked".to_string()
            },
            plugin_count: self.plugins.len(),
            blocked_count,
            capability_summary,
            extension_point_summary,
            blocked_reasons,
            dashboard_panel_summary,
            boundary: "Read-only plugin registry evidence summary; displays declarative descriptors and allowlisted dashboard panel descriptors without executing plugins, installing dependencies, running commands, mutating source, publishing, deploying, or writing trusted files.".to_string(),
        }
    }
}

pub fn write_plugin_registry_evidence(
    run_dir: impl AsRef<Path>,
    artifact: &PluginRegistryEvidenceArtifact,
) -> Result<EvidenceArtifact> {
    artifact.validate()?;
    let run_dir = run_dir.as_ref();
    let artifact_path = format!("evidence/plugins/{}.json", artifact.registry_id);
    let evidence_id = format!("plugin-registry-{}", artifact.registry_id);
    for existing in list_evidence_artifacts(run_dir)? {
        if existing.id == evidence_id {
            return Err(anyhow!(
                "plugin registry evidence id already exists: {evidence_id}"
            ));
        }
        if existing.path == artifact_path {
            return Err(anyhow!(
                "plugin registry evidence path already exists in evidence index: {artifact_path}"
            ));
        }
    }
    let output_path = run_dir.join(&artifact_path);
    if output_path.exists() {
        return Err(anyhow!(
            "plugin registry evidence output already exists: {artifact_path}"
        ));
    }
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create plugin registry evidence directory {}",
                parent.display()
            )
        })?;
    }
    fs::write(
        &output_path,
        serde_json::to_string_pretty(artifact)
            .context("failed to serialize plugin registry evidence")?,
    )
    .with_context(|| {
        format!(
            "failed to write plugin registry evidence {}",
            output_path.display()
        )
    })?;
    let read_model = artifact.read_model();
    add_evidence_artifact(
        run_dir,
        &evidence_id,
        "application/json",
        &artifact_path,
        serde_json::json!({
            "artifact": "plugin_registry_evidence",
            "schemaVersion": artifact.schema_version,
            "registryId": artifact.registry_id,
            "pluginCount": read_model.plugin_count,
            "blockedCount": read_model.blocked_count,
            "status": read_model.status,
            "boundary": "validated declarative plugin registry evidence; no plugins were executed and no trusted files were written"
        }),
    )
}

impl PluginGeneratedStatePolicy {
    fn validate(&self) -> Result<()> {
        require_generated_ref("plugin registry evidence generatedState.root", &self.root)?;
        if !self.fixture_scoped && !self.tracked_policy.contains("ignored") {
            return Err(anyhow!(
                "plugin registry evidence generatedState.trackedPolicy must keep non-fixture generated outputs ignored"
            ));
        }
        require_local_text(
            "plugin registry evidence generatedState.trackedPolicy",
            &self.tracked_policy,
        )
    }
}

impl PluginDescriptorEvidence {
    fn validate(&self) -> Result<()> {
        require_local_id("plugin descriptor evidence pluginId", &self.plugin_id)?;
        require_manifest_path(
            "plugin descriptor evidence manifestPath",
            &self.manifest_path,
        )?;
        require_hash_text(
            "plugin descriptor evidence manifestHash",
            &self.manifest_hash,
        )?;
        require_local_text(
            "plugin descriptor evidence manifestVersion",
            &self.manifest_version,
        )?;
        require_allowed_values(
            "plugin descriptor evidence declaredCapabilities",
            &self.declared_capabilities,
            ALLOWED_CAPABILITIES,
        )?;
        require_allowed_values(
            "plugin descriptor evidence extensionPoints",
            &self.extension_points,
            ALLOWED_EXTENSION_POINTS,
        )?;
        require_unique_ids(
            "plugin descriptor evidence evidenceRefs.id",
            self.evidence_refs
                .iter()
                .map(|evidence| evidence.id.as_str()),
        )?;
        for evidence in &self.evidence_refs {
            evidence.validate()?;
        }
        require_unique_ids(
            "plugin descriptor evidence dashboardPanels.panelId",
            self.dashboard_panels
                .iter()
                .map(|panel| panel.panel_id.as_str()),
        )?;
        for panel in &self.dashboard_panels {
            panel.validate()?;
        }
        if self.dashboard_panels.is_empty()
            && self
                .extension_points
                .iter()
                .any(|point| point == "dashboard.panels.readOnly")
        {
            return Err(anyhow!(
                "plugin descriptor evidence dashboard.panels.readOnly requires at least one dashboardPanels descriptor"
            ));
        }
        if !self.dashboard_panels.is_empty()
            && !self
                .extension_points
                .iter()
                .any(|point| point == "dashboard.panels.readOnly")
        {
            return Err(anyhow!(
                "plugin descriptor evidence dashboardPanels require dashboard.panels.readOnly extension point"
            ));
        }
        for reason in &self.blocked_reasons {
            require_local_text("plugin descriptor evidence blockedReasons", reason)?;
        }
        if self.validation_status == PluginValidationStatus::Valid
            && !self.blocked_reasons.is_empty()
        {
            return Err(anyhow!(
                "plugin descriptor evidence valid status must not include blockedReasons"
            ));
        }
        if self.validation_status == PluginValidationStatus::Blocked
            && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!(
                "plugin descriptor evidence blocked status requires blockedReasons"
            ));
        }
        if self.compatibility_status == PluginCompatibilityStatus::Incompatible
            && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!(
                "plugin descriptor evidence incompatible status requires blockedReasons"
            ));
        }
        Ok(())
    }
}

impl PluginDashboardPanelDescriptor {
    fn validate(&self) -> Result<()> {
        require_local_id("plugin dashboard panel descriptor panelId", &self.panel_id)?;
        require_local_text("plugin dashboard panel descriptor title", &self.title)?;
        require_allowed_value(
            "plugin dashboard panel descriptor dataSourceKey",
            &self.data_source_key,
            ALLOWED_DASHBOARD_PANEL_DATA_SOURCES,
        )?;
        require_allowed_value(
            "plugin dashboard panel descriptor templateRef",
            &self.template_ref,
            ALLOWED_DASHBOARD_PANEL_TEMPLATES,
        )?;
        require_allowed_value(
            "plugin dashboard panel descriptor layoutHint",
            &self.layout_hint,
            ALLOWED_DASHBOARD_PANEL_LAYOUTS,
        )?;
        require_unique_ids(
            "plugin dashboard panel descriptor displayHints",
            self.display_hints.iter().map(|hint| hint.as_str()),
        )?;
        for hint in &self.display_hints {
            require_local_id("plugin dashboard panel descriptor displayHints", hint)?;
        }
        require_local_text("plugin dashboard panel descriptor boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "declarative",
            "allowlisted",
            "read-only",
            "no javascript",
            "no command",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "plugin dashboard panel descriptor boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl PluginEvidenceRef {
    fn validate(&self) -> Result<()> {
        require_local_id("plugin descriptor evidence evidenceRefs.id", &self.id)?;
        require_local_id("plugin descriptor evidence evidenceRefs.kind", &self.kind)?;
        require_generated_ref("plugin descriptor evidence evidenceRefs.path", &self.path)
    }
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if value.len() > 96
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

fn require_local_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "</script",
        "javascript:",
        "http://",
        "https://",
        "onerror=",
        "onclick=",
        "onload=",
        "srcdoc=",
        "eval(",
        "dynamic import",
        "command bridge",
        "shell command",
        "credential",
        "native extension",
        "dynamic library",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden executable plugin or release-authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn require_manifest_path(field: &str, value: &str) -> Result<()> {
    require_local_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} must stay inside the local project tree without traversal"
        ));
    }
    if !value.ends_with("plugin.json") && !value.ends_with(".plugin.json") {
        return Err(anyhow!("{field} must point at a plugin.json manifest"));
    }
    Ok(())
}

fn require_generated_ref(field: &str, value: &str) -> Result<()> {
    require_local_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} must stay inside the local generated/evidence tree without traversal"
        ));
    }
    if !(value.starts_with("runs/")
        || value.starts_with("evidence/")
        || value.starts_with("dashboard-data/")
        || value.starts_with(".omx/"))
    {
        return Err(anyhow!(
            "{field} must be under runs/, evidence/, dashboard-data/, or .omx/"
        ));
    }
    Ok(())
}

fn require_hash_text(field: &str, value: &str) -> Result<()> {
    let Some((algorithm, digest)) = value.split_once(':') else {
        return Err(anyhow!("{field} must include algorithm:digest"));
    };
    if algorithm != "fnv1a64-canonical-json-v1" {
        return Err(anyhow!("{field} must use fnv1a64-canonical-json-v1"));
    }
    if digest.len() != 16 || !digest.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} digest must be 16 hex characters"));
    }
    Ok(())
}

fn require_allowed_value(field: &str, value: &str, allowed: &[&str]) -> Result<()> {
    require_local_text(field, value)?;
    let allowed = allowed.iter().copied().collect::<BTreeSet<_>>();
    if !allowed.contains(value) {
        return Err(anyhow!(
            "{field} value `{value}` is not in the v1 allowlist"
        ));
    }
    Ok(())
}

fn require_allowed_values(field: &str, values: &[String], allowed: &[&str]) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let allowed = allowed.iter().copied().collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    for value in values {
        require_local_text(field, value)?;
        if !allowed.contains(value.as_str()) {
            return Err(anyhow!(
                "{field} value `{value}` is not in the v1 allowlist"
            ));
        }
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{field} value `{value}` must be unique"));
        }
    }
    Ok(())
}

fn require_unique_ids<'a>(field: &str, values: impl IntoIterator<Item = &'a str>) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(anyhow!("{field} `{value}` must be unique"));
        }
    }
    Ok(())
}
