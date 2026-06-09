//! Self-improvement loop contract v1 (#2037 / Era L M71).
//!
//! This module specifies the re-verify and routing contract for a generated fix
//! proposal. It is a read-only contract over existing source-apply,
//! openchrome/evidence, trust-gradient, rollback, and kill-switch artifacts: it
//! does not run verification, apply patches, create a data plane, or bypass the
//! thin human go/no-go for high-risk/source-affecting changes.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::trust_gradient_auto_apply::AutoApplyOutcome;
use crate::{SourcePatchPreviewApplyStatus, SourcePatchPreviewRiskLevel};

pub const SELF_IMPROVEMENT_LOOP_CONTRACT_SCHEMA_VERSION: &str = "self-improvement-loop-contract-v1";
pub const SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION: &str = "self-improvement-routing-input-v1";
pub const SELF_IMPROVEMENT_ROUTING_DECISION_SCHEMA_VERSION: &str =
    "self-improvement-routing-decision-v1";
pub const SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION: &str =
    "self-improvement-apply-loop-input-v1";
pub const SELF_IMPROVEMENT_APPLY_LOOP_REPORT_SCHEMA_VERSION: &str =
    "self-improvement-apply-loop-report-v1";
pub const SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_INPUT_SCHEMA_VERSION: &str =
    "self-improvement-human-go-no-go-queue-input-v1";
pub const SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_ITEM_SCHEMA_VERSION: &str =
    "self-improvement-human-go-no-go-queue-item-v1";
pub const SELF_IMPROVEMENT_HUMAN_GO_NO_GO_DECISION_SCHEMA_VERSION: &str =
    "self-improvement-human-go-no-go-decision-v1";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfImprovementLoopContract {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "reverifyContract")]
    pub reverify_contract: SelfImprovementReverifyContract,
    #[serde(rename = "routingRules")]
    pub routing_rules: Vec<SelfImprovementRoutingRule>,
    #[serde(rename = "requiredPipelineRefs")]
    pub required_pipeline_refs: Vec<String>,
    pub boundary: String,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfImprovementReverifyContract {
    #[serde(rename = "openchromeRunCommand")]
    pub openchrome_run_command: String,
    #[serde(rename = "requiredGates")]
    pub required_gates: Vec<String>,
    #[serde(rename = "requiredEvidenceRefs")]
    pub required_evidence_refs: Vec<String>,
    #[serde(rename = "designIntegrityRequired")]
    pub design_integrity_required: bool,
    #[serde(rename = "rollbackRequiredBeforeApply")]
    pub rollback_required_before_apply: bool,
    #[serde(rename = "killSwitchRequired")]
    pub kill_switch_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfImprovementRoutingRule {
    pub route: SelfImprovementRoute,
    #[serde(rename = "whenAll")]
    pub when_all: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelfImprovementRoute {
    AutoApplyEligible,
    HumanGoNoGo,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfImprovementReverifyEvidence {
    #[serde(rename = "openchromeRerunRef")]
    pub openchrome_rerun_ref: String,
    #[serde(rename = "verdictRef")]
    pub verdict_ref: String,
    #[serde(rename = "journalRef")]
    pub journal_ref: String,
    #[serde(rename = "ledgerRef")]
    pub ledger_ref: String,
    #[serde(rename = "loopCoverageAttributionRef")]
    pub loop_coverage_attribution_ref: String,
    #[serde(rename = "sourceApplyRef")]
    pub source_apply_ref: String,
    #[serde(rename = "trustGradientRef")]
    pub trust_gradient_ref: String,
    #[serde(rename = "rollbackRef")]
    pub rollback_ref: String,
    #[serde(rename = "killSwitchRef")]
    pub kill_switch_ref: String,
    #[serde(rename = "mechanicalGatePassed")]
    pub mechanical_gate_passed: bool,
    #[serde(rename = "runtimeGatePassed")]
    pub runtime_gate_passed: bool,
    #[serde(rename = "visualGatePassed")]
    pub visual_gate_passed: bool,
    #[serde(rename = "semanticGatePassed")]
    pub semantic_gate_passed: bool,
    #[serde(rename = "designIntegrityPassed")]
    pub design_integrity_passed: bool,
    #[serde(rename = "noHumanInput")]
    pub no_human_input: bool,
    #[serde(rename = "noNewVerificationEngine")]
    pub no_new_verification_engine: bool,
    #[serde(rename = "noNewDataPlane")]
    pub no_new_data_plane: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfImprovementRoutingInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    #[serde(rename = "sourceApplyStatus")]
    pub source_apply_status: SourcePatchPreviewApplyStatus,
    #[serde(rename = "riskLevel")]
    pub risk_level: SourcePatchPreviewRiskLevel,
    #[serde(rename = "sourceAffecting")]
    pub source_affecting: bool,
    #[serde(rename = "reversible")]
    pub reversible: bool,
    #[serde(rename = "trustGradientOutcome")]
    pub trust_gradient_outcome: AutoApplyOutcome,
    #[serde(rename = "killSwitchEngaged")]
    pub kill_switch_engaged: bool,
    #[serde(rename = "reverifyEvidence")]
    pub reverify_evidence: SelfImprovementReverifyEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfImprovementRoutingDecision {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    pub route: SelfImprovementRoute,
    pub reasons: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfImprovementApplyLoopInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    #[serde(rename = "attributedMilestoneId")]
    pub attributed_milestone_id: String,
    #[serde(rename = "routingInput")]
    pub routing_input: SelfImprovementRoutingInput,
    #[serde(rename = "beforeEvidenceRefs")]
    pub before_evidence_refs: Vec<String>,
    #[serde(rename = "afterEvidenceRefs")]
    pub after_evidence_refs: Vec<String>,
    #[serde(rename = "beforeEvidenceScore")]
    pub before_evidence_score: u32,
    #[serde(rename = "afterEvidenceScore")]
    pub after_evidence_score: u32,
    #[serde(rename = "regressionDetected")]
    pub regression_detected: bool,
    #[serde(rename = "noHumanInput")]
    pub no_human_input: bool,
    #[serde(rename = "noNewDataPlane")]
    pub no_new_data_plane: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelfImprovementApplyLoopOutcome {
    AutoApplied,
    RejectedRolledBack,
    HumanGoNoGoQueued,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfImprovementApplyLoopReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    #[serde(rename = "attributedMilestoneId")]
    pub attributed_milestone_id: String,
    pub outcome: SelfImprovementApplyLoopOutcome,
    #[serde(rename = "routingDecision")]
    pub routing_decision: SelfImprovementRoutingDecision,
    #[serde(rename = "rollbackCommand")]
    pub rollback_command: Option<String>,
    #[serde(rename = "improvedAttributedMilestoneEvidence")]
    pub improved_attributed_milestone_evidence: bool,
    #[serde(rename = "sourceMutationApplied")]
    pub source_mutation_applied: bool,
    #[serde(rename = "humanInputRequired")]
    pub human_input_required: bool,
    pub reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfImprovementHumanGoNoGoQueueInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "applyLoopInput")]
    pub apply_loop_input: SelfImprovementApplyLoopInput,
    #[serde(rename = "queueProvenanceRef")]
    pub queue_provenance_ref: String,
    #[serde(rename = "unrelatedLoopWorkRefs")]
    pub unrelated_loop_work_refs: Vec<String>,
    #[serde(rename = "verificationStrengthScore")]
    pub verification_strength_score: u32,
    #[serde(rename = "broadeningThresholdScore")]
    pub broadening_threshold_score: u32,
    #[serde(rename = "oneClickOnly")]
    pub one_click_only: bool,
    #[serde(rename = "noDebuggingSession")]
    pub no_debugging_session: bool,
    #[serde(rename = "noNewDataPlane")]
    pub no_new_data_plane: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelfImprovementHumanGoNoGoQueueStatus {
    AwaitingDecision,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelfImprovementHumanGoNoGoDecision {
    Go,
    NoGo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfImprovementHumanGoNoGoQueueItem {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    #[serde(rename = "attributedMilestoneId")]
    pub attributed_milestone_id: String,
    pub status: SelfImprovementHumanGoNoGoQueueStatus,
    #[serde(rename = "verifiedReversibleHighRisk")]
    pub verified_reversible_high_risk: bool,
    #[serde(rename = "oneClickActions")]
    pub one_click_actions: Vec<String>,
    #[serde(rename = "queueProvenanceRef")]
    pub queue_provenance_ref: String,
    #[serde(rename = "decisionProvenanceRef")]
    pub decision_provenance_ref: Option<String>,
    #[serde(rename = "blocksUnrelatedLoopWork")]
    pub blocks_unrelated_loop_work: bool,
    #[serde(rename = "unrelatedLoopWorkRefs")]
    pub unrelated_loop_work_refs: Vec<String>,
    #[serde(rename = "broadeningHook")]
    pub broadening_hook: SelfImprovementHumanGateBroadeningHook,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfImprovementHumanGateBroadeningHook {
    #[serde(rename = "verificationStrengthScore")]
    pub verification_strength_score: u32,
    #[serde(rename = "broadeningThresholdScore")]
    pub broadening_threshold_score: u32,
    #[serde(rename = "eligibleToNarrowHumanGate")]
    pub eligible_to_narrow_human_gate: bool,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfImprovementHumanGoNoGoDecisionRecord {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    pub decision: SelfImprovementHumanGoNoGoDecision,
    #[serde(rename = "queueStatus")]
    pub queue_status: SelfImprovementHumanGoNoGoQueueStatus,
    #[serde(rename = "decisionProvenanceRef")]
    pub decision_provenance_ref: String,
    #[serde(rename = "oneClickOnly")]
    pub one_click_only: bool,
    #[serde(rename = "noDebuggingSession")]
    pub no_debugging_session: bool,
    #[serde(rename = "blocksUnrelatedLoopWork")]
    pub blocks_unrelated_loop_work: bool,
    pub boundary: String,
}

impl SelfImprovementLoopContract {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let contract: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse self-improvement loop contract: {err}"))?;
        contract.validate()?;
        Ok(contract)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_IMPROVEMENT_LOOP_CONTRACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-improvement loop contract schemaVersion must be {SELF_IMPROVEMENT_LOOP_CONTRACT_SCHEMA_VERSION}"
            ));
        }
        require_id("titleId", &self.title_id)?;
        self.reverify_contract.validate()?;
        require_nonempty("routingRules", self.routing_rules.len())?;
        let mut has_auto = false;
        let mut has_human = false;
        let mut has_blocked = false;
        for rule in &self.routing_rules {
            rule.validate()?;
            has_auto |= rule.route == SelfImprovementRoute::AutoApplyEligible;
            has_human |= rule.route == SelfImprovementRoute::HumanGoNoGo;
            has_blocked |= rule.route == SelfImprovementRoute::Blocked;
        }
        if !(has_auto && has_human && has_blocked) {
            return Err(anyhow!(
                "routingRules must include auto-apply-eligible, human-go-no-go, and blocked routes"
            ));
        }
        validate_pipeline_refs(&self.required_pipeline_refs)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "openchrome",
            "verdict",
            "journal.md",
            "ledger.jsonl",
            "loop-coverage",
            "source-apply",
            "trust-gradient",
            "rollback",
            "kill-switch",
            "no new verification engine",
            "no new data plane",
            "human go/no-go",
            "human ring 2",
            "#1 and #23 remain open",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!("boundary must mention {required}"));
            }
        }
        require_nonempty("guardrails", self.guardrails.len())?;
        Ok(())
    }
}

impl SelfImprovementReverifyContract {
    fn validate(&self) -> Result<()> {
        require_text("openchromeRunCommand", &self.openchrome_run_command)?;
        let command = self.openchrome_run_command.to_ascii_lowercase();
        for required in [
            "cargo run",
            "ouroforge-cli",
            "run",
            "dogfood-deckbuilder.yaml",
        ] {
            if !command.contains(required) {
                return Err(anyhow!("openchromeRunCommand must include {required}"));
            }
        }
        for required in [
            "mechanical",
            "runtime",
            "visual",
            "semantic",
            "design-integrity",
        ] {
            if !self
                .required_gates
                .iter()
                .any(|gate| gate.to_ascii_lowercase().contains(required))
            {
                return Err(anyhow!("requiredGates must include {required}"));
            }
        }
        validate_pipeline_refs(&self.required_evidence_refs)?;
        if !(self.design_integrity_required
            && self.rollback_required_before_apply
            && self.kill_switch_required)
        {
            return Err(anyhow!(
                "reverify contract must require design-integrity, rollback, and kill-switch"
            ));
        }
        Ok(())
    }
}

impl SelfImprovementRoutingRule {
    fn validate(&self) -> Result<()> {
        require_nonempty("routing rule whenAll", self.when_all.len())?;
        require_nonempty(
            "routing rule forbiddenActions",
            self.forbidden_actions.len(),
        )?;
        let text = self.when_all.join("\n").to_ascii_lowercase();
        match self.route {
            SelfImprovementRoute::AutoApplyEligible => {
                for required in ["low-risk", "reversible", "trust-gradient", "rollback"] {
                    if !text.contains(required) {
                        return Err(anyhow!("auto route must require {required}"));
                    }
                }
            }
            SelfImprovementRoute::HumanGoNoGo => {
                for required in ["high-risk", "source-affecting", "human go/no-go"] {
                    if !text.contains(required) {
                        return Err(anyhow!("human route must require {required}"));
                    }
                }
            }
            SelfImprovementRoute::Blocked => {
                for required in ["failed", "missing", "kill-switch"] {
                    if !text.contains(required) {
                        return Err(anyhow!("blocked route must mention {required}"));
                    }
                }
            }
        }
        let forbidden = self.forbidden_actions.join("\n").to_ascii_lowercase();
        for required in [
            "auto_merge",
            "bypass_gates",
            "new_verifier",
            "new_data_plane",
        ] {
            if !forbidden.contains(required) {
                return Err(anyhow!("routing forbiddenActions must include {required}"));
            }
        }
        Ok(())
    }
}

impl SelfImprovementReverifyEvidence {
    fn validate(&self) -> Result<()> {
        for (label, reference) in [
            ("openchromeRerunRef", &self.openchrome_rerun_ref),
            ("verdictRef", &self.verdict_ref),
            ("journalRef", &self.journal_ref),
            ("ledgerRef", &self.ledger_ref),
            (
                "loopCoverageAttributionRef",
                &self.loop_coverage_attribution_ref,
            ),
            ("sourceApplyRef", &self.source_apply_ref),
            ("trustGradientRef", &self.trust_gradient_ref),
            ("rollbackRef", &self.rollback_ref),
            ("killSwitchRef", &self.kill_switch_ref),
        ] {
            require_ref(label, reference)?;
        }
        validate_pipeline_refs(&[
            self.openchrome_rerun_ref.clone(),
            self.verdict_ref.clone(),
            self.journal_ref.clone(),
            self.ledger_ref.clone(),
            self.loop_coverage_attribution_ref.clone(),
            self.source_apply_ref.clone(),
            self.trust_gradient_ref.clone(),
            self.rollback_ref.clone(),
            self.kill_switch_ref.clone(),
        ])?;
        if !(self.no_human_input && self.no_new_verification_engine && self.no_new_data_plane) {
            return Err(anyhow!(
                "reverify evidence must preserve no human input, no new verification engine, and no new data plane"
            ));
        }
        Ok(())
    }

    fn all_gates_pass(&self) -> bool {
        self.mechanical_gate_passed
            && self.runtime_gate_passed
            && self.visual_gate_passed
            && self.semantic_gate_passed
            && self.design_integrity_passed
    }
}

impl SelfImprovementRoutingInput {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-improvement routing input schemaVersion must be {SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION}"
            ));
        }
        require_ref("proposalRef", &self.proposal_ref)?;
        self.reverify_evidence.validate()?;
        if self.source_apply_status != SourcePatchPreviewApplyStatus::Blocked {
            return Err(anyhow!(
                "source-apply preview status must remain blocked until routed through the existing apply path"
            ));
        }
        Ok(())
    }
}

pub fn route_self_improvement_fix(
    input: &SelfImprovementRoutingInput,
) -> Result<SelfImprovementRoutingDecision> {
    input.validate()?;
    let mut reasons = Vec::new();

    let route = if input.kill_switch_engaged {
        reasons.push("kill-switch engaged; no apply route is available".to_string());
        SelfImprovementRoute::Blocked
    } else if !input.reverify_evidence.all_gates_pass() {
        reasons.push(
            "re-verify evidence is missing or failed one of the four gates/design-integrity"
                .to_string(),
        );
        SelfImprovementRoute::Blocked
    } else if input.source_affecting
        || matches!(
            input.risk_level,
            SourcePatchPreviewRiskLevel::High | SourcePatchPreviewRiskLevel::Critical
        )
    {
        reasons.push(
            "high-risk/source-affecting proposal must queue for thin human go/no-go".to_string(),
        );
        SelfImprovementRoute::HumanGoNoGo
    } else if matches!(input.risk_level, SourcePatchPreviewRiskLevel::Low)
        && input.reversible
        && input.trust_gradient_outcome == AutoApplyOutcome::AutoApplied
    {
        reasons.push(
            "low-risk reversible proposal passed re-verify and trust-gradient auto-apply checks"
                .to_string(),
        );
        SelfImprovementRoute::AutoApplyEligible
    } else {
        reasons.push(
            "proposal is not both low-risk, reversible, and trust-gradient auto-apply eligible"
                .to_string(),
        );
        SelfImprovementRoute::HumanGoNoGo
    };

    let allowed_actions = match route {
        SelfImprovementRoute::AutoApplyEligible => vec![
            "route_to_existing_source_apply_transaction".to_string(),
            "record_existing_rollback_and_audit_refs".to_string(),
        ],
        SelfImprovementRoute::HumanGoNoGo => vec![
            "queue_thin_human_go_no_go".to_string(),
            "inspect_existing_evidence".to_string(),
        ],
        SelfImprovementRoute::Blocked => vec![
            "inspect_blocker_evidence".to_string(),
            "rerun_existing_openchrome_evidence_loop".to_string(),
        ],
    };

    Ok(SelfImprovementRoutingDecision {
        schema_version: SELF_IMPROVEMENT_ROUTING_DECISION_SCHEMA_VERSION.to_string(),
        proposal_ref: input.proposal_ref.clone(),
        route,
        reasons,
        allowed_actions,
        forbidden_actions: vec![
            "auto_merge".to_string(),
            "bypass_gates".to_string(),
            "bypass_source_apply".to_string(),
            "bypass_trust_gradient".to_string(),
            "self_apply_high_risk_source_change".to_string(),
            "create_new_verification_engine".to_string(),
            "create_new_data_plane".to_string(),
        ],
        boundary: "Read-only Era L M71 routing decision over existing openchrome verdict/journal.md/ledger.jsonl evidence, loop-coverage attribution, source-apply, trust-gradient, rollback, and kill-switch artifacts; no new verification engine, no new data plane, no auto-merge, high-risk/source-affecting tail keeps thin human go/no-go, fun/taste and release go/no-go remain human Ring 2, #1 and #23 remain open.".to_string(),
    })
}

impl SelfImprovementApplyLoopInput {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-improvement apply loop input schemaVersion must be {SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION}"
            ));
        }
        require_ref("proposalRef", &self.proposal_ref)?;
        require_id("attributedMilestoneId", &self.attributed_milestone_id)?;
        if self.proposal_ref != self.routing_input.proposal_ref {
            return Err(anyhow!(
                "apply loop proposalRef must match routingInput proposalRef"
            ));
        }
        self.routing_input.validate()?;
        validate_pipeline_refs(&self.before_evidence_refs)?;
        validate_pipeline_refs(&self.after_evidence_refs)?;
        if !(self.no_human_input && self.no_new_data_plane) {
            return Err(anyhow!(
                "apply loop must preserve no human input and no new data plane"
            ));
        }
        Ok(())
    }
}

pub fn run_self_improvement_reverify_apply_loop(
    input: &SelfImprovementApplyLoopInput,
) -> Result<SelfImprovementApplyLoopReport> {
    input.validate()?;
    let routing_decision = route_self_improvement_fix(&input.routing_input)?;
    let improved = input.after_evidence_score > input.before_evidence_score;
    let regression =
        input.regression_detected || input.after_evidence_score < input.before_evidence_score;
    let mut reasons = Vec::new();

    let (outcome, rollback_command, source_mutation_applied, human_input_required) = if regression {
        reasons.push(
            "post-apply re-verify detected regression; reject proposal and reuse existing rollback"
                .to_string(),
        );
        (
            SelfImprovementApplyLoopOutcome::RejectedRolledBack,
            Some(format!(
                "ouroforge rollback --proposal {} --evidence {}",
                input.proposal_ref, input.routing_input.reverify_evidence.rollback_ref
            )),
            false,
            false,
        )
    } else {
        match routing_decision.route {
            SelfImprovementRoute::AutoApplyEligible if improved => {
                reasons.push(
                    "low-risk reversible proposal re-verified and improved attributed milestone evidence"
                        .to_string(),
                );
                (
                    SelfImprovementApplyLoopOutcome::AutoApplied,
                    None,
                    true,
                    false,
                )
            }
            SelfImprovementRoute::HumanGoNoGo => {
                reasons.push(
                    "proposal queued for thin human go/no-go; no autonomous source mutation applied"
                        .to_string(),
                );
                (
                    SelfImprovementApplyLoopOutcome::HumanGoNoGoQueued,
                    None,
                    false,
                    true,
                )
            }
            SelfImprovementRoute::Blocked => {
                reasons.push("proposal blocked by re-verify/routing evidence".to_string());
                (SelfImprovementApplyLoopOutcome::Blocked, None, false, false)
            }
            SelfImprovementRoute::AutoApplyEligible => {
                reasons.push(
                    "auto-apply route requires improved attributed milestone evidence".to_string(),
                );
                (SelfImprovementApplyLoopOutcome::Blocked, None, false, false)
            }
        }
    };

    let report = SelfImprovementApplyLoopReport {
        schema_version: SELF_IMPROVEMENT_APPLY_LOOP_REPORT_SCHEMA_VERSION.to_string(),
        proposal_ref: input.proposal_ref.clone(),
        attributed_milestone_id: input.attributed_milestone_id.clone(),
        outcome,
        routing_decision,
        rollback_command,
        improved_attributed_milestone_evidence: improved && outcome == SelfImprovementApplyLoopOutcome::AutoApplied,
        source_mutation_applied,
        human_input_required,
        reasons,
        boundary: "Deterministic Era L M71 re-verify/apply-loop report over existing openchrome verdict/journal.md/ledger.jsonl evidence, loop-coverage attribution, source-apply, trust-gradient, rollback, and kill-switch artifacts; no human input for low-risk reversible auto-apply, high-risk/source-affecting changes queue for thin human go/no-go, regressions are rejected/rolled back, no new verification engine, no new data plane, no new store, #1 and #23 remain open.".to_string(),
    };
    validate_apply_loop_report(&report)?;
    Ok(report)
}

impl SelfImprovementHumanGoNoGoQueueInput {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_INPUT_SCHEMA_VERSION {
            return Err(anyhow!(
                "human go/no-go queue input schemaVersion must be {SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_INPUT_SCHEMA_VERSION}"
            ));
        }
        require_ref("queueProvenanceRef", &self.queue_provenance_ref)?;
        validate_pipeline_refs(&self.unrelated_loop_work_refs)?;
        if !(self.one_click_only && self.no_debugging_session && self.no_new_data_plane) {
            return Err(anyhow!(
                "human go/no-go queue must be one-click provenance only, not a debugging session or new data plane"
            ));
        }
        self.apply_loop_input.validate()?;
        Ok(())
    }
}

pub fn queue_self_improvement_human_go_no_go(
    input: &SelfImprovementHumanGoNoGoQueueInput,
) -> Result<SelfImprovementHumanGoNoGoQueueItem> {
    input.validate()?;
    let apply_report = run_self_improvement_reverify_apply_loop(&input.apply_loop_input)?;
    let routing = &input.apply_loop_input.routing_input;
    let verified_reversible_high_risk = apply_report.outcome
        == SelfImprovementApplyLoopOutcome::HumanGoNoGoQueued
        && routing.reversible
        && routing.reverify_evidence.all_gates_pass()
        && (routing.source_affecting
            || matches!(
                routing.risk_level,
                SourcePatchPreviewRiskLevel::High | SourcePatchPreviewRiskLevel::Critical
            ));
    if !verified_reversible_high_risk {
        return Err(anyhow!(
            "human go/no-go queue accepts only verified, reversible high-risk/source-affecting proposals"
        ));
    }

    let item = SelfImprovementHumanGoNoGoQueueItem {
        schema_version: SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_ITEM_SCHEMA_VERSION.to_string(),
        proposal_ref: apply_report.proposal_ref,
        attributed_milestone_id: apply_report.attributed_milestone_id,
        status: SelfImprovementHumanGoNoGoQueueStatus::AwaitingDecision,
        verified_reversible_high_risk,
        one_click_actions: vec!["go".to_string(), "no-go".to_string()],
        queue_provenance_ref: input.queue_provenance_ref.clone(),
        decision_provenance_ref: None,
        blocks_unrelated_loop_work: false,
        unrelated_loop_work_refs: input.unrelated_loop_work_refs.clone(),
        broadening_hook: SelfImprovementHumanGateBroadeningHook {
            verification_strength_score: input.verification_strength_score,
            broadening_threshold_score: input.broadening_threshold_score,
            eligible_to_narrow_human_gate: input.verification_strength_score
                >= input.broadening_threshold_score,
            note: "M44 broadening hook: as recorded verification strength rises, future trust-gradient policy can narrow the thin human tail without changing the evidence pipeline".to_string(),
        },
        boundary: "Thin Era L M71 human go/no-go queue item over existing verdict/journal.md/ledger.jsonl evidence, loop-coverage attribution, source-apply, trust-gradient, rollback, and kill-switch artifacts; one-click go/no-go provenance only, no debugging session, autonomous loop continues unrelated work without waiting, no new verification engine, no new data plane, no new store, #1 and #23 remain open.".to_string(),
    };
    validate_human_go_no_go_queue_item(&item)?;
    Ok(item)
}

pub fn record_self_improvement_human_go_no_go_decision(
    item: &SelfImprovementHumanGoNoGoQueueItem,
    decision: SelfImprovementHumanGoNoGoDecision,
    decision_provenance_ref: &str,
) -> Result<SelfImprovementHumanGoNoGoDecisionRecord> {
    validate_human_go_no_go_queue_item(item)?;
    require_ref("decisionProvenanceRef", decision_provenance_ref)?;
    if item.status != SelfImprovementHumanGoNoGoQueueStatus::AwaitingDecision {
        return Err(anyhow!(
            "human go/no-go decision can only record against an awaiting queue item"
        ));
    }
    let queue_status = match decision {
        SelfImprovementHumanGoNoGoDecision::Go => SelfImprovementHumanGoNoGoQueueStatus::Accepted,
        SelfImprovementHumanGoNoGoDecision::NoGo => SelfImprovementHumanGoNoGoQueueStatus::Rejected,
    };
    let record = SelfImprovementHumanGoNoGoDecisionRecord {
        schema_version: SELF_IMPROVEMENT_HUMAN_GO_NO_GO_DECISION_SCHEMA_VERSION.to_string(),
        proposal_ref: item.proposal_ref.clone(),
        decision,
        queue_status,
        decision_provenance_ref: decision_provenance_ref.to_string(),
        one_click_only: true,
        no_debugging_session: true,
        blocks_unrelated_loop_work: false,
        boundary: "One-click human go/no-go decision provenance over the existing source-apply/trust-gradient queue; records go/no-go only, does not become debugging, does not block unrelated autonomous loop work, and introduces no new verification engine or data plane.".to_string(),
    };
    validate_human_go_no_go_decision_record(&record)?;
    Ok(record)
}

fn validate_human_go_no_go_queue_item(item: &SelfImprovementHumanGoNoGoQueueItem) -> Result<()> {
    if item.schema_version != SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_ITEM_SCHEMA_VERSION {
        return Err(anyhow!(
            "human go/no-go queue item schemaVersion must be {SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_ITEM_SCHEMA_VERSION}"
        ));
    }
    require_ref("proposalRef", &item.proposal_ref)?;
    require_id("attributedMilestoneId", &item.attributed_milestone_id)?;
    require_ref("queueProvenanceRef", &item.queue_provenance_ref)?;
    validate_pipeline_refs(&item.unrelated_loop_work_refs)?;
    if !item.verified_reversible_high_risk {
        return Err(anyhow!(
            "queue item must be verified, reversible, and high risk/source-affecting"
        ));
    }
    if item.one_click_actions != ["go".to_string(), "no-go".to_string()] {
        return Err(anyhow!(
            "queue item must expose exactly one-click go/no-go actions"
        ));
    }
    if item.blocks_unrelated_loop_work {
        return Err(anyhow!(
            "human queue must not block unrelated autonomous loop work"
        ));
    }
    let boundary = item.boundary.to_ascii_lowercase();
    for required in [
        "one-click",
        "go/no-go",
        "provenance",
        "no debugging session",
        "continues unrelated work",
        "verdict",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "rollback",
        "kill-switch",
        "no new verification engine",
        "no new data plane",
        "no new store",
        "#1 and #23 remain open",
    ] {
        if !boundary.contains(required) {
            return Err(anyhow!("human queue boundary must mention {required}"));
        }
    }
    Ok(())
}

fn validate_human_go_no_go_decision_record(
    record: &SelfImprovementHumanGoNoGoDecisionRecord,
) -> Result<()> {
    if record.schema_version != SELF_IMPROVEMENT_HUMAN_GO_NO_GO_DECISION_SCHEMA_VERSION {
        return Err(anyhow!(
            "human go/no-go decision schemaVersion must be {SELF_IMPROVEMENT_HUMAN_GO_NO_GO_DECISION_SCHEMA_VERSION}"
        ));
    }
    require_ref("proposalRef", &record.proposal_ref)?;
    require_ref("decisionProvenanceRef", &record.decision_provenance_ref)?;
    if !(record.one_click_only && record.no_debugging_session) {
        return Err(anyhow!(
            "decision record must be one-click provenance, not debugging"
        ));
    }
    if record.blocks_unrelated_loop_work {
        return Err(anyhow!(
            "decision record must not block unrelated autonomous loop work"
        ));
    }
    Ok(())
}

fn validate_apply_loop_report(report: &SelfImprovementApplyLoopReport) -> Result<()> {
    if report.schema_version != SELF_IMPROVEMENT_APPLY_LOOP_REPORT_SCHEMA_VERSION {
        return Err(anyhow!(
            "self-improvement apply loop report schemaVersion must be {SELF_IMPROVEMENT_APPLY_LOOP_REPORT_SCHEMA_VERSION}"
        ));
    }
    if report.outcome == SelfImprovementApplyLoopOutcome::AutoApplied
        && !report.improved_attributed_milestone_evidence
    {
        return Err(anyhow!(
            "auto-applied fixes must improve the attributed milestone evidence"
        ));
    }
    if report.outcome == SelfImprovementApplyLoopOutcome::RejectedRolledBack
        && report.rollback_command.is_none()
    {
        return Err(anyhow!("rejected regressions must record rollback command"));
    }
    if report.routing_decision.route == SelfImprovementRoute::HumanGoNoGo
        && report.source_mutation_applied
    {
        return Err(anyhow!(
            "human go/no-go route must not apply source mutations autonomously"
        ));
    }
    let boundary = report.boundary.to_ascii_lowercase();
    for required in [
        "openchrome",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "rollback",
        "kill-switch",
        "no human input",
        "human go/no-go",
        "no new verification engine",
        "no new data plane",
        "#1 and #23 remain open",
    ] {
        if !boundary.contains(required) {
            return Err(anyhow!("apply loop boundary must mention {required}"));
        }
    }
    Ok(())
}

fn validate_pipeline_refs(refs: &[String]) -> Result<()> {
    require_nonempty("pipeline refs", refs.len())?;
    let refs = refs.join("\n").to_ascii_lowercase();
    for required in [
        "openchrome",
        "verdict",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "rollback",
        "kill-switch",
    ] {
        if !refs.contains(required) {
            return Err(anyhow!("pipeline refs must include {required}"));
        }
    }
    Ok(())
}

fn require_id(label: &str, value: &str) -> Result<()> {
    let valid = !value.trim().is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'));
    if !valid {
        return Err(anyhow!("{label} must be a non-empty local id"));
    }
    Ok(())
}

fn require_ref(label: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.starts_with('/')
        || trimmed.contains("..")
        || trimmed.contains('\\')
        || trimmed.contains("http://")
        || trimmed.contains("https://")
    {
        return Err(anyhow!("{label} must be a safe local/evidence ref"));
    }
    Ok(())
}

fn require_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    Ok(())
}

fn require_nonempty(label: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{label} must not be empty"));
    }
    Ok(())
}
