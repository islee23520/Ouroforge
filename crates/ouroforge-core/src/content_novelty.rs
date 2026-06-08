//! Deduplication and Novelty Metrics v1 (#1650).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. This module computes descriptive **dedup** and
//! **novelty/variety** metrics over a generated proposal set
//! ([`crate::content_scale_generation::CampaignProposalSet`]) so that scale does
//! not collapse into repetition.
//!
//! Reuse, not a new analysis engine: the per-item signature is a deterministic
//! digest over the *existing* generated artifact (the proposal's `to` payload),
//! optionally folded with the Milestone 28 difficulty/solver signal supplied by
//! the caller (e.g. a [`crate::puzzle_difficulty_metric::DifficultyMetric`]
//! summary). It introduces no similarity engine, embedding model, or external
//! service, and it reuses the existing `export_hash` digest helper.
//!
//! Dedup here is **read/measure-only**, never destructive: it identifies and
//! flags duplicate/near-duplicate items by their existing evidence; it never
//! deletes evidence, runs, or prior content to manufacture novelty. Every metric
//! is a descriptive measurement against a declared, evidence-backed threshold —
//! not a quality, fun, or taste claim. The metrics fail closed on a malformed
//! artifact or an out-of-range threshold.

use std::collections::BTreeMap;

use crate::content_scale_generation::CampaignProposalSet;
use crate::export_hash::sha256_hex;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Schema version for the content-novelty report.
pub const CONTENT_NOVELTY_SCHEMA_VERSION: &str = "ouroforge.content-novelty.v1";

/// Declared default novelty threshold: a set is flagged low-novelty when fewer
/// than this fraction of its items are structurally distinct. Conservative and
/// evidence-backed (a measurement threshold, not a quality bar).
pub const DEFAULT_NOVELTY_THRESHOLD: f64 = 0.5;

/// A per-item novelty record within a set.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ItemNovelty {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "gameClass")]
    pub game_class: String,
    /// Deterministic digest over the normalized artifact (and optional
    /// difficulty signal) used to detect duplicates.
    pub signature: String,
    /// True iff an earlier item in the set shares this signature.
    #[serde(rename = "isDuplicate")]
    pub is_duplicate: bool,
    /// The first item id that produced this signature, when this item is a
    /// duplicate of it.
    #[serde(
        rename = "duplicateOf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duplicate_of: Option<String>,
}

/// A group of items that share a signature (a duplicate cluster of size >= 2).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DuplicateGroup {
    pub signature: String,
    #[serde(rename = "itemIds")]
    pub item_ids: Vec<String>,
}

/// A descriptive, auditable novelty/dedup report over a generated proposal set.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NoveltyReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "itemCount")]
    pub item_count: usize,
    #[serde(rename = "distinctCount")]
    pub distinct_count: usize,
    #[serde(rename = "duplicateCount")]
    pub duplicate_count: usize,
    /// `distinct_count / item_count`, in `[0, 1]`. Higher is more varied.
    #[serde(rename = "noveltyRatio")]
    pub novelty_ratio: f64,
    pub threshold: f64,
    /// True iff `novelty_ratio < threshold`: the set is repetitive by the
    /// declared measure.
    #[serde(rename = "lowNovelty")]
    pub low_novelty: bool,
    /// Per-item records, in set order.
    pub items: Vec<ItemNovelty>,
    /// Duplicate clusters (size >= 2), sorted by signature.
    #[serde(rename = "duplicateGroups")]
    pub duplicate_groups: Vec<DuplicateGroup>,
}

impl NoveltyReport {
    /// Validate the report's internal consistency, failing closed.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != CONTENT_NOVELTY_SCHEMA_VERSION {
            return Err(anyhow!(
                "novelty report schemaVersion must be \"{CONTENT_NOVELTY_SCHEMA_VERSION}\""
            ));
        }
        if self.items.len() != self.item_count {
            return Err(anyhow!("novelty report itemCount must match items length"));
        }
        if self.distinct_count + self.duplicate_count != self.item_count {
            return Err(anyhow!(
                "novelty report distinctCount + duplicateCount must equal itemCount"
            ));
        }
        Ok(())
    }
}

/// Normalize a generated artifact for structural comparison: drop the
/// identity-only `id` field so two items that differ only by id collide as
/// duplicates. Other content (rows, legend, deck, cards, seed, enemy, ...) is
/// preserved. Fails closed on a non-object artifact.
fn normalized_artifact_bytes(artifact_json: &str) -> Result<Vec<u8>> {
    let mut artifact: Value = serde_json::from_str(artifact_json)
        .map_err(|err| anyhow!("novelty: proposal artifact is not valid JSON: {err}"))?;
    let object = artifact
        .as_object_mut()
        .ok_or_else(|| anyhow!("novelty: proposal artifact must be a JSON object"))?;
    object.remove("id");
    serde_json::to_vec(&Value::Object(object.clone()))
        .map_err(|err| anyhow!("novelty: failed to serialize normalized artifact: {err}"))
}

/// Compute the dedup/novelty report over a generated proposal set.
///
/// `difficulty_by_item` optionally supplies a per-item difficulty/solver signal
/// (keyed by proposal id) derived from existing Milestone 28/32 evidence; when
/// present it is folded into the item's signature so two items with identical
/// content but a different measured difficulty are not treated as duplicates.
/// When the map is empty, dedup is purely content-based. `threshold` must be in
/// `[0, 1]`. Deterministic: the same inputs always yield the same report.
pub fn compute_novelty(
    set: &CampaignProposalSet,
    threshold: f64,
    difficulty_by_item: &BTreeMap<String, String>,
) -> Result<NoveltyReport> {
    set.validate()?;
    if !(0.0..=1.0).contains(&threshold) {
        return Err(anyhow!(
            "novelty threshold must be in [0, 1], got {threshold}"
        ));
    }

    let mut items: Vec<ItemNovelty> = Vec::with_capacity(set.proposals.len());
    // First item id observed per signature, in set order.
    let mut first_by_signature: BTreeMap<String, String> = BTreeMap::new();
    // All item ids per signature, for duplicate grouping.
    let mut ids_by_signature: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for generative in &set.proposals {
        let item_id = generative.proposal.id.clone();
        let mut bytes = normalized_artifact_bytes(&generative.proposal.to)?;
        if let Some(signal) = difficulty_by_item.get(&item_id) {
            bytes.push(b'|');
            bytes.extend_from_slice(signal.as_bytes());
        }
        let signature = sha256_hex(&bytes);

        let duplicate_of = first_by_signature.get(&signature).cloned();
        let is_duplicate = duplicate_of.is_some();
        if !is_duplicate {
            first_by_signature.insert(signature.clone(), item_id.clone());
        }
        ids_by_signature
            .entry(signature.clone())
            .or_default()
            .push(item_id.clone());

        items.push(ItemNovelty {
            item_id,
            game_class: generative.provenance.game_class.clone(),
            signature,
            is_duplicate,
            duplicate_of,
        });
    }

    let item_count = items.len();
    let distinct_count = first_by_signature.len();
    let duplicate_count = item_count - distinct_count;
    let novelty_ratio = if item_count == 0 {
        0.0
    } else {
        distinct_count as f64 / item_count as f64
    };
    let low_novelty = novelty_ratio < threshold;

    // Duplicate clusters (size >= 2), sorted by signature (BTreeMap order).
    let duplicate_groups: Vec<DuplicateGroup> = ids_by_signature
        .into_iter()
        .filter(|(_, ids)| ids.len() >= 2)
        .map(|(signature, item_ids)| DuplicateGroup {
            signature,
            item_ids,
        })
        .collect();

    let report = NoveltyReport {
        schema_version: CONTENT_NOVELTY_SCHEMA_VERSION.to_string(),
        item_count,
        distinct_count,
        duplicate_count,
        novelty_ratio,
        threshold,
        low_novelty,
        items,
        duplicate_groups,
    };
    report.validate()?;
    Ok(report)
}
