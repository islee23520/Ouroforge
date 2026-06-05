use ouroforge_core::behavior_evidence::{
    validate_behavior_evidence_bundle, BehaviorEvidenceBundleArtifact,
    BehaviorEvidenceBundleStatus, BEHAVIOR_EVIDENCE_BUNDLE_SCHEMA_VERSION,
};

fn fixture_bundle() -> BehaviorEvidenceBundleArtifact {
    BehaviorEvidenceBundleArtifact::from_json_str(include_str!(
        "../../../examples/behavior-evidence-bundle-v1/behavior-evidence-bundle.complete.json"
    ))
    .expect("behavior evidence bundle fixture parses")
}

#[test]
fn behavior_evidence_bundle_round_trips_complete_lifecycle_fixture() {
    let bundle = fixture_bundle();

    assert_eq!(
        bundle.schema_version,
        BEHAVIOR_EVIDENCE_BUNDLE_SCHEMA_VERSION
    );
    assert_eq!(bundle.status, BehaviorEvidenceBundleStatus::Complete);
    assert_eq!(bundle.behavior_definition_refs.len(), 1);
    assert_eq!(bundle.runtime_event_refs.len(), 1);
    assert_eq!(bundle.scenario_outcome_refs.len(), 1);
    assert_eq!(bundle.draft_refs.len(), 1);
    assert_eq!(bundle.review_decision_refs.len(), 1);
    assert_eq!(bundle.apply_transaction_refs.len(), 1);
    assert_eq!(bundle.rollback_metadata_refs.len(), 1);
    assert_eq!(bundle.rerun_comparison_refs.len(), 1);
    assert_eq!(bundle.observed_failures.len(), 1);
    assert_eq!(bundle.next_step_hypotheses.len(), 1);

    let validation = validate_behavior_evidence_bundle(&bundle).expect("complete bundle passes");
    assert_eq!(validation.status, "passed");
    assert_eq!(validation.lifecycle_ref_count, 8);
    let value = serde_json::to_value(&bundle).expect("bundle serializes");
    assert!(value.get("applyCommand").is_none());
    assert!(value.get("scriptRuntime").is_none());
}

#[test]
fn behavior_evidence_bundle_supports_partial_blocked_and_stale_states() {
    let mut partial = fixture_bundle();
    partial.status = BehaviorEvidenceBundleStatus::Partial;
    partial.review_decision_refs.clear();
    partial.apply_transaction_refs.clear();
    partial.rollback_metadata_refs.clear();
    partial.rerun_comparison_refs.clear();
    validate_behavior_evidence_bundle(&partial).expect("partial lifecycle may omit later refs");

    let mut blocked = partial.clone();
    blocked.status = BehaviorEvidenceBundleStatus::Blocked;
    blocked.blocked_reasons = vec!["review decision evidence is missing".to_string()];
    validate_behavior_evidence_bundle(&blocked).expect("blocked bundle explains blocker");

    let mut stale = partial;
    stale.status = BehaviorEvidenceBundleStatus::Stale;
    stale.blocked_reasons = vec!["runtime event hash is stale".to_string()];
    validate_behavior_evidence_bundle(&stale).expect("stale bundle explains stale state");
}

#[test]
fn behavior_evidence_bundle_rejects_missing_refs_unsafe_paths_and_forbidden_runtime_claims() {
    let mut value = serde_json::to_value(fixture_bundle()).expect("fixture serializes");
    value["scriptRuntime"] = serde_json::json!("eval");
    let error = serde_json::from_value::<BehaviorEvidenceBundleArtifact>(value)
        .expect_err("unknown runtime authority fields must not parse");
    assert!(error.to_string().contains("unknown field"));

    let mut bundle = fixture_bundle();
    bundle.behavior_definition_refs.clear();
    bundle.linked_evidence.clear();
    bundle.runtime_event_refs[0].path = "../outside.json".to_string();
    bundle.guardrails = vec!["behavior lifecycle evidence".to_string()];
    bundle.next_step_hypotheses[0].summary = "try dynamic import based runtime".to_string();

    let validation = bundle.inspect();
    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("behaviorDefinitionRefs")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("runtimeEventRefs[0].path")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("linkedEvidence")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("guardrails")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("arbitrary executable scripting")));
}

#[test]
fn gameplay_logic_regression_v9_evidence_bundle_covers_gl10_14_2_lifecycle_refs() {
    let bundle = BehaviorEvidenceBundleArtifact::from_json_str(include_str!(
        "../../../examples/gameplay-logic-regression-v9/evidence/behavior-evidence-bundle.gl10.14.2.fixture.json"
    ))
    .expect("GL10.14.2 evidence bundle fixture parses");

    assert_eq!(
        bundle.bundle_id,
        "gameplay-logic-regression-v9-draft-apply-evidence"
    );
    assert_eq!(bundle.status, BehaviorEvidenceBundleStatus::Complete);
    assert_eq!(bundle.behavior_definition_refs.len(), 1);
    assert_eq!(bundle.runtime_event_refs.len(), 1);
    assert_eq!(bundle.scenario_outcome_refs.len(), 1);
    assert_eq!(bundle.draft_refs.len(), 1);
    assert_eq!(bundle.review_decision_refs.len(), 1);
    assert_eq!(bundle.apply_transaction_refs.len(), 1);
    assert_eq!(bundle.rollback_metadata_refs.len(), 1);
    assert_eq!(bundle.rerun_comparison_refs.len(), 1);

    let validation =
        validate_behavior_evidence_bundle(&bundle).expect("GL10.14.2 evidence bundle validates");
    assert_eq!(validation.status, "passed");
    assert_eq!(validation.lifecycle_ref_count, 8);
    assert!(bundle
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("no arbitrary script execution")));

    let stale = BehaviorEvidenceBundleArtifact::from_json_str(include_str!(
        "../../../examples/gameplay-logic-regression-v9/evidence/behavior-evidence-bundle.stale.fixture.json"
    ))
    .expect("GL10.14.2 stale evidence bundle fixture parses");
    assert_eq!(stale.status, BehaviorEvidenceBundleStatus::Stale);
    assert!(stale
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("stale")));
    validate_behavior_evidence_bundle(&stale).expect("stale bundle explains stale state");
}
