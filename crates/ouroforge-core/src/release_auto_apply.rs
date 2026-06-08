//! Broadened Bounded Auto-Apply and Game-Scale Rollback v1 (#1690, #1 Era H Milestone 44).
//!
//! This module extends the Milestone 22 trust gradient rather than introducing a
//! new writer. It composes the existing rollback-backed auto-apply decision and
//! append-only audit/kill-switch log with release-scale verification evidence and
//! a game-scale rollback command. The only auto-applied outcome is still
//! low-risk, high-confidence, all-gates-passing, in-budget, rollback-backed, and
//! autonomy-opted-in; high-risk/source-affecting proposals and an engaged kill
//! switch fall back to manual review. The module decides and records evidence; it
//! executes no command, writes no trusted file, releases nothing, and grants no
//! browser/Studio authority.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::trust_gradient_audit::AutoApplyAuditLog;
use crate::trust_gradient_auto_apply::{
    decide_auto_apply, AutoApplyDecision, AutoApplyOutcome, AutoApplyRequest,
};

pub const RELEASE_AUTO_APPLY_SCHEMA_VERSION: &str = "release-auto-apply-v1";

const BOUNDARY: &str = "broadened bounded auto-apply reusing the Milestone 22 trust gradient: \
proposal-only inputs, Rust/local evidence, browser and Studio read-only, no auto-merge, \
no self-approval, no release authority, no production-ready or quality/fun claim";

/// Stronger release-scale verification evidence required before a broadened
/// auto-apply may pass. These are evidence refs only; this module does not run
/// verification commands.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseVerificationEvidence {
    #[serde(rename = "preflightRef")]
    pub preflight_ref: String,
    #[serde(rename = "postApplyRerunRef")]
    pub post_apply_rerun_ref: String,
    #[serde(rename = "provenanceBundleRef")]
    pub provenance_bundle_ref: String,
    #[serde(rename = "complianceGateRef")]
    pub compliance_gate_ref: String,
    #[serde(rename = "allRequiredChecksPassed")]
    pub all_required_checks_passed: bool,
    #[serde(rename = "humanReleaseGateState")]
    pub human_release_gate_state: HumanReleaseGateState,
}

impl ReleaseVerificationEvidence {
    fn refs(&self) -> [&str; 4] {
        [
            &self.preflight_ref,
            &self.post_apply_rerun_ref,
            &self.provenance_bundle_ref,
            &self.compliance_gate_ref,
        ]
    }

    fn validate(&self) -> Vec<String> {
        let mut reasons = Vec::new();
        for reference in self.refs() {
            if !is_safe_fixture_ref(reference) {
                reasons.push(format!(
                    "release verification ref is missing or unsafe: {reference}"
                ));
            }
        }
        if !self.all_required_checks_passed {
            reasons.push("stronger release verification checks have not all passed".to_string());
        }
        if self.human_release_gate_state != HumanReleaseGateState::PendingHumanGoNoGo {
            reasons.push(
                "human release gate must remain pending; auto-apply is not release authority"
                    .to_string(),
            );
        }
        reasons
    }
}

/// The release gate state carried with the decision. A broadened auto-apply may
/// update a low-risk proposal through the trusted path, but it must not claim or
/// perform a release; the release gate stays human-owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum HumanReleaseGateState {
    PendingHumanGoNoGo,
    Approved,
    Missing,
}

/// Game-scale rollback evidence. The command is an inert one-command recovery
/// string that routes to the existing rollback path; it is returned for operators
/// and audit evidence, never executed here.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameScaleRollback {
    #[serde(rename = "rollbackScope")]
    pub rollback_scope: RollbackScope,
    #[serde(rename = "rollbackCommand")]
    pub rollback_command: String,
    #[serde(rename = "rollbackEvidenceRef")]
    pub rollback_evidence_ref: String,
    #[serde(rename = "coversWholeGameState")]
    pub covers_whole_game_state: bool,
}

impl GameScaleRollback {
    fn validate(&self) -> Vec<String> {
        let mut reasons = Vec::new();
        if self.rollback_scope != RollbackScope::GameScale {
            reasons.push("rollback scope is not game-scale".to_string());
        }
        if !self.covers_whole_game_state {
            reasons.push("rollback does not cover whole-game state".to_string());
        }
        if !is_safe_fixture_ref(&self.rollback_evidence_ref) {
            reasons.push("rollback evidence ref is missing or unsafe".to_string());
        }
        let command = self.rollback_command.trim();
        if !command.starts_with("ouroforge rollback --transaction ")
            || !command.contains(" --reverse ")
        {
            reasons.push(
                "rollback command must be a single existing ouroforge rollback command".to_string(),
            );
        }
        if command.contains(';')
            || command.contains("&&")
            || command.contains('|')
            || command.contains('`')
        {
            reasons.push("rollback command must not contain shell chaining".to_string());
        }
        reasons
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RollbackScope {
    GameScale,
    SingleArtifact,
}

/// Broadened auto-apply request for release-scale evidence. `trustDecision` is
/// the existing Milestone 22 decision input; no new writer or separate trust
/// classifier is introduced.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseAutoApplyRequest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "releaseCandidateRef")]
    pub release_candidate_ref: String,
    #[serde(rename = "trustDecision")]
    pub trust_decision: AutoApplyRequest,
    pub verification: ReleaseVerificationEvidence,
    #[serde(rename = "gameScaleRollback")]
    pub game_scale_rollback: GameScaleRollback,
    pub boundary: String,
}

impl ReleaseAutoApplyRequest {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let request: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse release auto-apply request: {err}"))?;
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RELEASE_AUTO_APPLY_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected schema version: {}",
                self.schema_version
            ));
        }
        if !is_safe_fixture_ref(&self.release_candidate_ref) {
            return Err(anyhow!(
                "releaseCandidateRef must be a safe fixture/evidence ref"
            ));
        }
        self.trust_decision.validate()?;
        if !self.boundary.contains("proposal-only")
            || !self.boundary.contains("read-only")
            || !self.boundary.contains("no auto-merge")
            || !self.boundary.contains("human")
        {
            return Err(anyhow!(
                "boundary must preserve proposal-only, read-only, no auto-merge, and human gate wording"
            ));
        }
        Ok(())
    }
}

/// Decision for the broadened release-scale tier.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseAutoApplyDecision {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "releaseCandidateRef")]
    pub release_candidate_ref: String,
    pub outcome: AutoApplyOutcome,
    pub reasons: Vec<String>,
    #[serde(
        rename = "rollbackCommand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rollback_command: Option<String>,
    #[serde(rename = "trustDecision")]
    pub trust_decision: AutoApplyDecision,
    pub boundary: String,
}

/// Decide whether a release-scale proposal can use the broadened bounded tier.
/// The audit log supplies the existing kill switch. If it is halted, the result
/// is manual fallback even when the embedded Milestone 22 request is otherwise
/// eligible.
pub fn decide_release_auto_apply(
    request: &ReleaseAutoApplyRequest,
    audit_log: &AutoApplyAuditLog,
) -> Result<ReleaseAutoApplyDecision> {
    request.validate()?;
    audit_log.validate()?;

    let trust_decision = decide_auto_apply(&request.trust_decision)?;
    let mut reasons = trust_decision.reasons.clone();
    let mut eligible = trust_decision.outcome == AutoApplyOutcome::AutoApplied;

    if audit_log.is_autonomy_halted() {
        eligible = false;
        reasons.push("kill switch engaged: autonomy halted".to_string());
    }

    let verification_reasons = request.verification.validate();
    if !verification_reasons.is_empty() {
        eligible = false;
        reasons.extend(verification_reasons);
    }

    let rollback_reasons = request.game_scale_rollback.validate();
    if !rollback_reasons.is_empty() {
        eligible = false;
        reasons.extend(rollback_reasons);
    }

    let (outcome, rollback_command) = if eligible {
        reasons.push(
            "release-scale verification passed; game-scale rollback is one-command; human release gate remains pending"
                .to_string(),
        );
        (
            AutoApplyOutcome::AutoApplied,
            Some(request.game_scale_rollback.rollback_command.clone()),
        )
    } else {
        (AutoApplyOutcome::ManualFallback, None)
    };

    Ok(ReleaseAutoApplyDecision {
        schema_version: RELEASE_AUTO_APPLY_SCHEMA_VERSION.to_string(),
        release_candidate_ref: request.release_candidate_ref.clone(),
        outcome,
        reasons,
        rollback_command,
        trust_decision,
        boundary: BOUNDARY.to_string(),
    })
}

fn is_safe_fixture_ref(reference: &str) -> bool {
    let trimmed = reference.trim();
    !trimmed.is_empty()
        && !trimmed.starts_with('/')
        && !trimmed.contains("..")
        && !trimmed.contains('\\')
        && !trimmed.contains(';')
        && !trimmed.contains("&&")
        && !trimmed.contains('|')
}
