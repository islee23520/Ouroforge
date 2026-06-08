//! Balance Tuning Co-Pilot v1 (#1870).
//!
//! This module surfaces Milestone 50 dominant-build findings as actionable
//! recommendations for human approval/tweak and then re-verifies against a later
//! balance report. It is deliberately advisory: it never mutates configs,
//! applies nerfs, grants browser/Studio write authority, or claims fun/quality.

use crate::balance_dominant_build::{BuildMetric, DominantBuildReport};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const BALANCE_COPILOT_SCHEMA_VERSION: &str = "ouroforge.balance-copilot.v1";
pub const BALANCE_COPILOT_GENERATOR: &str = "balance-tuning-copilot-v1";
pub const BALANCE_COPILOT_BOUNDARY: &str = "Rust/local advisory recommendations only; human approval required; re-verify before review/apply/trust-gradient; browser/Studio read-only; no auto-apply; no auto-merge; no fun/quality/production/Godot claim; #1 and #23 remain open";
pub const BALANCE_COPILOT_STATUS_REVERIFY_REQUIRED: &str = "reverify-required";
pub const BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED: &str = "reverified-improved";
pub const BALANCE_COPILOT_STATUS_REVERIFIED_STILL_FLAGGED: &str = "reverified-still-flagged";

const ALLOWED_DECISIONS: &[&str] = &["approved", "approved-with-tweak", "rejected", "deferred"];
const ALLOWED_ACTIONS: &[&str] = &[
    "reduce-dominant-build-synergy",
    "increase-counterplay-window",
    "raise-underused-modifier-visibility",
    "request-more-telemetry",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BalanceCopilotRecommendationSet {
    pub schema_version: String,
    pub recommendation_set_id: String,
    pub source_report_fixture_id: String,
    pub source_report_digest: String,
    pub generator: String,
    pub recommendations: Vec<BalanceCopilotRecommendation>,
    pub proposal_only: bool,
    pub human_approval_required: bool,
    pub auto_apply_allowed: bool,
    pub trusted_write_authority: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BalanceCopilotRecommendation {
    pub recommendation_id: String,
    pub finding_kind: String,
    pub target_build_id: String,
    pub source_replay_deck_seed: u32,
    pub source_replay_persona: String,
    pub pick_rate_bps: u32,
    pub win_rate_bps: u32,
    pub recommended_action: String,
    pub rationale: String,
    pub status: String,
    pub review_apply_required: bool,
    pub auto_apply_allowed: bool,
    pub trusted_write_authority: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct BalanceCopilotApprovalInput<'a> {
    pub approval_id: &'a str,
    pub recommendation_id: &'a str,
    pub decision: &'a str,
    pub human_actor: &'a str,
    pub rationale: &'a str,
    pub tweaked_action: Option<&'a str>,
    pub recorded_at_unix_ms: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BalanceCopilotHumanApproval {
    pub schema_version: String,
    pub approval_id: String,
    pub recommendation_set_id: String,
    pub recommendation_id: String,
    pub decision: String,
    pub human_actor: String,
    pub rationale: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tweaked_action: Option<String>,
    pub recorded_at_unix_ms: u128,
    pub reverify_required: bool,
    pub auto_apply_requested: bool,
    pub trusted_write_authority: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BalanceCopilotReverificationReport {
    pub schema_version: String,
    pub reverification_id: String,
    pub approval_id: String,
    pub recommendation_set_id: String,
    pub recommendation_id: String,
    pub before_report_digest: String,
    pub after_report_digest: String,
    pub status: String,
    pub dominant_build_still_flagged: bool,
    pub remaining_dominant_build_count: usize,
    pub human_approved: bool,
    pub review_apply_required: bool,
    pub auto_apply_performed: bool,
    pub trusted_write_authority: bool,
    pub boundary: String,
}

impl BalanceCopilotRecommendationSet {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("balance co-pilot recommendation set is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        require_schema(&self.schema_version, "recommendation set")?;
        require_text("recommendation set id", &self.recommendation_set_id)?;
        require_text("source report fixture id", &self.source_report_fixture_id)?;
        require_text("source report digest", &self.source_report_digest)?;
        if self.generator != BALANCE_COPILOT_GENERATOR {
            return Err(anyhow!(
                "recommendation set generator must be {BALANCE_COPILOT_GENERATOR}"
            ));
        }
        require_boundary(&self.boundary)?;
        if !self.proposal_only
            || !self.human_approval_required
            || self.auto_apply_allowed
            || self.trusted_write_authority
        {
            return Err(anyhow!(
                "recommendation set must be proposal-only, human-approved, and without auto-apply/trusted write authority"
            ));
        }
        if self.recommendations.is_empty() {
            return Err(anyhow!("recommendation set must not be empty"));
        }
        let mut ids = BTreeSet::new();
        for recommendation in &self.recommendations {
            recommendation.validate()?;
            if !ids.insert(recommendation.recommendation_id.as_str()) {
                return Err(anyhow!(
                    "duplicate recommendationId {}",
                    recommendation.recommendation_id
                ));
            }
        }
        Ok(())
    }
}

impl BalanceCopilotRecommendation {
    pub fn validate(&self) -> Result<()> {
        require_text("recommendation id", &self.recommendation_id)?;
        if self.finding_kind != "dominant-build" {
            return Err(anyhow!("recommendation findingKind must be dominant-build"));
        }
        require_text("target build id", &self.target_build_id)?;
        require_text("source replay persona", &self.source_replay_persona)?;
        if !ALLOWED_ACTIONS.contains(&self.recommended_action.as_str()) {
            return Err(anyhow!(
                "recommendation recommendedAction is unsupported: {}",
                self.recommended_action
            ));
        }
        require_text("recommendation rationale", &self.rationale)?;
        if self.status != BALANCE_COPILOT_STATUS_REVERIFY_REQUIRED {
            return Err(anyhow!(
                "recommendation status must be {BALANCE_COPILOT_STATUS_REVERIFY_REQUIRED}"
            ));
        }
        if !self.review_apply_required || self.auto_apply_allowed || self.trusted_write_authority {
            return Err(anyhow!(
                "recommendation must require review/apply and must not allow auto-apply/trusted writes"
            ));
        }
        Ok(())
    }
}

impl BalanceCopilotHumanApproval {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("balance co-pilot human approval is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        require_schema(&self.schema_version, "human approval")?;
        require_text("approval id", &self.approval_id)?;
        require_text(
            "approval recommendation set id",
            &self.recommendation_set_id,
        )?;
        require_text("approval recommendation id", &self.recommendation_id)?;
        if !ALLOWED_DECISIONS.contains(&self.decision.as_str()) {
            return Err(anyhow!(
                "human approval decision is unsupported: {}",
                self.decision
            ));
        }
        require_text("human actor", &self.human_actor)?;
        require_text("approval rationale", &self.rationale)?;
        if let Some(tweak) = &self.tweaked_action {
            if !ALLOWED_ACTIONS.contains(&tweak.as_str()) {
                return Err(anyhow!(
                    "human approval tweakedAction is unsupported: {tweak}"
                ));
            }
        }
        if self.recorded_at_unix_ms == 0 {
            return Err(anyhow!(
                "human approval recordedAtUnixMs must be greater than zero"
            ));
        }
        require_boundary(&self.boundary)?;
        if !self.reverify_required || self.auto_apply_requested || self.trusted_write_authority {
            return Err(anyhow!(
                "human approval must require re-verification and must not request auto-apply/trusted writes"
            ));
        }
        Ok(())
    }
}

impl BalanceCopilotReverificationReport {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text).map_err(|err| {
            anyhow!("balance co-pilot reverification report is not valid JSON: {err}")
        })
    }

    pub fn validate(&self) -> Result<()> {
        require_schema(&self.schema_version, "reverification report")?;
        require_text("reverification id", &self.reverification_id)?;
        require_text("reverification approval id", &self.approval_id)?;
        require_text(
            "reverification recommendation set id",
            &self.recommendation_set_id,
        )?;
        require_text("reverification recommendation id", &self.recommendation_id)?;
        require_text("before report digest", &self.before_report_digest)?;
        require_text("after report digest", &self.after_report_digest)?;
        if self.status != BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED
            && self.status != BALANCE_COPILOT_STATUS_REVERIFIED_STILL_FLAGGED
        {
            return Err(anyhow!(
                "reverification status is unsupported: {}",
                self.status
            ));
        }
        require_boundary(&self.boundary)?;
        if !self.human_approved
            || !self.review_apply_required
            || self.auto_apply_performed
            || self.trusted_write_authority
        {
            return Err(anyhow!(
                "reverification must be human-approved, review/apply-gated, and without auto-apply/trusted writes"
            ));
        }
        if self.dominant_build_still_flagged
            && self.status != BALANCE_COPILOT_STATUS_REVERIFIED_STILL_FLAGGED
        {
            return Err(anyhow!(
                "reverification status must report still-flagged when the target remains dominant"
            ));
        }
        if !self.dominant_build_still_flagged
            && self.status != BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED
        {
            return Err(anyhow!(
                "reverification status must report improved when the target is no longer dominant"
            ));
        }
        Ok(())
    }
}

pub fn surface_balance_recommendations(
    report: &DominantBuildReport,
    recommendation_set_id: &str,
) -> Result<BalanceCopilotRecommendationSet> {
    require_text("recommendation set id", recommendation_set_id)?;
    if report.dominant_builds.is_empty() {
        return Err(anyhow!(
            "balance co-pilot requires at least one dominant build finding"
        ));
    }
    let recommendations = report
        .dominant_builds
        .iter()
        .map(recommendation_from_dominant_build)
        .collect::<Vec<_>>();
    let set = BalanceCopilotRecommendationSet {
        schema_version: BALANCE_COPILOT_SCHEMA_VERSION.to_string(),
        recommendation_set_id: recommendation_set_id.to_string(),
        source_report_fixture_id: report.fixture_id.clone(),
        source_report_digest: report.digest.clone(),
        generator: BALANCE_COPILOT_GENERATOR.to_string(),
        recommendations,
        proposal_only: true,
        human_approval_required: true,
        auto_apply_allowed: false,
        trusted_write_authority: false,
        boundary: BALANCE_COPILOT_BOUNDARY.to_string(),
    };
    set.validate()?;
    Ok(set)
}

pub fn record_balance_copilot_human_approval(
    set: &BalanceCopilotRecommendationSet,
    input: BalanceCopilotApprovalInput<'_>,
) -> Result<BalanceCopilotHumanApproval> {
    set.validate()?;
    if !set
        .recommendations
        .iter()
        .any(|rec| rec.recommendation_id == input.recommendation_id)
    {
        return Err(anyhow!("human approval references unknown recommendation"));
    }
    let approval = BalanceCopilotHumanApproval {
        schema_version: BALANCE_COPILOT_SCHEMA_VERSION.to_string(),
        approval_id: input.approval_id.to_string(),
        recommendation_set_id: set.recommendation_set_id.clone(),
        recommendation_id: input.recommendation_id.to_string(),
        decision: input.decision.to_string(),
        human_actor: input.human_actor.to_string(),
        rationale: input.rationale.to_string(),
        tweaked_action: input.tweaked_action.map(str::to_string),
        recorded_at_unix_ms: input.recorded_at_unix_ms,
        reverify_required: true,
        auto_apply_requested: false,
        trusted_write_authority: false,
        boundary: BALANCE_COPILOT_BOUNDARY.to_string(),
    };
    approval.validate()?;
    Ok(approval)
}

pub fn reverify_balance_copilot_approval(
    set: &BalanceCopilotRecommendationSet,
    approval: &BalanceCopilotHumanApproval,
    before: &DominantBuildReport,
    after: &DominantBuildReport,
    reverification_id: &str,
) -> Result<BalanceCopilotReverificationReport> {
    set.validate()?;
    approval.validate()?;
    require_text("reverification id", reverification_id)?;
    if approval.recommendation_set_id != set.recommendation_set_id {
        return Err(anyhow!("approval recommendationSetId does not match set"));
    }
    if approval.decision != "approved" && approval.decision != "approved-with-tweak" {
        return Err(anyhow!("reverification requires a human approved decision"));
    }
    if before.digest != set.source_report_digest
        || before.fixture_id != set.source_report_fixture_id
    {
        return Err(anyhow!(
            "before report does not match recommendation source report"
        ));
    }
    let recommendation = set
        .recommendations
        .iter()
        .find(|rec| rec.recommendation_id == approval.recommendation_id)
        .ok_or_else(|| anyhow!("approval references unknown recommendation"))?;
    let still_flagged = after
        .dominant_builds
        .iter()
        .any(|metric| metric.build_id == recommendation.target_build_id);
    let report = BalanceCopilotReverificationReport {
        schema_version: BALANCE_COPILOT_SCHEMA_VERSION.to_string(),
        reverification_id: reverification_id.to_string(),
        approval_id: approval.approval_id.clone(),
        recommendation_set_id: set.recommendation_set_id.clone(),
        recommendation_id: recommendation.recommendation_id.clone(),
        before_report_digest: before.digest.clone(),
        after_report_digest: after.digest.clone(),
        status: if still_flagged {
            BALANCE_COPILOT_STATUS_REVERIFIED_STILL_FLAGGED.to_string()
        } else {
            BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED.to_string()
        },
        dominant_build_still_flagged: still_flagged,
        remaining_dominant_build_count: after.dominant_builds.len(),
        human_approved: true,
        review_apply_required: true,
        auto_apply_performed: false,
        trusted_write_authority: false,
        boundary: BALANCE_COPILOT_BOUNDARY.to_string(),
    };
    report.validate()?;
    Ok(report)
}

fn recommendation_from_dominant_build(metric: &BuildMetric) -> BalanceCopilotRecommendation {
    BalanceCopilotRecommendation {
        recommendation_id: format!("balance-rec-dominant-{}", metric.build_id),
        finding_kind: "dominant-build".to_string(),
        target_build_id: metric.build_id.clone(),
        source_replay_deck_seed: metric.replay_deck_seed,
        source_replay_persona: metric.replay_persona.clone(),
        pick_rate_bps: metric.pick_rate_bps,
        win_rate_bps: metric.win_rate_bps,
        recommended_action: "reduce-dominant-build-synergy".to_string(),
        rationale: format!(
            "Dominant build {} was picked {} bps with {} bps win rate; propose a human-reviewed tuning draft and re-verify before any trusted apply.",
            metric.build_id, metric.pick_rate_bps, metric.win_rate_bps
        ),
        status: BALANCE_COPILOT_STATUS_REVERIFY_REQUIRED.to_string(),
        review_apply_required: true,
        auto_apply_allowed: false,
        trusted_write_authority: false,
    }
}

fn require_schema(actual: &str, label: &str) -> Result<()> {
    if actual != BALANCE_COPILOT_SCHEMA_VERSION {
        return Err(anyhow!(
            "balance co-pilot {label} schemaVersion must be {BALANCE_COPILOT_SCHEMA_VERSION}"
        ));
    }
    Ok(())
}

fn require_text(label: &str, text: &str) -> Result<()> {
    if text.trim().is_empty() {
        return Err(anyhow!("balance co-pilot {label} must be non-empty"));
    }
    Ok(())
}

fn require_boundary(boundary: &str) -> Result<()> {
    require_text("boundary", boundary)?;
    let lower = boundary.to_ascii_lowercase();
    for required in [
        "rust/local",
        "human approval required",
        "browser/studio read-only",
        "no auto-apply",
        "no auto-merge",
        "no fun",
        "#1 and #23 remain open",
    ] {
        if !lower.contains(required) {
            return Err(anyhow!("balance co-pilot boundary missing {required}"));
        }
    }
    Ok(())
}
