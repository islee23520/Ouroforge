use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_ASSET_PLACEHOLDER_PLAN_SCHEMA_VERSION: &str = "gdd-asset-placeholder-plan-v1";
const SUPPORTED_ASSET_TYPES: &[&str] = &["sprite", "tileset", "audio", "ui-icon", "placeholder"];
const SUPPORTED_SOURCE_KINDS: &[&str] = &[
    "placeholder",
    "local-fixture",
    "manifest-ref",
    "missing",
    "unsupported",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GddAssetPlaceholderPlanArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: String,
    #[serde(rename = "feasibilityGateRef")]
    pub feasibility_gate_ref: String,
    #[serde(rename = "scaffoldPlanRef")]
    pub scaffold_plan_ref: String,
    #[serde(rename = "requirementIds")]
    pub requirement_ids: Vec<String>,
    #[serde(rename = "mechanicsMappingIds")]
    pub mechanics_mapping_ids: Vec<String>,
    #[serde(rename = "manifestRefs")]
    pub manifest_refs: Vec<String>,
    #[serde(rename = "assetEntries")]
    pub asset_entries: Vec<Value>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<Value>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddAssetPlaceholderPlanReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: String,
    #[serde(rename = "requirementCount")]
    pub requirement_count: usize,
    #[serde(rename = "mechanicsMappingCount")]
    pub mechanics_mapping_count: usize,
    #[serde(rename = "manifestRefCount")]
    pub manifest_ref_count: usize,
    #[serde(rename = "assetEntryCount")]
    pub asset_entry_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "assetTypeCounts")]
    pub asset_type_counts: BTreeMap<String, usize>,
    #[serde(rename = "sourceKindCounts")]
    pub source_kind_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddAssetPlaceholderPlanArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse GDD Asset Placeholder Plan JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddAssetPlaceholderPlanReadModel {
        let mut asset_type_counts = BTreeMap::new();
        let mut source_kind_counts = BTreeMap::new();
        for entry in &self.asset_entries {
            *asset_type_counts
                .entry(
                    str_field(entry, "assetType")
                        .unwrap_or("unknown")
                        .to_string(),
                )
                .or_insert(0) += 1;
            *source_kind_counts
                .entry(
                    str_field(entry, "sourceKind")
                        .unwrap_or("unknown")
                        .to_string(),
                )
                .or_insert(0) += 1;
        }
        GddAssetPlaceholderPlanReadModel {
            schema_version: self.schema_version.clone(),
            plan_id: self.plan_id.clone(),
            status: self.status.clone(),
            requirement_count: self.requirement_ids.len(),
            mechanics_mapping_count: self.mechanics_mapping_ids.len(),
            manifest_ref_count: self.manifest_refs.len(),
            asset_entry_count: self.asset_entries.len(),
            blocked_count: self.blocked_count(),
            asset_type_counts,
            source_kind_counts,
            validation_summary: vec![
                "asset entries link to GDD requirement ids and mechanics mapping ids".to_string(),
                "only placeholders, local fixtures, or existing manifest refs are accepted".to_string(),
                "missing license/source notes, remote refs, unsupported asset types, generated-root collisions, stale refs, and unclear ownership fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model with no asset generation, remote fetch, or trusted write authority".to_string(),
                "existing asset manifests, scenes, project manifests, dashboard exports, and Studio read models remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD asset placeholder plan read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_ASSET_PLACEHOLDER_PLAN_SCHEMA_VERSION {
            return Err(anyhow!("GDD asset placeholder plan schemaVersion must be {GDD_ASSET_PLACEHOLDER_PLAN_SCHEMA_VERSION}"));
        }
        require_id("GDD asset placeholder plan planId", &self.plan_id)?;
        require_ref(
            "GDD asset placeholder plan feasibilityGateRef",
            &self.feasibility_gate_ref,
        )?;
        require_ref(
            "GDD asset placeholder plan scaffoldPlanRef",
            &self.scaffold_plan_ref,
        )?;
        validate_id_list(
            "GDD asset placeholder plan requirementIds",
            &self.requirement_ids,
            true,
        )?;
        validate_id_list(
            "GDD asset placeholder plan mechanicsMappingIds",
            &self.mechanics_mapping_ids,
            true,
        )?;
        validate_manifest_ref_list(
            "GDD asset placeholder plan manifestRefs",
            &self.manifest_refs,
            true,
        )?;
        require_nonempty(
            "GDD asset placeholder plan assetEntries",
            self.asset_entries.len(),
        )?;
        require_nonempty(
            "GDD asset placeholder plan expectedEvidence",
            self.expected_evidence.len(),
        )?;
        if self.asset_entries.len() > 24 {
            return Err(anyhow!("GDD asset placeholder plan is overbroad for v1"));
        }
        let reqs: BTreeSet<&str> = self.requirement_ids.iter().map(String::as_str).collect();
        let maps: BTreeSet<&str> = self
            .mechanics_mapping_ids
            .iter()
            .map(String::as_str)
            .collect();
        let manifests: BTreeSet<&str> = self.manifest_refs.iter().map(String::as_str).collect();
        validate_assets(&self.asset_entries, &reqs, &maps, &manifests)?;
        validate_evidence(&self.expected_evidence)?;
        validate_text_list(
            "GDD asset placeholder plan blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        let stale = self
            .asset_entries
            .iter()
            .any(|entry| bool_field(entry, "staleRef"));
        let blocked = self.blocked_count() > 0;
        match self.status.as_str() {
            "ready" if stale || blocked => Err(anyhow!("ready GDD asset placeholder plan must not include stale refs, missing assets, unsupported assets, or blockers"))?,
            "partial" if !blocked => Err(anyhow!("partial GDD asset placeholder plan requires missing asset warnings or blocked reasons"))?,
            "blocked" if !blocked => Err(anyhow!("blocked GDD asset placeholder plan requires visible blocked reasons"))?,
            "stale" if !stale => Err(anyhow!("stale GDD asset placeholder plan requires at least one staleRef"))?,
            "ready" | "partial" | "blocked" | "stale" => {}
            _ => return Err(anyhow!("GDD asset placeholder plan status must be ready, partial, blocked, or stale")),
        }
        require_text("GDD asset placeholder plan boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "placeholder assets or known local refs",
            "license/source notes",
            "untrusted until rust/local validation",
            "review-gated apply",
            "no asset generation",
            "no remote fetch",
            "no autonomous unrestricted game creation",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD asset placeholder plan boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .asset_entries
                .iter()
                .filter(|entry| {
                    matches!(
                        str_field(entry, "sourceKind"),
                        Some("missing" | "unsupported")
                    ) || bool_field(entry, "staleRef")
                        || !array_empty(entry, "blockedReasons")
                })
                .count()
    }
}

fn validate_assets(
    values: &[Value],
    reqs: &BTreeSet<&str>,
    maps: &BTreeSet<&str>,
    manifests: &BTreeSet<&str>,
) -> Result<()> {
    let mut ids = BTreeSet::new();
    for entry in values {
        let id = required_id(entry, "assetId")?;
        if !ids.insert(id.to_string()) {
            return Err(anyhow!(
                "GDD asset placeholder plan assetId `{id}` is duplicated"
            ));
        }
        let asset_type = required_str(entry, "assetType")?;
        if !SUPPORTED_ASSET_TYPES.contains(&asset_type) {
            return Err(anyhow!(
                "GDD asset placeholder plan unsupported asset type `{asset_type}`"
            ));
        }
        let source_kind = required_str(entry, "sourceKind")?;
        if !SUPPORTED_SOURCE_KINDS.contains(&source_kind) {
            return Err(anyhow!(
                "GDD asset placeholder plan unsupported source kind `{source_kind}`"
            ));
        }
        validate_trace_links(
            "GDD asset placeholder plan assetEntries.traceLinks",
            entry.get("traceLinks"),
            reqs,
            maps,
        )?;
        require_text(
            "GDD asset placeholder plan assetEntries.styleNotes",
            required_str(entry, "styleNotes")?,
        )?;
        validate_required_metadata(entry.get("requiredMetadata"))?;
        let note = required_str(entry, "licenseSourceNote")?;
        require_license_note(note)?;
        let blockers = string_array(
            entry.get("blockedReasons"),
            "GDD asset placeholder plan assetEntries.blockedReasons",
            false,
        )?;
        validate_text_list(
            "GDD asset placeholder plan assetEntries.blockedReasons",
            &blockers,
            false,
        )?;
        if let Some(asset_ref) = str_field(entry, "assetRef") {
            require_asset_ref(
                "GDD asset placeholder plan assetEntries.assetRef",
                asset_ref,
            )?;
        }
        if let Some(manifest_ref) = str_field(entry, "manifestRef") {
            require_asset_ref(
                "GDD asset placeholder plan assetEntries.manifestRef",
                manifest_ref,
            )?;
            if !manifests.contains(manifest_ref) {
                return Err(anyhow!("GDD asset placeholder plan manifestRef `{manifest_ref}` is missing from declared manifestRefs"));
            }
        }
        match source_kind {
            "placeholder" if str_field(entry, "assetRef").is_some() => return Err(anyhow!("GDD asset placeholder plan placeholder assets must not point to uncontrolled generated files")),
            "local-fixture" if str_field(entry, "assetRef").is_none() => return Err(anyhow!("GDD asset placeholder plan local-fixture asset requires assetRef")),
            "manifest-ref" if str_field(entry, "manifestRef").is_none() => return Err(anyhow!("GDD asset placeholder plan manifest-ref asset requires manifestRef")),
            "missing" | "unsupported" if blockers.is_empty() => return Err(anyhow!("GDD asset placeholder plan missing or unsupported assets require blockedReasons")),
            _ => {}
        }
        if bool_field(entry, "staleRef") && blockers.is_empty() {
            return Err(anyhow!("GDD asset placeholder plan stale manifest refs or asset refs require blockedReasons"));
        }
    }
    Ok(())
}
fn validate_required_metadata(value: Option<&Value>) -> Result<()> {
    let metadata = value
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("GDD asset placeholder plan requiredMetadata must be an object"))?;
    for key in ["width", "height"] {
        let Some(number) = metadata.get(key).and_then(Value::as_u64) else {
            return Err(anyhow!(
                "GDD asset placeholder plan requiredMetadata.{key} must be positive"
            ));
        };
        if number == 0 || number > 8192 {
            return Err(anyhow!(
                "GDD asset placeholder plan requiredMetadata.{key} must be bounded"
            ));
        }
    }
    if let Some(format) = metadata.get("format").and_then(Value::as_str) {
        require_text("GDD asset placeholder plan requiredMetadata.format", format)?;
    }
    Ok(())
}
fn validate_evidence(values: &[Value]) -> Result<()> {
    let mut ids = BTreeSet::new();
    for item in values {
        let id = required_id(item, "evidenceId")?;
        if !ids.insert(id.to_string()) {
            return Err(anyhow!(
                "GDD asset placeholder plan evidenceId `{id}` is duplicated"
            ));
        }
        let path = required_str(item, "pathHint")?;
        require_ref("GDD asset placeholder plan expectedEvidence.pathHint", path)?;
        if !path.contains("evidence") && !path.contains("scenario") {
            return Err(anyhow!(
                "GDD asset placeholder plan expectedEvidence must point to scenario/evidence refs"
            ));
        }
        require_text(
            "GDD asset placeholder plan expectedEvidence.description",
            required_str(item, "description")?,
        )?;
    }
    Ok(())
}
fn validate_trace_links(
    field: &str,
    value: Option<&Value>,
    reqs: &BTreeSet<&str>,
    maps: &BTreeSet<&str>,
) -> Result<()> {
    let links = value
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("{field} must not be empty"))?;
    require_nonempty(field, links.len())?;
    for link in links {
        require_member(
            &format!("{field}.requirementId"),
            required_str(link, "requirementId")?,
            reqs,
        )?;
        require_member(
            &format!("{field}.mechanicsMappingId"),
            required_str(link, "mechanicsMappingId")?,
            maps,
        )?;
    }
    Ok(())
}
fn require_license_note(value: &str) -> Result<()> {
    require_text("GDD asset placeholder plan licenseSourceNote", value)?;
    let lower = value.to_ascii_lowercase();
    if !(lower.contains("license")
        && (lower.contains("source")
            || lower.contains("placeholder")
            || lower.contains("local fixture")))
    {
        return Err(anyhow!(
            "GDD asset placeholder plan asset entries require license/source notes"
        ));
    }
    for ambiguous in [
        "unknown license",
        "unclear license",
        "proprietary",
        "copyrighted",
        "download later",
        "remote source",
    ] {
        if contains_positive_phrase(&lower, ambiguous) {
            return Err(anyhow!("GDD asset placeholder plan license/source note has proprietary/copyright ambiguity `{ambiguous}`"));
        }
    }
    Ok(())
}
fn require_asset_ref(field: &str, value: &str) -> Result<()> {
    require_ref(field, value)?;
    for blocked in [
        "target/",
        "runs/",
        "evidence/",
        ".omx/",
        ".omc/",
        "generated/",
    ] {
        if value.starts_with(blocked) || value.contains(&format!("/{blocked}")) {
            return Err(anyhow!(
                "{field} collides with generated-root or evidence output `{blocked}`"
            ));
        }
    }
    Ok(())
}
fn array_empty(value: &Value, key: &str) -> bool {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|v| v.is_empty())
        .unwrap_or(true)
}
fn bool_field(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or(false)
}
fn str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}
fn required_str<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    str_field(value, key)
        .ok_or_else(|| anyhow!("GDD asset placeholder plan missing string field `{key}`"))
}
fn required_id<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    let id = required_str(value, key)?;
    require_id(&format!("GDD asset placeholder plan {key}"), id)?;
    Ok(id)
}
fn string_array(value: Option<&Value>, field: &str, required: bool) -> Result<Vec<String>> {
    let Some(value) = value else {
        if required {
            return Err(anyhow!("{field} must not be empty"));
        } else {
            return Ok(vec![]);
        }
    };
    let array = value
        .as_array()
        .ok_or_else(|| anyhow!("{field} must be an array"))?;
    if required {
        require_nonempty(field, array.len())?;
    }
    array
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_string)
                .ok_or_else(|| anyhow!("{field} must contain strings"))
        })
        .collect()
}
fn validate_id_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    let mut seen = BTreeSet::new();
    for value in values {
        require_id(field, value)?;
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{field} contains duplicate id `{value}`"));
        }
    }
    Ok(())
}
fn validate_manifest_ref_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    let mut seen = BTreeSet::new();
    for value in values {
        require_asset_ref(field, value)?;
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
fn require_member(field: &str, value: &str, allowed: &BTreeSet<&str>) -> Result<()> {
    require_id(field, value)?;
    if !allowed.contains(value) {
        return Err(anyhow!(
            "{field} `{value}` is missing from declared GDD requirements or mechanics mapping ids"
        ));
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
        return Err(anyhow!("{field} contains forbidden traversal and must stay inside local fixture/reference roots"));
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
        "dynamic import",
        "command bridge",
        "local server bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "self-approval",
        "godot replacement",
        "production-ready",
        "shipped-game",
        "commercial readiness",
        "hosted/cloud",
        "native export",
        "plugin runtime",
        "autonomous unrestricted game creation",
        "asset generation",
        "remote fetch",
        "download asset",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD asset authority text `{forbidden}`"
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
        "out of scope ",
    ];
    let hay = value;
    // Scope negation to the clause/sentence containing each occurrence so a
    // negated mention in one sentence cannot whitelist a positive mention in
    // another (fail-closed), while a single leading negation still covers a
    // list such as `no auto-apply or self-approval`. A contrastive conjunction
    // ends the negation's scope so wording like `no auto-fix, but auto-fix
    // enabled` still fails closed.
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
