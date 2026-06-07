//! Asset Replay-Proof and Provenance Binding v1 (#1614).
//!
//! Binds each published marketplace asset to a deterministic replay proof and a
//! Milestone 25 provenance lineage, then re-verifies that binding *on consume*:
//! the replay proof is re-run with the existing `evaluate_run` evaluator
//! (`provenance_replay.rs`, #1502) and the provenance lineage is re-checked for
//! tampering with the existing digest-bound bundle evaluation
//! (`provenance_bundle.rs`, #1500).
//!
//! This module reuses those two existing surfaces; it is **not** a new
//! provenance engine and re-runs no command of its own. It performs no trusted
//! write, installs nothing, and adds no hosted/paid/network capability (Layer-3
//! stays DEFER per #1508). Adoption of a verified asset still flows through the
//! existing review/apply/trust-gradient path, never a direct trusted write.
//!
//! Fail-closed: an unbound proof, a tampered provenance ref, a diverged or
//! not-replayable run, or an incomplete lineage all yield a non-verified status
//! rather than a silent pass.

use crate::evidence_marketplace_registry::MarketplaceAsset;
use crate::provenance_bundle::{
    ProvenanceBundleArtifact, ProvenanceBundleLinkKind, ProvenanceBundleStatus,
};
use crate::provenance_replay::{replay_provenance_bundle, ProvenanceReplayStatus};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const EVIDENCE_MARKETPLACE_PROOF_SCHEMA_VERSION: &str = "evidence-marketplace-proof-v1";

/// The lineage every verified asset must traverse in order, mirroring the
/// Milestone 25 provenance chain. Traversal is read-only.
const LINEAGE_ORDER: &[ProvenanceBundleLinkKind] = &[
    ProvenanceBundleLinkKind::IntentDesignBrief,
    ProvenanceBundleLinkKind::GeneratedEditedArtifact,
    ProvenanceBundleLinkKind::ValidationResult,
    ProvenanceBundleLinkKind::RuntimeObservation,
    ProvenanceBundleLinkKind::EvaluatorVerdict,
    ProvenanceBundleLinkKind::RegressionComparison,
    ProvenanceBundleLinkKind::JournalReviewDecision,
    ProvenanceBundleLinkKind::PromotionRollbackRecord,
];

/// Outcome of re-verifying an asset's replay proof and provenance binding on
/// consume. Only `Verified` clears the asset for local adoption (still through
/// the review/apply/trust-gradient path).
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AssetProofStatus {
    /// Proof re-ran and reproduced, provenance untampered, lineage traceable.
    Verified,
    /// The replay proof is not bound to the provenance lineage's replay inputs.
    ProofUnbound,
    /// A provenance ref was tampered (digest mismatch) or is missing/incomplete.
    ProvenanceTampered,
    /// The replay re-ran but the verdict diverged from the bound expectation.
    ReplayDiverged,
    /// The replay could not be re-run from the bound inputs.
    ReplayNotReplayable,
    /// The asset failed structural re-validation before any re-run.
    Invalid,
}

/// One read-only step of provenance lineage traversal.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetLineageStep {
    pub kind: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "ref")]
    pub reference: String,
    /// Link state from the existing bundle evaluation: `present`, `stale`,
    /// `dangling`, or `missing`.
    pub state: String,
}

/// A read-only report binding an asset to its re-run replay proof and traversed
/// provenance lineage. It records, never applies.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetProofReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub status: AssetProofStatus,
    #[serde(rename = "replayStatus")]
    pub replay_status: Option<ProvenanceReplayStatus>,
    #[serde(rename = "provenanceStatus")]
    pub provenance_status: ProvenanceBundleStatus,
    /// Ordered provenance lineage traversal.
    pub lineage: Vec<AssetLineageStep>,
    /// True only when every lineage step resolved `present`.
    #[serde(rename = "lineageTraceable")]
    pub lineage_traceable: bool,
    pub issues: Vec<String>,
    pub boundary: String,
}

impl AssetProofReport {
    /// True only when the asset re-verified end to end.
    pub fn is_verified(&self) -> bool {
        self.status == AssetProofStatus::Verified
    }
}

/// Re-verify an asset's replay proof and provenance binding by re-running the
/// proof and re-checking the lineage against on-disk evidence under
/// `bundle_root`, reconstructing the replay inside `replay_workspace`.
///
/// Reuses [`replay_provenance_bundle`] for the deterministic re-run and the
/// bundle's digest-bound evaluation for tamper detection. Fail-closed: every
/// failure mode produces a non-`Verified` status with explicit issues.
pub fn verify_asset_proof(
    asset: &MarketplaceAsset,
    bundle_root: impl AsRef<Path>,
    replay_workspace: impl AsRef<Path>,
) -> AssetProofReport {
    let bundle_root = bundle_root.as_ref();
    let replay_workspace = replay_workspace.as_ref();

    // Structural re-validation first: a proof-less or provenance-gapped asset
    // never reaches the re-run (fail-closed, mirrors registry consume).
    if let Err(error) = asset.validate() {
        return report(
            asset,
            AssetProofStatus::Invalid,
            None,
            ProvenanceBundleStatus::Incomplete,
            Vec::new(),
            false,
            vec![format!("asset failed re-validation: {error}")],
        );
    }

    let proof = match &asset.replay_proof {
        Some(proof) => proof,
        None => {
            return report(
                asset,
                AssetProofStatus::Invalid,
                None,
                ProvenanceBundleStatus::Incomplete,
                Vec::new(),
                false,
                vec!["asset is missing its replay proof".to_string()],
            );
        }
    };
    let bundle = match &asset.provenance {
        Some(bundle) => bundle,
        None => {
            return report(
                asset,
                AssetProofStatus::Invalid,
                None,
                ProvenanceBundleStatus::Incomplete,
                Vec::new(),
                false,
                vec!["asset is missing its provenance lineage".to_string()],
            );
        }
    };

    // Binding: the replay proof must be the same deterministic inputs the
    // provenance lineage records, so the proof cannot float free of its
    // lineage. Reuses the bundle's own replayInputs; introduces no new field.
    let lineage = traverse_lineage(bundle, bundle_root);
    let lineage_traceable = lineage.iter().all(|step| step.state == "present");
    if bundle.replay_inputs.as_ref() != Some(proof) {
        return report(
            asset,
            AssetProofStatus::ProofUnbound,
            None,
            ProvenanceBundleStatus::Incomplete,
            lineage,
            lineage_traceable,
            vec!["replay proof is not bound to the provenance lineage replay inputs".to_string()],
        );
    }

    // Tamper detection on the lineage: reuse the digest-bound bundle evaluation.
    // A mutated ref reports `stale`; a missing ref reports `dangling`.
    let evaluation = bundle.evaluate_with_root(bundle_root);
    if evaluation.computed_status != ProvenanceBundleStatus::Complete {
        let mut issues = vec![format!(
            "provenance lineage is not intact: computed status {:?}",
            evaluation.computed_status
        )];
        issues.extend(evaluation.issues);
        return report(
            asset,
            AssetProofStatus::ProvenanceTampered,
            None,
            evaluation.computed_status,
            lineage,
            lineage_traceable,
            issues,
        );
    }

    // Re-run the replay proof with the existing evaluator. `Reproduced` is the
    // only verified outcome; `Diverged`/`NotReplayable` fail closed.
    let replay = replay_provenance_bundle(bundle, bundle_root, replay_workspace);
    let status = match replay.status {
        ProvenanceReplayStatus::Reproduced if lineage_traceable => AssetProofStatus::Verified,
        ProvenanceReplayStatus::Reproduced => AssetProofStatus::ProvenanceTampered,
        ProvenanceReplayStatus::Diverged => AssetProofStatus::ReplayDiverged,
        ProvenanceReplayStatus::NotReplayable => AssetProofStatus::ReplayNotReplayable,
    };

    let mut issues = Vec::new();
    if status != AssetProofStatus::Verified {
        if !lineage_traceable {
            issues.push("provenance lineage is not fully traceable".to_string());
        }
        if replay.status == ProvenanceReplayStatus::Diverged {
            issues.push("replay proof diverged from the bound expected verdict".to_string());
        }
        issues.extend(replay.issues.iter().cloned());
    }

    report(
        asset,
        status,
        Some(replay.status),
        evaluation.computed_status,
        lineage,
        lineage_traceable,
        issues,
    )
}

fn traverse_lineage(
    bundle: &ProvenanceBundleArtifact,
    bundle_root: &Path,
) -> Vec<AssetLineageStep> {
    let evaluation = bundle.evaluate_with_root(bundle_root);
    LINEAGE_ORDER
        .iter()
        .map(|kind| {
            let label = link_kind_label(*kind);
            let link = bundle.chain_links.iter().find(|link| link.kind == *kind);
            AssetLineageStep {
                kind: label.to_string(),
                artifact_id: link
                    .map(|link| link.artifact_id.clone())
                    .unwrap_or_default(),
                reference: link.map(|link| link.reference.clone()).unwrap_or_default(),
                state: evaluation
                    .link_states
                    .get(label)
                    .cloned()
                    .unwrap_or_else(|| "missing".to_string()),
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn report(
    asset: &MarketplaceAsset,
    status: AssetProofStatus,
    replay_status: Option<ProvenanceReplayStatus>,
    provenance_status: ProvenanceBundleStatus,
    lineage: Vec<AssetLineageStep>,
    lineage_traceable: bool,
    issues: Vec<String>,
) -> AssetProofReport {
    AssetProofReport {
        schema_version: EVIDENCE_MARKETPLACE_PROOF_SCHEMA_VERSION.to_string(),
        asset_id: asset.asset_id.clone(),
        status,
        replay_status,
        provenance_status,
        lineage,
        lineage_traceable,
        issues,
        boundary: proof_boundary(),
    }
}

fn proof_boundary() -> String {
    "Local Rust re-verification only; re-runs the bound replay proof via the existing evaluator and \
     re-checks the Milestone 25 provenance lineage for tampering without executing commands, \
     applying patches, or mutating source."
        .to_string()
}

fn link_kind_label(kind: ProvenanceBundleLinkKind) -> &'static str {
    match kind {
        ProvenanceBundleLinkKind::IntentDesignBrief => "intent-design-brief",
        ProvenanceBundleLinkKind::GeneratedEditedArtifact => "generated-edited-artifact",
        ProvenanceBundleLinkKind::ValidationResult => "validation-result",
        ProvenanceBundleLinkKind::RuntimeObservation => "runtime-observation",
        ProvenanceBundleLinkKind::EvaluatorVerdict => "evaluator-verdict",
        ProvenanceBundleLinkKind::RegressionComparison => "regression-comparison",
        ProvenanceBundleLinkKind::JournalReviewDecision => "journal-review-decision",
        ProvenanceBundleLinkKind::PromotionRollbackRecord => "promotion-rollback-record",
    }
}
