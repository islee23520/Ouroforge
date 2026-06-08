//! Post-launch patch re-verify and re-package loop v1 (#1845).
//!
//! This module is a Rust/local contract for the Era I Milestone 55 patch loop:
//! a patch candidate must pass the full declared gate set before re-packaging
//! through the existing Steam desktop export descriptor path. It performs no
//! command execution, no filesystem writes, no browser/Studio trusted writes,
//! no Steam upload, and no release action.

use crate::export_hash::sha256_prefixed;
use crate::steam_export_build::{
    SteamDepotConfig, SteamExportBuildManifest, SteamPackageDescriptor,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Component, Path};

pub const PATCH_REVERIFY_SCHEMA_VERSION: &str = "patch-reverify-loop-v1";
pub const PATCH_REPACKAGE_EVIDENCE_SCHEMA_VERSION: &str = "patch-repackage-evidence-v1";

const REQUIRED_GATE_KINDS: &[&str] = &[
    "rust-tests",
    "clippy",
    "scenario-coverage",
    "evaluator-gates",
    "compare-provenance",
    "save-compatibility",
    "steam-export-package",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatchReverifyPlan {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub issue: u32,
    #[serde(rename = "patchId")]
    pub patch_id: String,
    #[serde(rename = "baselinePackageDescriptorHash")]
    pub baseline_package_descriptor_hash: String,
    #[serde(rename = "sourceApplyRef")]
    pub source_apply_ref: String,
    #[serde(rename = "gateSet")]
    pub gate_set: Vec<PatchGateEvidence>,
    #[serde(rename = "generatedStatePolicy")]
    pub generated_state_policy: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatchGateEvidence {
    pub kind: String,
    pub status: PatchGateStatus,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PatchGateStatus {
    Pass,
    Fail,
    Inconclusive,
    Missing,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatchRepackageEvidence {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "patchId")]
    pub patch_id: String,
    pub status: String,
    #[serde(rename = "gateEvidenceHash")]
    pub gate_evidence_hash: String,
    #[serde(rename = "packageDescriptor")]
    pub package_descriptor: SteamPackageDescriptor,
    #[serde(rename = "packageDescriptorHash")]
    pub package_descriptor_hash: String,
    #[serde(rename = "provenanceRefs")]
    pub provenance_refs: Vec<String>,
    #[serde(rename = "releaseAuthority")]
    pub release_authority: String,
    pub boundary: String,
}

impl PatchReverifyPlan {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let plan: Self =
            serde_json::from_str(input).context("failed to parse patch re-verify plan JSON")?;
        plan.validate()?;
        Ok(plan)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PATCH_REVERIFY_SCHEMA_VERSION {
            return Err(anyhow!(
                "patch re-verify schemaVersion must be {PATCH_REVERIFY_SCHEMA_VERSION}"
            ));
        }
        if self.issue != 1845 {
            return Err(anyhow!("patch re-verify issue must be 1845"));
        }
        require_local_id("patchId", &self.patch_id)?;
        require_sha256(
            "baselinePackageDescriptorHash",
            &self.baseline_package_descriptor_hash,
        )?;
        validate_relative_path("sourceApplyRef", &self.source_apply_ref)?;
        if self.gate_set.is_empty() {
            return Err(anyhow!("gateSet must not be empty"));
        }
        let mut kinds = BTreeSet::new();
        for gate in &self.gate_set {
            gate.validate()?;
            if !kinds.insert(gate.kind.as_str()) {
                return Err(anyhow!("gateSet contains duplicate gate `{}`", gate.kind));
            }
        }
        for required in REQUIRED_GATE_KINDS {
            if !kinds.contains(required) {
                return Err(anyhow!("gateSet missing required full-gate `{required}`"));
            }
        }
        require_contains(
            "generatedStatePolicy",
            &self.generated_state_policy,
            &["untracked", "fixture-scoped"],
        )?;
        validate_boundary("patch re-verify boundary", &self.boundary)?;
        Ok(())
    }

    pub fn all_gates_pass(&self) -> bool {
        self.gate_set
            .iter()
            .all(|gate| gate.status == PatchGateStatus::Pass)
    }

    pub fn blocking_gates(&self) -> Vec<&PatchGateEvidence> {
        self.gate_set
            .iter()
            .filter(|gate| gate.status != PatchGateStatus::Pass)
            .collect()
    }
}

impl PatchGateEvidence {
    fn validate(&self) -> Result<()> {
        if !REQUIRED_GATE_KINDS.contains(&self.kind.as_str()) {
            return Err(anyhow!("unsupported patch gate kind `{}`", self.kind));
        }
        validate_relative_path("gate evidenceRef", &self.evidence_ref)?;
        require_text("gate summary", &self.summary)?;
        reject_forbidden_wording("gate summary", &self.summary)?;
        Ok(())
    }
}

impl PatchRepackageEvidence {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize patch repackage evidence")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PATCH_REPACKAGE_EVIDENCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "patch repackage evidence schemaVersion must be {PATCH_REPACKAGE_EVIDENCE_SCHEMA_VERSION}"
            ));
        }
        require_local_id("patch repackage patchId", &self.patch_id)?;
        if self.status != "repackaged-after-reverify" {
            return Err(anyhow!(
                "patch repackage status must be repackaged-after-reverify"
            ));
        }
        require_sha256("gateEvidenceHash", &self.gate_evidence_hash)?;
        self.package_descriptor.validate()?;
        require_sha256("packageDescriptorHash", &self.package_descriptor_hash)?;
        if self.provenance_refs.is_empty() {
            return Err(anyhow!("provenanceRefs must not be empty"));
        }
        for reference in &self.provenance_refs {
            validate_relative_path("provenanceRefs", reference)?;
        }
        if self.release_authority != "human-ring3-required" {
            return Err(anyhow!(
                "patch repackage releaseAuthority must remain human-ring3-required"
            ));
        }
        validate_boundary("patch repackage boundary", &self.boundary)?;
        Ok(())
    }
}

pub fn reverify_and_repackage(
    plan: &PatchReverifyPlan,
    manifest: &SteamExportBuildManifest,
    depot: &SteamDepotConfig,
) -> Result<PatchRepackageEvidence> {
    plan.validate()?;
    if !plan.all_gates_pass() {
        let blocked = plan
            .blocking_gates()
            .into_iter()
            .map(|gate| format!("{}={:?}", gate.kind, gate.status))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(anyhow!(
            "patch `{}` cannot re-package before full re-verify passes: {blocked}",
            plan.patch_id
        ));
    }

    let descriptor = SteamPackageDescriptor::from_manifest_and_depot(manifest, depot)?;
    let descriptor_json = serde_json::to_string(&descriptor)
        .context("failed to canonicalize patch package descriptor")?;
    let plan_json =
        serde_json::to_string(plan).context("failed to canonicalize patch gate plan")?;
    let mut provenance_refs = plan
        .gate_set
        .iter()
        .map(|gate| gate.evidence_ref.clone())
        .collect::<Vec<_>>();
    provenance_refs.sort();

    let evidence = PatchRepackageEvidence {
        schema_version: PATCH_REPACKAGE_EVIDENCE_SCHEMA_VERSION.to_string(),
        patch_id: plan.patch_id.clone(),
        status: "repackaged-after-reverify".to_string(),
        gate_evidence_hash: sha256_prefixed(plan_json.as_bytes()),
        package_descriptor: descriptor,
        package_descriptor_hash: sha256_prefixed(descriptor_json.as_bytes()),
        provenance_refs,
        release_authority: "human-ring3-required".to_string(),
        boundary: "Patch re-package reuses existing web runtime and Steam export; Rust/local owns trusted validation; browser/Studio/Electron/Steamworks surfaces are read-only with no direct trusted writes; release remains human/Ring-3 with no Release button automation; not Layer-3 cloud/mobile.".to_string(),
    };
    evidence.validate()?;
    Ok(evidence)
}

fn validate_boundary(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    for required in [
        "existing web runtime",
        "rust/local",
        "read-only",
        "no direct trusted writes",
        "human/ring-3",
        "no release button",
        "not layer-3",
    ] {
        if !lower.contains(&required.to_ascii_lowercase()) {
            return Err(anyhow!("{field} must state boundary `{required}`"));
        }
    }
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn require_contains(field: &str, value: &str, required: &[&str]) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    for needle in required {
        if !lower.contains(&needle.to_ascii_lowercase()) {
            return Err(anyhow!("{field} must contain `{needle}`"));
        }
    }
    reject_forbidden_wording(field, value)?;
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
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > 768 {
        return Err(anyhow!("{field} must be non-empty text up to 768 bytes"));
    }
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn require_sha256(field: &str, value: &str) -> Result<()> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(anyhow!("{field} must start with sha256:"));
    };
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must contain a sha256 digest"));
    }
    Ok(())
}

fn validate_relative_path(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.contains('\\') {
        return Err(anyhow!("{field} must not contain backslashes"));
    }
    let path = Path::new(value);
    if path.is_absolute() || value.starts_with('/') {
        return Err(anyhow!("{field} must be relative"));
    }
    for component in path.components() {
        if !matches!(component, Component::Normal(_)) {
            return Err(anyhow!("{field} must be a normalized relative path"));
        }
    }
    Ok(())
}

fn reject_forbidden_wording(field: &str, value: &str) -> Result<()> {
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "production-ready",
        "godot replacement",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "release button is automated",
        "market demand is automated",
        "trusted writes are authorized",
        "layer-3 cloud/mobile is go",
        "automated fun score",
        "quality score",
        "signing key",
        "credential",
        "password",
        "secret",
        "publish token",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden patch re-verify wording `{forbidden}`"
            ));
        }
    }
    Ok(())
}
