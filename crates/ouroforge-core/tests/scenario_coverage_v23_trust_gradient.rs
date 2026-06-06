//! Scenario Coverage v23 (#1483): drive the enumerated Trust Gradient regression
//! matrix through the PRODUCTION Rust classifier, bounded auto-apply decision, and
//! audit-log validation so a breaking change to any trusted Trust Gradient path
//! fails CI.
//!
//! The Node mirror (`examples/trust-gradient-v1/scenario-coverage-v23-trust-gradient.test.cjs`)
//! documents the same matrix for the browser-facing demo, but a Node
//! reimplementation cannot catch a Rust regression. This test reads the shared
//! `coverage-matrix.json` and exercises the real `classify_mutation_risk_tier`,
//! `decide_auto_apply`, and `AutoApplyAuditLog::validate` so the trusted guard is
//! Rust-owned.

use ouroforge_core::trust_gradient_audit::{
    AutoApplyAuditLog, TRUST_GRADIENT_AUDIT_SCHEMA_VERSION,
};
use ouroforge_core::trust_gradient_auto_apply::{
    decide_auto_apply, AutoApplyRequest, TRUST_GRADIENT_AUTO_APPLY_SCHEMA_VERSION,
};
use ouroforge_core::trust_gradient_risk_tier::{
    classify_mutation_risk_tier, MutationProposalDescriptor,
    TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION,
};
use serde_json::{json, Value};

fn matrix() -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/trust-gradient-v1/coverage-v23/coverage-matrix.json");
    let text = std::fs::read_to_string(&path).expect("coverage-matrix.json exists");
    serde_json::from_str(&text).expect("coverage-matrix.json parses")
}

#[test]
fn risk_tier_cases_drive_production_classifier() {
    let matrix = matrix();
    let cases = matrix["riskTierCases"]
        .as_array()
        .expect("riskTierCases is an array");
    assert!(!cases.is_empty(), "matrix enumerates risk-tier cases");
    for case in cases {
        let id = case["id"].as_str().unwrap_or("<no id>");
        // The matrix descriptor omits the schema version / proposal ref envelope;
        // inject them so the production deserializer accepts the trusted shape.
        let mut descriptor = case["descriptor"].clone();
        descriptor["schemaVersion"] = json!(TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION);
        descriptor["proposalRef"] = json!(format!("coverage-v23/{id}"));
        let parsed = MutationProposalDescriptor::from_json_str(&descriptor.to_string())
            .unwrap_or_else(|err| panic!("{id}: descriptor parses: {err}"));
        let classification = classify_mutation_risk_tier(&parsed)
            .unwrap_or_else(|err| panic!("{id}: classifies: {err}"));
        assert_eq!(
            serde_json::to_value(classification.tier).unwrap(),
            case["expectedTier"],
            "{id}: tier"
        );
        assert_eq!(
            serde_json::to_value(classification.eligibility).unwrap(),
            case["expectedEligibility"],
            "{id}: eligibility"
        );
    }
}

#[test]
fn auto_apply_cases_drive_production_decision() {
    let matrix = matrix();
    let mut cases = matrix["autoApplyCases"]
        .as_array()
        .expect("autoApplyCases is an array")
        .clone();
    // The autonomy-off backward-compatibility golden is its own object.
    cases.push(matrix["backwardCompatAutonomyOff"].clone());
    assert!(cases.len() > 1, "matrix enumerates auto-apply cases");
    for case in &cases {
        let id = case["id"].as_str().unwrap_or("<no id>");
        let mut request = case["request"].clone();
        request["schemaVersion"] = json!(TRUST_GRADIENT_AUTO_APPLY_SCHEMA_VERSION);
        request["proposalRef"] = json!(format!("coverage-v23/{id}"));
        let parsed = AutoApplyRequest::from_json_str(&request.to_string())
            .unwrap_or_else(|err| panic!("{id}: request parses: {err}"));
        let decision =
            decide_auto_apply(&parsed).unwrap_or_else(|err| panic!("{id}: decides: {err}"));
        assert_eq!(
            serde_json::to_value(decision.outcome).unwrap(),
            case["expectedOutcome"],
            "{id}: outcome"
        );
        if let Some(expected_rollback) = case["expectedRollback"].as_bool() {
            assert_eq!(
                decision.rollback_command.is_some(),
                expected_rollback,
                "{id}: rollback command presence"
            );
        }
    }
}

#[test]
fn audit_cases_drive_production_validation() {
    let matrix = matrix();
    let cases = matrix["auditCases"]
        .as_array()
        .expect("auditCases is an array");
    assert!(!cases.is_empty(), "matrix enumerates audit cases");
    for case in cases {
        let id = case["id"].as_str().unwrap_or("<no id>");
        let mut log = case["log"].clone();
        log["schemaVersion"] = json!(TRUST_GRADIENT_AUDIT_SCHEMA_VERSION);
        // Enrich each compact matrix entry with the fields the production audit
        // entry requires (proposalRef/tier/gates/budgetRemaining) without altering
        // the invariants under test (sequence contiguity, rollback intactness,
        // kill-switch reason).
        if let Some(entries) = log["entries"].as_array_mut() {
            for entry in entries.iter_mut() {
                let sequence = entry["sequence"].as_u64().unwrap_or(0);
                entry["proposalRef"] = json!(format!("coverage-v23/{id}/{sequence}"));
                entry["tier"] = json!("low");
                entry["gates"] = json!({
                    "mechanical": "pass",
                    "runtime": "pass",
                    "visual": "pass",
                    "semantic": "pass"
                });
                entry["budgetRemaining"] = json!(0);
            }
        }
        let parsed: AutoApplyAuditLog = serde_json::from_value(log)
            .unwrap_or_else(|err| panic!("{id}: audit log deserializes: {err}"));
        let expect_valid = case["expectValid"].as_bool().unwrap_or(false);
        assert_eq!(parsed.validate().is_ok(), expect_valid, "{id}: validity");
        if let Some(expect_halted) = case["expectHalted"].as_bool() {
            assert_eq!(
                parsed.is_autonomy_halted(),
                expect_halted,
                "{id}: autonomy halted"
            );
        }
    }
}
