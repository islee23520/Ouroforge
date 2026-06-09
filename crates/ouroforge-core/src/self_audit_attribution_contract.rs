//! Self-audit bottleneck attribution contract v1 (#2028 / Era L M69).
//!
//! This is a machine-checkable contract for mapping real-build evidence to
//! milestones, gates, and #1 success criteria. It extends loop-coverage
//! attribution by reference; it does not run a new verifier, store telemetry, or
//! decide fun/taste/release go/no-go.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::loop_coverage_attribution::LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION;

pub const SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION: &str =
    "self-audit-attribution-contract-v1";

const REQUIRED_PIPELINE_REFS: &[&str] = &[
    "openchrome",
    "scenario-verdicts",
    "four-gates",
    "design-integrity",
    "journal.md",
    "ledger.jsonl",
    "loop-coverage-attribution",
    "evolve",
    "source-apply",
    "trust-gradient",
];

const REQUIRED_LOOP_STAGES: &[&str] = &[
    "detect",
    "explain",
    "trace",
    "attribute",
    "propose",
    "re-verify",
    "apply-or-queue",
];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelfAuditTrendDirection {
    Improved,
    Unchanged,
    Regressed,
    InsufficientEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelfAuditAcceptanceStatus {
    Satisfied,
    Unsatisfied,
    Regressed,
    InsufficientEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfAuditAttributionContract {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "extendsSchemaVersion")]
    pub extends_schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "anchorIssueRefs")]
    pub anchor_issue_refs: Vec<String>,
    #[serde(rename = "evidencePipeline")]
    pub evidence_pipeline: Vec<String>,
    #[serde(rename = "loopStages")]
    pub loop_stages: Vec<String>,
    #[serde(rename = "milestoneMappings")]
    pub milestone_mappings: Vec<SelfAuditMilestoneMapping>,
    #[serde(rename = "acceptanceAudits")]
    pub acceptance_audits: Vec<SelfAuditAcceptanceAudit>,
    #[serde(rename = "trendDefinitions")]
    pub trend_definitions: Vec<SelfAuditTrendDefinition>,
    pub boundary: String,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfAuditMilestoneMapping {
    #[serde(rename = "milestoneId")]
    pub milestone_id: String,
    #[serde(rename = "issueRef")]
    pub issue_ref: String,
    #[serde(rename = "gateKind")]
    pub gate_kind: String,
    #[serde(rename = "loopStage")]
    pub loop_stage: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "attributionRefs")]
    pub attribution_refs: Vec<String>,
    #[serde(rename = "failureSignalKinds")]
    pub failure_signal_kinds: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfAuditAcceptanceAudit {
    #[serde(rename = "milestoneId")]
    pub milestone_id: String,
    #[serde(rename = "successCriterionRef")]
    pub success_criterion_ref: String,
    #[serde(rename = "evidencePredicates")]
    pub evidence_predicates: Vec<SelfAuditEvidencePredicate>,
    #[serde(rename = "status")]
    pub status: SelfAuditAcceptanceStatus,
    #[serde(rename = "trendRef")]
    pub trend_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfAuditEvidencePredicate {
    #[serde(rename = "sourceRef")]
    pub source_ref: String,
    pub path: String,
    pub operator: String,
    #[serde(rename = "expectedValue")]
    pub expected_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfAuditTrendDefinition {
    #[serde(rename = "trendId")]
    pub trend_id: String,
    pub direction: SelfAuditTrendDirection,
    #[serde(rename = "baselineRef")]
    pub baseline_ref: String,
    #[serde(rename = "currentRef")]
    pub current_ref: String,
    #[serde(rename = "regressionWhen")]
    pub regression_when: Vec<String>,
}

impl SelfAuditAttributionContract {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let contract: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse self-audit attribution contract: {err}"))?;
        contract.validate()?;
        Ok(contract)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-audit attribution schemaVersion must be {SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION}"
            ));
        }
        if self.extends_schema_version != LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION {
            return Err(anyhow!(
                "self-audit attribution must extend {LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION}"
            ));
        }
        require_id("titleId", &self.title_id)?;
        require_contains_all(
            "evidencePipeline",
            &self.evidence_pipeline,
            REQUIRED_PIPELINE_REFS,
        )?;
        require_contains_all("loopStages", &self.loop_stages, REQUIRED_LOOP_STAGES)?;
        require_contains_all("anchorIssueRefs", &self.anchor_issue_refs, &["#1", "#23"])?;
        validate_boundary(&self.boundary)?;
        validate_guardrails(&self.guardrails)?;

        if self.milestone_mappings.is_empty() {
            return Err(anyhow!("milestoneMappings must not be empty"));
        }
        if self.acceptance_audits.is_empty() {
            return Err(anyhow!("acceptanceAudits must not be empty"));
        }
        if self.trend_definitions.is_empty() {
            return Err(anyhow!("trendDefinitions must not be empty"));
        }

        let trend_ids: BTreeSet<_> = self
            .trend_definitions
            .iter()
            .map(|trend| trend.trend_id.as_str())
            .collect();
        let milestone_ids: BTreeSet<_> = self
            .milestone_mappings
            .iter()
            .map(|mapping| mapping.milestone_id.as_str())
            .collect();

        for mapping in &self.milestone_mappings {
            mapping.validate(&self.loop_stages)?;
        }
        for audit in &self.acceptance_audits {
            audit.validate(&milestone_ids, &trend_ids)?;
        }
        for trend in &self.trend_definitions {
            trend.validate()?;
        }
        Ok(())
    }
}

impl SelfAuditMilestoneMapping {
    fn validate(&self, loop_stages: &[String]) -> Result<()> {
        require_id("milestoneId", &self.milestone_id)?;
        require_issue_ref("issueRef", &self.issue_ref)?;
        require_id("gateKind", &self.gate_kind)?;
        if !loop_stages.iter().any(|stage| stage == &self.loop_stage) {
            return Err(anyhow!("loopStage `{}` is not declared", self.loop_stage));
        }
        validate_refs("evidenceRefs", &self.evidence_refs, true)?;
        validate_refs("attributionRefs", &self.attribution_refs, true)?;
        validate_texts("failureSignalKinds", &self.failure_signal_kinds, true)?;
        if !self
            .attribution_refs
            .iter()
            .any(|reference| reference.contains("loop-coverage"))
        {
            return Err(anyhow!(
                "milestoneMappings attributionRefs must include loop-coverage attribution"
            ));
        }
        Ok(())
    }
}

impl SelfAuditAcceptanceAudit {
    fn validate(&self, milestone_ids: &BTreeSet<&str>, trend_ids: &BTreeSet<&str>) -> Result<()> {
        require_id("acceptance milestoneId", &self.milestone_id)?;
        if !milestone_ids.contains(self.milestone_id.as_str()) {
            return Err(anyhow!(
                "acceptance audit references undeclared milestoneId `{}`",
                self.milestone_id
            ));
        }
        require_ref("successCriterionRef", &self.success_criterion_ref)?;
        if !self.success_criterion_ref.starts_with("#1") {
            return Err(anyhow!(
                "successCriterionRef must point at #1 success criteria"
            ));
        }
        if self.evidence_predicates.is_empty() {
            return Err(anyhow!("evidencePredicates must not be empty"));
        }
        for predicate in &self.evidence_predicates {
            predicate.validate()?;
        }
        require_id("trendRef", &self.trend_ref)?;
        if !trend_ids.contains(self.trend_ref.as_str()) {
            return Err(anyhow!(
                "acceptance audit trendRef `{}` is undeclared",
                self.trend_ref
            ));
        }
        Ok(())
    }
}

impl SelfAuditEvidencePredicate {
    fn validate(&self) -> Result<()> {
        require_ref("predicate sourceRef", &self.source_ref)?;
        require_text("predicate path", &self.path)?;
        match self.operator.as_str() {
            "equals" | "exists" | "not-empty" | "contains" | "lte" | "gte" => {}
            other => return Err(anyhow!("unsupported evidence predicate operator `{other}`")),
        }
        require_text("predicate expectedValue", &self.expected_value)
    }
}

impl SelfAuditTrendDefinition {
    fn validate(&self) -> Result<()> {
        require_id("trendId", &self.trend_id)?;
        require_ref("baselineRef", &self.baseline_ref)?;
        require_ref("currentRef", &self.current_ref)?;
        validate_texts("regressionWhen", &self.regression_when, true)?;
        if !self
            .regression_when
            .iter()
            .any(|rule| rule.to_ascii_lowercase().contains("regress"))
        {
            return Err(anyhow!("trend regressionWhen must define regression"));
        }
        Ok(())
    }
}

fn validate_boundary(boundary: &str) -> Result<()> {
    let lower = boundary.to_ascii_lowercase();
    for required in [
        "loop-coverage attribution",
        "ledger.jsonl",
        "journal.md",
        "verdict",
        "no new verification engine",
        "no new data plane",
        "human ring 2",
    ] {
        if !lower.contains(required) {
            return Err(anyhow!("boundary must mention {required}"));
        }
    }
    Ok(())
}

fn validate_guardrails(guardrails: &[String]) -> Result<()> {
    validate_texts("guardrails", guardrails, true)?;
    let joined = guardrails.join("\n").to_ascii_lowercase();
    for required in [
        "no human input",
        "never auto-applied",
        "source-apply",
        "trust-gradient",
    ] {
        if !joined.contains(required) {
            return Err(anyhow!("guardrails must mention {required}"));
        }
    }
    Ok(())
}

fn require_contains_all(label: &str, values: &[String], required: &[&str]) -> Result<()> {
    validate_texts(label, values, true)?;
    let set: BTreeSet<_> = values.iter().map(String::as_str).collect();
    for item in required {
        if !set.contains(item) {
            return Err(anyhow!("{label} missing required value `{item}`"));
        }
    }
    Ok(())
}

fn validate_refs(label: &str, refs: &[String], non_empty: bool) -> Result<()> {
    if non_empty && refs.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    let mut seen = BTreeSet::new();
    for reference in refs {
        require_ref(label, reference)?;
        if !seen.insert(reference) {
            return Err(anyhow!("{label} contains duplicate ref `{reference}`"));
        }
    }
    Ok(())
}

fn validate_texts(label: &str, values: &[String], non_empty: bool) -> Result<()> {
    if non_empty && values.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    for value in values {
        require_text(label, value)?;
    }
    Ok(())
}

fn require_issue_ref(label: &str, value: &str) -> Result<()> {
    if value.starts_with('#') && value[1..].chars().all(|ch| ch.is_ascii_digit()) {
        Ok(())
    } else {
        Err(anyhow!(
            "{label} must be a local GitHub issue ref like #2028"
        ))
    }
}

fn require_ref(label: &str, value: &str) -> Result<()> {
    require_text(label, value)?;
    if value.starts_with('/')
        || value.contains("..")
        || value.contains('\\')
        || value.contains(';')
        || value.contains("&&")
        || value.contains('|')
    {
        return Err(anyhow!("{label} must be a safe local evidence ref"));
    }
    Ok(())
}

fn require_id(label: &str, value: &str) -> Result<()> {
    require_text(label, value)?;
    if value.len() <= 128
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        Ok(())
    } else {
        Err(anyhow!("{label} must be a bounded local id"))
    }
}

fn require_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{label} must not be empty"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_contracts_that_do_not_extend_loop_coverage() {
        let mut contract = fixture_contract();
        contract.extends_schema_version = "new-attribution-stack-v1".to_string();
        let error = contract.validate().expect_err("wrong extension rejected");
        assert!(error.to_string().contains("loop-coverage-attribution-v1"));
    }

    #[test]
    fn rejects_missing_acceptance_or_trend_links() {
        let mut contract = fixture_contract();
        contract.acceptance_audits[0].trend_ref = "missing-trend".to_string();
        let error = contract.validate().expect_err("missing trend rejected");
        assert!(error.to_string().contains("undeclared"));
    }

    fn fixture_contract() -> SelfAuditAttributionContract {
        SelfAuditAttributionContract {
            schema_version: SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION.to_string(),
            extends_schema_version: LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION.to_string(),
            title_id: "era-i-engine-builder-deckbuilder".to_string(),
            anchor_issue_refs: vec!["#1".to_string(), "#23".to_string()],
            evidence_pipeline: REQUIRED_PIPELINE_REFS.iter().map(|value| value.to_string()).collect(),
            loop_stages: REQUIRED_LOOP_STAGES.iter().map(|value| value.to_string()).collect(),
            milestone_mappings: vec![SelfAuditMilestoneMapping {
                milestone_id: "m68-real-title-run".to_string(),
                issue_ref: "#2025".to_string(),
                gate_kind: "four-gates".to_string(),
                loop_stage: "attribute".to_string(),
                evidence_refs: vec!["examples/real-title-dogfood-v1/run/verdict.json".to_string()],
                attribution_refs: vec!["examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json".to_string()],
                failure_signal_kinds: vec!["gate-fail".to_string()],
            }],
            acceptance_audits: vec![SelfAuditAcceptanceAudit {
                milestone_id: "m68-real-title-run".to_string(),
                success_criterion_ref: "#1:era-l:m68-real-title-run".to_string(),
                evidence_predicates: vec![SelfAuditEvidencePredicate {
                    source_ref: "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
                    path: "status".to_string(),
                    operator: "equals".to_string(),
                    expected_value: "pass".to_string(),
                }],
                status: SelfAuditAcceptanceStatus::Satisfied,
                trend_ref: "m68-verdict-regression".to_string(),
            }],
            trend_definitions: vec![SelfAuditTrendDefinition {
                trend_id: "m68-verdict-regression".to_string(),
                direction: SelfAuditTrendDirection::Unchanged,
                baseline_ref: "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
                current_ref: "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
                regression_when: vec!["regressed when current gate status drops below baseline pass".to_string()],
            }],
            boundary: "Extends loop-coverage attribution over ledger.jsonl, journal.md, verdict evidence; no new verification engine and no new data plane; human Ring 2 fun/taste and release go/no-go remain outside automation.".to_string(),
            guardrails: vec![
                "Autonomous path has no human input.".to_string(),
                "High-risk/source-affecting changes are never auto-applied.".to_string(),
                "Engine fixes route through source-apply and trust-gradient.".to_string(),
            ],
        }
    }
}
