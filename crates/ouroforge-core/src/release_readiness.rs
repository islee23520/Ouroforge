//! Release-readiness bundle and go/no-go surface v1 (#1871).
//!
//! Composes existing four-gate, balance, fun-feel, compliance, Steam-export,
//! and Milestone 25/44 provenance evidence into a Rust/local read-only bundle.
//! The bundle is not a release button: a separate human go/no-go record is
//! required, browser/Studio surfaces remain read-only, and no auto-merge,
//! trusted write, production, quality, fun, or Godot-parity claim is granted.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const RELEASE_READINESS_SCHEMA_VERSION: &str = "ouroforge.release-readiness.v1";
pub const RELEASE_READINESS_GENERATOR: &str = "release-readiness-bundle-v1";
pub const RELEASE_READINESS_BOUNDARY: &str = "Rust/local read-only release-readiness composition; extends Milestone 25/44 provenance; human go/no-go required and recorded separately; browser/Studio read-only; no release authority; no auto-merge; no auto-apply; no fun/quality/production/Godot claim; #1 and #23 remain open";

const REQUIRED_GATE_KINDS: &[&str] = &[
    "four-gate-visual",
    "four-gate-semantic",
    "four-gate-design-integrity",
    "four-gate-provenance",
    "balance",
    "fun-feel-human-gate",
    "release-compliance",
    "steam-export-readiness",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseReadinessStatus {
    Ready,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseReadinessGateStatus {
    Pass,
    Blocked,
    Missing,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseGoNoGoDecision {
    Go,
    NoGo,
    Deferred,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReleaseReadinessInput {
    pub schema_version: String,
    pub release_candidate_id: String,
    pub provenance_bundle_ref: String,
    pub release_provenance_bundle_ref: String,
    pub gate_evidence: Vec<ReleaseReadinessGateEvidence>,
    pub generated_state_policy: String,
    pub browser_studio_mode: String,
    pub trusted_write_requested: bool,
    pub auto_merge_requested: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReleaseReadinessGateEvidence {
    pub gate_id: String,
    pub gate_kind: String,
    pub status: ReleaseReadinessGateStatus,
    pub evidence_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReleaseReadinessBundle {
    pub schema_version: String,
    pub bundle_id: String,
    pub release_candidate_id: String,
    pub provenance_bundle_ref: String,
    pub release_provenance_bundle_ref: String,
    pub generator: String,
    pub status: ReleaseReadinessStatus,
    pub gate_results: Vec<ReleaseReadinessGateEvidence>,
    pub missing_gate_kinds: Vec<String>,
    pub blocked_reasons: Vec<String>,
    pub human_go_no_go_required: bool,
    pub human_go_no_go_recorded: bool,
    pub read_only_surface: bool,
    pub release_authority_granted: bool,
    pub auto_merge_allowed: bool,
    pub trusted_write_authority: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReleaseGoNoGoInput {
    pub decision_id: String,
    pub bundle_id: String,
    pub decision: ReleaseGoNoGoDecision,
    pub human_actor: String,
    pub human_confirmed: bool,
    pub rationale: String,
    pub decided_at_unix_ms: u128,
    pub read_only_surface: bool,
    pub release_authority_granted: bool,
    pub auto_merge_requested: bool,
    pub trusted_write_authority: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReleaseGoNoGoRecord {
    pub schema_version: String,
    pub decision_id: String,
    pub bundle_id: String,
    pub release_candidate_id: String,
    pub bundle_status: ReleaseReadinessStatus,
    pub decision: ReleaseGoNoGoDecision,
    pub human_actor: String,
    pub human_confirmed: bool,
    pub rationale: String,
    pub decided_at_unix_ms: u128,
    pub release_ready: bool,
    pub read_only_surface: bool,
    pub release_authority_granted: bool,
    pub auto_merge_allowed: bool,
    pub trusted_write_authority: bool,
    pub boundary: String,
}

impl ReleaseReadinessInput {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("release-readiness input is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        require_schema(&self.schema_version, "input")?;
        require_text("release candidate id", &self.release_candidate_id)?;
        require_ref("provenance bundle ref", &self.provenance_bundle_ref)?;
        require_ref(
            "release provenance bundle ref",
            &self.release_provenance_bundle_ref,
        )?;
        if self.gate_evidence.is_empty() {
            return Err(anyhow!("release-readiness gateEvidence must not be empty"));
        }
        let mut ids = BTreeSet::new();
        for gate in &self.gate_evidence {
            gate.validate()?;
            if !ids.insert(gate.gate_id.as_str()) {
                return Err(anyhow!(
                    "duplicate release-readiness gateId {}",
                    gate.gate_id
                ));
            }
        }
        require_text("generated state policy", &self.generated_state_policy)?;
        let generated_policy = self.generated_state_policy.to_ascii_lowercase();
        if !generated_policy.contains("fixture-scoped") || !generated_policy.contains("untracked") {
            return Err(anyhow!(
                "generatedStatePolicy must keep generated runs/artifacts untracked unless fixture-scoped"
            ));
        }
        if self.browser_studio_mode != "read-only" {
            return Err(anyhow!("browserStudioMode must be read-only"));
        }
        if self.trusted_write_requested || self.auto_merge_requested {
            return Err(anyhow!(
                "release-readiness input must not request trusted writes or auto-merge"
            ));
        }
        require_boundary(&self.boundary)?;
        Ok(())
    }
}

impl ReleaseReadinessGateEvidence {
    pub fn validate(&self) -> Result<()> {
        require_text("gate id", &self.gate_id)?;
        require_text("gate kind", &self.gate_kind)?;
        require_ref("gate evidence ref", &self.evidence_ref)?;
        if matches!(
            self.status,
            ReleaseReadinessGateStatus::Blocked | ReleaseReadinessGateStatus::Missing
        ) && self
            .blocked_reason
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .is_empty()
        {
            return Err(anyhow!(
                "blocked or missing release-readiness gates require blockedReason"
            ));
        }
        Ok(())
    }
}

impl ReleaseReadinessBundle {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("release-readiness bundle is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        require_schema(&self.schema_version, "bundle")?;
        require_text("bundle id", &self.bundle_id)?;
        require_text("release candidate id", &self.release_candidate_id)?;
        require_ref("provenance bundle ref", &self.provenance_bundle_ref)?;
        require_ref(
            "release provenance bundle ref",
            &self.release_provenance_bundle_ref,
        )?;
        if self.generator != RELEASE_READINESS_GENERATOR {
            return Err(anyhow!(
                "release-readiness generator must be {RELEASE_READINESS_GENERATOR}"
            ));
        }
        if self.gate_results.is_empty() {
            return Err(anyhow!(
                "release-readiness bundle gateResults must not be empty"
            ));
        }
        for gate in &self.gate_results {
            gate.validate()?;
        }
        let has_blockers = !self.missing_gate_kinds.is_empty() || !self.blocked_reasons.is_empty();
        if (self.status == ReleaseReadinessStatus::Ready) == has_blockers {
            return Err(anyhow!(
                "release-readiness bundle status must match missing/blocked gate evidence"
            ));
        }
        if !self.human_go_no_go_required
            || self.human_go_no_go_recorded
            || !self.read_only_surface
            || self.release_authority_granted
            || self.auto_merge_allowed
            || self.trusted_write_authority
        {
            return Err(anyhow!(
                "release-readiness bundle must require a separate human go/no-go and grant no release/merge/write authority"
            ));
        }
        require_boundary(&self.boundary)?;
        Ok(())
    }
}

impl ReleaseGoNoGoInput {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("release go/no-go input is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        require_text("decision id", &self.decision_id)?;
        require_text("decision bundle id", &self.bundle_id)?;
        require_text("human actor", &self.human_actor)?;
        require_text("go/no-go rationale", &self.rationale)?;
        if !self.human_confirmed {
            return Err(anyhow!("release go/no-go must be confirmed by a human"));
        }
        if self.decided_at_unix_ms == 0 {
            return Err(anyhow!("decidedAtUnixMs must be greater than zero"));
        }
        if !self.read_only_surface
            || self.release_authority_granted
            || self.auto_merge_requested
            || self.trusted_write_authority
        {
            return Err(anyhow!(
                "release go/no-go record must stay read-only and grant no release/merge/write authority"
            ));
        }
        require_boundary(&self.boundary)?;
        Ok(())
    }
}

impl ReleaseGoNoGoRecord {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("release go/no-go record is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        require_schema(&self.schema_version, "go/no-go record")?;
        require_text("decision id", &self.decision_id)?;
        require_text("bundle id", &self.bundle_id)?;
        require_text("release candidate id", &self.release_candidate_id)?;
        require_text("human actor", &self.human_actor)?;
        require_text("go/no-go rationale", &self.rationale)?;
        if !self.human_confirmed
            || !self.read_only_surface
            || self.release_authority_granted
            || self.auto_merge_allowed
            || self.trusted_write_authority
        {
            return Err(anyhow!(
                "release go/no-go record must be human-confirmed, read-only, and grant no release/merge/write authority"
            ));
        }
        if self.release_ready
            != (self.bundle_status == ReleaseReadinessStatus::Ready
                && self.decision == ReleaseGoNoGoDecision::Go)
        {
            return Err(anyhow!(
                "releaseReady must require a mechanically ready bundle and human go decision"
            ));
        }
        require_boundary(&self.boundary)?;
        Ok(())
    }
}

pub fn build_release_readiness_bundle(
    input: &ReleaseReadinessInput,
) -> Result<ReleaseReadinessBundle> {
    input.validate()?;
    let present = input
        .gate_evidence
        .iter()
        .map(|gate| gate.gate_kind.as_str())
        .collect::<BTreeSet<_>>();
    let missing_gate_kinds = REQUIRED_GATE_KINDS
        .iter()
        .filter(|kind| !present.contains(**kind))
        .map(|kind| (*kind).to_string())
        .collect::<Vec<_>>();
    let mut blocked_reasons = Vec::new();
    for gate in &input.gate_evidence {
        match gate.status {
            ReleaseReadinessGateStatus::Pass => {}
            ReleaseReadinessGateStatus::Blocked => blocked_reasons.push(format!(
                "{} blocked: {}",
                gate.gate_kind,
                gate.blocked_reason.as_deref().unwrap_or("blocked")
            )),
            ReleaseReadinessGateStatus::Missing => blocked_reasons.push(format!(
                "{} missing: {}",
                gate.gate_kind,
                gate.blocked_reason.as_deref().unwrap_or("missing")
            )),
        }
    }
    let status = if missing_gate_kinds.is_empty() && blocked_reasons.is_empty() {
        ReleaseReadinessStatus::Ready
    } else {
        ReleaseReadinessStatus::Blocked
    };
    let bundle = ReleaseReadinessBundle {
        schema_version: RELEASE_READINESS_SCHEMA_VERSION.to_string(),
        bundle_id: format!("release-readiness-{}", input.release_candidate_id),
        release_candidate_id: input.release_candidate_id.clone(),
        provenance_bundle_ref: input.provenance_bundle_ref.clone(),
        release_provenance_bundle_ref: input.release_provenance_bundle_ref.clone(),
        generator: RELEASE_READINESS_GENERATOR.to_string(),
        status,
        gate_results: input.gate_evidence.clone(),
        missing_gate_kinds,
        blocked_reasons,
        human_go_no_go_required: true,
        human_go_no_go_recorded: false,
        read_only_surface: true,
        release_authority_granted: false,
        auto_merge_allowed: false,
        trusted_write_authority: false,
        boundary: RELEASE_READINESS_BOUNDARY.to_string(),
    };
    bundle.validate()?;
    Ok(bundle)
}

pub fn record_release_go_no_go(
    bundle: &ReleaseReadinessBundle,
    input: &ReleaseGoNoGoInput,
) -> Result<ReleaseGoNoGoRecord> {
    bundle.validate()?;
    input.validate()?;
    if input.bundle_id != bundle.bundle_id {
        return Err(anyhow!(
            "release go/no-go input references a different bundle"
        ));
    }
    let record = ReleaseGoNoGoRecord {
        schema_version: RELEASE_READINESS_SCHEMA_VERSION.to_string(),
        decision_id: input.decision_id.clone(),
        bundle_id: bundle.bundle_id.clone(),
        release_candidate_id: bundle.release_candidate_id.clone(),
        bundle_status: bundle.status.clone(),
        decision: input.decision.clone(),
        human_actor: input.human_actor.clone(),
        human_confirmed: input.human_confirmed,
        rationale: input.rationale.clone(),
        decided_at_unix_ms: input.decided_at_unix_ms,
        release_ready: bundle.status == ReleaseReadinessStatus::Ready
            && input.decision == ReleaseGoNoGoDecision::Go,
        read_only_surface: true,
        release_authority_granted: false,
        auto_merge_allowed: false,
        trusted_write_authority: false,
        boundary: RELEASE_READINESS_BOUNDARY.to_string(),
    };
    record.validate()?;
    Ok(record)
}

fn require_schema(actual: &str, label: &str) -> Result<()> {
    if actual != RELEASE_READINESS_SCHEMA_VERSION {
        return Err(anyhow!(
            "release-readiness {label} schemaVersion must be {RELEASE_READINESS_SCHEMA_VERSION}"
        ));
    }
    Ok(())
}

fn require_text(label: &str, text: &str) -> Result<()> {
    if text.trim().is_empty() {
        return Err(anyhow!("release-readiness {label} must be non-empty"));
    }
    Ok(())
}

fn require_ref(label: &str, value: &str) -> Result<()> {
    require_text(label, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "release-readiness {label} must be a safe repo-relative ref"
        ));
    }
    Ok(())
}

fn require_boundary(boundary: &str) -> Result<()> {
    require_text("boundary", boundary)?;
    let lower = boundary.to_ascii_lowercase();
    for required in [
        "rust/local",
        "read-only",
        "milestone 25/44 provenance",
        "human go/no-go required",
        "browser/studio read-only",
        "no release authority",
        "no auto-merge",
        "no fun",
        "#1 and #23 remain open",
    ] {
        if !lower.contains(required) {
            return Err(anyhow!("release-readiness boundary missing {required}"));
        }
    }
    Ok(())
}
