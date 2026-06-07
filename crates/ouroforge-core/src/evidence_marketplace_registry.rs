//! Local Verifiable-Asset Registry v1 (#1613).
//!
//! A local registry over the free OSS core: publish and consume verifiable
//! asset templates, validating on publish that each asset carries its
//! acceptance suite, a deterministic replay proof, and a Milestone 25
//! provenance lineage (`provenance_bundle.rs`, #1500). Proof-less or
//! provenance-gapped assets are rejected fail-closed.
//!
//! This module reuses the existing provenance bundle; it is **not** a new
//! provenance engine. It performs no trusted write, executes nothing, installs
//! nothing, and adds no hosted/paid/network capability (Layer-3 stays DEFER
//! per #1508). Adoption of a consumed asset flows through the existing
//! review/apply/trust-gradient path, never a direct trusted write.
//!
//! See `docs/evidence-marketplace-v1.md` (#1612) for the design gate.

use crate::provenance_bundle::{
    ProvenanceBundleArtifact, ProvenanceBundleGeneratedState, ProvenanceBundleLinkKind,
    ProvenanceBundleReplayInputs, ProvenanceBundleStatus,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

pub const EVIDENCE_MARKETPLACE_REGISTRY_SCHEMA_VERSION: &str = "evidence-marketplace-registry-v1";

/// The provenance lineage a published asset must carry in full: a verifiable
/// asset binds to a *complete* Milestone 25 provenance bundle, with no gap.
const REQUIRED_PROVENANCE_LINK_KINDS: &[ProvenanceBundleLinkKind] = &[
    ProvenanceBundleLinkKind::IntentDesignBrief,
    ProvenanceBundleLinkKind::GeneratedEditedArtifact,
    ProvenanceBundleLinkKind::ValidationResult,
    ProvenanceBundleLinkKind::RuntimeObservation,
    ProvenanceBundleLinkKind::EvaluatorVerdict,
    ProvenanceBundleLinkKind::RegressionComparison,
    ProvenanceBundleLinkKind::JournalReviewDecision,
    ProvenanceBundleLinkKind::PromotionRollbackRecord,
];

/// A publishable verifiable asset: a template plus the evidence required to
/// re-verify it locally on consume.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MarketplaceAsset {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    /// Local reference to the published template (scene/seed/source bundle).
    #[serde(rename = "templateRef")]
    pub template_ref: String,
    #[serde(
        default,
        rename = "templateDigest",
        skip_serializing_if = "Option::is_none"
    )]
    pub template_digest: Option<String>,
    /// Local reference to the asset's acceptance (scenario/evaluator) suite.
    #[serde(rename = "acceptanceSuiteRef")]
    pub acceptance_suite_ref: String,
    /// Deterministic replay proof, reusing the provenance bundle replay inputs.
    /// Absent => proof-less asset (rejected on publish, fail-closed).
    #[serde(
        default,
        rename = "replayProof",
        skip_serializing_if = "Option::is_none"
    )]
    pub replay_proof: Option<ProvenanceBundleReplayInputs>,
    /// Provenance lineage, reusing the Milestone 25 provenance bundle.
    /// Absent or incomplete => provenance gap (rejected on publish).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProvenanceBundleArtifact>,
    #[serde(rename = "generatedState")]
    pub generated_state: ProvenanceBundleGeneratedState,
    pub boundary: String,
    pub guardrails: Vec<String>,
}

/// A read-only record of a successful local publish. It enumerates the
/// allowed/forbidden actions to keep the registry proposal-and-record only.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PublishReceipt {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "acceptanceSuiteRef")]
    pub acceptance_suite_ref: String,
    #[serde(rename = "hasReplayProof")]
    pub has_replay_proof: bool,
    #[serde(rename = "provenanceStatus")]
    pub provenance_status: ProvenanceBundleStatus,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

/// An in-memory local registry. Publishing and consuming are both local and
/// free (no paywall, no network, no credential).
#[derive(Debug, Default, Clone)]
pub struct LocalAssetRegistry {
    entries: BTreeMap<String, MarketplaceAsset>,
}

impl MarketplaceAsset {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let asset: Self =
            serde_json::from_str(input).context("failed to parse marketplace asset JSON")?;
        asset.validate()?;
        Ok(asset)
    }

    /// Validate that the asset carries its acceptance suite, a deterministic
    /// replay proof, and a complete provenance lineage. Fail-closed: a missing
    /// proof or a provenance gap is an error, not a warning.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != EVIDENCE_MARKETPLACE_REGISTRY_SCHEMA_VERSION {
            return Err(anyhow!(
                "marketplace asset schemaVersion must be {EVIDENCE_MARKETPLACE_REGISTRY_SCHEMA_VERSION}"
            ));
        }
        require_local_id("marketplace asset assetId", &self.asset_id)?;
        require_local_ref("marketplace asset templateRef", &self.template_ref)?;
        if let Some(digest) = &self.template_digest {
            require_sha256_digest("marketplace asset templateDigest", digest)?;
        }
        require_local_ref(
            "marketplace asset acceptanceSuiteRef",
            &self.acceptance_suite_ref,
        )?;

        match &self.replay_proof {
            Some(proof) => validate_replay_proof(proof)?,
            None => {
                return Err(anyhow!(
                    "marketplace asset `{}` is missing its deterministic replay proof",
                    self.asset_id
                ));
            }
        }

        match &self.provenance {
            Some(bundle) => validate_provenance_lineage(&self.asset_id, bundle)?,
            None => {
                return Err(anyhow!(
                    "marketplace asset `{}` is missing its provenance lineage",
                    self.asset_id
                ));
            }
        }

        validate_generated_state(&self.generated_state)?;
        require_text("marketplace asset boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        if !boundary.contains("local") {
            return Err(anyhow!(
                "marketplace asset boundary must state it is a local registry"
            ));
        }
        validate_text_list("marketplace asset guardrails", &self.guardrails)?;
        Ok(())
    }

    fn publish_receipt(&self) -> PublishReceipt {
        PublishReceipt {
            schema_version: EVIDENCE_MARKETPLACE_REGISTRY_SCHEMA_VERSION.to_string(),
            asset_id: self.asset_id.clone(),
            acceptance_suite_ref: self.acceptance_suite_ref.clone(),
            has_replay_proof: self.replay_proof.is_some(),
            provenance_status: self
                .provenance
                .as_ref()
                .map(|bundle| bundle.status)
                .unwrap_or(ProvenanceBundleStatus::Incomplete),
            allowed_actions: vec![
                "inspect_asset".to_string(),
                "replay_proof_locally".to_string(),
                "run_acceptance_suite_locally".to_string(),
            ],
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "merge_branch".to_string(),
                "promote_without_review".to_string(),
                "execute_command".to_string(),
                "host_or_sell_remotely".to_string(),
            ],
        }
    }
}

impl LocalAssetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Publish an asset into the local registry. Rejects proof-less or
    /// provenance-gapped assets, and rejects a duplicate id (fail-closed).
    pub fn publish(&mut self, asset: MarketplaceAsset) -> Result<PublishReceipt> {
        asset.validate()?;
        if self.entries.contains_key(&asset.asset_id) {
            return Err(anyhow!(
                "marketplace asset `{}` is already published in the local registry",
                asset.asset_id
            ));
        }
        let receipt = asset.publish_receipt();
        self.entries.insert(asset.asset_id.clone(), asset);
        Ok(receipt)
    }

    /// Consume a published asset by id, re-verifying it locally before
    /// returning it. Consuming an unpublished id fails closed.
    pub fn consume(&self, asset_id: &str) -> Result<&MarketplaceAsset> {
        let asset = self.entries.get(asset_id).ok_or_else(|| {
            anyhow!("marketplace asset `{asset_id}` is not published in the local registry")
        })?;
        asset.validate().with_context(|| {
            format!("published marketplace asset `{asset_id}` failed local re-verification")
        })?;
        Ok(asset)
    }

    pub fn published_ids(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

fn validate_replay_proof(proof: &ProvenanceBundleReplayInputs) -> Result<()> {
    require_local_ref("marketplace asset replayProof.runRef", &proof.run_ref)?;
    require_local_ref(
        "marketplace asset replayProof.expectedVerdictRef",
        &proof.expected_verdict_ref,
    )?;
    for reference in &proof.deterministic_metadata_refs {
        require_local_ref(
            "marketplace asset replayProof.deterministicMetadataRefs",
            reference,
        )?;
    }
    if !proof.deterministic_inputs {
        return Err(anyhow!(
            "marketplace asset replayProof must declare deterministic inputs"
        ));
    }
    Ok(())
}

fn validate_provenance_lineage(asset_id: &str, bundle: &ProvenanceBundleArtifact) -> Result<()> {
    bundle.validate_shape().with_context(|| {
        format!("marketplace asset `{asset_id}` has an invalid provenance bundle")
    })?;
    if bundle.status != ProvenanceBundleStatus::Complete {
        return Err(anyhow!(
            "marketplace asset `{asset_id}` has a provenance gap: bundle status must be complete"
        ));
    }
    let present: BTreeSet<_> = bundle.chain_links.iter().map(|link| link.kind).collect();
    for required in REQUIRED_PROVENANCE_LINK_KINDS {
        if !present.contains(required) {
            return Err(anyhow!(
                "marketplace asset `{asset_id}` has a provenance gap: missing chain link `{}`",
                link_kind_label(*required)
            ));
        }
    }
    Ok(())
}

fn validate_generated_state(state: &ProvenanceBundleGeneratedState) -> Result<()> {
    if state.generated && state.tracked && !state.fixture_scoped {
        return Err(anyhow!(
            "generated marketplace assets must be untracked unless fixture-scoped"
        ));
    }
    Ok(())
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

fn validate_text_list(field: &str, values: &[String]) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 128
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
    let path = Path::new(value);
    if path.is_absolute()
        || value.contains('\\')
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(anyhow!(
            "{field} must stay inside the local registry root (no absolute, parent, or backslash paths)"
        ));
    }
    Ok(())
}

fn require_sha256_digest(field: &str, value: &str) -> Result<()> {
    let digest = value.strip_prefix("sha256:").unwrap_or_default();
    if digest.len() != 64 || !digest.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a sha256:<64 hex> digest"));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}
