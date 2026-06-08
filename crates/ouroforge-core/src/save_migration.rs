//! Save-Migration and Version-Compatibility v1 (#1846).
//!
//! Rust/local owns deterministic forward migration over the existing
//! `save_profile_scale` save/restore and replay-digest model. This module adds
//! no new persistence mechanism, performs no browser/Studio trusted write, and
//! emits fixture-scoped migration evidence only.

use crate::export_hash::sha256_prefixed;
use crate::save_profile_scale::{
    SaveStore, SAVE_PROFILE_SCALE_LEGACY_V0_SCHEMA_VERSION, SAVE_PROFILE_SCALE_SCHEMA_VERSION,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SAVE_MIGRATION_EVIDENCE_SCHEMA_VERSION: &str = "save-migration-evidence-v1";
pub const SAVE_MIGRATION_BOUNDARY: &str = "Rust/local forward save migration over existing save/restore and replay-digest; browser/Studio read-only; no new persistence mechanism; incompatible saves fail closed; generated migration evidence fixture-scoped unless explicitly tracked.";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveMigrationEvidence {
    pub schema_version: String,
    pub issue: u32,
    pub migration_id: String,
    pub from_schema_version: String,
    pub to_schema_version: String,
    pub status: SaveMigrationStatus,
    pub source_hash: String,
    pub migrated_hash: Option<String>,
    pub replay_digests_preserved: bool,
    pub diagnostic: Option<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SaveMigrationStatus {
    Migrated,
    AlreadyCompatible,
    Incompatible,
}

pub fn migrate_save_forward(
    migration_id: &str,
    input: &str,
) -> Result<(SaveStore, SaveMigrationEvidence)> {
    require_local_id("migrationId", migration_id)?;
    let source_hash = sha256_prefixed(input.as_bytes());
    let schema = schema_version(input)?;
    let before_replay_digests = replay_digests(input)?;

    let (store, status) = match schema.as_str() {
        SAVE_PROFILE_SCALE_LEGACY_V0_SCHEMA_VERSION => (
            SaveStore::from_legacy_v0_json(input)
                .context("failed to migrate legacy save-profile-v0 forward")?,
            SaveMigrationStatus::Migrated,
        ),
        SAVE_PROFILE_SCALE_SCHEMA_VERSION => (
            SaveStore::from_json_str(input).context("failed to verify compatible save store")?,
            SaveMigrationStatus::AlreadyCompatible,
        ),
        other => {
            return Err(anyhow!(
                "save schema `{other}` is incompatible with forward migration target {SAVE_PROFILE_SCALE_SCHEMA_VERSION}"
            ));
        }
    };

    store.verify_integrity()?;
    let migrated_json = store.to_json()?;
    let after_replay_digests = replay_digests(&migrated_json)?;
    let replay_digests_preserved = before_replay_digests == after_replay_digests;
    if !replay_digests_preserved {
        return Err(anyhow!(
            "save migration `{migration_id}` did not preserve replay digests"
        ));
    }

    let evidence = SaveMigrationEvidence {
        schema_version: SAVE_MIGRATION_EVIDENCE_SCHEMA_VERSION.to_string(),
        issue: 1846,
        migration_id: migration_id.to_string(),
        from_schema_version: schema,
        to_schema_version: SAVE_PROFILE_SCALE_SCHEMA_VERSION.to_string(),
        status,
        source_hash,
        migrated_hash: Some(sha256_prefixed(migrated_json.as_bytes())),
        replay_digests_preserved,
        diagnostic: None,
        boundary: SAVE_MIGRATION_BOUNDARY.to_string(),
    };
    evidence.validate()?;
    Ok((store, evidence))
}

pub fn incompatible_save_evidence(
    migration_id: &str,
    input: &str,
) -> Result<SaveMigrationEvidence> {
    require_local_id("migrationId", migration_id)?;
    let source_hash = sha256_prefixed(input.as_bytes());
    let schema = schema_version(input).unwrap_or_else(|_| "malformed-json".to_string());
    let diagnostic = format!(
        "save schema `{schema}` is incompatible with forward migration target {SAVE_PROFILE_SCALE_SCHEMA_VERSION}"
    );
    let evidence = SaveMigrationEvidence {
        schema_version: SAVE_MIGRATION_EVIDENCE_SCHEMA_VERSION.to_string(),
        issue: 1846,
        migration_id: migration_id.to_string(),
        from_schema_version: schema,
        to_schema_version: SAVE_PROFILE_SCALE_SCHEMA_VERSION.to_string(),
        status: SaveMigrationStatus::Incompatible,
        source_hash,
        migrated_hash: None,
        replay_digests_preserved: false,
        diagnostic: Some(diagnostic),
        boundary: SAVE_MIGRATION_BOUNDARY.to_string(),
    };
    evidence.validate()?;
    Ok(evidence)
}

impl SaveMigrationEvidence {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SAVE_MIGRATION_EVIDENCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "save migration evidence schemaVersion must be {SAVE_MIGRATION_EVIDENCE_SCHEMA_VERSION}"
            ));
        }
        if self.issue != 1846 {
            return Err(anyhow!("save migration evidence issue must be 1846"));
        }
        require_local_id("migrationId", &self.migration_id)?;
        require_text("fromSchemaVersion", &self.from_schema_version)?;
        if self.to_schema_version != SAVE_PROFILE_SCALE_SCHEMA_VERSION {
            return Err(anyhow!(
                "save migration target must be {SAVE_PROFILE_SCALE_SCHEMA_VERSION}"
            ));
        }
        require_sha256("sourceHash", &self.source_hash)?;
        match self.status {
            SaveMigrationStatus::Migrated | SaveMigrationStatus::AlreadyCompatible => {
                let Some(hash) = &self.migrated_hash else {
                    return Err(anyhow!(
                        "compatible migration evidence requires migratedHash"
                    ));
                };
                require_sha256("migratedHash", hash)?;
                if !self.replay_digests_preserved {
                    return Err(anyhow!(
                        "compatible migration evidence must preserve replay digests"
                    ));
                }
                if self.diagnostic.is_some() {
                    return Err(anyhow!(
                        "compatible migration evidence must not carry an incompatibility diagnostic"
                    ));
                }
            }
            SaveMigrationStatus::Incompatible => {
                if self.migrated_hash.is_some() {
                    return Err(anyhow!("incompatible saves must not emit migratedHash"));
                }
                let Some(diagnostic) = &self.diagnostic else {
                    return Err(anyhow!("incompatible saves require explicit diagnostic"));
                };
                require_contains("diagnostic", diagnostic, &["incompatible", "target"])?;
            }
        }
        validate_boundary(&self.boundary)?;
        Ok(())
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize save migration evidence")
    }
}

fn schema_version(input: &str) -> Result<String> {
    let value: serde_json::Value =
        serde_json::from_str(input).context("failed to parse save JSON for schemaVersion")?;
    value
        .get("schemaVersion")
        .and_then(|schema| schema.as_str())
        .map(str::to_string)
        .ok_or_else(|| anyhow!("save JSON missing schemaVersion"))
}

fn replay_digests(input: &str) -> Result<Vec<String>> {
    let value: serde_json::Value =
        serde_json::from_str(input).context("failed to parse save JSON for replay digests")?;
    let mut digests = Vec::new();
    collect_replay_digests(&value, &mut digests);
    digests.sort();
    Ok(digests)
}

fn collect_replay_digests(value: &serde_json::Value, digests: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(digest) = map.get("replayDigest").and_then(|digest| digest.as_str()) {
                digests.push(digest.to_string());
            }
            for nested in map.values() {
                collect_replay_digests(nested, digests);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_replay_digests(item, digests);
            }
        }
        _ => {}
    }
}

fn validate_boundary(value: &str) -> Result<()> {
    require_text("boundary", value)?;
    let lower = value.to_ascii_lowercase();
    for required in [
        "rust/local",
        "existing save/restore",
        "replay-digest",
        "browser/studio read-only",
        "no new persistence mechanism",
        "incompatible saves fail closed",
        "fixture-scoped",
    ] {
        if !lower.contains(&required.to_ascii_lowercase()) {
            return Err(anyhow!("boundary must state `{required}`"));
        }
    }
    reject_forbidden_wording("boundary", value)?;
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

fn reject_forbidden_wording(field: &str, value: &str) -> Result<()> {
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "production-ready",
        "godot replacement",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "release button is automated",
        "trusted writes are authorized",
        "layer-3 cloud/mobile is go",
        "automated fun score",
        "quality score",
        "adds a new persistence mechanism",
        "new persistence mechanism is authorized",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden save migration wording `{forbidden}`"
            ));
        }
    }
    Ok(())
}
