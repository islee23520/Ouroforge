//! Proposal Quality Gate v1 (#2377, #1 M125).
//!
//! Rule-based, inspectable gate for evidence-linked proposals. The gate consumes
//! proposal workbench JSON as data and never delegates pass/fail authority to an
//! LLM. Every rule emits a stable id and actionable message for UI/reporting.

use crate::proposal_workbench_model::ProposalWorkbenchModel;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

pub const PROPOSAL_QUALITY_GATE_SCHEMA_VERSION: &str = "proposal-quality-gate-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ProposalQualityGateRule {
    EvidenceRequired,
    SupportedFileClass,
    BoundedScope,
    RollbackRequired,
    ExpectedImpactRequired,
    NoSelfApproval,
    NoHiddenAuthority,
}

impl ProposalQualityGateRule {
    pub fn id(self) -> &'static str {
        match self {
            Self::EvidenceRequired => "evidence-required",
            Self::SupportedFileClass => "supported-file-class",
            Self::BoundedScope => "bounded-scope",
            Self::RollbackRequired => "rollback-required",
            Self::ExpectedImpactRequired => "expected-impact-required",
            Self::NoSelfApproval => "no-self-approval",
            Self::NoHiddenAuthority => "no-hidden-authority",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::EvidenceRequired => "proposal must cite observed evidence refs",
            Self::SupportedFileClass => "proposal target paths must stay in scene/asset/behavior data",
            Self::BoundedScope => "proposal scope must have a small bounded change count and few targets",
            Self::RollbackRequired => "proposal must include rollback plan and rollback refs",
            Self::ExpectedImpactRequired => "proposal must state an evidence-linked expected impact",
            Self::NoSelfApproval => "proposal must not request self-approval or reviewer bypass",
            Self::NoHiddenAuthority => "proposal must not request hidden commands, network, installs, deploys, or auto-apply",
        }
    }
}

pub fn proposal_quality_gate_rules() -> Vec<ProposalQualityGateRule> {
    vec![
        ProposalQualityGateRule::EvidenceRequired,
        ProposalQualityGateRule::SupportedFileClass,
        ProposalQualityGateRule::BoundedScope,
        ProposalQualityGateRule::RollbackRequired,
        ProposalQualityGateRule::ExpectedImpactRequired,
        ProposalQualityGateRule::NoSelfApproval,
        ProposalQualityGateRule::NoHiddenAuthority,
    ]
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProposalQualityGateStatus {
    Passed,
    Failed,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProposalQualityGateFinding {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    pub message: String,
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProposalQualityGateReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    pub status: ProposalQualityGateStatus,
    #[serde(rename = "ruleCatalog")]
    pub rule_catalog: Vec<ProposalQualityRuleCatalogEntry>,
    pub findings: Vec<ProposalQualityGateFinding>,
    #[serde(rename = "llmSoleGate")]
    pub llm_sole_gate: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProposalQualityRuleCatalogEntry {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    pub description: String,
}

impl ProposalQualityGateReport {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PROPOSAL_QUALITY_GATE_SCHEMA_VERSION {
            anyhow::bail!(
                "proposal quality gate schemaVersion must be {PROPOSAL_QUALITY_GATE_SCHEMA_VERSION}"
            );
        }
        if self.llm_sole_gate {
            anyhow::bail!("proposal quality gate must not use LLM judgment as the sole gate");
        }
        Ok(())
    }
}

pub fn evaluate_proposal_quality_gate_json(input: &str) -> Result<ProposalQualityGateReport> {
    let value: Value = serde_json::from_str(input).context("failed to parse proposal JSON")?;
    Ok(evaluate_proposal_quality_gate_value(&value))
}

pub fn evaluate_proposal_quality_gate_value(value: &Value) -> ProposalQualityGateReport {
    let proposal_id = string_at(value, &["proposalId"])
        .or_else(|| string_at(value, &["proposal_id"]))
        .unwrap_or_else(|| "unknown-proposal".to_string());
    let mut findings = Vec::new();
    rule_evidence_required(value, &mut findings);
    rule_supported_file_class(value, &mut findings);
    rule_bounded_scope(value, &mut findings);
    rule_rollback_required(value, &mut findings);
    rule_expected_impact_required(value, &mut findings);
    rule_forbidden_text(
        value,
        ProposalQualityGateRule::NoSelfApproval,
        &[
            "self-approval",
            "self approval",
            "self-approve",
            "self approve",
            "reviewer bypass",
        ],
        &mut findings,
    );
    rule_forbidden_text(
        value,
        ProposalQualityGateRule::NoHiddenAuthority,
        &[
            "hidden command",
            "command bridge",
            "dependency install",
            "npm install",
            "cargo add",
            "network request",
            "http://",
            "https://",
            "auto-apply",
            "auto apply",
            "auto-merge",
            "publish",
            "deploy",
            "upload",
        ],
        &mut findings,
    );

    if ProposalWorkbenchModel::from_json_str(&value.to_string()).is_err() {
        // Keep the report actionable without relying on the model parser as the
        // sole gate; the explicit rules above still enumerate individual causes.
        if findings.is_empty() {
            findings.push(finding(
                ProposalQualityGateRule::BoundedScope,
                "proposal workbench model validation failed; inspect schema and field names",
                "$",
            ));
        }
    }

    ProposalQualityGateReport {
        schema_version: PROPOSAL_QUALITY_GATE_SCHEMA_VERSION.to_string(),
        proposal_id,
        status: if findings.is_empty() {
            ProposalQualityGateStatus::Passed
        } else {
            ProposalQualityGateStatus::Failed
        },
        rule_catalog: proposal_quality_gate_rules()
            .into_iter()
            .map(|rule| ProposalQualityRuleCatalogEntry {
                rule_id: rule.id().to_string(),
                description: rule.description().to_string(),
            })
            .collect(),
        findings,
        llm_sole_gate: false,
    }
}

fn rule_evidence_required(value: &Value, findings: &mut Vec<ProposalQualityGateFinding>) {
    if array_at(value, &["problemEvidenceRefs"]).is_empty()
        && array_at(value, &["problem_evidence_refs"]).is_empty()
    {
        findings.push(finding(
            ProposalQualityGateRule::EvidenceRequired,
            "problemEvidenceRefs must include at least one observed artifact",
            "problemEvidenceRefs",
        ));
    }
}

fn rule_supported_file_class(value: &Value, findings: &mut Vec<ProposalQualityGateFinding>) {
    for path in target_paths(value) {
        if blocked_target_path(&path) {
            findings.push(finding(
                ProposalQualityGateRule::SupportedFileClass,
                format!("target path `{path}` is outside scene/asset/behavior proposal data"),
                "diffScope.targetPaths",
            ));
        }
    }
}

fn rule_bounded_scope(value: &Value, findings: &mut Vec<ProposalQualityGateFinding>) {
    let paths = target_paths(value);
    if paths.is_empty() || paths.len() > 3 {
        findings.push(finding(
            ProposalQualityGateRule::BoundedScope,
            "proposal must target between 1 and 3 files",
            "diffScope.targetPaths",
        ));
    }
    let mut seen = BTreeSet::new();
    for path in &paths {
        if !seen.insert(path.clone()) {
            findings.push(finding(
                ProposalQualityGateRule::BoundedScope,
                format!("duplicate target path `{path}`"),
                "diffScope.targetPaths",
            ));
        }
    }
    let change_count = number_at(value, &["diffScope", "boundedChangeCount"])
        .or_else(|| number_at(value, &["diff_scope", "bounded_change_count"]))
        .unwrap_or(0);
    if change_count == 0 || change_count > 20 {
        findings.push(finding(
            ProposalQualityGateRule::BoundedScope,
            "boundedChangeCount must be between 1 and 20",
            "diffScope.boundedChangeCount",
        ));
    }
}

fn rule_rollback_required(value: &Value, findings: &mut Vec<ProposalQualityGateFinding>) {
    let plan = string_at(value, &["rollback", "rollbackPlan"])
        .or_else(|| string_at(value, &["rollback", "rollback_plan"]))
        .unwrap_or_default();
    let refs = array_at(value, &["rollback", "rollbackRefs"]);
    let refs_snake = array_at(value, &["rollback", "rollback_refs"]);
    if plan.trim().is_empty() || (refs.is_empty() && refs_snake.is_empty()) {
        findings.push(finding(
            ProposalQualityGateRule::RollbackRequired,
            "rollback plan and rollback refs are required",
            "rollback",
        ));
    }
}

fn rule_expected_impact_required(value: &Value, findings: &mut Vec<ProposalQualityGateFinding>) {
    let impact = string_at(value, &["expectedImpact"])
        .or_else(|| string_at(value, &["expected_impact"]))
        .unwrap_or_default();
    if impact.trim().is_empty() {
        findings.push(finding(
            ProposalQualityGateRule::ExpectedImpactRequired,
            "expectedImpact must describe the evidence-linked product effect",
            "expectedImpact",
        ));
    }
}

fn rule_forbidden_text(
    value: &Value,
    rule: ProposalQualityGateRule,
    terms: &[&str],
    findings: &mut Vec<ProposalQualityGateFinding>,
) {
    let text = value.to_string().to_ascii_lowercase();
    for term in terms {
        if text.contains(term) {
            findings.push(finding(
                rule,
                format!("proposal text contains forbidden authority `{term}`"),
                "$",
            ));
            return;
        }
    }
}

fn target_paths(value: &Value) -> Vec<String> {
    let mut paths = array_strings_at(value, &["diffScope", "targetPaths"]);
    paths.extend(array_strings_at(value, &["diff_scope", "target_paths"]));
    paths
}

fn blocked_target_path(path: &str) -> bool {
    path.starts_with(".github/")
        || path.starts_with("scripts/")
        || path.starts_with("crates/")
        || path.starts_with(".git/")
        || path.contains("Cargo.toml")
        || path.contains("Cargo.lock")
        || path.contains("package.json")
        || path.contains("runtime.js")
        || path.contains("cockpit.js")
        || path.contains("..")
        || path.starts_with('/')
}

fn string_at(value: &Value, path: &[&str]) -> Option<String> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str().map(ToString::to_string)
}

fn array_at<'a>(value: &'a Value, path: &[&str]) -> Vec<&'a Value> {
    let mut current = value;
    for key in path {
        let Some(next) = current.get(*key) else {
            return Vec::new();
        };
        current = next;
    }
    current
        .as_array()
        .map(|items| items.iter().collect())
        .unwrap_or_default()
}

fn array_strings_at(value: &Value, path: &[&str]) -> Vec<String> {
    array_at(value, path)
        .into_iter()
        .filter_map(|value| value.as_str().map(ToString::to_string))
        .collect()
}

fn number_at(value: &Value, path: &[&str]) -> Option<u64> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_u64()
}

fn finding(
    rule: ProposalQualityGateRule,
    message: impl Into<String>,
    field_path: impl Into<String>,
) -> ProposalQualityGateFinding {
    ProposalQualityGateFinding {
        rule_id: rule.id().to_string(),
        message: message.into(),
        field_path: field_path.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_proposal_passes_every_rule_without_llm_gate() {
        let report = evaluate_proposal_quality_gate_json(ACCEPTED).unwrap();
        report.validate().unwrap();
        assert_eq!(report.status, ProposalQualityGateStatus::Passed);
        assert_eq!(report.findings.len(), 0);
        assert!(!report.llm_sole_gate);
        assert_eq!(
            report.rule_catalog.len(),
            proposal_quality_gate_rules().len()
        );
    }

    #[test]
    fn vague_or_unsafe_proposal_fails_with_actionable_rule() {
        let mut value: Value = serde_json::from_str(ACCEPTED).unwrap();
        value["expectedImpact"] = Value::String(String::new());
        value["hypothesis"] = Value::String("use hidden command to install dependency".to_string());
        let report = evaluate_proposal_quality_gate_value(&value);
        assert_eq!(report.status, ProposalQualityGateStatus::Failed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.rule_id == "expected-impact-required"));
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.rule_id == "no-hidden-authority"));
    }

    const ACCEPTED: &str = r#"{
      "schemaVersion": "proposal-workbench-v1",
      "proposalId": "proposal-quality-good",
      "category": "gameplay",
      "problemEvidenceRefs": [{ "runId": "run-before", "path": "runs/session-h-2377/evidence/world-state.json", "digest": "sha256:world" }],
      "hypothesis": "hazard threshold can be made readable with one bounded behavior-data change",
      "diffScope": { "kind": "behavior-data", "targetPaths": ["examples/playable-demo-v2/collect-and-exit/behavior.patch.json"], "operationSummary": "adjust hazard threshold data", "boundedChangeCount": 1 },
      "expectedImpact": "replay evidence should show the hazard warning before failure",
      "risk": { "level": "low", "rationale": "single behavior-data target", "mitigations": ["review before apply"] },
      "rollback": { "rollbackPlan": "restore previous behavior data", "rollbackRefs": [{ "runId": "run-before", "path": "runs/session-h-2377/rollback/behavior.json", "digest": "sha256:rollback" }] },
      "reviewerRequirements": ["reviewer verifies before-after evidence"],
      "nonGoals": ["no runtime code changes"],
      "guardrails": ["Safe Source Apply review required"]
    }"#;
}
