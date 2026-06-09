//! Milestone acceptance meta-evaluation v1 (#2030 / Era L M69).
//!
//! This module evaluates the #2028 self-audit success-criteria predicates over
//! existing real-title dogfood evidence and emits per-milestone acceptance
//! verdict data. It intentionally mirrors the evaluator's declared-gate shape
//! (`declared-gate-and`) while staying read-only: no browser runner, no new
//! verifier, no persistent store, and no apply authority.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

use crate::{
    SelfAuditAcceptanceAudit, SelfAuditAcceptanceStatus, SelfAuditAttributionContract,
    SelfAuditEvidencePredicate, SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION,
};

pub const SELF_AUDIT_ACCEPTANCE_EVALUATOR_SCHEMA_VERSION: &str =
    "self-audit-acceptance-evaluator-v1";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfAuditEvidenceDocument {
    #[serde(rename = "sourceRef")]
    pub source_ref: String,
    pub document: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfAuditAcceptanceReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "contractSchemaVersion")]
    pub contract_schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "declaredGateOperator")]
    pub declared_gate_operator: String,
    pub status: SelfAuditAcceptanceStatus,
    #[serde(rename = "milestoneVerdicts")]
    pub milestone_verdicts: Vec<SelfAuditMilestoneAcceptanceVerdict>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfAuditMilestoneAcceptanceVerdict {
    #[serde(rename = "milestoneId")]
    pub milestone_id: String,
    #[serde(rename = "successCriterionRef")]
    pub success_criterion_ref: String,
    #[serde(rename = "trendRef")]
    pub trend_ref: String,
    #[serde(rename = "declaredGate")]
    pub declared_gate: String,
    pub status: SelfAuditAcceptanceStatus,
    #[serde(rename = "predicateResults")]
    pub predicate_results: Vec<SelfAuditPredicateResult>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfAuditPredicateResult {
    #[serde(rename = "sourceRef")]
    pub source_ref: String,
    pub path: String,
    pub operator: String,
    #[serde(rename = "expectedValue")]
    pub expected_value: String,
    #[serde(rename = "actualValue")]
    pub actual_value: Option<String>,
    pub passed: bool,
    pub reason: String,
}

pub fn evaluate_self_audit_acceptance(
    contract: &SelfAuditAttributionContract,
    evidence: &[SelfAuditEvidenceDocument],
) -> Result<SelfAuditAcceptanceReport> {
    contract.validate()?;
    validate_evidence_documents(evidence)?;
    let evidence_by_ref: BTreeMap<_, _> = evidence
        .iter()
        .map(|document| (document.source_ref.as_str(), &document.document))
        .collect();

    let mut milestone_verdicts = Vec::new();
    for audit in &contract.acceptance_audits {
        milestone_verdicts.push(evaluate_milestone_audit(audit, &evidence_by_ref));
    }
    let status = if milestone_verdicts
        .iter()
        .any(|verdict| verdict.status == SelfAuditAcceptanceStatus::Regressed)
    {
        SelfAuditAcceptanceStatus::Regressed
    } else if milestone_verdicts
        .iter()
        .any(|verdict| verdict.status == SelfAuditAcceptanceStatus::InsufficientEvidence)
    {
        SelfAuditAcceptanceStatus::InsufficientEvidence
    } else if milestone_verdicts
        .iter()
        .all(|verdict| verdict.status == SelfAuditAcceptanceStatus::Satisfied)
    {
        SelfAuditAcceptanceStatus::Satisfied
    } else {
        SelfAuditAcceptanceStatus::Unsatisfied
    };

    let evidence_refs = evidence
        .iter()
        .map(|document| document.source_ref.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    Ok(SelfAuditAcceptanceReport {
        schema_version: SELF_AUDIT_ACCEPTANCE_EVALUATOR_SCHEMA_VERSION.to_string(),
        contract_schema_version: SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION.to_string(),
        title_id: contract.title_id.clone(),
        declared_gate_operator: "declared-gate-and".to_string(),
        status,
        milestone_verdicts,
        evidence_refs,
        boundary: "Read-only self-audit meta-evaluation over existing verdict, journal.md, ledger.jsonl, and loop-coverage attribution evidence; no new verification engine and no new data plane; autonomous path requires no human input; source-apply plus trust-gradient keep high-risk/source-affecting changes never auto-applied; human Ring 2 fun/taste and release go/no-go remain outside automation.".to_string(),
    })
}

fn evaluate_milestone_audit(
    audit: &SelfAuditAcceptanceAudit,
    evidence_by_ref: &BTreeMap<&str, &Value>,
) -> SelfAuditMilestoneAcceptanceVerdict {
    let predicate_results: Vec<_> = audit
        .evidence_predicates
        .iter()
        .map(|predicate| evaluate_predicate(predicate, evidence_by_ref))
        .collect();
    let evidence_refs = predicate_results
        .iter()
        .map(|result| result.source_ref.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let missing = predicate_results
        .iter()
        .any(|result| result.actual_value.is_none());
    let failed = predicate_results.iter().any(|result| !result.passed);
    let status = if missing {
        SelfAuditAcceptanceStatus::InsufficientEvidence
    } else if failed && audit.status == SelfAuditAcceptanceStatus::Satisfied {
        SelfAuditAcceptanceStatus::Regressed
    } else if failed {
        SelfAuditAcceptanceStatus::Unsatisfied
    } else {
        SelfAuditAcceptanceStatus::Satisfied
    };
    let reason = match status {
        SelfAuditAcceptanceStatus::Satisfied => {
            "all success-criteria predicates passed against real evidence".to_string()
        }
        SelfAuditAcceptanceStatus::Regressed => {
            "one or more previously satisfied predicates regressed against real evidence"
                .to_string()
        }
        SelfAuditAcceptanceStatus::Unsatisfied => {
            "one or more success-criteria predicates failed".to_string()
        }
        SelfAuditAcceptanceStatus::InsufficientEvidence => {
            "one or more required evidence documents or paths are missing".to_string()
        }
    };

    SelfAuditMilestoneAcceptanceVerdict {
        milestone_id: audit.milestone_id.clone(),
        success_criterion_ref: audit.success_criterion_ref.clone(),
        trend_ref: audit.trend_ref.clone(),
        declared_gate: "self-audit-acceptance".to_string(),
        status,
        predicate_results,
        evidence_refs,
        reason,
    }
}

fn evaluate_predicate(
    predicate: &SelfAuditEvidencePredicate,
    evidence_by_ref: &BTreeMap<&str, &Value>,
) -> SelfAuditPredicateResult {
    let Some(document) = evidence_by_ref.get(predicate.source_ref.as_str()) else {
        return SelfAuditPredicateResult {
            source_ref: predicate.source_ref.clone(),
            path: predicate.path.clone(),
            operator: predicate.operator.clone(),
            expected_value: predicate.expected_value.clone(),
            actual_value: None,
            passed: false,
            reason: "missing evidence document".to_string(),
        };
    };
    let Some(value) = select_json_path(document, &predicate.path) else {
        return SelfAuditPredicateResult {
            source_ref: predicate.source_ref.clone(),
            path: predicate.path.clone(),
            operator: predicate.operator.clone(),
            expected_value: predicate.expected_value.clone(),
            actual_value: None,
            passed: false,
            reason: "missing evidence path".to_string(),
        };
    };
    let actual = normalize_value(value);
    let passed = predicate_matches(
        &predicate.operator,
        value,
        &predicate.expected_value,
        &actual,
    );
    SelfAuditPredicateResult {
        source_ref: predicate.source_ref.clone(),
        path: predicate.path.clone(),
        operator: predicate.operator.clone(),
        expected_value: predicate.expected_value.clone(),
        actual_value: Some(actual.clone()),
        passed,
        reason: if passed {
            "predicate passed".to_string()
        } else {
            format!(
                "expected {} {} but found {actual}",
                predicate.operator, predicate.expected_value
            )
        },
    }
}

fn select_json_path<'a>(document: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = document;
    for segment in path.split('.') {
        if segment.trim().is_empty() {
            return None;
        }
        current = match current {
            Value::Object(map) => map.get(segment)?,
            Value::Array(items) => {
                let index = segment.parse::<usize>().ok()?;
                items.get(index)?
            }
            _ => return None,
        };
    }
    Some(current)
}

fn predicate_matches(operator: &str, value: &Value, expected: &str, actual: &str) -> bool {
    match operator {
        "equals" => actual == expected,
        "exists" => true,
        "not-empty" => match value {
            Value::Null => false,
            Value::String(text) => !text.is_empty(),
            Value::Array(items) => !items.is_empty(),
            Value::Object(map) => !map.is_empty(),
            _ => true,
        },
        "contains" => actual.contains(expected),
        "lte" => numeric_value(value)
            .zip(expected.parse::<f64>().ok())
            .is_some_and(|(a, e)| a <= e),
        "gte" => numeric_value(value)
            .zip(expected.parse::<f64>().ok())
            .is_some_and(|(a, e)| a >= e),
        _ => false,
    }
}

fn normalize_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::Null => "null".to_string(),
        _ => serde_json::to_string(value).unwrap_or_else(|_| "<unserializable>".to_string()),
    }
}

fn numeric_value(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64(),
        Value::String(text) => text.parse::<f64>().ok(),
        _ => None,
    }
}

fn validate_evidence_documents(evidence: &[SelfAuditEvidenceDocument]) -> Result<()> {
    if evidence.is_empty() {
        return Err(anyhow!("self-audit acceptance evidence must not be empty"));
    }
    let mut seen = BTreeSet::new();
    for document in evidence {
        require_ref("sourceRef", &document.source_ref)?;
        if !seen.insert(document.source_ref.as_str()) {
            return Err(anyhow!(
                "self-audit acceptance evidence contains duplicate sourceRef `{}`",
                document.source_ref
            ));
        }
    }
    let joined = seen
        .iter()
        .copied()
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase();
    for required in ["ledger.jsonl", "journal.md", "verdict", "loop-coverage"] {
        if !joined.contains(required) {
            return Err(anyhow!(
                "self-audit acceptance evidence must reference existing {required} artifact"
            ));
        }
    }
    Ok(())
}

fn require_ref(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.starts_with('/')
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_existing_pipeline_refs_fail_closed() {
        let evidence = vec![SelfAuditEvidenceDocument {
            source_ref: "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
            document: serde_json::json!({"status":"pass"}),
        }];
        let error = validate_evidence_documents(&evidence).expect_err("missing refs rejected");
        assert!(error.to_string().contains("ledger.jsonl"));
    }

    #[test]
    fn predicate_evaluation_detects_regression() {
        let predicate = SelfAuditEvidencePredicate {
            source_ref: "evidence.json".to_string(),
            path: "hiddenFriction".to_string(),
            operator: "equals".to_string(),
            expected_value: "false".to_string(),
        };
        let mut evidence = BTreeMap::new();
        let value = serde_json::json!({"hiddenFriction": true});
        evidence.insert("evidence.json", &value);
        let result = evaluate_predicate(&predicate, &evidence);
        assert!(!result.passed);
        assert_eq!(result.actual_value.as_deref(), Some("true"));
    }
}
