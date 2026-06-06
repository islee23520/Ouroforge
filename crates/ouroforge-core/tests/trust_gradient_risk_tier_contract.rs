use ouroforge_core::trust_gradient_risk_tier::{
    classify_mutation_risk_tier, AutoApplyEligibility, MutationProposalDescriptor, RiskTier,
    TRUST_GRADIENT_HIGH_CONFIDENCE_THRESHOLD, TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture(name: &str) -> MutationProposalDescriptor {
    let path = workspace_path(&format!("examples/trust-gradient-v1/fixtures/{name}"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    MutationProposalDescriptor::from_json_str(&text).unwrap_or_else(|err| panic!("{name}: {err}"))
}

#[test]
fn low_risk_scene_only_is_the_only_auto_apply_eligible_tier() {
    let descriptor = fixture("risk-tier-low-eligible.json");
    let classification = classify_mutation_risk_tier(&descriptor).expect("classifies");
    assert_eq!(
        classification.schema_version,
        TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION
    );
    assert_eq!(classification.tier, RiskTier::Low);
    assert_eq!(
        classification.eligibility,
        AutoApplyEligibility::AutoApplyEligible
    );
    // Conservative wording boundary travels with every classification.
    assert!(classification.boundary.contains("eligibility only"));
    assert!(classification.boundary.contains("not quality"));
    assert!(classification.boundary.contains("no auto-apply"));
    assert!(classification.boundary.contains("read-only"));
}

#[test]
fn review_required_data_is_medium_manual_only() {
    let descriptor = fixture("risk-tier-medium-manifest.json");
    let classification = classify_mutation_risk_tier(&descriptor).expect("classifies");
    assert_eq!(classification.tier, RiskTier::Medium);
    assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
}

#[test]
fn source_affecting_is_high_manual_only() {
    let descriptor = fixture("risk-tier-high-source.json");
    let classification = classify_mutation_risk_tier(&descriptor).expect("classifies");
    assert_eq!(classification.tier, RiskTier::High);
    assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
    assert!(classification
        .reasons
        .iter()
        .any(|reason| reason.contains("source-affecting")));
}

#[test]
fn ambiguous_kind_is_high_manual_only() {
    let descriptor = fixture("risk-tier-high-ambiguous.json");
    let classification = classify_mutation_risk_tier(&descriptor).expect("classifies");
    assert_eq!(classification.tier, RiskTier::High);
    assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
}

#[test]
fn missing_confidence_resolves_conservatively_to_manual_only() {
    let descriptor = fixture("risk-tier-manual-missing-confidence.json");
    let classification = classify_mutation_risk_tier(&descriptor).expect("classifies");
    assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
    assert!(classification
        .reasons
        .iter()
        .any(|reason| reason.contains("confidence is missing")));
}

#[test]
fn gate_regression_resolves_to_manual_only() {
    let descriptor = fixture("risk-tier-manual-gate-regression.json");
    let classification = classify_mutation_risk_tier(&descriptor).expect("classifies");
    assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
    assert!(classification
        .reasons
        .iter()
        .any(|reason| reason.contains("four gates")));
}

#[test]
fn stale_refs_resolve_to_manual_only() {
    let descriptor = fixture("risk-tier-manual-stale-ref.json");
    let classification = classify_mutation_risk_tier(&descriptor).expect("classifies");
    assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
    assert!(classification
        .reasons
        .iter()
        .any(|reason| reason.contains("stale")));
}

#[test]
fn confidence_just_below_threshold_is_manual_only() {
    let mut descriptor = fixture("risk-tier-low-eligible.json");
    descriptor.confidence = Some(TRUST_GRADIENT_HIGH_CONFIDENCE_THRESHOLD - 0.01);
    let classification = classify_mutation_risk_tier(&descriptor).expect("classifies");
    assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
}

#[test]
fn unexpected_schema_version_is_rejected() {
    let descriptor = fixture("risk-tier-low-eligible.json");
    let mut json = serde_json::to_value(&descriptor).expect("serializes");
    json["schemaVersion"] = serde_json::Value::String("bogus".to_string());
    let text = serde_json::to_string(&json).expect("serializes");
    assert!(MutationProposalDescriptor::from_json_str(&text).is_err());
}

#[test]
fn unrecognized_mutation_kind_fails_closed_to_high_manual_only() {
    // A new or misspelled `mutationKind` must deserialize to the conservative
    // `Unknown` catch-all (`#[serde(other)]`) and classify as high/manual-only,
    // not fail to parse before it can be classified (#1477).
    let descriptor = fixture("risk-tier-low-eligible.json");
    let mut json = serde_json::to_value(&descriptor).expect("serializes");
    json["mutationKind"] = serde_json::Value::String("brand-new-kind".to_string());
    let text = serde_json::to_string(&json).expect("serializes");
    let parsed = MutationProposalDescriptor::from_json_str(&text)
        .expect("unrecognized mutationKind parses to the Unknown catch-all");
    let classification = classify_mutation_risk_tier(&parsed).expect("classifies");
    assert_eq!(classification.tier, RiskTier::High);
    assert_eq!(classification.eligibility, AutoApplyEligibility::ManualOnly);
}
