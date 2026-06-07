//! Save/Profile and Run-History at Scale v1 (#1659) — multi-profile saves and a
//! large run-history with digest-verified integrity, extending the existing
//! runtime save/restore and replay-digest.
//!
//! This module owns the trusted, deterministic save store for many profiles,
//! each holding an ordered run-history. It is not a new persistence mechanism:
//! each run-history entry carries the run's existing runtime replay-digest, and
//! per-profile history integrity is sealed with the same in-tree SHA-256 hasher
//! (`export_hash::sha256_hex`) the export and source-patch surfaces already use.
//!
//! Determinism: profiles are keyed in an ordered `BTreeMap` and history is an
//! ordered `Vec`, so a store serializes canonically and round-trips through JSON
//! unchanged. The per-profile `history_digest` is a pure function of the ordered
//! history.
//!
//! Integrity at scale: `verify_integrity` recomputes every profile's history
//! digest and fails closed on any mismatch, so a tampered or truncated large
//! history is detected on restore rather than silently trusted.
//!
//! Multi-profile isolation: each profile is an independent keyed entry; a write
//! to one profile never alters another, and the store rejects a key that does
//! not match its profile's own id.
//!
//! Migration / back-compat: a prior single-profile `save-profile-v0` document
//! loads through [`SaveStore::from_legacy_v0_json`], which upgrades it into the
//! v1 multi-profile store and computes its history digest.
//!
//! Boundary: Rust/local owns this trusted store. The in-game save/profile UI is
//! read-only JavaScript runtime presentation of the exposed state
//! (`read_model`); the browser never writes back. Generated content is a
//! proposal through the existing review/apply/trust-gradient path, never a direct
//! trusted write. See `docs/long-form-systems-v1.md`.

use crate::export_hash::sha256_hex;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const SAVE_PROFILE_SCALE_SCHEMA_VERSION: &str = "save-profile-scale-v1";

/// The prior single-profile save schema this module can migrate from.
pub const SAVE_PROFILE_SCALE_LEGACY_V0_SCHEMA_VERSION: &str = "save-profile-v0";

/// Canonical trust boundary recorded on every store and profile so the
/// read-only/proposal-only contract travels with the persisted data.
pub const SAVE_PROFILE_SCALE_BOUNDARY: &str =
    "rust-trusted-state; browser/studio read-only; generation proposal-only via review/apply/trust-gradient";

/// One run-history entry. `replay_digest` is the run's existing runtime
/// replay-digest, carried here for integrity rather than recomputed.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunHistoryEntry {
    pub run_id: String,
    pub recorded_at: String,
    pub replay_digest: String,
}

/// One profile's save: an ordered run-history sealed by a history digest.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveProfile {
    pub schema_version: String,
    pub profile_id: String,
    pub run_history: Vec<RunHistoryEntry>,
    pub history_digest: String,
    pub boundary: String,
}

/// The multi-profile save store: independent profiles keyed by id.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveStore {
    pub schema_version: String,
    pub profiles: BTreeMap<String, SaveProfile>,
    pub boundary: String,
}

/// The prior single-profile save format, accepted only for migration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LegacyProfileSaveV0 {
    pub schema_version: String,
    pub profile_id: String,
    pub run_history: Vec<RunHistoryEntry>,
}

/// Read-only summary for browser/Studio presentation. Derived from the trusted
/// store; the browser renders this and never writes trusted state.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveStoreReadModel {
    pub schema_version: String,
    pub profile_count: usize,
    pub run_counts: BTreeMap<String, usize>,
    pub total_runs: usize,
    pub boundary: String,
}

/// The digest seed for an empty history.
fn empty_history_digest() -> String {
    sha256_hex(b"save-profile-scale-v1:empty-history")
}

/// Fold one entry into the running history digest, reusing the in-tree SHA-256
/// hasher: `next = sha256(prev_digest : entry)`. Chaining makes the digest a
/// pure function of the ordered history and tamper-evident from the first
/// altered entry onward, while keeping each append O(1) at scale.
fn chain_step(prev_digest: &str, entry: &RunHistoryEntry) -> Result<String> {
    let entry_bytes = serde_json::to_vec(entry)
        .map_err(|err| anyhow!("failed to serialize run-history entry for digest: {err}"))?;
    let mut buf = Vec::with_capacity(prev_digest.len() + 1 + entry_bytes.len());
    buf.extend_from_slice(prev_digest.as_bytes());
    buf.push(b':');
    buf.extend_from_slice(&entry_bytes);
    Ok(sha256_hex(&buf))
}

/// Compute the integrity digest of an ordered run-history by chaining each
/// entry from the empty seed. A pure function of the ordered entries.
fn history_digest(run_history: &[RunHistoryEntry]) -> Result<String> {
    let mut digest = empty_history_digest();
    for entry in run_history {
        digest = chain_step(&digest, entry)?;
    }
    Ok(digest)
}

fn validate_entry(profile_id: &str, entry: &RunHistoryEntry) -> Result<()> {
    if entry.run_id.trim().is_empty() {
        return Err(anyhow!(
            "run-history entry for profile '{profile_id}' has an empty run_id"
        ));
    }
    if entry.replay_digest.trim().is_empty() {
        return Err(anyhow!(
            "run '{}' in profile '{profile_id}' has an empty replay_digest",
            entry.run_id
        ));
    }
    Ok(())
}

impl SaveProfile {
    fn sealed(profile_id: &str, run_history: Vec<RunHistoryEntry>) -> Result<Self> {
        let digest = history_digest(&run_history)?;
        Ok(SaveProfile {
            schema_version: SAVE_PROFILE_SCALE_SCHEMA_VERSION.to_string(),
            profile_id: profile_id.to_string(),
            run_history,
            history_digest: digest,
            boundary: SAVE_PROFILE_SCALE_BOUNDARY.to_string(),
        })
    }
}

impl Default for SaveStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SaveStore {
    /// A new, empty multi-profile store carrying the canonical boundary.
    pub fn new() -> Self {
        SaveStore {
            schema_version: SAVE_PROFILE_SCALE_SCHEMA_VERSION.to_string(),
            profiles: BTreeMap::new(),
            boundary: SAVE_PROFILE_SCALE_BOUNDARY.to_string(),
        }
    }

    /// Append a run-history entry to a profile, creating the profile if needed.
    /// Fails closed on a blank profile id, a blank run id or replay-digest, or a
    /// duplicate run id within that profile. Only the targeted profile changes;
    /// its history digest is resealed.
    pub fn append_run(&mut self, profile_id: &str, entry: RunHistoryEntry) -> Result<()> {
        if profile_id.trim().is_empty() {
            return Err(anyhow!("save store profile_id must not be empty"));
        }
        validate_entry(profile_id, &entry)?;
        let profile = match self.profiles.remove(profile_id) {
            Some(existing) => existing,
            None => SaveProfile::sealed(profile_id, Vec::new())?,
        };
        if profile.run_history.iter().any(|e| e.run_id == entry.run_id) {
            // Re-insert the untouched profile before failing so the store is
            // unchanged on a rejected append.
            self.profiles.insert(profile_id.to_string(), profile);
            return Err(anyhow!(
                "run '{}' is already in profile '{profile_id}' history",
                entry.run_id
            ));
        }
        // Extend the chained digest incrementally so appends stay O(1) even on
        // a large history; this equals a full recompute by construction and is
        // re-checked end-to-end by `verify_integrity`.
        let next_digest = chain_step(&profile.history_digest, &entry)?;
        let mut run_history = profile.run_history;
        run_history.push(entry);
        let resealed = SaveProfile {
            schema_version: SAVE_PROFILE_SCALE_SCHEMA_VERSION.to_string(),
            profile_id: profile_id.to_string(),
            run_history,
            history_digest: next_digest,
            boundary: SAVE_PROFILE_SCALE_BOUNDARY.to_string(),
        };
        self.profiles.insert(profile_id.to_string(), resealed);
        Ok(())
    }

    /// Borrow a profile by id.
    pub fn profile(&self, profile_id: &str) -> Option<&SaveProfile> {
        self.profiles.get(profile_id)
    }

    /// Verify the whole store fails closed on any structural or integrity
    /// violation: wrong schema/boundary, a map key that does not match its
    /// profile id, a duplicate run id within a profile, or a history whose
    /// recomputed digest does not match the sealed `history_digest`.
    pub fn verify_integrity(&self) -> Result<()> {
        if self.schema_version != SAVE_PROFILE_SCALE_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected save store schema version: {} (expected {})",
                self.schema_version,
                SAVE_PROFILE_SCALE_SCHEMA_VERSION
            ));
        }
        if self.boundary != SAVE_PROFILE_SCALE_BOUNDARY {
            return Err(anyhow!(
                "save store boundary must be the canonical read-only/proposal-only contract"
            ));
        }
        for (key, profile) in &self.profiles {
            if key.trim().is_empty() || profile.profile_id.trim().is_empty() {
                return Err(anyhow!("save store profile id must not be empty"));
            }
            if key != &profile.profile_id {
                return Err(anyhow!(
                    "save store key '{key}' does not match profile id '{}'",
                    profile.profile_id
                ));
            }
            if profile.schema_version != SAVE_PROFILE_SCALE_SCHEMA_VERSION {
                return Err(anyhow!(
                    "profile '{}' has unexpected schema version {}",
                    profile.profile_id,
                    profile.schema_version
                ));
            }
            if profile.boundary != SAVE_PROFILE_SCALE_BOUNDARY {
                return Err(anyhow!(
                    "profile '{}' boundary must be the canonical read-only/proposal-only contract",
                    profile.profile_id
                ));
            }
            let mut seen = BTreeSet::new();
            for entry in &profile.run_history {
                validate_entry(&profile.profile_id, entry)?;
                if !seen.insert(entry.run_id.as_str()) {
                    return Err(anyhow!(
                        "profile '{}' has duplicate run id '{}'",
                        profile.profile_id,
                        entry.run_id
                    ));
                }
            }
            let expected = history_digest(&profile.run_history)?;
            if expected != profile.history_digest {
                return Err(anyhow!(
                    "profile '{}' history digest mismatch: integrity check failed",
                    profile.profile_id
                ));
            }
        }
        Ok(())
    }

    /// Serialize the trusted store to canonical JSON for persistence.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|err| anyhow!("failed to serialize save store: {err}"))
    }

    /// Parse a persisted store from JSON and verify its integrity, failing
    /// closed on malformed input, a wrong schema version, or a digest mismatch.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let store: Self =
            serde_json::from_str(input).map_err(|err| anyhow!("invalid save store json: {err}"))?;
        store.verify_integrity()?;
        Ok(store)
    }

    /// Migrate a prior single-profile `save-profile-v0` document into a v1
    /// multi-profile store, sealing its history digest. Fails closed on a wrong
    /// legacy schema version, a blank profile id, or an invalid entry.
    pub fn from_legacy_v0_json(input: &str) -> Result<Self> {
        let legacy: LegacyProfileSaveV0 = serde_json::from_str(input)
            .map_err(|err| anyhow!("invalid legacy save json: {err}"))?;
        if legacy.schema_version != SAVE_PROFILE_SCALE_LEGACY_V0_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected legacy save schema version: {} (expected {})",
                legacy.schema_version,
                SAVE_PROFILE_SCALE_LEGACY_V0_SCHEMA_VERSION
            ));
        }
        if legacy.profile_id.trim().is_empty() {
            return Err(anyhow!("legacy save profile_id must not be empty"));
        }
        let mut seen = BTreeSet::new();
        for entry in &legacy.run_history {
            validate_entry(&legacy.profile_id, entry)?;
            if !seen.insert(entry.run_id.as_str()) {
                return Err(anyhow!(
                    "legacy save profile '{}' has duplicate run id '{}'",
                    legacy.profile_id,
                    entry.run_id
                ));
            }
        }
        let mut store = SaveStore::new();
        let profile = SaveProfile::sealed(&legacy.profile_id, legacy.run_history)?;
        store.profiles.insert(legacy.profile_id, profile);
        store.verify_integrity()?;
        Ok(store)
    }

    /// Derive the read-only presentation summary for browser/Studio surfaces.
    pub fn read_model(&self) -> SaveStoreReadModel {
        let run_counts: BTreeMap<String, usize> = self
            .profiles
            .iter()
            .map(|(id, profile)| (id.clone(), profile.run_history.len()))
            .collect();
        let total_runs = run_counts.values().sum();
        SaveStoreReadModel {
            schema_version: self.schema_version.clone(),
            profile_count: self.profiles.len(),
            run_counts,
            total_runs,
            boundary: self.boundary.clone(),
        }
    }
}
