//! Self-audit attribution + acceptance demo v1 (#2031 / Era L M69).
//!
//! This is a thin composition layer over the existing self-audit attribution
//! contract, bottleneck attribution, and acceptance evaluator. It demonstrates a
//! planted defect by feeding already-collected dogfood evidence and in-memory
//! planted evidence into the existing read models. It does not run a new
//! verifier, persist a new store, or grant source-apply authority.

use anyhow::{anyhow, Result};
use serde::Serialize;

use crate::{
    attribute_self_audit_bottlenecks, evaluate_self_audit_acceptance, SelfAuditAcceptanceReport,
    SelfAuditAcceptanceStatus, SelfAuditAttributionContract, SelfAuditBottleneckInput,
    SelfAuditBottleneckReport, SelfAuditEvidenceDocument,
};

pub const SELF_AUDIT_DEMO_SCHEMA_VERSION: &str = "self-audit-demo-v1";

const LOOP_STAGES: &[&str] = &[
    "detect",
    "explain",
    "trace",
    "attribute",
    "propose",
    "re-verify",
    "apply-or-queue",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfAuditDemoReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "plantedDefectId")]
    pub planted_defect_id: String,
    #[serde(rename = "autonomousLoopStages")]
    pub autonomous_loop_stages: Vec<String>,
    #[serde(rename = "bottleneckAttribution")]
    pub bottleneck_attribution: SelfAuditBottleneckReport,
    #[serde(rename = "acceptanceAudit")]
    pub acceptance_audit: SelfAuditAcceptanceReport,
    #[serde(rename = "outputSummary")]
    pub output_summary: SelfAuditDemoSummary,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfAuditDemoSummary {
    #[serde(rename = "topBottleneckMilestoneId")]
    pub top_bottleneck_milestone_id: String,
    #[serde(rename = "topBottleneckIssueRef")]
    pub top_bottleneck_issue_ref: String,
    #[serde(rename = "acceptanceStatus")]
    pub acceptance_status: SelfAuditAcceptanceStatus,
    #[serde(rename = "regressedMilestoneIds")]
    pub regressed_milestone_ids: Vec<String>,
    #[serde(rename = "humanInputRequired")]
    pub human_input_required: bool,
    #[serde(rename = "highRiskAutoApplied")]
    pub high_risk_auto_applied: bool,
    #[serde(rename = "newVerificationEngine")]
    pub new_verification_engine: bool,
    #[serde(rename = "newDataPlane")]
    pub new_data_plane: bool,
}

pub fn run_self_audit_demo(
    contract: &SelfAuditAttributionContract,
    bottleneck_input: &SelfAuditBottleneckInput,
    acceptance_evidence: &[SelfAuditEvidenceDocument],
    planted_defect_id: &str,
) -> Result<SelfAuditDemoReport> {
    if planted_defect_id.trim().is_empty() {
        return Err(anyhow!("plantedDefectId must not be empty"));
    }

    let ranked_bottlenecks = attribute_self_audit_bottlenecks(contract, bottleneck_input)?;
    let acceptance_audit = evaluate_self_audit_acceptance(contract, acceptance_evidence)?;
    let top = ranked_bottlenecks
        .ranked_bottlenecks
        .first()
        .ok_or_else(|| anyhow!("self-audit demo requires at least one attributed bottleneck"))?;

    let regressed_milestone_ids = acceptance_audit
        .milestone_verdicts
        .iter()
        .filter(|verdict| verdict.status == SelfAuditAcceptanceStatus::Regressed)
        .map(|verdict| verdict.milestone_id.clone())
        .collect::<Vec<_>>();

    let summary = SelfAuditDemoSummary {
        top_bottleneck_milestone_id: top.milestone_id.clone(),
        top_bottleneck_issue_ref: top.issue_ref.clone(),
        acceptance_status: acceptance_audit.status.clone(),
        regressed_milestone_ids,
        human_input_required: false,
        high_risk_auto_applied: false,
        new_verification_engine: false,
        new_data_plane: false,
    };

    Ok(SelfAuditDemoReport {
        schema_version: SELF_AUDIT_DEMO_SCHEMA_VERSION.to_string(),
        title_id: contract.title_id.clone(),
        planted_defect_id: planted_defect_id.to_string(),
        autonomous_loop_stages: LOOP_STAGES.iter().map(|stage| (*stage).to_string()).collect(),
        bottleneck_attribution: ranked_bottlenecks,
        acceptance_audit,
        output_summary: summary,
        boundary: "Thin read-only demo over existing openchrome/scenario verdicts, four gates plus design-integrity, journal.md, ledger.jsonl, loop-coverage attribution, evolve, source-apply, and trust-gradient evidence; no new verification engine and no new data plane; autonomous path completes with zero human input; high-risk/source-affecting fixes are never auto-applied and stay queued for the thin human go/no-go; fun/taste and release go/no-go remain human Ring 2.".to_string(),
    })
}
